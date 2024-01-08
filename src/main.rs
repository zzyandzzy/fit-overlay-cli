use crate::echarts::Echarts;
use crate::fit_utils::{FilterDataArgs, FitRecord};
use crate::frame::NumberedFrame;
use clap::{Args, Parser, Subcommand};
use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::event::{FfmpegEvent, LogLevel};
use image::RgbaImage;
use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::io::{Read, Write};
use std::process::ChildStdin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};
use threadpool::ThreadPool;

mod echarts;
mod errors;
mod fit_utils;
mod frame;
mod interpolation;

thread_local! {
    static ECHARTS_INSTANCE: RefCell<Option<Echarts>> = RefCell::new(None);
}

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// generate video
    Gen(GenArgs),

    /// print ffmpeg logs from input ffmpeg args
    Ffmpeg(FFmpegArgs),
}

#[derive(Args, Debug)]
pub struct GenArgs {
    #[clap(flatten)]
    pub video: VideoArgs,

    /// echart js path
    #[arg(long, default_value = "./chart.js")]
    pub chart_js: Option<String>,

    /// set_option js path
    #[arg(long, default_value = "./set_option.js")]
    pub set_option_js: Option<String>,

    /// input fit path
    #[arg(short, long)]
    pub fit_path: String,

    /// input fit start timestamp
    #[arg(short, long)]
    pub start_timestamp: Option<u32>,

    /// input fit delay
    #[arg(long, allow_hyphen_values = true)]
    pub delay: Option<i32>,
}

#[derive(Args, Debug)]
pub struct FFmpegArgs {
    /// ffmpeg args
    #[arg(short, long, allow_hyphen_values = true)]
    pub args: String,
}

#[derive(Args, Debug)]
pub struct VideoArgs {
    /// output video fps
    #[arg(long, default_value_t = 10)]
    pub fps: u32,

    /// output video width
    #[arg(long, default_value_t = 1920)]
    pub width: u32,

    /// output video height
    #[arg(long, default_value_t = 1080)]
    pub height: u32,

    /// output video duration second
    #[arg(short, long, default_value_t = 10)]
    pub duration: u32,

    /// output video path
    #[arg(long)]
    pub output_path: Option<String>,

    /// output video codec, use fit-overlay-cli ffmpeg --args -encoders | grep h264
    #[arg(long, default_value = Some("h264"))]
    pub codec: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Gen(args) => {
            generate_video(args)?;
        }
        Commands::Ffmpeg(ffmpeg) => {
            ffmpeg_sidecar::download::auto_download().unwrap();
            print_to_screen(ffmpeg.args);
        }
    }
    Ok(())
}

fn generate_video(args: GenArgs) -> anyhow::Result<()> {
    // init args
    let fps = args.video.fps;
    let duration = args.video.duration;
    let total_frames = fps * duration;
    let width = args.video.width;
    let height = args.video.height;
    let output_path = args
        .video
        .output_path
        .unwrap_or_else(|| format!("./output_{}x{}_{}_{}s.mp4", width, height, fps, duration));
    let codec = args.video.codec.unwrap_or_default();
    let chart_js = args.chart_js.unwrap_or_default();
    let set_option_js = args.set_option_js.unwrap_or_default();
    // parse fit
    let fit_record_vec: Vec<FitRecord> = fit_utils::filter_data(FilterDataArgs {
        fit_path: args.fit_path,
        start_timestamp: args.start_timestamp,
        delay: args.delay,
        duration,
    })?;
    // init output video
    let mut output = FfmpegCommand::new()
        .args("-f rawvideo -pix_fmt rgba".split(' '))
        .size(width, height)
        .rate(fps as f32)
        .duration(duration.to_string())
        .input("-")
        .codec_video(codec)
        .overwrite()
        .output(&output_path)
        .print_command()
        .spawn()?;
    let stdin = output.take_stdin().unwrap();
    let heap_size = Arc::new(AtomicUsize::new(0));
    let heap_size1 = Arc::clone(&heap_size);
    // init echarts
    let chart_js = fs::read_to_string(chart_js).expect("echart js file not found.");
    let set_option_js = fs::read_to_string(set_option_js).expect("set option js file not found.");
    // replace chart_js
    let chart_js = echarts::replace_chart_js(chart_js, &fit_record_vec, width, height);
    let (sender, receiver) = std::sync::mpsc::channel();

    // start producer thread
    let producer_thread = thread::spawn(move || {
        producer(
            sender,
            &fit_record_vec,
            fps,
            chart_js,
            set_option_js,
            width,
            height,
            heap_size,
        );
    });
    // start consumer thread
    let consumer_thread = thread::spawn(move || {
        consumer(receiver, stdin, total_frames, heap_size1);
    });
    output.iter().unwrap().for_each(|e| match e {
        FfmpegEvent::Log(LogLevel::Error, e) => println!("Error: {}", e),
        FfmpegEvent::Progress(p) => {
            if p.frame != 0 {
                println!(
                    "Progress: {:.4}%, raw_log: {}",
                    (p.frame as f32 * 100.0) / total_frames as f32,
                    p.raw_log_message
                );
            }
        }
        _ => {}
    });
    // wait producer thread complete
    producer_thread.join().unwrap();
    // wait consumer thread complete
    consumer_thread.join().unwrap();
    println!("Export {} complete!", output_path);
    Ok(())
}

fn print_to_screen(args: String) {
    let mut buf = String::new();
    FfmpegCommand::new()
        .args(args.split(' '))
        .print_command()
        .spawn()
        .unwrap()
        .take_stdout()
        .unwrap()
        .read_to_string(&mut buf)
        .expect("Run command error");
    println!("{}", buf);
}

fn producer(
    sender: Sender<NumberedFrame>,
    fit_record_slice: &[FitRecord],
    fps: u32,
    chart_js: String,
    set_option_js: String,
    width: u32,
    height: u32,
    heap_size: Arc<AtomicUsize>,
) {
    let pool = ThreadPool::new(fps as usize);
    for (record_index, current_record) in fit_record_slice.iter().enumerate() {
        let next_record = fit_record_slice
            .get(record_index + 1)
            .map_or(current_record, |next| next);

        for i in 0..fps {
            let sender_clone = sender.clone();
            let heap_size_clone = Arc::clone(&heap_size);
            let frame_index = (record_index as u32 * fps) + i;
            let chart_js = chart_js.clone();
            let set_option_js = set_option_js.clone();
            let mut tmp_record = current_record.clone();
            let t = i as f64 / fps as f64;
            tmp_record.s = interpolation::lerp(current_record.s, next_record.s, t) as f32;
            tmp_record.p = interpolation::lerp(current_record.p, next_record.p, t) as u16;
            tmp_record.g = interpolation::lerp(current_record.g, next_record.g, t) as f32;
            tmp_record.d = interpolation::lerp(current_record.d, next_record.d, t) as f32;
            tmp_record.c = interpolation::lerp(current_record.c, next_record.c, t) as u8;
            tmp_record.h = interpolation::lerp(current_record.h, next_record.h, t) as u8;

            pool.execute(move || {
                ECHARTS_INSTANCE.with(|echarts_cell| {
                    let mut echarts = echarts_cell.borrow_mut();
                    if echarts.is_none() {
                        *echarts = Some(Echarts::new(chart_js, set_option_js, width, height));
                    }
                    let echarts_instance = echarts.as_mut().unwrap();
                    let image: RgbaImage = echarts_instance.render_format(tmp_record);
                    let num_frame = NumberedFrame {
                        frame_num: frame_index,
                        image,
                    };
                    sender_clone.send(num_frame).unwrap();
                    while heap_size_clone.load(Ordering::Relaxed) > fps as usize {
                        thread::sleep(Duration::from_millis(5));
                    }
                });
            });
        }
    }
    pool.join();
}

fn consumer(
    receiver: Receiver<NumberedFrame>,
    mut stdin: ChildStdin,
    total_frames: u32,
    heap_size: Arc<AtomicUsize>,
) {
    let mut heap = BinaryHeap::new();
    let mut current_frame_num = 0;
    loop {
        while let Ok(numbered_frame) = receiver.try_recv() {
            heap.push(numbered_frame);
            heap_size.store(heap.len(), Ordering::Relaxed);
        }

        if let Some(top) = heap.peek() {
            if top.frame_num == current_frame_num {
                let numbered_frame = heap.pop().unwrap();
                heap_size.store(heap.len(), Ordering::Relaxed);
                current_frame_num = current_frame_num + 1;
                let f = numbered_frame.image.into_raw();
                stdin
                    .write(&f)
                    .expect("Failed to write frame data to stdin");
                if numbered_frame.frame_num == total_frames - 1 {
                    stdin.flush().expect("Failed to flush frame data to stdin");
                    break;
                }
            } else {
                thread::sleep(Duration::from_millis(5));
            }
        }
    }
}

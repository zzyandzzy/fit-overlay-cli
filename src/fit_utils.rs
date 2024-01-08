use fit_rust::protocol::data_field::DataField;
use fit_rust::protocol::message_type::MessageType;
use fit_rust::protocol::FitMessage;
use fit_rust::Fit;
use serde::Serialize;
use std::fs;

#[derive(Clone, Debug, Serialize)]
pub struct FitRecord {
    /// lat
    pub la: f32,
    /// long
    pub lo: f32,
    /// alt
    pub a: f32,
    /// heart
    pub h: u8,
    /// cadence
    pub c: u8,
    /// distance
    pub d: f32,
    /// speed
    pub s: f32,
    /// power
    pub p: u16,
    /// grade
    pub g: f32,
    /// temperature
    pub te: i8,
    /// right_balance
    pub rb: u8,
    /// timestamp
    pub t: u32,
}

#[derive(Clone, Debug)]
pub struct FilterDataArgs {
    pub(crate) fit_path: String,
    pub(crate) start_timestamp: Option<u32>,
    pub(crate) delay: Option<i32>,
    pub(crate) duration: u32,
}

fn add_u32_i32(u: u32, i: i32) -> u32 {
    if i < 0 {
        u - i.abs() as u32
    } else {
        u + i as u32
    }
}

pub fn filter_data(args: FilterDataArgs) -> anyhow::Result<Vec<FitRecord>> {
    let fit_file = fs::read(args.fit_path).expect("fit file not found.");
    let fit = Fit::read(fit_file)?;
    let mut vec: Vec<Vec<DataField>> = Vec::new();
    let start_timestamp = args
        .start_timestamp
        .unwrap_or_else(|| get_first_record(&fit.data));
    let start_timestamp = match args.delay {
        None => start_timestamp,
        Some(delay) => add_u32_i32(start_timestamp, delay),
    };
    let duration = args.duration;
    for message in fit.data {
        match message {
            FitMessage::Definition(_) => {}
            FitMessage::Data(msg) => match msg.data.message_type {
                MessageType::Record => {
                    for item in &msg.data.values {
                        match item.field_num {
                            // 253 is start timestamp
                            253 => {
                                let timestamp: u32 = item.value.clone().into();
                                if start_timestamp <= 0 {
                                    vec.push(msg.data.values.clone());
                                } else {
                                    if timestamp >= start_timestamp
                                        && timestamp < start_timestamp + duration
                                    {
                                        vec.push(msg.data.values.clone());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
        }
    }
    let mut record_vec: Vec<FitRecord> = Vec::with_capacity(duration as usize);
    let mut pre_timestamp: Option<u32> = None;
    for (i, values) in vec.iter().enumerate() {
        let prev_values = match i {
            0 => None,
            _ => vec.get(i - 1),
        };
        let mut lat = 0_f32;
        let mut long = 0_f32;
        let mut alt = 0_u16;
        let mut heart = 0_u8;
        let mut cadence = 0_u8;
        let mut distance = 0_u32;
        let mut speed = 0_u16;
        let mut power = 0_u16;
        let mut temp = 0_i8;
        let mut right_balance = 0_u8;
        let mut timestamp = 0_u32;
        for item in values {
            let v = item.value.clone();
            match item.field_num {
                0 => lat = v.into(),
                1 => long = v.into(),
                2 => alt = v.into(),
                3 => heart = v.into(),
                4 => cadence = v.into(),
                5 => distance = v.into(),
                6 => speed = v.into(),
                7 => power = v.into(),
                13 => temp = v.into(),
                30 => right_balance = v.into(),
                253 => {
                    timestamp = v.into();
                    match pre_timestamp {
                        Some(pre) => {
                            if pre != timestamp - 1 {
                                match record_vec.get(i - 1) {
                                    None => {}
                                    Some(record) => {
                                        let mut pre_record = record.clone();
                                        pre_record.t = timestamp - 1;
                                        record_vec.push(pre_record);
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                    pre_timestamp = Some(timestamp);
                }
                _ => {}
            }
        }
        let alt = alt as f32 / 5.0 - 500.0;
        let grade = prev_values.map_or(0.0, |fields| {
            let (prev_alt, prev_distance) =
                fields.iter().fold((0_u16, 0_u32), |(alt, dist), field| {
                    let v = field.value.clone();
                    match field.field_num {
                        2 => (v.into(), dist),
                        5 => (alt, v.into()),
                        _ => (alt, dist),
                    }
                });
            let prev_alt = prev_alt as f32 / 5.0 - 500.0;
            let alt_change = alt - prev_alt;
            let dist_change = (distance - prev_distance) as f32;
            let dist_change = dist_change / 100.0;
            if dist_change != 0.0 {
                alt_change / dist_change
            } else {
                0.0
            }
        });
        let grade = grade * 100.0;
        let distance = distance as f32 / 100000.0;
        let speed = speed as f32 / 1000.0 * 3.6;
        let right_balance = right_balance & 0x7f;
        record_vec.push(FitRecord {
            la: lat,
            lo: long,
            a: alt,
            h: heart,
            c: cadence,
            d: distance,
            s: speed,
            p: power,
            g: grade,
            te: temp,
            rb: right_balance,
            t: timestamp,
        })
    }
    if record_vec.len() != duration as usize {
        let len = duration - record_vec.len() as u32;
        let last_record = record_vec.last().expect("Can pop empty vec!").clone();
        for _ in 0..len {
            record_vec.push(last_record.clone());
        }
    }
    Ok(record_vec)
}

fn get_first_record(fit_data: &Vec<FitMessage>) -> u32 {
    for message in fit_data {
        match message {
            FitMessage::Definition(_) => {}
            FitMessage::Data(msg) => match msg.data.message_type {
                MessageType::Record => {
                    for item in &msg.data.values {
                        match item.field_num {
                            253 => {
                                return item.value.clone().into();
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
        }
    }
    0
}

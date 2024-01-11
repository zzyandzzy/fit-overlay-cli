#![feature(test)]

extern crate test;

use fit_overlay_cli::echarts::Echarts;
use std::fs;
use test::Bencher;

#[bench]
fn bench_svg_to_png(b: &mut Bencher) {
    let width = 3840;
    let height = 2160;
    let svg_str = fs::read_to_string("tests/chart_1000.svg").unwrap();
    b.iter(|| {
        let mut echarts = Echarts::new("\"Ok\"".to_string(), String::new(), width, height);

        let _ = echarts.svg_to_png(&svg_str);
    })
}

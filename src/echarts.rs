use crate::fit_utils::FitRecord;
use charming::ImageRenderer;
use image::{ImageFormat, RgbaImage};

pub struct Echarts {
    render: ImageRenderer,
    set_option_js: String,
}

impl Echarts {
    pub fn new(chart_js: String, set_option_js: String, width: u32, height: u32) -> Self {
        let mut renderer = ImageRenderer::new(width, height);
        let _init = renderer
            .execute_script(chart_js)
            .expect("TODO: panic message");
        Self {
            render: renderer,
            set_option_js,
        }
    }

    pub fn render_format(&mut self, record: FitRecord) -> RgbaImage {
        let set_option_js = self
            .set_option_js
            .replace("${long}", &format!("{}", record.lo))
            .replace("${lat}", &format!("{}", record.la))
            .replace("${alt}", &format!("{}", record.a))
            .replace("${heart}", &format!("{}", record.h))
            .replace("${cadence}", &format!("{}", record.c))
            .replace("${speed}", &format!("{}", record.s))
            .replace("${power}", &format!("{}", record.p))
            .replace("${grade}", &format!("{}", record.g))
            .replace("${temperature}", &format!("{}", record.te))
            .replace("${right_balance}", &format!("{}", record.rb))
            .replace("${timestamp}", &format!("{}", record.t));

        self.render
            .render_format_script(ImageFormat::Png, set_option_js)
            .expect("TODO: panic message")
    }
}

pub fn replace_chart_js(
    chart_js: String,
    fit_record_slice: &[FitRecord],
    width: u32,
    height: u32,
) -> String {
    chart_js
        .replace(
            "${all_record}",
            &serde_json::to_string(fit_record_slice).unwrap(),
        )
        .replace("${width}", &width.to_string())
        .replace("${height}", &height.to_string())
}

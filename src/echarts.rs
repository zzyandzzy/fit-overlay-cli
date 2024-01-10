use crate::fit_utils::FitRecord;
use charming::ImageRenderer;
use image::{ImageFormat, RgbaImage};
use std::collections::HashMap;

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
        let replacements = HashMap::from([
            ("{long}", record.lo.to_string()),
            ("{lat}", record.la.to_string()),
            ("{alt}", record.a.to_string()),
            ("{heart}", record.h.to_string()),
            ("{cadence}", record.c.to_string()),
            ("{speed}", record.s.to_string()),
            ("{power}", record.p.to_string()),
            ("{grade}", record.g.to_string()),
            ("{temperature}", record.te.to_string()),
            ("{right_balance}", record.rb.to_string()),
            ("{timestamp}", record.t.to_string()),
        ]);

        let set_option_js = replacements
            .iter()
            .fold(self.set_option_js.clone(), |acc, (key, value)| {
                acc.replace(key, value)
            });

        match self
            .render
            .render_format_script(ImageFormat::Png, set_option_js)
        {
            Ok(image) => image,
            Err(e) => {
                panic!("Echart err: {:?}", e);
            }
        }
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
            "{all_record}",
            &serde_json::to_string(fit_record_slice).unwrap(),
        )
        .replace("{width}", &width.to_string())
        .replace("{height}", &height.to_string())
}

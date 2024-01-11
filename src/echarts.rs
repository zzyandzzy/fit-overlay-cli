use crate::fit_utils::FitRecord;
use charming::ImageRenderer;
use resvg::tiny_skia;
use usvg::{fontdb, TreeParsing, TreeTextToPath};

pub struct Echarts {
    render: ImageRenderer,
    set_option_js: String,
    fontdb: fontdb::Database,
    opt: usvg::Options,
}

impl Echarts {
    pub fn new(chart_js: String, set_option_js: String, width: u32, height: u32) -> Self {
        let mut renderer = ImageRenderer::new(width, height);
        let _init = renderer
            .execute_script(chart_js)
            .expect("Echart new error.");

        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        let opt = usvg::Options::default();
        Self {
            render: renderer,
            set_option_js,
            fontdb,
            opt,
        }
    }

    pub fn render_format(&mut self, record: FitRecord) -> Vec<u8> {
        let svg = self.render_format_str(record);
        self.svg_to_png(&svg)
    }

    pub fn render_format_str(&mut self, record: FitRecord) -> String {
        let set_option_js = self
            .set_option_js
            .replace("{long}", &format!("{}", record.lo))
            .replace("{lat}", &format!("{}", record.la))
            .replace("{alt}", &format!("{}", record.a))
            .replace("{heart}", &format!("{}", record.h))
            .replace("{cadence}", &format!("{}", record.c))
            .replace("{distance}", &format!("{}", record.d))
            .replace("{speed}", &format!("{}", record.s))
            .replace("{power}", &format!("{}", record.p))
            .replace("{grade}", &format!("{}", record.g))
            .replace("{temperature}", &format!("{}", record.te))
            .replace("{right_balance}", &format!("{}", record.rb))
            .replace("{timestamp}", &format!("{}", record.t));

        self.render
            .execute_script(set_option_js)
            .unwrap_or_else(|e| panic!("Echart render format str err: {:?}", e))
    }

    fn svg_to_png(&mut self, svg_str: &str) -> Vec<u8> {
        let rtree = {
            let mut tree = usvg::Tree::from_str(svg_str, &self.opt)
                .unwrap_or_else(|e| panic!("Echart init tree error: {e}"));
            tree.convert_text(&self.fontdb);
            resvg::Tree::from_usvg(&tree)
        };

        let pixmap_size = rtree.size.to_int_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .expect("Echart init pixmap error.");
        rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
        // RgbaImage::from_vec(pixmap.width(), pixmap.height(), pixmap.take())
        pixmap.take()
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

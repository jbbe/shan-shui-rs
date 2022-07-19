use shan_shui::Painting;
// extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init(seed: i32) -> *mut Painting {
    console_error_panic_hook::set_once();

    let p = Box::new(Painting::new(seed as f64));
    Box::into_raw(p)
}

#[wasm_bindgen]
pub fn update(p: *mut Painting, x_min: f64, x_max: f64) -> String {
    console_error_panic_hook::set_once();
    log("rust::update!!!");
    let x_face = unsafe { &mut *p };
    log("rust::Unboxed!!");
    let ret = x_face.update(x_min, x_max);
    log("rust::update complete");
    ret
}

#[wasm_bindgen]
pub fn preload(p: *mut Painting, x_min: f64, x_max: f64) {
    let painting = unsafe { &mut *p };
    painting.preload(x_min, x_max);
}

#[wasm_bindgen]
pub fn render(p: *mut Painting, x_min: f64, x_max: f64) -> String {
    let painting = unsafe { &mut *p };
    painting.chunk_render(x_min, x_max)
}

#[wasm_bindgen]
pub fn dispose(_p: *mut Painting) {
    // p.dispose();
}

#[wasm_bindgen]
pub fn draw_background(seed: f64) -> String {
    console_error_panic_hook::set_once();
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("bg-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    log("rust::draw_background canvas grabbed");
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let mut noise = shan_shui::Noise::new(seed);
    // let perlins = noise.perlins();
    // log(&format!("Perlins {:?}", perlins).to_string());
    let resolution = 512.;
    let indexes = ((resolution / 2.) + 1.) as usize;
    let matrix: Vec<Vec<String>> = (0..indexes)
        .map(|i| {
            (0..indexes)
                .map(|j| {
                    let rand_decr = noise.rand() * 20.;
                    let c = (245. + noise.noise(i as f64 * 0.1, j as f64 * 0.1 as f64, 0.) * 10.) - rand_decr;
                    let r = (c) as u8 % 255;
                    let g = (c * 0.98) as u8;
                    let b = (c * 0.97) as u8;
                    let color = shan_shui::color(r, g, b);
                    color
                })
                .collect()
        })
        .collect();
    for i in 0..indexes {
        for j in 0..indexes {
            ctx.set_fill_style(&JsValue::from_str(&matrix[i][j]));
            ctx.fill_rect(i as f64, j as f64, 1., 1.);
            ctx.fill_rect(resolution - i as f64, j as f64, 1., 1.);
            ctx.fill_rect(i as f64, resolution - j as f64, 1., 1.);
            ctx.fill_rect(resolution - i as f64, resolution - j as f64, 1., 1.);
        }
    }
    log("Drew background returning url");
    // let img =
    canvas.to_data_url().unwrap()
    // let bg =
    // document.get_elements_by_tag_name("body")[0].
    // document.body()
    //     .unwrap()
    //     .style()
    //     .unwrap()
    //     .set_property("backgroundImage", format!("url({})", img));
    // ()
}

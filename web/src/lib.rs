// extern crate wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

// fn init_canv_ctx() -> CanvasRenderingContext2d {
//     let document = window().unwrap().document().unwrap();
//     let canvas= document
//         .get_element_by_id(&"main-canvas".to_string())
//         .unwrap()
//         .dyn_into::<HtmlCanvasElement>()
//         .map_err(|_| ())
//         .unwrap();
//     log("canvas grabbed");
//     canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<CanvasRenderingContext2d>()
//         .unwrap()
// }

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let svg_str = shan_shui::svg_string(200.);
    log("shan shui generated");
    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let _div = document.create_element("svg")
        .unwrap() ;
    let div = _div 
        .dyn_ref::<HtmlElement>() 
        .unwrap();
    div.set_inner_html(&svg_str);
    div.set_class_name("svg-container");
    body.append_child(&div);
    ()
}

#[wasm_bindgen]
pub fn draw_background(seed: f64) -> String {
    console_error_panic_hook::set_once();
    let document = window().unwrap().document().unwrap();
    let canvas= document
        .get_element_by_id("bg-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    log("canvas grabbed");
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
    for i in 0..indexes {
        for j in 0..indexes {
            let rand_decr = noise.rand() * 20.;
            let c = 
                (245.
                    + noise.noise(i as f64 * 0.1,j as f64 * 0.1 as f64, 0.) * 10.) 
                    - rand_decr;
            let r = (c ) as u8 % 255;
            let g  = (c * 0.98) as u8;
            let b = (c * 0.97) as u8;
            let color = shan_shui::color(r, g, b);
            ctx.set_fill_style(&JsValue::from_str(&color));
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

// #[wasm_bindgen(update)]
// fn update(curx: f64, windx) {
// 
// }


// #[wasm_bindgen]
// pub struct ShanShuiClient {
//     // canvas: web_sys::HtmlCanvasElement,
//     ctx: CanvasRenderingContext2d
//     // js_sys::,
// }
// // https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html
// impl ShanShuiClient {

//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Self {
//         console_error_panic_hook::set_once();
//         let ctx = init_canv_ctx();
//         Self {
//             // canvas,
//             ctx
//         }
//     }
//     pub fn render(&self) {
//         let svg = shan_shui::gen_svg();
//         // let ctx = self.canvas.get_context("2d").unwrap();
//         // ctx.append(svg);
//         // self.canvas.
//     }

//     pub fn draw_background(&self) {
//         // self.ctx.draw_rect(512, 512, )
//         self.ctx.begin_path();

//     }
// }


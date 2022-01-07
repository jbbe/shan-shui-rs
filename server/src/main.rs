#[macro_use] extern crate nickel;

use nickel::{Nickel};
fn main() {
    let mut server = Nickel::new();
    // let mut sessions = std::collections::HashMap::new();
    // let shan_shui = 
    server.utilize(nickel_cors::enable_cors);
    server.utilize(router! {
        get "**" => |_req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let mut painting = shan_shui::Painting::new();
            // let id = rand::random::<i32>();
            // sessions = sessions.insert(id, painting);
            // shan_shui::svg_string(false)
            painting.write_svg(256., 256.)
        }
    });
    server.listen("127.0.0.1:6767").unwrap();
}
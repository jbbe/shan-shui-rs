#[macro_use] extern crate nickel;

use nickel::{Nickel};
fn main() {
    let mut server = Nickel::new();

    server.utilize(nickel_cors::enable_cors);
    server.utilize(router! {
        get "**" => |_req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            shan_shui::svg_string(false)
        }
    });
    server.listen("127.0.0.1:6767").unwrap();
}
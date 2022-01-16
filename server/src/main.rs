#[macro_use] extern crate nickel;

use nickel::{Nickel};
use shan_shui::Painting;
fn main() {
    let mut server = Nickel::new();
    // let mut sessions = std::collections::HashMap::new();
    // let shan_shui = 
    server.utilize(nickel_cors::enable_cors);
    server.utilize(router! {
        get "/favicon" => |_req, mut res| {
            "".to_string();
        }
        get "/:seed" => |req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let seed_str = req.param("seed").unwrap();
            println!("seed str{}", seed_str);
            let seed = if seed_str == "" {
                std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("Tim went backwards")
                        .as_secs_f64() * 1000.
            }else {
                seed_str.parse::<f64>().unwrap()
            };

            println!("Generating from seed {}", seed );

            let mut painting = Painting::new(seed);
            painting.full_svg(3000., 800.)
            // let id = rand::random::<i32>();
            // sessions = sessions.insert(id, painting);
            // shan_shui::svg_string(false)
            // painting.write_svg(1024., 512.)
        }
        get "/mount/:seed" => |req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let seed_str = req.param("seed").unwrap();
            let seed = seed_str.parse::<f64>().unwrap();
            let mut painting = shan_shui::Painting::new(seed);
            // let id = rand::random::<i32>();
            // sessions = sessions.insert(id, painting);
            // shan_shui::svg_string(false)
            println!("route mount");
            painting.draw_mount()
        }
        get "/boat/:seed" => |req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let seed_str = req.param("seed").unwrap();
            let seed = seed_str.parse::<f64>().unwrap();
            let mut painting = shan_shui::Painting::new(seed);
            // let id = rand::random::<i32>();
            // sessions = sessions.insert(id, painting);
            // shan_shui::svg_string(false)
            println!("route boat");
            painting.draw_boat()
        }
    });
    server.listen("127.0.0.1:6767").unwrap();
}
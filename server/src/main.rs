#[macro_use]
extern crate nickel;

use std::env;
use nickel::Nickel;
use shan_shui::Painting;

fn parse_seed(seed_str: &str) -> f64 {
    if seed_str == "" {
        rand_seed()
    } else {
        seed_str.parse::<f64>().unwrap()
    }
}

fn rand_seed() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Tim went backwards")
        .as_secs_f64()
        * 1000.

}

fn main() {
    let mut server = Nickel::new();
    let args: Vec<String> = env::args().collect();
    
    let port = if args.len() < 2 {
        "6767"
    } else { 
        &args[1] 
    };
    server.utilize(nickel_cors::enable_cors);
    server.utilize(router! {
        get "/favicon" => |_req, mut res| {
            "".to_string();
        }
        get "/mount/:seed" => |req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let seed = parse_seed(req.param("seed").unwrap());
            let mut painting = shan_shui::Painting::new(seed);
            println!("route mount");
            painting.draw_mount()
        }
        get "/boat/:seed" => |req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let seed = parse_seed(req.param("seed").unwrap());
            let mut painting = shan_shui::Painting::new(seed);
            println!("route boat");
            painting.draw_boat()
        }
        get "/transmissiontower" => |_req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let seed = rand_seed();
            let mut painting = shan_shui::Painting::new(seed);
            println!("route transmission tower");
            painting.draw_transmission_tower()
        }
        get "/man" => |_req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let seed = rand_seed();
            let mut painting = shan_shui::Painting::new(seed);
            println!("route boat");
            painting.draw_man()
        }

        get "/:seed" => |req, mut res| {
            res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
            res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin X-Requested-With Content-Type Accept".to_vec()]);
            let seed = parse_seed(req.param("seed").unwrap());

            println!("Generating from seed {}", seed );

            let mut painting = Painting::new(seed);
            painting.full_svg(3000., 800.)
        }
    });
    server.listen(format!("127.0.0.1:{}", &port)).unwrap();
}

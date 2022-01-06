#[macro_use] extern crate rocket;
use hyper::header::{Headers, AccessControlAllowOrigin};
#[cfg(test)] mod tests;

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}


let mut headers = Headers::new();
headers.set(
    AccessControlAllowOrigin::Value("http://hyper.rs".to_owned())
);

// Note: without the `..` in `opt..`, we'd need to pass `opt.emoji`, `opt.name`.
//
// Try visiting:
//   http://127.0.0.1:8000/?emoji
//   http://127.0.0.1:8000/?name=Rocketeer
//   http://127.0.0.1:8000/?lang=ру
//   http://127.0.0.1:8000/?lang=ру&emoji
//   http://127.0.0.1:8000/?emoji&lang=en
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en
//   http://127.0.0.1:8000/?emoji&name=Rocketeer
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en&emoji
//   http://127.0.0.1:8000/?lang=ru&emoji&name=Rocketeer
#[get("/")]
fn get_svg() -> String {
    shan_shui::gen_svg().to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/shan-shui", routes![hello])
}

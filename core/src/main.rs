
// fn serve() {
//     let svg = gen_svg();

// }

fn main() {
    let svg_file = "out/image.svg";
    let out_file = "out/image15.png";
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
            .expect("Tim went backwards")
            .as_secs_f64();
    shan_shui::write_svg(svg_file, &shan_shui::gen_svg(seed, true));
    shan_shui::convert(svg_file, out_file);
}

// fn serve() {
//     let svg = gen_svg();

// }

fn main() {
    let svg_file = "out/image.svg";
    let out_file = "out/image15.png";
    shan_shui::write_svg(svg_file, &shan_shui::gen_svg(true));
    shan_shui::convert(svg_file, out_file);
}
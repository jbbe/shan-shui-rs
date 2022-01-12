pub fn to_png(in_file: &str, out_file: &str) {
    println!("Writing final output to {:?}", out_file);
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(&in_file)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    opt.fontdb.load_system_fonts();

    let svg_data = std::fs::read(in_file).unwrap();
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();

    let resolution = 512;

    let fit_to = usvg::FitTo::Zoom(1.);
    let mut pixmap = tiny_skia::Pixmap::new(resolution, resolution).unwrap();
    resvg::render(&rtree, fit_to, pixmap.as_mut()).unwrap();

    pixmap.save_png(out_file).unwrap();
}


fn main() {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
            .expect("Tim went backwards")
            .as_secs_f64();
    let svg_file = format!("out/seed-{}.svg", seed);
    let out_file = format!("out/seed-{}.png", seed);
    println!("Creating art with seed {}", seed);
    shan_shui::write_svg(svg_file, &shan_shui::gen_svg(seed, true));
    to_png(svg_file, out_file);
}
use super::super::*;

pub struct WaterArgs {
    height: f64,
    len: f64,
    clu: usize,
}
impl WaterArgs {
    pub fn default() -> Self {
        Self {
            height: 2.,
            len: 2.,
            clu: 10,
        }
    }
}

pub fn water(noise: &mut Noise, x_off: f64, y_off: f64, args: WaterArgs) -> String {
    let mut g = Group::new();
    let mut pt_list = Vec::new();
    let mut yk = 0.;
    let len_4 = args.len / 4.;
    for _ in 0..(args.clu) {
        pt_list.push(Vec::new());
        let xk = (noise.rand() - 0.5) * ((args.len) / 8.);
        yk += noise.rand() * 5.;
        let lk = len_4 + noise.rand() * len_4;
        let mut j = -lk;
        while j < lk {
            let idx = pt_list.len() - 1;
            pt_list[idx].push(Point {
                x: j + xk,
                y: f64::sin(j * 0.2) * args.height * noise.noise(j * 0.1, 0., 0.) - 20. + yk,
            });
            j = j + 5.; // 5 is called reso in source
        }
    }

    // println!("Drawing {} points of water",pt_list.len()); always 10
    for j in 1..(pt_list.len()) {
        let pts = pt_list[j]
            .iter()
            .map(|p| Point {
                x: p.x + x_off,
                y: p.y + y_off,
            })
            .collect();
        g.add(stroke(
                noise,
                &pts,
                StrokeArgs {
                    width: 1.,
                    // col: color_a(100, 100, 100, 0.3 + _rand() * 0.3),
                    col: blue(),
                    ..StrokeArgs::default("wawa".to_string())
                },
            )
        );
    }
    g.to_string()
}
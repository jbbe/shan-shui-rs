use super::super::*;
use core::f64::consts::PI;

pub struct RockArgs {
    pub height: f64,
    pub width: f64,
    pub tex: usize,
    pub sha: f64
}

impl RockArgs { 
    pub fn default() -> Self {
        Self {
            height: 80.,
            width: 100.,
            tex: 40,
            sha: 10. 
        }
    }
}
pub fn rock(noise: &mut Noise, x_off: f64, y_off: f64, seed: f64, args: RockArgs) -> String {
    let mut g = Group::new("rok".to_string());

    let reso = [10, 50];
    let resof = [10., 50.];
    let mut pt_list = Vec::with_capacity(reso[0]);

    for i in 0 ..reso[0] {
        let i_f = i as f64;
        pt_list.push(Vec::with_capacity(reso[1]));
        let mut ns_list = Vec::with_capacity(reso[1]);
        for j in 0..reso[1] {
            ns_list.push(noise.noise(i_f, j as f64 * 0.2, seed));
        }
        noise.loop_noise(&mut ns_list);

        let pt_last = pt_list.len() - 1;
        for j in 0..reso[1] {
            let a = (j as f64 / resof[1]) * PI * 2. - (PI / 2.);
            let l = // should give this a better name but i'm not sure entirely what it represents length?
                ((args.width * args.height) / 
                f64::sqrt(
                    f64::powi(args.height * f64::cos(a), 2) 
                        * f64::powi(args.width * f64::sin(a), 2)))
                * 0.7 + 0.3 * ns_list[j];

            let p = 1. - (i_f / resof[0]);

            let nx = f64::cos(a) * l * p;
            let mut ny = -1. * f64::sin(a) * l * p;

            if PI < a || a < 0. {
                ny *= 0.2;
            }
            ny += args.height * (i_f / resof[0]) * 0.2;

            pt_list[pt_last].push(Point { x: nx, y: ny });
       }
    }

    //white bg
    pt_list[0].push(Point { x: 0., y: 0. });
    g.add(poly(&pt_list[0], PolyArgs { 
        x_off, 
        y_off, 
        fil: white(),
        stroke: none_str(),
        ..PolyArgs::default(Some("whit bg rock".to_string()))
    }));
    pt_list[0].pop();
    // 0 0 is only added to the first point list for this poly
   
    // outline
    let outline_pts = pt_list[0]
        .iter()
        .map(|p| { Point { x: p.x + x_off, y: p.y + y_off } })
        .collect();
    g.add(stroke(noise, &outline_pts, StrokeArgs { 
        col: color_a(100, 100, 100, 0.3),
        noi: 1.,
        width: 3.,
        ..StrokeArgs::default("rockouln".to_string())
    } ));
    
    g.add(texture(noise, &pt_list, TextureArgs {
        x_off,
        y_off,
        density: args.tex,
        width: 3.,
        shading: args.sha,
        dis: |n| {
            if n.rand() > 0.5 {
                0.15 + 0.15 * n.rand()
            } else {
                0.85 - 0.15 * n.rand()
            }
        },
        ..TextureArgs::default()
    } )); // todo correct hese

    g.to_string()
}

use svg::node::element::{Group, Path, Rectangle};
use super::point::*;
use super::noise::Noise;

const PI: f64 = std::f64::consts::PI;

pub fn color(r: u8, b: u8, g: u8) -> String {
    format!("rgb({},{},{})", r, g, b)
}

pub fn red() -> String {
    color(255, 0, 0)
}

pub fn blue() -> String {
    color(0, 255, 0)
}

pub fn green() -> String {
    color(0, 0, 255)
}

pub fn color_a(r: u8, b: u8, g: u8, a: f64) -> String {
    format!("rgb({},{},{},{})", r, g, b, a)
}

pub struct PolyArgs {
    pub x_off: f64,
    pub y_off: f64,
    pub fil: String,
    pub stroke: String,
    pub width: f64,
}

impl PolyArgs {
    pub fn default() -> Self {
        Self {
            x_off: 0.,
            y_off: 0.,
            fil: color_a(0, 0, 0, 0.,),
            stroke: color_a(0, 0, 0, 0.,),
            width: 0.,
        }
    }
}

pub fn poly(p_list: &Vec<Point>, args: PolyArgs) -> Path {
    let mut data = svg::node::element::path::Data::new();
    // let mut path_data = Rc::new(usvg::PathData::new());
    let p_count = p_list.len();
    // let points = p_list.map
    // let p_data_vec = Vec::new
    for i in 0..p_count {
        let x = p_list[i].x + args.x_off;
        let y = p_list[i].y + args.y_off;
        if i == 0 {
            data = data.move_to((x, y));
        } else {
            data = data.line_by((x, y));
        }
    }
    data = data.close();
    // usvg::Path::()
    Path::new()
        .set("fill", "none")
        .set("fill", args.fil)
        .set("stroke", args.stroke)
        .set("stroke-width", args.width)
        .set("d", data)
}

pub struct StrokeArgs {
    pub x_off: f64,
    pub y_off: f64,
    pub width: f64,
    pub col: String,
    pub noi: f64,
    pub out: f64,
    pub fun: fn(x: f64) -> f64,
}

impl StrokeArgs {
    pub fn default() -> Self {
        Self {
            x_off: 0.,
            y_off: 0.,
            width: 2.,
            col: color_a(200, 200, 200, 0.9),
            noi: 0.5,
            out: 1.,
            fun: |x| f64::sin(x * PI),
        }
    }
}
// zip three arrays of vertices for stroke method
fn stroke_zip(
    pt_list: &Vec<Point>,
    vtx_list0: &mut Vec<Point>,
    vtx_list1: &mut Vec<Point>,
) -> Vec<Point> {
    let vtx_total = (pt_list.len() * 2) + 1;
    let mut vtx_list = Vec::with_capacity(vtx_total);
    vtx_list.push(pt_list[0]); // start point
                               // vtx_list0.reverse();
    vtx_list.append(vtx_list0);
    vtx_list.push(pt_list[pt_list.len() - 1]);
    vtx_list1.reverse();
    vtx_list.append(vtx_list1);
    vtx_list.push(pt_list[0]); // return to start

    vtx_list
}

pub fn stroke(noise: &mut Noise, pt_list: &Vec<Point>, args: StrokeArgs) -> Option<Path> {
    if pt_list.len() == 0 {
        return None;
    }
    let pt_len = pt_list.len();
    let mut vtx_list0: Vec<Point> = Vec::with_capacity(pt_len);
    let mut vtx_list1: Vec<Point> = Vec::with_capacity(pt_len);

    let noi = args.noi;
    let width = args.width;
    let fun = args.fun;

    let n0 = noise.rand() * 10.;
    let pt_lim = pt_len - 1;
    for i in 1..pt_lim {
        let wa = width * fun(i as f64 / (pt_len as f64));
        let wb = wa * (1. - noi) + wa * noi * noise.noise(i as f64 * 0.5, n0, 0.);
        let a1 = f64::atan2(
            pt_list[i].y - pt_list[i - 1].y,
            pt_list[i].x - pt_list[i - 1].x,
        );
        let a2 = f64::atan2(
            pt_list[i].y - pt_list[i + 1].y,
            pt_list[i].x - pt_list[i + 1].x,
        );
        let mut a = (a1 + a2) / 2.;
        if a < a2 {
            a = a + PI;
        }
        vtx_list0.push(Point {
            x: pt_list[i].x + wb * f64::cos(a),
            y: pt_list[i].y + wb * f64::sin(a),
        });
        vtx_list1.push(Point {
            x: pt_list[i].x - wb * f64::cos(a),
            y: pt_list[i].y - wb * f64::sin(a),
        });
    }

    // let mut vtx_list: Vec<Point> = !Vec(pt_list[0]).append(vtx_list1.reverse());
    let vtx_list = stroke_zip(&pt_list, &mut vtx_list0, &mut vtx_list1);

    Some(poly(
        &vtx_list, 
        PolyArgs { 
            x_off: args.x_off,
            y_off: args.y_off,
            fil: args.col.clone().to_string(),
            stroke: args.col.clone().to_string(),
            width: args.out ,
        }
    ))
}

pub struct BlobArgs {
    pub len: f64,
    pub width: f64,
    pub angle: f64,
    pub col: String,
    pub noi: f64,
    pub fun: fn(x: f64) -> f64,
}

impl BlobArgs {
    pub fn default() -> Self {
        Self {
            len: 20.,
            width: 5.,
            angle: 0.,
            col: color_a(200, 200, 200, 0.9),
            noi: 0.5,
            fun: |x| {
                if x <= 1. {
                    f64::powf(f64::sin(x * PI), 0.5)
                } else {
                    -1. * f64::powf(f64::sin(x + 1.), 0.5)
                }
            },
        }
    }
}

pub fn blob(noise: &mut Noise, x: f64, y: f64, args: BlobArgs) -> Path {
    let reso = 20.;
    let mut la_list = Vec::new();
    let i_lim = reso as usize + 1;
    for i in 0..i_lim {
        let p = (i as f64 / reso) * 2.;
        let xo = (args.len / 2.) - f64::abs(p - 1.) * args.len;
        let yo = ((args.fun)(p) * args.width) / 2.;
        let a = f64::atan2(yo, xo);
        let l = f64::sqrt((xo * xo) + (yo * yo));
        la_list.push(Point { x: l, y: a });
    }
    let mut ns_list = Vec::new();
    let n0 = noise.rand() * 10.;

    for i in 0..i_lim {
        ns_list.push(noise.noise(i as f64 * 0.05, n0, 0.));
    }

    // ns_list = 
    noise.loop_noise(&mut ns_list);
    
    let mut p_list = Vec::new();
    let la_len = la_list.len();
    for i in 0..la_len {
        let ns = ns_list[i] * args.noi + (1. - args.noi);
        let nx = x + f64::cos(la_list[i].y + args.angle) * la_list[i].x * ns;
        let ny = y + f64::sin(la_list[i].y + args.angle) * la_list[i].x * ns;
        p_list.push(Point { x: nx, y: ny });
    }

    poly(&p_list, PolyArgs {
        fil: args.col.clone(),
        stroke: args.col,
        width: 0.,
        ..PolyArgs::default()
    }
        // 0., 0., args.col.clone(), args.col, 0.
    )
}
#[allow(dead_code)]
pub struct TextureArgs {
    pub x_off: f64,
    pub y_off: f64,
    pub tex: usize,
    pub width: f64,
    pub len: f64,
    pub sha: f64,
    pub col: String,
    pub noi: fn(f64) -> f64,
    pub dis: fn(&mut Noise) -> f64,
}
impl TextureArgs {
    pub fn default() -> Self {
        Self {
            x_off: 0.,
            y_off: 0.,
            tex: 400,
            len: 0.2,
            width: 1.5,
            sha: 0.,
            col: color_a(200, 200, 200, 0.9),
            noi: |x| 30. / x,
            dis: |noise: &mut Noise| {
                if noise.rand() <= 0.5 {
                    (1. / 3.) * noise.rand()
                } else {
                    (2. / 3.) + (1. / 3.) * noise.rand() // ??? orignal make so sense beyond just being rand
                }
            },
        }
    }
}
pub fn texture(noise: &mut Noise, pt_list: &Vec<Vec<Point>>, args: TextureArgs) -> Group {
    let reso = [pt_list.len(), pt_list[0].len()];
    let reso_f = [pt_list.len() as f64, pt_list[0].len() as f64];
    let mut tex_list: Vec<Vec<Point>> = Vec::new();

    let dis = |noise: &mut Noise| {
        if noise.rand() <= 0.5 {
            (1. / 3.) * noise.rand()
        } else {
            (2. / 3.) + (1. / 3.) * noise.rand() // ??? orignal make so sense beyond just being rand
        }
    };
    for i in 0..args.tex {
        let mid = ((dis(noise)) * reso[1] as f64) as i32 | 0;
        let h_len = f64::floor(noise.rand() * (reso[1] as f64 * args.len)) as i32;
        let start = mid - h_len;
        let end = mid + h_len;
        let u_start = i32::min(i32::max(start, 0), reso[1] as i32) as usize;
        let u_end = i32::min(i32::max(end, 0), reso[1] as i32) as usize;

        let layer = (i as f64 / args.tex as f64) * reso_f[0] - 1.;
        let layer_floor = f64::floor(layer) as usize;
        let layer_ceil = f64::ceil(layer) as usize;
        tex_list.push(Vec::new());
        for j in u_start..u_end {
            let p = layer - f64::floor(layer);

            let x = pt_list[layer_floor][j].x * p + pt_list[layer_ceil][j].x * (1. - p);

            let y = pt_list[layer_floor][j].y * p + pt_list[layer_ceil][j].y * (1. - p);

            let ns0 = (args.noi)(layer + 1.) * noise.noise(x, j as f64 * 0.5, 0.) - 0.5;
            let ns1 = (args.noi)(layer + 1.) * noise.noise(x, j as f64 * 0.5, 0.) - 0.5;
            let t_last = tex_list.len() - 1;
            tex_list[t_last].push(Point {
                x: x + ns0,
                y: y + ns1,
            });
        }
    }

    let t_len = tex_list.len();
    let mut g = Group::new();
    // shade
    if args.sha != 0. {
        for j in 0..t_len {
            let pts = tex_list[j]
                .iter()
                .map(|p| Point {
                    x: p.x + args.x_off,
                    y: p.y + args.y_off,
                })
                .collect();
            let s = stroke(
                noise,
                &pts,
                StrokeArgs {
                    width: args.width,
                    col: color_a(100, 100, 100, 0.1),
                    ..StrokeArgs::default()
                },
            );
            if !s.is_none() {
                g = g.add(s.unwrap());
            }
        }
    }

    let u_sha = args.sha as usize;
    // texture
    for j in (u_sha..t_len).step_by(1 + u_sha) {
        let pts = tex_list[j]
            .iter()
            .map(|p| Point {
                x: p.x + args.x_off,
                y: p.y + args.y_off,
            })
            .collect();
        let s = stroke(
            noise,
            &pts,
            StrokeArgs {
                width: args.width,
                col: color_a(100, 100, 100, j as f64 / t_len as f64),
                ..StrokeArgs::default()
            },
        );
        if !s.is_none() {
            g = g.add(s.unwrap());
        }
    }
    g
}

pub fn rect(x: f64, y: f64, w: f64, h: f64, r: u8, g: u8, b: u8) -> Rectangle {
    let fill = color(r, g, b);
    Rectangle::new()
        .set("fill", fill)
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
}

/*
* Tests
*/
#[test]
fn test_stroke_zip() {
    let pt_list = vec![
        Point { x: 0., y: 0. },
        Point { x: 3., y: 3. },
        Point { x: 999., y: 999. },
    ];
    let mut vtx_list0 = vec![
        Point { x: 0.1, y: 0.1 },
        Point { x: 0.2, y: 0.2 },
        Point { x: 0.3, y: 0.3 },
    ];
    let mut vtx_list1 = vec![
        Point { x: 1.1, y: 1.1 },
        Point { x: 1.2, y: 1.2 },
        Point { x: 1.3, y: 1.3 },
    ];

    let correct = vec![
        Point { x: 0., y: 0. },
        Point { x: 0.1, y: 0.1 },
        Point { x: 0.2, y: 0.2 },
        Point { x: 0.3, y: 0.3 },
        Point { x: 999., y: 999. },
        Point { x: 1.3, y: 1.3 },
        Point { x: 1.2, y: 1.2 },
        Point { x: 1.1, y: 1.1 },
        Point { x: 0., y: 0. },
    ];
    let res = stroke_zip(&pt_list, &mut vtx_list0, &mut vtx_list1);
    assert_eq!(correct, res);
}
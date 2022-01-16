use std::collections::VecDeque;
use super::noise::Noise;
use super::point::*;
use core::f64::consts::PI;

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

pub fn white() -> String {
    "white".to_string()
}

pub fn black_a0() -> String {
    "rgba(0,0,0,0)".to_string()
}

pub fn none_str() -> String {
    "none".to_string()
}

pub fn color_a(r: u8, b: u8, g: u8, a: f64) -> String {
    format!("rgb({},{},{},{})", r, g, b, a)
}

pub struct Group {
    contents: Vec<String>,
    name: String,
}

impl Group {
    pub fn new(n: String) -> Self {
        Self {
            contents: vec![],
            name: n,
        }
    }

    pub fn add(&mut self, s: String) {
        self.contents.push(s)
    }

    pub fn to_string(&mut self) -> String {
        if self.contents.len() == 0 {
            "".to_string()
        } else {
            vec!["<g name='".to_string(),
                self.name.clone(),
                "'>".to_string(),
                self.contents.join(""),
                "</g>".to_string()
            ].join("")
        }
    }
}

pub struct PolyArgs {
    pub x_off: f64,
    pub y_off: f64,
    pub fil: String,
    pub stroke: String,
    pub width: f64,
    pub name: Option<String>
}

impl PolyArgs {
    pub fn default(name: Option<String>) -> Self {
        Self {
            x_off: 0.,
            y_off: 0.,
            fil: black_a0(),
            stroke: black_a0(),
            width: 0.,
            name
        }
    }
}

pub fn poly(p_list: &Vec<Point>, args: PolyArgs) -> String {
    let p_data: Vec<String> = p_list.iter().map(|p| {
        let x = p.x + args.x_off;
        let y = p.y + args.y_off;
        format!("{:.1},{:.1}", x, y)
    }).collect();
    let n = match args.name {
        Some(s) => s,
        None => "n".to_string(),
    };
    format!("<polyline name='{}' points='{}' style='fill:{};stroke:{};stroke-width:{}' />",
        n,
        p_data.join(" "),
        args.fil,
        args.stroke,
        args.width)
}

pub struct StrokeArgs {
    pub x_off: f64,
    pub y_off: f64,
    pub width: f64,
    pub col: String,
    pub noi: f64,
    pub out: f64,
    pub fun: fn(x: f64) -> f64,
    pub name: String,
}

impl StrokeArgs {
    pub fn default(name: String) -> Self {
        Self {
            x_off: 0.,
            y_off: 0.,
            width: 2.,
            col: color_a(200, 200, 200, 0.9),
            noi: 0.5,
            out: 1.,
            fun: |x| f64::sin(x * PI),
            name,
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

pub fn stroke(noise: &mut Noise, pt_list: &Vec<Point>, args: StrokeArgs) -> String {
    if pt_list.len() == 0 {
        return "".to_string();
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
        let a = if a2 > a1 {
                (a1 + a2) / 2. + PI
            } else { 
                (a1 + a2) / 2.
        } ;
        vtx_list0.push(Point {
            x: pt_list[i].x + wb * f64::cos(a),
            y: pt_list[i].y + wb * f64::sin(a),
        });
        vtx_list1.push(Point {
            x: pt_list[i].x - wb * f64::cos(a),
            y: pt_list[i].y - wb * f64::sin(a),
        });
    }

    let vtx_list = stroke_zip(&pt_list, &mut vtx_list0, &mut vtx_list1);

    poly(
        &vtx_list,
        PolyArgs {
            x_off: args.x_off,
            y_off: args.y_off,
            fil: args.col.clone().to_string(),
            stroke: args.col.clone().to_string(),
            width: args.out,
            name: Some(args.name),
        },
    )
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

pub fn blob(noise: &mut Noise, x: f64, y: f64, args: BlobArgs) -> String {
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

    poly(&p_list,
        PolyArgs {
            fil: args.col.clone(),
            stroke: args.col,
            width: 0.,
            ..PolyArgs::default(Some("blob".to_string()))
        }, // 0., 0., args.col.clone(), args.col, 0.
    )
}
/*
* creates and returns a new vec
* reso should not be 0
*/
pub fn div(p_list: &VecDeque<Point>, reso: f64) -> VecDeque<Point> {
    let tl = p_list.len() - 1 * (reso as usize);
    // let mut lx = 0.;
    // let mut ly = 0.;
    let mut r_list = VecDeque::new();
    r_list.reserve(p_list.len());

    for i in 0..tl {
        let last_i = f64::floor(i as f64 / reso) as usize;
        let next_i = f64::ceil(i as f64 / reso) as usize;
        let last_p = p_list[last_i];
        let next_p = p_list[next_i];
        let p = (i as f64 % reso) / reso;
        let nx = last_p.x * (1. - p) + next_p.x * p;
        let ny = last_p.y * (1. - p) + next_p.y * p;

        // let ang = f64::atan2(ny - ly, nx -lx);
        r_list.push_back(Point { x: nx, y: ny });
        // lx = nx;
        // ly = ny;
    }

    if p_list.len() > 0 {
        r_list.push_back(p_list[p_list.len() - 1]);
    }

    r_list
}

// #[allow(dead_code)]
pub struct TextureArgs {
    pub x_off: f64,
    pub y_off: f64,
    pub density: usize,
    pub width: f64,
    pub len: f64,
    pub shading: f64,
    pub col: fn(&mut Noise, f64) -> String,
    pub noi: fn(f64) -> f64,
    pub dis: fn(&mut Noise) -> f64,
}
impl TextureArgs {
    pub fn default() -> Self {
        Self {
            x_off: 0.,
            y_off: 0.,
            density: 400,
            len: 0.2,
            width: 1.5,
            shading: 0.,
            col: |n, _| {
                color_a(180, 180, 180, 0.3 + (n.rand() * 0.3))
            },
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

pub fn texture(noise: &mut Noise, layers: &Vec<Vec<Point>>, args: TextureArgs) -> String {
    let pt_cnt = layers[0].len();
    let pt_cntf = pt_cnt as f64;
    let col = args.col;
    let dis = args.dis;
    /*
     * tex_layers are not necessarily all the same length
     */
    let tex_layers: Vec<Vec<Point>> = (0..args.density).map(|i| {
        let mid = (dis(noise) * pt_cntf) as i32 | 0;
        let h_len = (noise.rand() * (pt_cntf * args.len)).floor() as i32;
        let start = mid - h_len;
        let end = mid + h_len;
        let u_start = i32::min(i32::max(start, 0), pt_cnt as i32) as usize;
        let u_end = i32::min(i32::max(end, 0), pt_cnt as i32) as usize;

        let mut layer = (i as f64 / args.density as f64) * (layers.len() as f64 - 1.);
        if (layer - -1.).abs() < 0.001 {
            println!("layer must not be -1 in Texture "); // args.noi cannot accept -1
            layer = -1.1;
        }

        let lay_floor = layer.floor() as usize;
        let lay_ceil = layer.ceil() as usize;

        (u_start..u_end).map(|j| {
            let deci = layer - layer.floor();
            let x = layers[lay_floor][j].x * deci + 
                         layers[lay_ceil][j].x * (1. - deci);
            let y = layers[lay_floor][j].y * deci +
                         layers[lay_ceil][j].y * (1. - deci);

            let noi_res = (args.noi)(layer + 1.);
            let ns0 =  noi_res * noise.noise(x, j as f64 * 0.5, 0.) - 0.5;
            let ns1 = noi_res * noise.noise(x, j as f64 * 0.5, 0.) - 0.5;
            Point { x: x + ns0, y: y + ns1 }
        }).collect()
    }).collect();

    let t_len = tex_layers.len();
    let mut g = Group::new("texture".to_string());
    // shade
    if args.shading != 0. {
        // no shading on first layer
        for j in 0..t_len {
            let pts = tex_layers[j].iter()
                .map(|p| Point {
                    x: p.x + args.x_off,
                    y: p.y + args.y_off,
                })
                .collect();
            g.add(stroke(noise,
                &pts,
                StrokeArgs {
                        width: args.shading,
                    // col: color_a(100, 100, 100, 0.1), //debug
                        col: "cyan".to_string(),
                        ..StrokeArgs::default(format!("tex-sha {}", j))
                },
            ));
            
        }
    }

    let u_sha = args.shading as usize;
    // texture
    // i think shading is shading density since it determines
    // how many layers to skip 
    for j in (u_sha..t_len).step_by(1 + u_sha) {
        let pts = tex_layers[j].iter()
            .map(|p| Point {
                x: p.x + args.x_off,
                y: p.y + args.y_off,
            }).collect();
        let args = StrokeArgs {
                width: args.width,
                col: col(noise, j as f64 / t_len as f64),
                ..StrokeArgs::default(format!("tex-tx {}", j))
        };
        g.add(stroke(noise, &pts, args));
    }
    g.to_string()
}

pub fn gr_zip(a: &VecDeque<Point>, b: &VecDeque<Point>) -> Vec<Point> {
    //   grlist1.reverse().concat(grlist2.concat([grlist1[0]]));
    // note that the reverse on grlist1 means that we end with 
    // the last point in grlist1
    let mut res  = Vec::with_capacity(a.len() + b.len());
    let a_len = a.len();
    for i in 0..a_len {
        res.push(a[a_len - i - 1].clone());
    }
    for b_i in b.iter() {
        res.push((*b_i).clone());
    }
    // res.append(&mut Vec::from(&b));
    res.push(a[a_len - 1].clone());
    res
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

#[test]
fn test_gr_zip() {
    //   var grlist = grlist1.reverse().concat(grlist2.concat([grlist1[0]]));
    let mut gr_list1 = VecDeque::from(vec![
        Point { x: 0., y: 0. },
        Point { x: 3., y: 3. },
        Point { x: 999., y: 999. },
    ]);
    let mut gr_list2 = VecDeque::from(vec![
        Point { x: 0.1, y: 0.1 },
        Point { x: 0.2, y: 0.2 },
        Point { x: 0.3, y: 0.3 },
    ]);

    let correct = vec![
        Point { x: 999., y: 999. },
        Point { x: 3., y: 3. },
        Point { x: 0., y: 0. },
        Point { x: 0.1, y: 0.1 },
        Point { x: 0.2, y: 0.2 },
        Point { x: 0.3, y: 0.3 },
        Point { x: 999., y: 999. },
    ];
    let res = gr_zip(&mut gr_list1, &mut gr_list2);

    assert_eq!(correct, res);
}
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Group;
use svg::node::element::Rectangle;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64
}

const EPSILON: f64 = 0.001;
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x - other.x < EPSILON && self.y - other.y < EPSILON
    }

    fn ne(&self, other: &Self) -> bool {
        self.x - other.x > EPSILON || self.y - other.y > EPSILON
    }
}
fn distance(p1: &Point, p2: &Point) -> f64 {
    f64::sqrt(f64::powf(p1.x - p2.x, 2.) + f64::powf(p1.x - p2.x, 2.))
}

// GLOBAL CONSTS
const PI : f64 = std::f64::consts::PI;

/*
* Perlin Noise
*/
const PERLIN_SIZE: usize = 4096;
const PERLIN_LAST: usize = 4094;
// const PERLIN_ARRAY_SIZE: usize = 4096;
pub struct Noise {
    perlin_octaves: usize,
    perlin_amp_falloff: f64,
    perlin: [f64; PERLIN_SIZE],
}

const PERLIN_YWRAPB: usize = 4;
const PERLIN_YWRAP: usize = 1 << PERLIN_YWRAPB;
const PERLIN_ZWRAPB: usize = 8;
const PERLIN_ZWRAP: usize = 1 << PERLIN_ZWRAPB;
impl Noise {

    fn scaled_cosine(i: f64) -> f64 {
        0.5 * (1. - f64::cos(i * PI))
    }

    pub fn new() -> Self {

        let mut perlin = [0.0; PERLIN_SIZE];

        for i in 0..PERLIN_SIZE {
        perlin[i] = _rand(); 
        }

        Self {
            perlin_octaves: 4,
            perlin_amp_falloff: 0.5,
            perlin
        }
    }
    
    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let _x = if x < 0.0 { 0. - x } else { x };
        let _y = if y < 0.0 { 0. - y } else { y };
        let _z = if z < 0.0 { 0. - z } else { z };

        let mut xi = _x as usize;
        // let yi = f64::floor(y);
        let mut yi = _y as u64;
        // let zi = f64::floor(z);
        let mut zi = _z as i64;
        let mut xf = _x - f64::floor(_x);
        let mut yf = _y - f64::floor(_y);
        let mut zf = _z - f64::floor(_z);
        let mut r = 0.;
        let mut ampl = 0.5;
        let mut n1: f64;
        let mut n2: f64;
        let mut n3: f64;

        let mut o = 0;
        while o < self.perlin_octaves {
            // let of = xi + (yi << PERLIN_YWRAPB) + (zi << PERLIN_ZWRAPB);
            let mut of = xi;
            let rxf = Noise::scaled_cosine(xf);
            let ryf = Noise::scaled_cosine(yf);
            n1 = self.perlin[of & PERLIN_LAST];
            n1 += rxf * (self.perlin[(of + 1) & PERLIN_LAST] - n1);
            n2 = self.perlin[(of + PERLIN_YWRAP) & PERLIN_LAST];
            n2 += rxf * (self.perlin[(of + PERLIN_YWRAP + 1) & PERLIN_LAST] - n2);
            n1 += ryf * (n2 - n1);
            of += PERLIN_ZWRAP;
            n2 = self.perlin[of & PERLIN_LAST];
            n2 += rxf * (self.perlin[(of + 1) & PERLIN_LAST] - n2);
            n3 = self.perlin[(of + PERLIN_YWRAP) & PERLIN_LAST];
            n3 += rxf * (self.perlin[(of + PERLIN_YWRAP + 1) & PERLIN_LAST] - n3);
            n2 += ryf * (n3 - n2);
            n1 += Noise::scaled_cosine(zf) * (n2 - n1);
            r += n1 * ampl;
            ampl *= self.perlin_amp_falloff;
            xi = xi << 1;
            xf = xf * 2.;
            yi = yi << 1;
            yf = yf * 2.;
            zi = zi << 1;
            zf = zf * 2.;
            if xf >= 1.0 {
              xi = xi + 1;
              xf = xf - 1.;
            }
            if yf >= 1.0 {
              yi = yi + 1;
              yf = yf - 1.;
            }
            if zf >= 1.0 {
              zi = zi + 1;
              zf = zf - 1.;
            }
            // increment for loop
            o = o + 1;
        }
        r
    }
}

/*
* Utils
*/

fn poly(p_list: &Vec<Point>,
        x_off: f64,
        y_off: f64, 
        fill: String,
        stroke: String,
        width: f64
    ) -> Path {
    
    let mut data = svg::node::element::path::Data::new();
    // let mut path_data = Rc::new(usvg::PathData::new());
    let p_count = p_list.len();
    // let points = p_list.map
    // let p_data_vec = Vec::new
    for i in 0..p_count {
        let x = p_list[i].x + x_off;
        let y = p_list[i].y + y_off;
        if i == 0 {
            data = data.move_to((x, y));
        } else {
            data = data.line_by((x, y));
        }
    }
    data = data.close();
    // usvg::Path::()
    Path::new().set("fill", "none")
            .set("fill", fill)
            .set("stroke", stroke)
            .set("stroke-width", width)
            .set("d", data)
}

struct StrokeArgs {
    x_off: f64,
        y_off: f64, 
        width: f64,
        col: String,
        noi: f64,
        out: f64,
        fun: fn(x: f64) -> f64
}

impl StrokeArgs {
    fn default() -> Self {
        Self {
            x_off: 0.,
            y_off: 0.,
            width: 2.,
            col: color_a(200, 200, 200, 0.9),
            noi: 0.5,
            out: 1.,
            fun: |x| f64::sin(x * PI)
        }
    }
}
// zip three arrays of vertices for stroke method
fn stroke_zip(pt_list: &Vec<Point>,
            vtx_list0: &mut Vec<Point>,
            vtx_list1: &mut Vec<Point>
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


fn stroke(noise: &Noise,
        pt_list: &Vec<Point>,
        args: StrokeArgs,
        ) -> Option<Path> {
    if pt_list.len() == 0 {
        return None;
    }
    let pt_len = pt_list.len();
    let mut vtx_list0: Vec<Point> = Vec::with_capacity(pt_len);
    let mut vtx_list1: Vec<Point> = Vec::with_capacity(pt_len);

    let noi = args.noi;
    let width = args.width;
    let fun = args.fun;

    let n0 = _rand() * 10.;
    let pt_lim = pt_len - 1;
    for i in 1..pt_lim {
        let wa = width * fun(i as f64 / (pt_len as f64));
        let wb = wa * (1. - noi) + wa * noi * noise.noise(i as f64 * 0.5, n0, 0.);
        let a1 = f64::atan2(pt_list[i].y - pt_list[i - 1].y,
            pt_list[i].x - pt_list[i - 1].x,
        );
        let a2 = f64::atan2(pt_list[i].y - pt_list[i + 1].y,
                                pt_list[i].x - pt_list[i + 1].x,
        );
        let mut a = (a1 + a2) / 2.;
        if a < a2 {
            a = a + PI;
        }
        vtx_list0.push(Point {
            x: pt_list[i].x + wb * f64::cos(a),
            y: pt_list[i].y + wb * f64::sin(a)
        });
        vtx_list1.push(Point {
            x: pt_list[i].x - wb * f64::cos(a),
            y: pt_list[i].y - wb * f64::sin(a)
        });
    }

    // let mut vtx_list: Vec<Point> = !Vec(pt_list[0]).append(vtx_list1.reverse());
    let vtx_list = stroke_zip(&pt_list, &mut vtx_list0, &mut vtx_list1);

    Some(poly(&vtx_list, args.x_off, args.y_off, args.col.clone().to_string(), args.col.clone(), args.out))
}

struct BlobArgs {
    len: f64,
    width: f64,
    angle: f64,
    col: String,
    noi: f64,
    fun: fn(x: f64) -> f64,
}

impl BlobArgs {
    fn default() -> Self {
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
            }
        }
    }
}

fn blob(noise: &Noise, x: f64, y: f64, args: BlobArgs) -> Path {
    let reso = 20.;
    let mut la_list = Vec::new();
    let i_lim = reso as usize + 1;
    for i in 0..i_lim {
        let p = (i as f64 / reso) * 2.;
        let xo = (args.len / 2.) - f64::abs(p - 1.) * args.len;
        let yo = ((args.fun)(p) * args.width) / 2.;
        let a = f64::atan2(yo, xo);
        let l = f64::sqrt((xo * xo) + (yo * yo));
        la_list.push(Point { x: l, y: a});
    }
    let mut ns_list = Vec::new();
    let n0 = _rand() * 10.;

    for i in 0..i_lim {
        ns_list.push(noise.noise(i as f64 * 0.05, n0, 0.));
    }

    // ns_list = 
    loop_noise(&mut ns_list);
    let mut p_list = Vec::new();
    let la_len = la_list.len();
    for i in 0..la_len {
        let ns = ns_list[i] * args.noi + (1. - args.noi);
        let nx = x + f64::cos(la_list[i].y + args.angle) * la_list[i].x * ns;
        let ny = y + f64::sin(la_list[i].y + args.angle) * la_list[i].x * ns;
        p_list.push(Point { x: nx, y: ny });
    }

    poly(&p_list, 
        0.,
        0.,
        args.col.clone(),
        args.col,
        0.)
}
#[allow(dead_code)]
struct TextureArgs {
    x_off: f64,
    y_off: f64,
    tex: usize,
    width: f64,
    len: f64,
    sha: f64,
    col: String,
    noi: fn(f64) -> f64,
    dis: fn() -> f64,
}
impl TextureArgs {
    fn default() -> Self {
        Self {
            x_off: 0.,
            y_off: 0.,
            tex: 400,
            len: 0.2,
            width: 1.5,
            sha: 0.,
            col: color_a(200, 200, 200, 0.9),
            noi: |x| { 30. / x},
            dis: || { 
                if _rand() <= 0.5 { 
                    (1. / 3.) * _rand()
                } else {
                    (2. / 3.) + (1. / 3.) * _rand() // ??? orignal make so sense beyond just being rand
                }
            }
        }
    }
}
fn texture(noise: &Noise, pt_list: &Vec<Vec<Point>>, args: TextureArgs) -> Group {
    let reso = [pt_list.len(), pt_list[0].len()];
    let reso_f = [pt_list.len() as f64, pt_list[0].len() as f64];
    let mut tex_list: Vec<Vec<Point>> = Vec::new();

    let dis = || { 
            if _rand() <= 0.5 { 
                (1. / 3.) * _rand()
            } else {
                (2. / 3.) + (1. / 3.) * _rand() // ??? orignal make so sense beyond just being rand
            }
    };
    for i in 0..args.tex {
        let mid = ((dis()) * reso[1] as f64) as i32 | 0;
        let h_len = f64::floor(_rand() * (reso[1] as f64 * args.len)) as i32;
        let start = mid - h_len;
        let  end = mid + h_len;
        let u_start = i32::min(i32::max(start, 0), reso[1] as i32) as usize;
        let u_end = i32::min(i32::max(end, 0), reso[1] as i32) as usize;

        let layer = (i as f64 / args.tex as f64) * reso_f[0] - 1.;
        let layer_floor = f64::floor(layer) as usize;
        let layer_ceil = f64::ceil(layer) as usize;
        tex_list.push(Vec::new());
        for j in u_start..u_end {
            let p = layer - f64::floor(layer);
            
            let x = 
                pt_list[layer_floor][j].x * p +
                pt_list[layer_ceil][j].x * (1. - p);

            let y = 
                pt_list[layer_floor][j].y * p +
                pt_list[layer_ceil][j].y * (1. - p);

            let ns0 = (args.noi)(layer + 1.) * noise.noise(x, j as f64 * 0.5, 0.) - 0.5;
            let ns1 = (args.noi)(layer + 1.) * noise.noise(x, j as f64 * 0.5, 0.) - 0.5;
            let t_last = tex_list.len() - 1;
            tex_list[t_last].push(Point { x: x + ns0, y: y + ns1 });
        }
    }

    let t_len = tex_list.len();
    let mut g = Group::new();
    // shade
    if args.sha != 0. {
        for j in 0..t_len {
            let pts = tex_list[j].iter()
                .map(|p| { Point { x: p.x + args.x_off, y: p.y + args.y_off} })
                .collect();
            let s = stroke(
                noise, 
                &pts,
                StrokeArgs {
                        width: args.width,
                        col: color_a(100, 100, 100, 0.1),
                        ..StrokeArgs::default()
                }
            );
            if !s.is_none() {
                g = g.add(s.unwrap());
            }
        }
    }

    let u_sha = args.sha as usize;
    // texture
    for j in (u_sha..t_len).step_by(1 + u_sha) {
            let pts = tex_list[j].iter()
                .map(|p| { Point { x: p.x + args.x_off, y: p.y + args.y_off} })
                .collect();
            let s = stroke(
                noise, 
                &pts,
                StrokeArgs {
                    width: args.width,
                    col: color_a(100, 100, 100, j as f64  / t_len as f64),
                    ..StrokeArgs::default()
                }
            );
            if ! s.is_none() {
                g = g.add(s.unwrap());
            }
    }
    g
}

fn rect(x: f64, y: f64, w: f64, h: f64, r: u8, g: u8, b: u8) -> Rectangle {
    let fill = color(r, g, b);
    Rectangle::new()
        .set("fill", fill)
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
}

pub fn color(r: u8, b: u8, g: u8) -> String {
    format!("rgb({},{},{})", r, g, b)
}

fn red() -> String {
    color(255, 0, 0)
}

fn blue() -> String {
    color( 0, 255,0)
}

fn green() -> String {
    color(0, 0,255)
}

fn color_a(r: u8, b: u8, g: u8, a: f64) -> String {
    format!("rgb({},{},{},{})", r, g, b, a)
}

fn map_val(val: f64, i_start: f64, i_stop: f64, o_start: f64, o_stop: f64) -> f64 {
    o_start + (o_stop - o_start) * (((val - i_start) * 1.0) / (i_stop - i_start))
}

fn loop_noise(ns_list: &mut Vec<f64>)  {
    let dif = ns_list[ns_list.len() - 1] - ns_list[0];
    let mut bds = [100., -100.];
    let i_lim = ns_list.len();
    for i in 0..i_lim {
        ns_list[i] = ns_list[i] 
            * (dif * (ns_list.len() as f64 - 1. - i as f64)) 
            / ((ns_list.len() - 1) as f64);
        if ns_list[i] < bds[0] {
            bds[0] = ns_list[i];
        }
        if ns_list[i] > bds[1] {
            bds[1] = ns_list[i];
        }
    }
    for i in 0..i_lim {
        ns_list[i] = map_val(ns_list[i], bds[0], bds[1], 0., 1.);
    }
    // ns_list
}


fn _rand() -> f64 {
    rand::random::<f64>()
}

fn norm_rand(little_m: f64, big_m: f64) -> f64{
    map_val(_rand(), 0., 1., little_m, big_m)
}

fn rand_bool() -> bool {
    if _rand() > 0.5 {
        true
    } else {
        false
    }
}

fn wt_rand(f: fn(f64) -> f64) -> f64 {
    let x = _rand();
    let y = _rand();
    if y < f(x) {
        x
    } else {
        wt_rand(f)
    }
}

fn rand_gauss() -> f64 {
    wt_rand(|x| { f64::powf(std::f64::consts::E, -24. * f64::powf(x - 0.5, 2.))})
        * 2. - 1.
}

// fn rand_choice<T>(arr: Vec<T>) -> T {
//     let idx = f64::floor(arr.len() as f64 * _rand()) as usize;
//     arr[idx]
// }
fn rand_choice_arr(arr: &[usize]) -> usize {
    let idx = f64::floor(arr.len() as f64 * _rand()) as usize;
    arr[idx]
}
fn rand_choice_arrf(arr: &[f64]) -> f64 {
    let idx = f64::floor(arr.len() as f64 * _rand()) as usize;
    arr[idx]
}

/*
* Trees
*/
#[allow(dead_code)]
struct TreeArgs {
    height: f64,
    width: f64,
    clu: f64,
    col: String,
    noi: f64
}

fn default_tree1_args() -> TreeArgs{
    TreeArgs {
        height: 50.,
        width: 3.,
        clu: 0.,
        col: "rgba(100,100,100,0.5)".to_string(),
        noi: 0.5,
    }
}

fn tree01(noise: &Noise, x: f64, y: f64, args: TreeArgs) -> Group {
    let reso = 10;
    let mut ns_list : Vec<Point> = Vec::new();
    for i in 0..reso {
        ns_list.push(Point {
            x: noise.noise(i as f64 * 0.5, 0., 0.),
            y: noise.noise(i as f64 * 0.5, 0.5, 0.)
        });
   }
   let mut g = Group::new();
//    let leaf_col = [100, 100, 100, 0.5];
   let mut line1 = Vec::new();
   let mut line2 = Vec::new();

   for i in 0..reso {
       let nx = x;
       let ny = y - (i as f64 * args.height) / reso as f64;
        if i >= reso / 4 {
            let j_limm = (reso - i) / 5;
            for _ in 0..j_limm {
                g = g.add(blob(
                    noise,
                    nx + (_rand()  - 0.5) * args.width * 1.2 * (reso - i) as f64,
                    ny + (_rand() - 0.5) * args.width,
                    BlobArgs {
                        len: _rand() * 20. * (reso - i) as f64 * 0.2 + 10.,
                        width: _rand() * 6. +3.,
                        angle: ((_rand() - 0.5) * PI) / 6.,
                        col: color_a(100, 100, 100, 
                            _rand() * 0.2 + 0.5 ),
                        ..BlobArgs::default()
                    }
                ))
            }
        }
        line1.push(Point { 
            x: nx + (ns_list[i].x - 0.5) * args.width - args.width / 2.,
            y: ny
        });
        line2.push(Point { 
            x: nx + (ns_list[i].y - 0.5) * args.width - args.width / 2.,
            y: ny
        });
   }
   g = g.add(poly(&line1, 0., 0., "none".to_string(), args.col.clone(), 1.5))
        .add(poly(&line2, 0., 0., "none".to_string(), args.col, 1.5));
   g
}

fn default_tree2_args() -> TreeArgs{
    TreeArgs {
        height: 16.,
        width: 8.,
        clu: 5.,
        col: "rgba(100,100,100,0.5)".to_string(),
        noi: 0.5,
    }
}

fn tree02(noise: &Noise, x: f64, y: f64, args: TreeArgs) -> Group {
   let clu = args.clu as u8;
   let mut g = Group::new();
   for _ in 0..clu {
       g = g.add(
           blob(noise,
            x + rand_gauss() * args.clu * 4.,
            y + rand_gauss() * args.clu * 4.,
            BlobArgs {
                angle: PI / 2.,
                // col: color_a(100, 100, 100, 0.8),
                col: args.col.to_string(),
                // default fun
                width: _rand() * (args.width * 0.75) + (args.width * 0.5),
                len: _rand() * (args.height * 0.75) + (args.height * 0.5),
                ..BlobArgs::default()
            })
        );
   }
   g
}
// struct Tre
const ONE_TWO_ARR: [usize; 2] = [1, 2];
fn mountain(noise: &Noise, x_off: f64, y_off: f64, seed: f64) -> Group {
    
    fn foot(noise: &Noise, 
            pt_list: &Vec<Vec<Point>>, 
            x_off: f64,
            y_off: f64) -> Group {
        let mut ft_list : Vec<Vec<Point>> = Vec::new();
        let span = 10;
        let mut ni = 0;
        let loop_limit = pt_list.len() - 2;
        for i in 0..loop_limit {
            if i == ni {
                ni = usize::min(ni + rand_choice_arr(&ONE_TWO_ARR), pt_list.len() - 1);
                ft_list.push(Vec::new());
                ft_list.push(Vec::new());
                let j_lim = usize::min(pt_list[i].len() / 8, 10);
                for j in 0..j_lim {
                    let idx1 = ft_list.len() - 2;
                    ft_list[idx1].push(Point {
                        x: pt_list[i][j].x + noise.noise(j as f64 * 0.1, i as f64, 0.) * 10.,
                        y: pt_list[i][j].y,
                    });
                    let idx2 = ft_list.len() - 1;
                    ft_list[idx2].push(Point {
                        x: pt_list[i][pt_list[i].len() -1 - j].x - noise.noise(j as f64* 0.1, i as f64, 0.) * 10.,
                        y: pt_list[i][pt_list[i].len() - 1 - j].y,
                    });
                }

                let idx1 = ft_list.len() - 2;
                let idx2 = ft_list.len() - 1;
                ft_list[idx1].reverse();
                ft_list[idx2].reverse();

                for j in 0..span {
                    let p = j as f64 / span as f64;
                    let x1 = pt_list[i][0].x * (1. - p) * pt_list[ni][0].x * p;
                    let mut y1 = pt_list[i][0].y * (1. - p) * pt_list[ni][0].y * p;

                    let pt_last = pt_list.len() - 1;
                    let x2 = pt_list[i][pt_last].x * (1. - p) +
                        pt_list[i][pt_last].x * p;
                    let mut y2 = pt_list[i][pt_last].y * (1. - p) +
                        pt_list[i][pt_last].y * p;

                    let vib = -1.7 * (p - 1.) * f64::powf(p, 0.2);

                    y1 = y1 + (vib * 5. + noise.noise(x_off * 0.05, i as f64, 0.));
                    y2 = y2 + (vib * 5. + noise.noise(x_off * 0.05, i as f64, 0.));


                    let idx1 = ft_list.len() - 2;
                    let idx2 = ft_list.len() - 1;
                    ft_list[idx1].push(Point { x: x1, y: y1});
                    ft_list[idx2].push(Point { x: x2, y: y2});
                }
            }
        }
        let mut g = Group::new();
        let f_len = ft_list.len();
        for i in 0..f_len {
            g = g.add(poly(&ft_list[i], 
                    x_off, 
                    y_off, 
                    "white".to_string(), 
                    "none".to_string(), 
                    0.));
        }
        for j in 0..f_len {
            // let f_list = ft_list[j];
            let stroke_pts = ft_list[j].clone()
                .into_iter()
                .map(|p| Point { x: p.x + x_off, y: p.y + y_off})
                .collect::<Vec<_>>();
            g = g.add(stroke(noise, 
                &stroke_pts, 
                        StrokeArgs {
                            // col: color_a(100, 100, 100, 0.1 +(_rand() * 0.1)),
                            col: green(),
                            width: 1.,
                            ..StrokeArgs::default() 
                        }).unwrap(),);
        }
        g
    }

    fn vegetate(noise: &Noise,
                pt_list: &Vec<Vec<Point>>,
                x_off: f64,
                y_off: f64,
                seed: f64,
                h: f64,
                tree_func: fn(noise: &Noise, x: f64, y: f64, x_off: f64, y_off: f64, h: f64) -> Group, 
                growth_rule: fn(noise: &Noise, pts: &Vec<Vec<Point>>, i: usize, j: usize, seed: f64, h: f64) -> bool,
                proof_rule: fn(pts: &Vec<Point>, y: f64) -> bool,
            ) -> Group {
        let mut veg_list: Vec<Point> = Vec::new();
        let mut g = Group::new();
        // might be error in original impl here he uses len straightI
        let i_lim = pt_list.len() - 1;
        for i in 0..i_lim {
            // same possibl error as above
            let j_lim = pt_list[i].len() - 1;
            for j in 0..j_lim {
                if growth_rule(noise, pt_list, i, j, seed, h) {
                    veg_list.push(Point { x: pt_list[i][j].x, y: pt_list[i][j].x });
                }
            }
        }
        if veg_list.len() > 1 {
            let veg_len = veg_list.len() - 1;
            for i in 0..veg_len {
                if proof_rule(&veg_list, i as f64) {
                    g = g.add(tree_func(noise, veg_list[i].x, veg_list[i].y, x_off, y_off, h))
                }
            }
        }
        g
    }


    let height = 100. + _rand() * 400.;
    let width = 400. + _rand() * 200.;
    // let tex = 200.;
    let veg = true;


    let mut pt_list : Vec<Vec<Point>> = Vec::new();
    let reso = [10, 50];
    let mut hoff = 0.;

    let mut group = Group::new();
    // let g = usvg::Node<usvg::NodeKind::Group>;
    for j in 0..reso[0] {
        hoff = hoff + ((_rand() * y_off) / 100.);
        pt_list.push(Vec::new());
        for i in 0..reso[1] {
            let x = (i as f64 / reso[1] as f64 - 0.5) * PI;
            let mut y = f64::cos(x);
            y = y * noise.noise(x + 10., j as f64 * 0.15, seed);
            let p = 1. - ((j as f64)  / (reso[0] as f64));
            let idx = pt_list.len() - 1; 
            pt_list[idx].push(Point {
                x: (x / PI) * width * p,
                y: -y * height * p * hoff,
            });
        }
    }
    // fn tree_func
    // Rim
    group = group.add(
    vegetate(noise,
            &pt_list,
            x_off,
            y_off,
            seed,
            height,
            |noise, x, y, x_off, y_off, _| { 
                tree01(noise, 
                    x + x_off, 
                    y+ y_off - 5., 
                    TreeArgs {
                        col: color_a(100, 100, 100, noise.noise(0.01 * x, 0.01 * y, 0.) * 0.5 * 0.3 + 0.5),
                        ..default_tree1_args()
                    }
                )},
            |noise, pt_list, i, j, seed, h| {
                let ns = noise.noise(j as f64 * 0.1, seed, 0.);
                i == 0 && ns * ns * ns < 0.1 
                    && f64::abs(pt_list[i][j].y) / h > 0.2
            },
            |_veg_list, _i| { true }));

    // White background
    let white_bg = poly(&pt_list[0], x_off, y_off, "white".to_string(), "none".to_string(), 0.);
    group = group.add(white_bg);

    // Outline
    let stroke_pt_list: Vec<Point> = pt_list[0].iter()
        .map(|p|  Point{ x: p.x + x_off, y: p.y + y_off} )
        .collect();
    if stroke_pt_list.len() > 1 {
        group = group.add(
        stroke(noise, 
                &stroke_pt_list,
                StrokeArgs {
                    col: color_a(100,100,100, 0.3), 
                    noi: 1.,
                    width: 3.,
                    ..StrokeArgs::default()
                }).unwrap());
    } else {
        println!("Stroke pt_list len < 1 {:?} ", stroke_pt_list,  );
    }

    // foot
    group = group.add(foot(noise, &pt_list, x_off, y_off));

    // texture
    let arr = [0., 0., 0., 0., 5.];
    let sha = rand_choice_arrf(&arr);
    let col = color(100, 0, 0);
    group = group.add(texture(noise, &pt_list, TextureArgs {
        x_off,
        y_off,
        tex: 200,
        sha,
        col,
        // args.col.clone(),
        ..TextureArgs::default()
    }
        
    ));

    // Top 
    group = group.add(vegetate( 
        noise,
        &pt_list,
        x_off,
        y_off,
        seed,
        height,
        |noise, x, y, x_off, y_off, _| {
            // todo should be tree 02
            tree02(noise,x + x_off, y + y_off,
                TreeArgs {
                    col: color_a(100, 100, 100, noise.noise(0.01 * x, 0.01 * y, 0.) * 0.5 * 0.3 + 0.5),
                    ..default_tree2_args()
                }
            )
        },
        |noise, pt_list, i, j, seed, h| {
            let ns = noise.noise(i as f64 * 0.1, j as f64 * 0.1, seed + 2.);
            ns * ns * ns < 0.1 && f64::abs(pt_list[i][j].y / h) > 0.5
        },
        |_veg_list, _i| { true },
    ));
    if veg {
        // middle 
        group = group.add(vegetate( 
            noise,
        &pt_list,
        x_off,
        y_off,
        seed,
        height,
        |noise, x, y, x_off, y_off, h| {
            // todo should be tree 02
            let mut ht = ((h + y) / h) * 70.;
            ht = ht * 0.3 + _rand() * ht * 0.7;
            tree01(noise,x + x_off, y + y_off,
                TreeArgs {
                    height: ht,
                    width: _rand() * 3. + 1.,
                    col: color_a(100, 100, 100, noise.noise(0.01 * x, 0.01 * y, 0.) * 0.5 * 0.3 + 0.3),
                    ..default_tree2_args()
                }
            )
        },
        |noise, pt_list, i, j, seed, h| {
            let ns = noise.noise(i as f64 * 0.2, j as f64 * 0.5, seed );
            j % 2 != 0 && 
                ns * ns * ns * ns < 0.012 && 
                f64::abs(pt_list[i][j].y / h) < 0.3
        },
        |_veg_list, _i| { true },
    ));
    }
    group
}

struct Man {
}
#[allow(dead_code)]
struct ManArgs {
    sca: f64,
    hat: fn(Point, Point, bool) -> Path,
    ite: fn(&Noise, Point, Point) -> Group,
    fli: bool,
    angle: [f64; 9],
    len: [f64; 9],
} 

impl ManArgs {
    fn default() -> Self {
        Self {
            angle: [
              0.,
              -PI / 2.,
              norm_rand(0., 0.),
              (PI / 4.) * _rand(),
              ((PI * 3.) / 4.) * _rand(),
              (PI * 3.) / 4.,
              -PI / 4.,
              (-PI * 3.) / 4. - (PI / 4.) * _rand(),
              -PI / 4.,
            ],
            ite: |_n, _p0, _p1| {Group::new()},
            hat: |p0, p1, f| {Man::hat02(p0, p1, f)},
            fli: true,
            sca: 0.5,
            len:  [0., 30., 20., 30., 30., 30., 30., 30., 30.],
        }
    }
}

struct StickArgs {
    fli: bool,
}
impl StickArgs {
    fn default() -> Self {
        Self {
            fli: false
        }
    }
}

impl Man {
    fn man(_x_off: f64, _y_off: f64, _args: ManArgs) -> Group {
        let  g = Group::new();
        g
    }

    fn tran_poly(p0: Point, p1: Point, pt_list: &Vec<Point>) -> Vec<Point> {
        let p_list: Vec<Point> = pt_list.iter()
            .map(|p| { Point { x: -p.x, y: p.y}})
            .collect();
        let ang = f64::atan2(p1.y - p0.y, p1.x -p0.x) - PI / 2.;
        let scl = distance(&p0, &p1);
        let origin = Point { x: 0., y: 0. };
        p_list.iter()
            .map(|p| {
                let d = distance(&p, &origin);
                let a = f64::atan2(p.y, p.x);
                Point {
                    x: p0.x + d * scl * f64::cos(ang + a),
                    y: p0.y + d * scl * f64::sin(ang + a)
                }
            })
            .collect()
    }

    fn hat02(p0: Point, p1: Point, fli: bool) -> Path {
        // let seed = _rand();

        let f = if fli { Man::flipper } else { |x| x };
        let arr = f(vec![
            Point { x: -0.3, y: 0.5 },
            Point { x: -1.1, y: 0.5 },
            Point { x: -1.2, y: 0.6 },
            Point { x: -1.1, y: 0.7 },
            Point { x: -0.3, y: 0.8 },
            Point { x: 0.3, y: 0.8 },
            Point { x: 1.0, y: 0.7 },
            Point { x: 1.3, y: 0.6 },
            Point { x: 1.2, y: 0.5 },
            Point { x: 0.3, y: 0.5 },
          ]);
        let pts = Man::tran_poly(p0, p1, &arr);
        
        poly(&pts,
          0.,
          0.,
          color_a(100, 100, 100, 0.8),
          color_a(100, 100, 100, 0.8),
          0.
        )
    }

    fn stick01(noise: &Noise, _p0: Point, _p2: Point, _args: StickArgs) -> Group {
        let seed = _rand();
        // let f = if args.fli { Man::flipper } else { |x: Vec<Point>| -> Vec<Point> {x} };

        let mut q_list1 = Vec::new();
        let l = 12;
        for i in 0..l {
            q_list1.push(Point {
                x: -noise.noise(i as f64 * 0.1, seed, 0.) * 0.1 * f64::sin((i as f64 / 1.) * PI) * 5.,
                y: i as f64 * 0.3
            });
        }
        Group::new()
    }

    fn flipper(p_list: Vec<Point>) -> Vec<Point> {
        p_list.iter()
            .map(|p| { Point { x: -p.x, y: p.y }})
            .collect()
    }

}

struct BoatArgs {
    len: f64,
    scale: f64,
    fli: bool
}

impl BoatArgs {
    fn default() -> Self {
        Self {
            len: 120.,
            scale: 1.,
            fli: false
        }
    }
}

fn boat01(x_off: f64, y_off: f64, args: BoatArgs) -> Group {
    let mut g = Group::new();
    let dir = if args.fli { -1. } else { 1. };
    g = g.add(Man::man(x_off + 20. * (args.scale) * dir, y_off, ManArgs {
        ite: |n,p0,p1 | {Man::stick01(n, p0, p1, StickArgs::default())},
        hat: Man::hat02,
        sca: 0.5 * (args.scale),
        fli: !(args.fli),
        len: [0., 30., 20., 30., 10., 30., 30., 30., 30.],
        ..ManArgs::default()
    }));
    // g = g.add()

    g
}


struct WaterArgs {
    height: f64,
    len: f64,
    clu: usize
}
impl WaterArgs {
    fn default() -> Self {
        Self {
            height: 2.,
            len: 2.,
            clu: 10,
        }
    }
}

fn water(noise: &Noise, x_off: f64, y_off: f64, args: WaterArgs) -> Group {
    let mut g = Group::new();
    let mut pt_list = Vec::new();
    let mut yk = 0.;
    for _ in 0..(args.clu) {
        pt_list.push(Vec::new());
        let xk = (_rand() - 0.5) * ((args.len) / 8.);
        yk = yk + _rand() * 5.;
        let lk = ((args.len) / 4.) + _rand() * (args.len / 4.);
        let mut j = -lk;
        while j < lk {
            let idx = pt_list.len() - 1;
            pt_list[idx].push(Point {
                x: j + xk,
                y: f64::sin(j * 0.2) * args.height * noise.noise(j* 0.1, 0.,0.) - 20. + yk
            });
            j = j+ 5.;
        }
    }

    for j in 1..(pt_list.len()) {
        let pts = pt_list[j].iter()
            .map(|p| { Point { x: p.x + x_off, y: p.y + y_off}})
            .collect();
        g = g.add(stroke(noise, 
                &pts, 
                StrokeArgs {
                    width: 1.,
                    // col: color_a(100, 100, 100, 0.3 + _rand() * 0.3),
                    col: blue(),
                    ..StrokeArgs::default()
                }
            ).unwrap());
    }
    g
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tag {
    Mount,
    DistMount,
    FlatMount,
    Boat,
    RedCircle,
    GreenCircle,
}

#[allow(dead_code)]
struct Plan {
    tag: Tag,
    x: f64,
    y: f64,
    h: f64 // what does this represent? usually generated by ns() func
}

impl Plan {
    fn new(tag: Tag, x: f64, y: f64, h: f64) -> Self {
        Self {
            tag,
            x,
            y,
            h
        }
    }
}

/*
* Mount planner
*/
const SAMP: f64 = 0.03;
fn mount_planner(app_state: &mut State, noise: &Noise, x_min: f64, x_max: f64) -> Vec<Plan> {
    fn loc_max(noise: &Noise, 
                x: f64, 
                y: f64, 
                f: fn(noise: &Noise, x: f64, y: f64) -> f64, 
                r: f64
            )-> bool {
        let z0 = f(noise, x, y);
        if z0 <= 0.3 {
            false
        } else {
            let min_x = x - r;
            let max_x = x + r;
            let min_y = y - r;
            let max_y = y + r;
            let mut i = min_x;
            while i < max_x {
                let mut j = min_y;
                while j < max_y {
                    if f(noise, i, j) > z0 {
                        return false
                    }
                    j = j + 1.;
                }
                i = i + 1.;
            }
            true
        }
    }
    fn chadd_mind(plans: &mut Vec<Plan>, plan: Plan, mind: f64) -> bool {
        // let len = reg.len();
        for k in 0..(plans.len()) {
            // we only add the new chunk if
            // the difference between the new plan's x
            // and any other plan's x is less than mind
            // which defaults to 10
            if f64::abs(plans[k].x - plan.x) < mind {
                return false
            }
        }
        println!("+");
        plans.push(plan);
        true
    }
    /*
    * returns whether plan was succesfully added.
    */
    fn chadd(plans: &mut Vec<Plan>, plan: Plan) -> bool {
        chadd_mind(plans, plan, 10.)
    }
    let mut plans : Vec<Plan> = Vec::new();

    let ns = |noise: &Noise, x: f64, _: f64| -> f64 { 
        f64::max(noise.noise(x * SAMP, 0., 0.) - 0.55, 0.) * 2.
    };

    // let nns = |x: f64, y: f64| {
    //     1. - noise.noise(x * SAMP, y, 0.)
    // };
    
    // let nnns = |x: f64, y: f64 | {
    //     f64::max(noise.noise(x * SAMP * 2., 2., 0.) - 0.55, 0.) * 2.
    // };

    let yr = |x| {
        noise.noise(x * 0.01, PI, 0.)
    };

    let x_step = 5.;
    let m_wid = 200.;
    // original does this index by index we do a single rezie
    // line 3757
    let new_len = f64::floor(x_max / x_step) as usize + 2;
    app_state.plan_mtx.resize_with(new_len, || { 0 });

    let mut i = x_min;
    while i < x_max {
        let mut j = 0.;
        while j < yr(i) * 480. {
            if loc_max(noise, i, j, ns, 2.) {
                let xof = i + 2. * (_rand() - 0.5) * 500.;
                let yof = j + 300.;
                let r: Plan = Plan::new(
                    Tag::Mount, 
                    xof,
                    yof,
                    ns(noise, i, j)
                );
                let res = chadd(&mut plans, r);
                if res {
                    let lower_lim = f64::floor((xof - m_wid) / x_step) as usize;
                    let upper_lim = f64::floor((xof + m_wid) / x_step) as usize;
                    for k in lower_lim..upper_lim {
                        if k < app_state.plan_mtx.len() {
                            // is this determining the crest of the mountains?
                            app_state.plan_mtx[k] += 1;
                        } else {
                            println!("!! k is out of bounds idxng into plan_mtx len: {:?} k {:?}",
                             app_state.plan_mtx.len(), 
                             k);
                             // The desired behavior here might be to increase the vec size to upper limit
                             break;
                        }
                    }
                } // for k
            } // if res

            j = j + 30.;
        } // while j
        if f64::abs(i) % 1000. < x_step - 1. {
            // distmount is only added when i < 4
            println!("adding distmount");
            let r = Plan::new(
                Tag::DistMount, 
                i, 
                280. - _rand() * 50.,
                ns(noise, i, j)
            );
            chadd(&mut plans, r);
        }

        i = i + x_step;
    }
    println!("Xmin {:?} xmax {:?}", x_min, x_max);

    let mut i = x_min;
    while i < f64::floor(x_max) {
        let idx = f64::floor(i / x_step) as usize;
        println!("Xmax {:?} i {:?} idx {:?} step {:?}", x_min, i, idx, x_step);
        if idx >= app_state.plan_mtx.len() - 1 {
            println!("!! idx {:?} out of bounds len: {:?}", idx, app_state.plan_mtx.len());
        } else if app_state.plan_mtx[idx] == 0 {
            if _rand() < 0.01 {
                let mut j = 0.;
                while j < (4. * _rand()) {
                    let r = Plan::new(
                        Tag::FlatMount,
                        i + 2. * (_rand() - 0.5) * 700.,
                        700. - j * 50.,
                        ns(noise, i, j)
                    );
                    chadd(&mut plans, r);
                    j = j + 1.;
                } // while j
            }
        } else {
            // (commented out in original )
                // r = tag: greencirc
                //chadd(r)
        }
        i = i + x_step;
    } // while i 

    let mut i = x_min;
    while i < x_max {
        if _rand() < 0.2 {
            let r = Plan::new(
                Tag::Boat, 
                i, 
                300. + _rand() * 390.,
                0.
            );
            chadd_mind(&mut plans, r, 400.);
        }
        i = i + x_step;
    }
    plans
}

struct State {
    plan_mtx: Vec<u32>,
    x_min: f64,
    x_max: f64,
    c_wid: f64
}

impl State {
    fn new() -> Self {
        Self {
            plan_mtx: Vec::new(),
            x_min: 0.,
            x_max: 0.,
            c_wid: 512.
        }
    }
}

// as opposed to creating and saving a 'chunk' like in the js version
// load chunk returns a group that contains the svg for this section
fn load_chunk(app_state: &mut State, 
            noise_generator: &Noise, 
            x_min: f64,
            x_max: f64
        ) -> Group {

    let mut g = Group::new();
    while x_max > app_state.x_max  - app_state.c_wid
        || x_min < app_state.x_min + app_state.c_wid {
        println!("Generating new chunk...", );
        // generate new chunk
        let plans: Vec<Plan>;
        if x_max > app_state.x_max - app_state.c_wid { 
            plans = mount_planner(app_state, noise_generator, app_state.x_max, app_state.x_max + app_state.c_wid);
            app_state.x_max = app_state.x_max + app_state.c_wid;
        } else {
            plans = mount_planner(app_state, noise_generator, app_state.x_min, app_state.x_min - app_state.c_wid);
            app_state.x_min = app_state.x_min - app_state.c_wid;
        }
        let len = plans.len();
        println!("Generated {:?} plans", len);
        for i in 0..len {
            let p = &plans[i];
            println!("create svg for chunk {:?} {:?} {:?}", p.tag, p.x, p.y);
            if p.tag == Tag::Mount {
                let svg_node = mountain(noise_generator, p.x, p.y, (i * 2) as f64);
                let w = water(noise_generator,
                        p.x, 
                        p.y,
                        WaterArgs::default()
                    );
                g = g.add(svg_node)
                    .add(w);
            } else if p.tag == Tag::FlatMount {
                // g = g.add()
            } else if p.tag == Tag::DistMount {

            } else if p.tag == Tag::Boat {
                // let b = boat01(p.x, p.y, BoatArgs {
                //     scale: p.y / 800.,
                //     fli: rand_bool(),
                //     ..BoatArgs::default()
                // });
                // g = g.add(b);
                // if(b.)
                // g = g.add(
            // );
            } else if p.tag == Tag::RedCircle {

            } else if p.tag == Tag::GreenCircle {

            }
        }
    }
    g
}

pub fn gen_svg(draw_background: bool) -> Document {
    let mut app_state: State = State::new();
    let noise_generator = Noise::new();
    let resolution = 512.;

    let mut nodes = Group::new();
    
    if draw_background {
        let indexes = ((resolution / 2.) + 1.) as usize;
        for i in 0..indexes {
            for j in 0..indexes {
                let rand_decr = _rand() * 255.;
                let c = (245. + noise_generator.noise(i as f64 * 0.1,j as f64 * 0.1 as f64, 0.) * 10.) - rand_decr;
                let r = c as u8;
                let g  = (c * 0.95) as u8;
                let b = (c * 0.85) as u8;

            nodes = nodes.add(rect(i as f64, j as f64, 1., 1., r, g, b))
                    .add(rect(resolution - i as f64, j as f64, 1., 1., r, g, b))
                    .add(rect(i as f64, resolution - j as f64, 1., 1., r, g, b))
                    .add(rect(resolution - i as f64, resolution - j as f64, 1., 1., r, g, b));
            }
        }
    }

    nodes = nodes.add(load_chunk(&mut app_state, &noise_generator, 0., 256.));
    Document::new()
                .set("viewbox", (0., 0., resolution, resolution))
                .set("style", "mix-blend-mode:multiply")
                .add(nodes)
}

pub fn svg_string(draw_background: bool) -> String {
    gen_svg(draw_background).to_string()
}

pub fn write_svg(svg_file: &str, doc: &Document) {
    svg::save(svg_file, doc).unwrap();
}

pub struct Painting {
    state: State,
    noise: Noise
}

impl Painting {

    pub fn new() -> Self {
        Self {
            state: State::new(),
            noise: Noise::new()
        }
    }

    pub fn write_svg(&mut self, width: f64, _height: f64) -> String {
        let resolution = 512.;
        Document::new()
                .set("viewbox", (0., 0., resolution, resolution))
                .set("style", "mix-blend-mode:multiply")
                .add(load_chunk(&mut self.state, &mut self.noise, 0., width))
                .to_string()
    }
}

pub fn convert(in_file: &str, out_file: &str) {
    println!("Writing final output to {:?}", out_file);
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(&in_file).ok().and_then(|p| p.parent().map(|p| p.to_path_buf()));
    opt.fontdb.load_system_fonts();

    let svg_data = std::fs::read(in_file).unwrap();
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();

    let resolution = 512;

    let fit_to = usvg::FitTo::Zoom(1.);
    let mut pixmap = tiny_skia::Pixmap::new(resolution, resolution).unwrap();
    resvg::render(&rtree, fit_to, pixmap.as_mut()).unwrap();

    pixmap.save_png(out_file).unwrap();
}


/*
* Tests
*/ 
#[test]
fn test_stroke_zip() {
    let pt_list = vec![Point{x:0., y:0.}, Point{x:3., y: 3.}, Point{x:999., y:999.}];
    let mut vtx_list0 = vec![Point{x:0.1, y:0.1}, Point{x:0.2, y: 0.2}, Point{x: 0.3, y: 0.3}];
    let mut vtx_list1 = vec![
            Point{ x: 1.1, y: 1.1}, Point{ x: 1.2, y: 1.2}, Point{ x: 1.3, y: 1.3}
            ];

        let correct = vec![
            Point{x:0., y:0.}, 
            Point{x:0.1, y:0.1}, Point{x:0.2, y: 0.2}, Point{x: 0.3, y: 0.3},
            Point{x : 999., y: 999.},
            Point{ x: 1.3, y: 1.3}, Point{ x: 1.2, y: 1.2}, Point{ x: 1.1, y: 1.1},
            Point{x:0., y:0. }, 
        ];
        let res =  stroke_zip(&pt_list, &mut vtx_list0, &mut vtx_list1);
        assert_eq!(correct, res);
}
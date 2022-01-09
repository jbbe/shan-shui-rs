use std::collections::{HashMap, VecDeque};
use svg::node::element::{Group, Polyline};
use svg::Document;

pub mod noise;
pub mod draw;
mod point;

pub use noise::Noise;
pub use draw::*;
pub use point::*;

const PI: f64 = std::f64::consts::PI;
/*
* Trees
*/
#[allow(dead_code)]
struct TreeArgs {
    height: f64,
    width: f64,
    clu: f64,
    col: String,
    noi: f64,
}

impl TreeArgs {

    pub fn tree1_default() -> Self {
        Self {
            height: 50.,
            width: 3.,
            clu: 0.,
            col: "rgba(100,100,100,0.5)".to_string(),
            noi: 0.5,
        }
    }
}


fn tree01(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> Group {
    let reso = 10;
    let mut ns_list: Vec<Point> = Vec::new();
    for i in 0..reso {
        ns_list.push(Point {
            x: noise.noise(i as f64 * 0.5, 0., 0.),
            y: noise.noise(i as f64 * 0.5, 0.5, 0.),
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
                let r1 = noise.rand();
                let r2 = noise.rand();
                let r3= noise.rand();
                let r4= noise.rand();
                let r5= noise.rand();
                let r6= noise.rand();
                g = g.add(blob(
                    noise,
                    nx + (r1 - 0.5) * args.width * 1.2 * (reso - i) as f64,
                    ny + (r2 - 0.5) * args.width,
                    BlobArgs {
                        len: r3 * 20. * (reso - i) as f64 * 0.2 + 10.,
                        width: r4 * 6. + 3.,
                        angle: ((r5 - 0.5) * PI) / 6.,
                        col: color_a(100, 100, 100, r6 * 0.2 + 0.5),
                        ..BlobArgs::default()
                    },
                ))
            }
        }
        line1.push(Point {
            x: nx + (ns_list[i].x - 0.5) * args.width - args.width / 2.,
            y: ny,
        });
        line2.push(Point {
            x: nx + (ns_list[i].y - 0.5) * args.width - args.width / 2.,
            y: ny,
        });
    }
    g = g
        .add(poly(
            &line1,
            PolyArgs {
                fil: "none".to_string(),
                stroke: args.col.clone(),
                width: 1.5,
                ..PolyArgs::default()
            }
            // 0.,
            // 0.,
            // "none".to_string(),
            // args.col.clone(),
            // 1.5,
        ))
        .add(poly(&line2, PolyArgs {
            fil: "none".to_string(), 
            stroke: args.col,
            width: 1.5,
            ..PolyArgs::default()
        }
    ));
    g
}

fn default_tree2_args() -> TreeArgs {
    TreeArgs {
        height: 16.,
        width: 8.,
        clu: 5.,
        col: "rgba(100,100,100,0.5)".to_string(),
        noi: 0.5,
    }
}

fn tree02(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> Group {
    let clu = args.clu as u8;
    let mut g = Group::new();
    for _ in 0..clu {
        let r1 = noise.rand();
        let r2 = noise.rand();
        let rg1 = noise.rand_gauss();
        let rg2 = noise.rand_gauss();
        g = g.add(blob(
            noise,
            x + rg1 * args.clu * 4.,
            y + rg2 * args.clu * 4.,
            BlobArgs {
                angle: PI / 2.,
                // col: color_a(100, 100, 100, 0.8),
                col: args.col.to_string(),
                // default fun
                width: r1 * (args.width * 0.75) + (args.width * 0.5),
                len: r2 * (args.height * 0.75) + (args.height * 0.5),
                ..BlobArgs::default()
            },
        ));
    }
    g
}

struct MountainArgs {
    height: f64,
    width: f64,
    tex: f64,
    veg: bool,
    col: Option<String>,
}

impl MountainArgs {
    pub fn default (noise: &mut Noise) -> Self {
        let r1 = noise.rand();
        let r2 = noise.rand();

        Self {
            height: 100. + (r1 * 400.), // rand 100-500
            width: 400. + (r2 * 200.), // rand 400-600,
            tex: 200.,
            veg: true,
            col: None,
        }
    }
}

// struct Tre
const ONE_TWO_ARR: [usize; 2] = [1, 2];
fn mountain(noise: &mut Noise, x_off: f64, y_off: f64, seed: f64, args: MountainArgs) -> Group {
    fn foot(noise: &mut Noise, pt_list: &Vec<Vec<Point>>, x_off: f64, y_off: f64) -> Group {
        let mut ft_list: Vec<Vec<Point>> = Vec::new();
        let span = 10;
        let mut ni = 0;
        let loop_limit = pt_list.len() - 2;
        for i in 0..loop_limit {
            if i == ni {
                ni = usize::min(ni + noise.rand_choice_arr(&ONE_TWO_ARR), pt_list.len() - 1);
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
                        x: pt_list[i][pt_list[i].len() - 1 - j].x
                            - noise.noise(j as f64 * 0.1, i as f64, 0.) * 10.,
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
                    let x2 = pt_list[i][pt_last].x * (1. - p) + pt_list[i][pt_last].x * p;
                    let mut y2 = pt_list[i][pt_last].y * (1. - p) + pt_list[i][pt_last].y * p;

                    let vib = -1.7 * (p - 1.) * f64::powf(p, 0.2);

                    y1 = y1 + (vib * 5. + noise.noise(x_off * 0.05, i as f64, 0.));
                    y2 = y2 + (vib * 5. + noise.noise(x_off * 0.05, i as f64, 0.));

                    let idx1 = ft_list.len() - 2;
                    let idx2 = ft_list.len() - 1;
                    ft_list[idx1].push(Point { x: x1, y: y1 });
                    ft_list[idx2].push(Point { x: x2, y: y2 });
                }
            }
        }
        let mut g = Group::new();
        let f_len = ft_list.len();
        for i in 0..f_len {
            g = g.add(poly(
                &ft_list[i],
                PolyArgs {
                    x_off,
                    y_off,
                    fil: "white".to_string(),
                    stroke: "none".to_string(),
                    width: 0.,
                }
            ));
        }
        for j in 0..f_len {
            // let f_list = ft_list[j];
            let stroke_pts = ft_list[j]
                .clone()
                .into_iter()
                .map(|p| Point {
                    x: p.x + x_off,
                    y: p.y + y_off,
                })
                .collect::<Vec<_>>();
            let r1 = noise.rand();
            g = g.add(
                stroke(
                    noise,
                    &stroke_pts,
                    StrokeArgs {
                        col: color_a(100, 100, 100, 0.1 +(r1 * 0.1)),
                        // col: green(),
                        width: 1.,
                        ..StrokeArgs::default()
                    },
                )
                .unwrap(),
            );
        }
        g
    }

    fn vegetate(
        noise: &mut Noise,
        pt_list: &Vec<Vec<Point>>,
        x_off: f64,
        y_off: f64,
        seed: f64,
        h: f64,
        tree_func: fn(noise: &mut Noise, x: f64, y: f64, x_off: f64, y_off: f64, h: f64) -> Group,
        growth_rule: fn(
            noise: &mut Noise,
            pts: &Vec<Vec<Point>>,
            i: usize,
            j: usize,
            seed: f64,
            h: f64,
        ) -> bool,
        proof_rule: fn(pts: &Vec<Point>, y: f64) -> bool,
    ) -> Group {
        let mut veg_list: Vec<Point> = Vec::new();
        let mut g = Group::new();
        // might be error in original impl here he uses len straightI
        // /*
        let i_lim = pt_list.len() - 1;
        for i in 0..i_lim {
            // same possibl error as above
            let j_lim = pt_list[i].len() - 1;
            for j in 0..j_lim {
                if growth_rule(noise, pt_list, i, j, seed, h) {
                    veg_list.push(Point {
                        x: pt_list[i][j].x,
                        y: pt_list[i][j].x,
                    });
                }
            }
        }
        if veg_list.len() > 1 {
            let veg_len = veg_list.len() - 1;
            for i in 0..veg_len {
                if proof_rule(&veg_list, i as f64) {
                    g = g.add(tree_func(
                        noise,
                        veg_list[i].x,
                        veg_list[i].y,
                        x_off,
                        y_off,
                        h,
                    ))
                }
            }
        }
        // */
        g
    }

    let height = args.height;
    let width = args.width;
    // let tex = 200.;

    let mut pt_list: Vec<Vec<Point>> = Vec::new();
    let reso = [10, 50];
    let mut hoff = 0.;

    let mut group = Group::new();
    for j in 0..reso[0] {
        hoff += (noise.rand() * y_off) / 100.;
        pt_list.push(Vec::new());
        for i in 0..reso[1] {
            let x = (i as f64 / reso[1] as f64 - 0.5) * PI;
            let mut y = f64::cos(x);
            y = y * noise.noise(x + 10., j as f64 * 0.15, seed);
            let p = 1. - ((j as f64) / (reso[0] as f64));
            let idx = pt_list.len() - 1;
            pt_list[idx].push(Point {
                x: (x / PI) * width * p,
                y: -y * height * p * hoff,
            });
        }
    }
    // fn tree_func
    // Rim
    group = group.add(vegetate(
        noise,
        &pt_list,
        x_off,
        y_off,
        seed,
        height,
        |noise, x, y, x_off, y_off, _| {
            tree01(
                noise,
                x + x_off,
                y + y_off - 5.,
                TreeArgs {
                    col: color_a(
                        100,
                        100,
                        100,
                        noise.noise(0.01 * x, 0.01 * y, 0.) * 0.5 * 0.3 + 0.5,
                    ),
                    ..TreeArgs::tree1_default()
                },
            )
        },
        |noise, pt_list, i, j, seed, h| {
            let ns = noise.noise(j as f64 * 0.1, seed, 0.);
            i == 0 && ns * ns * ns < 0.1 && f64::abs(pt_list[i][j].y) / h > 0.2
        },
        |_veg_list, _i| true,
    ));

    // White background
    let mut white_pg_pts = pt_list[0].clone();
    white_pg_pts.push(Point { x: 0., y: reso[0] as f64 * 4.});
    // println!("poly pts{:?}", poly_pts );
    let white_bg = poly(
        &white_pg_pts,
        PolyArgs {
            x_off,
            y_off,
            fil: "white".to_string(),
            stroke: "none".to_string(),
            width: 0.,

        }
    );
    group = group.add(white_bg);

    // Outline
    let outline_pts: Vec<Point> = pt_list[0]
        .iter()
        .map(|p| Point {
            x: p.x + x_off,
            y: p.y + y_off,
        })
        .collect();
    if outline_pts.len() > 1 {
        group = group.add(
            stroke(
                noise,
                &outline_pts,
                StrokeArgs {
                    col: color_a(100, 100, 100, 0.3),
                    // col: blue(),
                    noi: 1.,
                    width: 3.,
                    ..StrokeArgs::default()
                },
            )
            .unwrap(),
        );
    } else {
        println!("Stroke pt_list len < 1 {:?} ", outline_pts,);
    }

    // foot
    group = group.add(foot(noise, &pt_list, x_off, y_off));

    // texture
    let arr = [0., 0., 0., 0., 5.];
    let sha = noise.rand_choice_arrf(&arr);

    group = group.add(texture(
        noise,
        &pt_list,
        TextureArgs {
            x_off,
            y_off,
            tex: 200,
            sha,
            col: args.col,
            ..TextureArgs::default()
        },
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
            tree02(
                noise,
                x + x_off,
                y + y_off,
                TreeArgs {
                    col: color_a(
                        100,
                        100,
                        100,
                        noise.noise(0.01 * x, 0.01 * y, 0.) * 0.5 * 0.3 + 0.5,
                    ),
                    ..default_tree2_args()
                },
            )
        },
        |noise, pt_list, i, j, seed, h| {
            let ns = noise.noise(i as f64 * 0.1, j as f64 * 0.1, seed + 2.);
            ns * ns * ns < 0.1 && f64::abs(pt_list[i][j].y / h) > 0.5
        },
        |_veg_list, _i| true,
    ));
    if args.veg {
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
                ht = ht * 0.3 + noise.rand() * ht * 0.7;
                let r1 = noise.rand();
                let noise_val = noise.noise(0.01 * x, 0.01 * y, 0.);
                tree01(
                    noise,
                    x + x_off,
                    y + y_off,
                    TreeArgs {
                        height: ht,
                        width: r1 * 3. + 1.,
                        col: color_a(
                            100,
                            100,
                            100,
                            noise_val * 0.5 * 0.3 + 0.3,
                        ),
                        ..default_tree2_args()
                    },
                )
            },
            |noise, pt_list, i, j, seed, h| {
                let ns = noise.noise(i as f64 * 0.2, j as f64 * 0.5, seed);
                j % 2 != 0 && ns * ns * ns * ns < 0.012 && f64::abs(pt_list[i][j].y / h) < 0.3
            },
            |_veg_list, _i| true,
        ));
    }
    group
}


struct FlatMountArgs {
    height: f64,
    width: f64,
    tex: usize,
    cho: f64,
    seed: f64,
}

impl FlatMountArgs {
    pub fn default(noise: &mut Noise) -> Self {
        let height = 40. + (noise.rand() * 400.);
        let width = 400. + (noise.rand() * 200.);
        Self {
            height,
            width,
            tex: 80,
            cho: 0.5,
            seed: 0.
        }
    }
}

fn flat_mount(noise: &mut Noise, x_off: f64, y_off: f64, args: FlatMountArgs) -> Group {
    let mut g = Group::new();

    let mut pt_list: Vec<Vec<Point>>  = Vec::new();
    let mut flat : Vec<Vec<Point>> = Vec::new();
    let reso  = [5, 50];
    let reso_f  = [5., 50.];
    let mut hoff = 0.;

    for j in 0..reso[0] {
        let j_f = j as f64;
        hoff += (noise.rand() * y_off) / 100.;
        pt_list.push(Vec::new());
        flat.push(Vec::new());
        
        for i in 0..reso[1] {
            let i_f = i as f64;
            let x = (i_f / (reso_f[1] - 0.5)) * PI;
            let mut y = f64::cos(x * 2.) + 1.;
            y *= noise.noise(x + 10., j_f * 0.1, args.seed);
            let p = 1. - (j_f /reso_f[0]) * 0.6;
            let nx = (x / PI) * args.width * p;
            let mut ny = (-y) * args.height * p + hoff;
            let h = 100.;
            if ny < -h * args.cho + hoff {
                ny = -h * args.cho + hoff;
                let flat_last = flat.len() - 1;
                if flat[flat_last].len() % 2 == 0 {
                    let val = vec!([nx, ny]);
                    flat[flat_last].push(Point { x: nx, y: ny });
                }
            } else {
                if flat[(flat.len() -1)].len() % 2 == 1 {
                    // TODO 2125
                    // Don't think it would be possible to get into this tate?
                    let pt_last = pt_list.len() - 1;
                    let pt_last_last = pt_list[pt_last].len() - 1;
                    let flat_last = flat.len() - 1;
                    flat[flat_last].push(pt_list[pt_last][pt_last_last].clone());
                        // Point
                        // pt_list[pt_list.len() - 1][pt_list.len() - 1]
                        // pt_list[pt_last][pt_last_last].y,
                    // );
                    //     x: pt_list[(pt_list.len() - 1)], 
                    //     y: ny 
                    // });
                }
            }
            let pt_last = pt_list.len() - 1;
            pt_list[pt_last].push(Point { x: nx, y: ny});
        }
    }

    // White BG
    let end_p = Point { x: 0., y: reso_f[0] * 4.};
    let mut bg_pts = pt_list[0].clone();
    bg_pts.push(end_p);
    g = g.add(poly(&bg_pts, PolyArgs {
        x_off, 
        y_off,
        fil: "white".to_string(),
        stroke: "none".to_string(),
        ..PolyArgs::default()
    }));

    // Outline
    let outln_pts = pt_list[0]
        .iter()
        .map(|p| { Point { x: p.x + x_off, y: p.y + y_off }})
        .collect();
    let outline = stroke(noise, &outln_pts, StrokeArgs {
            col: color_a(100, 100, 100, 0.3),
            noi: 1.,
            width: 3.,
            ..StrokeArgs::default()
        });
    if !outline.is_none() {
        g= g.add(outline.unwrap());
    }
    g = g.add(texture(noise, &pt_list, TextureArgs {
            x_off,
            y_off,
            tex: args.tex,
            width: 2.,
            dis: |n| {
                if n.rand() > 0.5 {
                    0.1 + 04. * n.rand()
                } else {
                    0.9 - 0.4 * n.rand()
                }
            },
            ..TextureArgs::default()
        }));

    let mut gr_list_1: VecDeque<Point> = VecDeque::new();
    gr_list_1.reserve(10);
    let mut gr_list_2 = VecDeque::new();
    gr_list_2.reserve(10);
    for i in (0..(flat.len())).step_by(2) {
        if flat[i].len() >= 2 {
            gr_list_1.push_back(flat[i][0]);
            gr_list_2.push_back(flat[i][flat[i].len() - 1]);
        }
    }

    if gr_list_1.len() == 0 {
        return g
    }

    let mut wb = [gr_list_1[0].x, gr_list_2[0].y];
    // wb.reserve(10);
    // wb.push([gr_list_1[0][0]);
    // wb.push(gr_list_2[0][0]]);
    for i in 0..3 {
        let p = 0.8 - i as f64 * 0.2;

        gr_list_1.push_front(Point { x: wb[0] * p, y: gr_list_1[0].y - 5.});
        gr_list_2.push_front(Point { x: wb[1] * p, y: gr_list_2[0].y - 5.});
    }
    wb[0] = gr_list_1[gr_list_1.len() - 1].x;
    wb[1] = gr_list_2[gr_list_2.len() - 1].x; 
    for i in 0..3 {
        let i_f = i as f64;
        let  p = 0.6 - i_f * i_f * 0.1;

        gr_list_1.push_back(Point { 
            x: wb[0] * p,
            y: gr_list_1[gr_list_1.len() - 1].y + 1.
        });
        gr_list_2.push_back(Point { 
            x: wb[1] * p,
            y: gr_list_2[gr_list_2.len() - 1].y + 1.
        });
    }
    let d = 5.;

    gr_list_1 = div(&gr_list_1, d);
    gr_list_2 = div(&gr_list_2, d);

    let gr_list = gr_zip(&mut gr_list_1, &mut gr_list_2);

    let str_pts = gr_list
        .iter()
        .map(|p| { Point { x: p.x + x_off, y: p.y + y_off} })
        .collect();
     g = g.add(poly(&gr_list, PolyArgs { 
         x_off,
         y_off, 
         fil: "white".to_string(),
         stroke: "none".to_string(),
         width: 2. 
        }));
    let stro = stroke(noise, &str_pts, StrokeArgs {
            width: 3.,
            col: color_a(100, 100, 100, 0.2),
            ..StrokeArgs::default()
        } ) ;
    match stro {
        Some(s) => {
            g = g.add(s);
        },
        None => {},
    }

    if gr_list.len() > 0 {
        let bnd = bound(gr_list);
        // g = g.add(flat_dec(noise, x_off, y_off, bnd));
        // g - g.add()
    }
    // fn bound(p_list: Vec<Point>) -> 
    g
}

struct Bound {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}
fn bound(p_list: Vec<Point>) -> Bound {
        let mut x_min = p_list[0].x;
        let mut x_max = x_min;
        let mut y_min = p_list[0].y; 
        let mut y_max = y_min;
        for p in p_list.iter() {
            if  x_min > p.x {
                x_min = p.x; 
            }
            if  x_min < p.x {
                x_max = p.x;
            }
            if y_min > p.y {
                y_min = p.y;
            }
            if y_max < p.y {
                y_max = p.y
            }
            
        }
        Bound { x_min, x_max, y_min, y_max }
}


fn flatDec(noise: &mut Noise, x_off: f64, y_off: f64, gr_bound: Bound) -> Group{
    let mut g = Group::new();

    let tt = noise.rand_choice_arr(&[0, 0, 1, 3, 3, 4]);
    for i in 0..(f64::floor(noise.rand() * 5.) as usize) {
        let seed = noise.rand() * 100.;
        let width = 10. + (noise.rand() * 20.);
        let height = 10. + (noise.rand() * 20.);
        let args = RockArgs {
            width,
            height,
            sha: 2.,
            ..RockArgs::default()
        };
        g = g.add(rock(noise, x_off, y_off, seed, args));
    }
    for j in 0..(noise.rand_choice_arr(&[0, 0, 1, 2])) {
        let xr = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
        let yr = y_off + (gr_bound.y_min + gr_bound.y_max) / 2. + noise.norm_rand(-5., 5.) + 20.;
        let mut k = 0.;
        while k < 2. + (noise.rand() * 3.) {
            // add tree08 
            k += 1.;
        }
    }

    // let createRock: FnMut(&Noise) = |noise: &mut Noise|  {
    //     let x = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
    //     let y = y_off + (gr_bound.y_min + gr_bound.y_max) / 2. + noise.norm_rand(-5., 5.) + 20.;
    //     let seed = noise.rand() * 100.;
    //     let width = 50. + noise.rand() * 20.;
    //     let height = 40. + noise.rand() * 20.;
    //     g = g.add(rock(
    //             noise,
    //             x,
    //             y,
    //             seed,
    //             RockArgs {
    //                 width,
    //                 height,
    //                 sha: 5.,
    //                 ..RockArgs::default()
    //             }
    //     ));
    //     ()
    // };

    if tt == 0 {
        let mut j = 0.;
        while j < (noise.rand() * 3.) {
            // createRock(noise);
        let x = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
        let y = y_off + (gr_bound.y_min + gr_bound.y_max) / 2. + noise.norm_rand(-5., 5.) + 20.;
        let seed = noise.rand() * 100.;
        let width = 50. + noise.rand() * 20.;
        let height = 40. + noise.rand() * 20.;
        g = g.add(rock(
                noise,
                x,
                y,
                seed,
                RockArgs {
                    width,
                    height,
                    sha: 5.,
                    ..RockArgs::default()
                }
        ));
            j += 1.;
        }
    } else if tt == 1 {
        let p_min = noise.rand() * 0.5;
        let p_max = noise.rand() * 0.5 + 0.5;
        let x_min = gr_bound.x_min * (1. - p_min) + (gr_bound.x_max * p_min);
        let x_max = gr_bound.x_min * (1. - p_max) + (gr_bound.x_max * p_max);
        // for i 
        // loop tree 05

        // loop rock
        let mut j = 0.;
        while j < noise.rand() * 4. {
            // createRock(noise);
        let x = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
        let y = y_off + (gr_bound.y_min + gr_bound.y_max) / 2. + noise.norm_rand(-5., 5.) + 20.;
        let seed = noise.rand() * 100.;
        let width = 50. + noise.rand() * 20.;
        let height = 40. + noise.rand() * 20.;
        g = g.add(rock(
                noise,
                x,
                y,
                seed,
                RockArgs {
                    width,
                    height,
                    sha: 5.,
                    ..RockArgs::default()
                }
        ));
            j += 1.;
        }
    } else if tt == 2 {
        for i in 0..(noise.rand_choice_arr(&[1, 1, 1, 1, 2, 2, 3])) {
            let xr = noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
            let yr = (gr_bound.y_min + gr_bound.y_max) / 2.;
            // add tree 04

            let mut j = 0.;
            while j < noise.rand() * 2. {
                // createRock(noise);
        let x = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
        let y = y_off + (gr_bound.y_min + gr_bound.y_max) / 2. + noise.norm_rand(-5., 5.) + 20.;
        let seed = noise.rand() * 100.;
        let width = 50. + noise.rand() * 20.;
        let height = 40. + noise.rand() * 20.;
        g = g.add(rock(
                noise,
                x,
                y,
                seed,
                RockArgs {
                    width,
                    height,
                    sha: 5.,
                    ..RockArgs::default()
                }
        ));
                j += 1.;
            }
        }
    } else if tt == 3 {
        for _ in 0..(noise.rand_choice_arr(&[1, 1, 1, 1, 2, 2, 3])) {
            //  add tree06
        }

    } else if tt == 4 {
        let p_min = noise.rand() * 0.5;
        let p_max = noise.rand() * 0.5 + 0.5;
        let x_min = gr_bound.x_min * (1. - p_min) + gr_bound.x_max * p_min;
        let x_min = gr_bound.x_min * (1. - p_max) + gr_bound.x_max * p_min;
        // for i in 0..x_max as 
        //  loop tree 07

    }

    let mut i = 0.;
    while i < 50. * noise.rand() {
        // add tree02
        let x = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
        let y = y_off + noise.norm_rand(gr_bound.y_min, gr_bound.y_max);
        g = g.add(tree02(noise, x, y, TreeArgs::tree1_default())); // FIXME ( default args for tree2)

        i += 1.;
    }

    let ts = noise.rand_choice_arr(&[0, 0, 0, 0, 0, 1]);
    if ts == 1 && tt != 4 {
        // Add arch
    }
    g
}

struct RockArgs {
    height: f64,
    width: f64,
    tex: f64,
    sha: f64
}

impl RockArgs { 
    fn default() -> Self {
        Self {
            height: 80.,
            width: 100.,
            tex: 40.,
            sha: 10. 
        }
    }
}
fn rock(noise: &mut Noise, x_off: f64, y_off: f64, seed: f64, args: RockArgs) -> Group {
    let mut g = Group::new();
    g
}

struct Man {}
#[allow(dead_code)]
struct ManArgs {
    sca: f64,
    hat: fn(Point, Point, bool) -> Polyline,
    ite: fn(&mut Noise, Point, Point) -> Group,
    fli: bool,
    angle: [f64; 9],
    len: [f64; 9],
}

impl ManArgs {
    fn default(n: &mut Noise) -> Self {
        Self {
            angle: [
                0.,
                -PI / 2.,
                n.norm_rand(0., 0.),
                (PI / 4.) * n.rand(),
                ((PI * 3.) / 4.) * n.rand(),
                (PI * 3.) / 4.,
                -PI / 4.,
                (-PI * 3.) / 4. - (PI / 4.) * n.rand(),
                -PI / 4.,
            ],
            ite: |_n, _p0, _p1| Group::new(),
            hat: |p0, p1, f| Man::hat02(p0, p1, f),
            fli: true,
            sca: 0.5,
            len: [0., 30., 20., 30., 30., 30., 30., 30., 30.],
        }
    }
}

struct StickArgs {
    fli: bool,
}
impl StickArgs {
    fn default() -> Self {
        Self { fli: false }
    }
}

impl Man {
    fn man(_x_off: f64, _y_off: f64, _args: ManArgs) -> Group {
        let g = Group::new();
        g
    }

    fn tran_poly(p0: Point, p1: Point, pt_list: &Vec<Point>) -> Vec<Point> {
        let p_list: Vec<Point> = pt_list.iter().map(|p| Point { x: -p.x, y: p.y }).collect();
        let ang = f64::atan2(p1.y - p0.y, p1.x - p0.x) - PI / 2.;
        let scl = distance(&p0, &p1);
        let origin = Point { x: 0., y: 0. };
        p_list
            .iter()
            .map(|p| {
                let d = distance(&p, &origin);
                let a = f64::atan2(p.y, p.x);
                Point {
                    x: p0.x + d * scl * f64::cos(ang + a),
                    y: p0.y + d * scl * f64::sin(ang + a),
                }
            })
            .collect()
    }

    fn hat02(p0: Point, p1: Point, fli: bool) -> Polyline {
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
            PolyArgs {
                fil: color_a(100, 100, 100, 0.8),
                stroke: color_a(100, 100, 100, 0.8),
                ..PolyArgs::default()

            }
        )
    }

    fn stick01(noise: &mut Noise, _p0: Point, _p2: Point, _args: StickArgs) -> Group {
        let seed = noise.rand();
        // let f = if args.fli { Man::flipper } else { |x: Vec<Point>| -> Vec<Point> {x} };

        let mut q_list1 = Vec::new();
        let l = 12;
        for i in 0..l {
            q_list1.push(Point {
                x: -noise.noise(i as f64 * 0.1, seed, 0.)
                    * 0.1
                    * f64::sin((i as f64 / 1.) * PI)
                    * 5.,
                y: i as f64 * 0.3,
            });
        }
        Group::new()
    }

    fn flipper(p_list: Vec<Point>) -> Vec<Point> {
        p_list.iter().map(|p| Point { x: -p.x, y: p.y }).collect()
    }
}

struct BoatArgs {
    len: f64,
    scale: f64,
    fli: bool,
}

impl BoatArgs {
    fn default() -> Self {
        Self {
            len: 120.,
            scale: 1.,
            fli: false,
        }
    }
}

fn boat01(noise: &mut Noise, x_off: f64, y_off: f64, args: BoatArgs) -> Group {
    let mut g = Group::new();
    let dir = if args.fli { -1. } else { 1. };
    g = g.add(Man::man(
        x_off + 20. * (args.scale) * dir,
        y_off,
        ManArgs {
            ite: |n, p0, p1| Man::stick01(n, p0, p1, StickArgs::default()),
            hat: Man::hat02,
            sca: 0.5 * (args.scale),
            fli: !(args.fli),
            len: [0., 30., 20., 30., 10., 30., 30., 30., 30.],
            ..ManArgs::default(noise)
        },
    ));
    // g = g.add()

    g
}

struct WaterArgs {
    height: f64,
    len: f64,
    clu: usize,
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

fn water(noise: &mut Noise, x_off: f64, y_off: f64, args: WaterArgs) -> Group {
    let mut g = Group::new();
    let mut pt_list = Vec::new();
    let mut yk = 0.;
    for _ in 0..(args.clu) {
        pt_list.push(Vec::new());
        let xk = (noise.rand() - 0.5) * ((args.len) / 8.);
        yk = yk + (noise.rand() * 5.);
        let lk = ((args.len) / 4.) + noise.rand() * (args.len / 4.);
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
        g = g.add(
            stroke(
                noise,
                &pts,
                StrokeArgs {
                    width: 1.,
                    // col: color_a(100, 100, 100, 0.3 + _rand() * 0.3),
                    col: blue(),
                    ..StrokeArgs::default()
                },
            )
            .unwrap(),
        );
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

struct Plan {
    tag: Tag,
    x: f64,
    y: f64,
    h: f64, // what does this represent? usually generated by ns() func
}

impl Plan {
    fn new(tag: Tag, x: f64, y: f64, h: f64) -> Self {
        Self { tag, x, y, h }
    }
}

/*
* Mount planner
*/
const SAMP: f64 = 0.03;
fn mount_planner(app_state: &mut State, noise: &mut Noise, x_min: f64, x_max: f64) -> Vec<Plan> {
    fn loc_max(
        noise: &mut Noise,
        x: f64,
        y: f64,
        mut f: fn(&mut Noise, f64, f64) -> f64,
        r: f64,
    ) -> bool {
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
                        return false;
                    }
                    j = j + 1.;
                }
                i = i + 1.;
            }
            true
        }
    }
    fn chadd_mind(registry: &mut Vec<Plan>, plan: Plan, mind: f64) -> bool {
        // let len = reg.len();
        for k in 0..(registry.len()) {
            // we only add the new chunk if
            // the difference between the new plan's x
            // and any other plan's x is less than mind
            // which defaults to 10
            if f64::abs(registry[k].x - plan.x) < mind {
                return false;
            }
        }
        println!("+");
        registry.push(plan);
        true
    }
    /*
     * returns whether plan was succesfully added.
     */
    fn chadd(registry: &mut Vec<Plan>, plan: Plan) -> bool {
        chadd_mind(registry, plan, 10.)
    }
    // ensures that no x is placed at exactly the same line
    let mut registry: Vec<Plan> = Vec::new(); 

    let ns = |noise: &mut Noise, x: f64, _: f64| -> f64 {
        f64::max(noise.noise(x * SAMP, 0., 0.) - 0.55, 0.) * 2.
    };

    // let nns = |x: f64, y: f64| {
    //     1. - noise.noise(x * SAMP, y, 0.)
    // };

    // let nnns = |x: f64, y: f64 | {
    //     f64::max(noise.noise(x * SAMP * 2., 2., 0.) - 0.55, 0.) * 2.
    // };

    let yr = |noise: &mut Noise, x| noise.noise(x * 0.01, PI, 0.);

    let x_step = 5.;
    let m_wid = 200.;
    // original does this index by index we do a single rezie
    // line 3757
    // let i_x_min = f64::floor(x_min) as i32;
    // let i_x_max = f64::floor(x_max) as i32;
    // let i_step = x_step as i32;
    let mut i = x_min;
    while i < x_max {
        let i1 = f64::floor(i / x_step) as i32;
        println!("i1 {}", i1);
        app_state.plan_mtx.entry(i1).or_insert_with_key(|_k| { 0 });
        i = i + x_step;
    }
    // for i in i_x_min..i_x_max.step_by(i_step) {
    //     let i1 = f64::floor(i / x_step);
    //     println!("i1 {}", i1);
    // }
    // let new_len = f64::floor(x_max / x_step) as usize + 2;
    // app_state.plan_mtx.resize_with(new_len, || 0);

    i = x_min;
    while i < x_max {
        let mut j = 0.;
        while j < yr(noise, i) * 480. {
            if loc_max(noise, i, j, ns, 2.) {
                let xof = i + 2. * (noise.rand() - 0.5) * 500.;
                let yof = j + 300.;
                let r: Plan = Plan::new(Tag::Mount, xof, yof, ns(noise, i, j));
                let res = chadd(&mut registry, r);
                if res {
                    let lower_lim = f64::floor((xof - m_wid) / x_step) as i32; 
                    let upper_lim = f64::floor((xof + m_wid) / x_step) as i32;
                    for k in lower_lim..upper_lim {
                        // is this determining the crest of the mountains?
                        *(app_state.plan_mtx.entry(k).or_insert(0)) += 1;
                    }
                } // for k
            } // if res

            j = j + 30.;
        } // while j
        if f64::abs(i) % 1000. < x_step - 1. {
            // distmount is only added when i < 4
            println!("adding distmount");
            let r1 = noise.rand();
            let r = Plan::new(Tag::DistMount, i, 280. - r1 * 50., ns(noise, i, j));
            chadd(&mut registry, r);
        }

        i = i + x_step;
    }
    println!("Xmin {:?} xmax {:?}", x_min, x_max);

    let mut i = x_min;
    while i < f64::floor(x_max) {
        let idx = f64::floor(i / x_step) as i32;
        println!("Xmax {:?} i {:?} idx {:?} step {:?}", x_min, i, idx, x_step);
        if app_state.plan_mtx[&idx] == 0 {
            if noise.rand() < 0.01 {
                let mut j = 0.;
                while j < (4. * noise.rand()) {
                    let r = Plan::new(
                        Tag::FlatMount,
                        i + 2. * (noise.rand() - 0.5) * 700.,
                        700. - j * 50.,
                        ns(noise, i, j),
                    );
                    chadd(&mut registry, r);
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
        if noise.rand() < 0.2 {
            let r = Plan::new(Tag::Boat, i, 300. + noise.rand() * 390., 0.);
            chadd_mind(&mut registry, r, 400.);
        }
        i = i + x_step;
    }
    registry
}

struct State {
    plan_mtx: HashMap<i32, u32>,
    x_min: f64,
    x_max: f64,
    c_wid: f64,
}

impl State {
    fn new() -> Self {
        Self {
            plan_mtx: HashMap::new(),
            x_min: 0.,
            x_max: 0.,
            c_wid: 512.,
        }
    }
}

// as opposed to creating and saving a 'chunk' like in the js version
// load chunk returns a group that contains the svg for this section
fn load_chunk(app_state: &mut State, noise: &mut Noise, x_min: f64, x_max: f64) -> Group {
    let mut g = Group::new();
    while x_max > app_state.x_max - app_state.c_wid 
        || x_min < app_state.x_min + app_state.c_wid 
        {
        println!("Generating new chunk...",);

        // generate new chunk
        let plans: Vec<Plan>;
        if x_max > app_state.x_max - app_state.c_wid {
            plans = mount_planner(
                app_state,
                noise,
                app_state.x_max,
                app_state.x_max + app_state.c_wid,
            );
            app_state.x_max = app_state.x_max + app_state.c_wid;
        } else {
            plans = mount_planner(
                app_state,
                noise,
                app_state.x_min - app_state.c_wid,
                app_state.x_min,
            );
            app_state.x_min = app_state.x_min - app_state.c_wid;
        }

        let len = plans.len();
        println!("Generated {:?} plans", len);
        for i in 0..len {
            let p = &plans[i];
            println!("create svg for chunk {:?} {:?} {:?}", p.tag, p.x, p.y);
            if p.tag == Tag::Mount {
                let seed = (i * 2 ) as f64 * noise.rand();
                let args = MountainArgs::default(noise);
                let svg_node = mountain(noise, p.x, p.y, seed, args);
                let w = water(noise, p.x, p.y - 1000., WaterArgs::default());
                g = g.add(svg_node)
                .add(w)
                ;
            } else if p.tag == Tag::FlatMount {
                let seed = 2. * noise.rand();
                let width = 600. + (noise.rand() * 400.);
                let cho = 0.5 + (noise.rand() * 0.2);
                let args = FlatMountArgs {
                    width,
                    height: 100.,
                    cho,
                    seed,
                    ..FlatMountArgs::default(noise)
                };
                g = g.add(flat_mount(noise, p.x, p.y, args))
            } else if p.tag == Tag::DistMount {

            } else if p.tag == Tag::Boat {
                let args = BoatArgs {
                    scale: p.y / 800.,
                    fli: noise.rand_bool(),
                    ..BoatArgs::default()
                };
                g = g.add(boat01(noise,p.x, p.y, args));
            } else if p.tag == Tag::RedCircle {
            } else if p.tag == Tag::GreenCircle {
            }
        }
    }
    g
}

pub fn gen_svg(seed: f64, draw_background: bool) -> Document {
    let mut app_state: State = State::new();
    let mut noise = Noise::new(seed);
    let resolution = 512.;

    let mut nodes = Group::new();

    if draw_background {
        let indexes = ((resolution / 2.) + 1.) as usize;
        for i in 0..indexes {
            for j in 0..indexes {
                let rand_decr = noise.rand() * 255.;
                let c = (245.
                    + noise.noise(i as f64 * 0.1, j as f64 * 0.1 as f64, 0.) * 10.)
                    - rand_decr;
                let r = c as u8;
                let g = (c * 0.95) as u8;
                let b = (c * 0.85) as u8;

                nodes = nodes
                    .add(rect(i as f64, j as f64, 1., 1., r, g, b))
                    .add(rect(resolution - i as f64, j as f64, 1., 1., r, g, b))
                    .add(rect(i as f64, resolution - j as f64, 1., 1., r, g, b))
                    .add(rect(
                        resolution - i as f64,
                        resolution - j as f64,
                        1.,
                        1.,
                        r,
                        g,
                        b,
                    ));
            }
        }
    }

    nodes = nodes.add(load_chunk(&mut app_state, &mut noise, 0., 256.));
    Document::new()
        .set("viewbox", (0., 0., resolution, resolution))
        .set("style", "mix-blend-mode:multiply")
        .add(nodes)
}

pub fn svg_string(seed: f64) -> String {
    gen_svg(seed, false).to_string()
}

pub fn write_svg(svg_file: &str, doc: &Document) {
    svg::save(svg_file, doc).unwrap();
}

pub struct Painting {
    state: State,
    noise: Noise,
}

impl Painting {
    pub fn new(seed: f64) -> Self {
        Self {
            state: State::new(),
            noise: Noise::new(seed),
        }
    }

    pub fn write_svg(&mut self, width: f64, _height: f64) -> String {
        // println!("Perlins {:?}", self.noise.perlins());
        let resolution = 512.;
        Document::new()
            .set("viewbox", (0., 0., resolution, resolution))
            .set("style", "mix-blend-mode:multiply")
            .add(load_chunk(&mut self.state, &mut self.noise, 0., width))
            .to_string()
    }
    pub fn draw_boat(&mut self) -> String {
        let resolution = 512.;
        Document::new()
            .set("viewbox", (0., 0., resolution, resolution))
            .set("style", "mix-blend-mode:multiply")
            .add(boat01(&mut self.noise, 256., 256., BoatArgs::default()))
            .to_string()
    }

    pub fn draw_mount(&mut self) -> String {
        let resolution = 512.;
        let seed = ( 2. ) * self.noise.rand();
        let args = MountainArgs::default(&mut self.noise);
        Document::new()
            .set("viewbox", (0., 0., resolution, resolution))
            .set("style", "mix-blend-mode:multiply")
            .add(mountain(&mut self.noise, 10., 300., seed, args))
            .to_string()
    }
}

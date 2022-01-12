use super::super::*;
// {Noise, Point, poly, PolyArgs, texture, TextureArgs, stroke, };
use super::*;
use svg::node::element::{Group};
use std::collections::{VecDeque};
use core::f64::consts::PI;
#[allow(DeadCode)]
pub struct MountainArgs {
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
pub fn mountain(noise: &mut Noise, x_off: f64, y_off: f64, seed: f64, args: MountainArgs) -> Group {
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
            // col: args.col,
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


pub struct FlatMountArgs {
    pub height: f64,
    pub width: f64,
    pub tex: usize,
    pub cho: f64,
    pub seed: f64,
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

pub fn flat_mount(noise: &mut Noise, x_off: f64, y_off: f64, args: FlatMountArgs) -> Group {
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
        g = g.add(flat_dec(noise, x_off, y_off, bnd));
        // g - g.add()
    }
    // fn bound(p_list: Vec<Point>) -> 
    g
}

pub struct Bound {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}
pub fn bound(p_list: Vec<Point>) -> Bound {
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


pub fn flat_dec(noise: &mut Noise, x_off: f64, y_off: f64, gr_bound: Bound) -> Group {
    let mut g = Group::new();

    let tt = noise.rand_choice_arr(&[0, 0, 1, 3, 3, 4]);
    for _ in 0..(f64::floor(noise.rand() * 5.) as usize) {
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
    for _ in 0..(noise.rand_choice_arr(&[0, 0, 1, 2])) {
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
pub struct DistMountArgs {
    pub(crate) height: f64,
    pub len: f64,
    pub seg: f64,
}
impl DistMountArgs {
    pub fn default() -> Self {
        Self {
            height: 300.,
            len: 2000.,
            seg: 5.,
        }
    }
}
pub fn dist_mount(noise: &mut Noise, x_off: f64, y_off: f64, seed: f64, args: DistMountArgs) -> Group {
    let mut g = Group::new();
    let span = 10.;

    let mut pt_list = Vec::new();


    let push_cnt = (args.len / span / args.seg) as usize;

    // let inner_vec_capacity = (args.seg + 1. + (args.seg /2.)+ 1. + (args.seg /2.) + 1.) as usize;
    let lower_half_of_vec_capacity = (args.seg /2.) as usize + 1;
    let upper_half_of_vec_capacity = args.seg as usize + 1;
    let inner_vec_capacity = lower_half_of_vec_capacity + upper_half_of_vec_capacity;

    for i in 0..push_cnt {
        let i_f = i as f64;
        pt_list.push(Vec::with_capacity(inner_vec_capacity));
        let pt_last = pt_list.len() - 1;
        pt_list[pt_last].resize(inner_vec_capacity, Point { x: 0., y: 0. });
        for j in 0..upper_half_of_vec_capacity {
            let tran = |noise: &mut Noise, k: f64| {
                Point {
                    x: x_off + k * span,
                    y: y_off - args.height * noise.noise(k as f64 * 0.05, seed, 0.) *
                        f64::powf(f64::sin(PI * k) / (args.len / span), 0.5)
                }
            };
            let pt_last = pt_list.len() - 1;
            pt_list[pt_last][lower_half_of_vec_capacity + j] = tran(noise, i_f * args.seg * j as f64 * 2.)
            // let t = polytools.triangulate(pt_list[i])
        }

        for j in 0..lower_half_of_vec_capacity {
            let tran = |noise: &mut Noise, k: f64| {
                Point {
                    x: x_off + k * span,
                    y: y_off + 24. * noise.noise(k * 0.05, 2., seed) *
                        f64::powf(f64::sin(PI * k) / (args.len / span), 1.)
                }
            };
            let pt_last = pt_list.len() - 1;
            pt_list[pt_last][j] = tran(noise, i_f * args.seg + j as f64 * 2.)
        }
    }

    
    for i in 0..(pt_list.len()) {
        let get_col = |n: &mut Noise, x, y| {
            let c= (n.noise(x * 0.02, y * 0.02, y_off) * 55. + 200.) as u8 | 0;
            color(c, c, c)
        };
        let p = pt_list[i][pt_list[i].len() - 1];
        // let p_v = &pt_list[i];
        let mut v= Vec::with_capacity(pt_list[i].len());
        for j in 0..pt_list[i].len() {
            // fix this but it works for now sigh
            v.push(pt_list[i][j]);
        }
        // for j in 0..
        // let p2: Vec<Point> = p_v.iter().collect();
        // let v = &Vec::from(p_v.iter().collect());
        g = g.add(poly(&v, PolyArgs { 
            fil: get_col(noise, p.x, p.y),
            stroke: none_str(),
            width: 1. ,
            ..PolyArgs::default()
        }));
        //  let t = polytools.triangulate
        // for k in 0..(t.len()) {
            // let m =
        // }
    }
    g
}


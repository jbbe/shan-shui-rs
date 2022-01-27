use super::super::*;
use super::*;
use core::f64::consts::PI;
use std::collections::VecDeque;
#[allow(dead_code)]
pub struct MountainArgs {
    height: f64,
    width: f64,
    tex_density: usize,
    veg: bool,
    col: Option<String>,
}

impl MountainArgs {
    pub fn default(noise: &mut Noise) -> Self {
        let r1 = noise.rand();
        let r2 = noise.rand();

        Self {
            height: 100. + (r1 * 400.), // rand 100-500
            width: 400. + (r2 * 200.),  // rand 400-600,
            tex_density: 200,
            veg: true,
            col: None,
        }
    }
}

/** Generates several layers.
 * Each layer is an arc inside of all previous layers
 * each arc is adjusted by noise.
 * The outer (0th) layer is used to draw the outline and
 * the rest of the layers are used for the interior texture.
 * Vegetate is then called with a variety of functions to
 * draw other forms.
 * Each form has its own uniqre set of conditions that determine
 * whether it appear
 */
pub fn mountain(
    noise: &mut Noise,
    x_off: f64,
    y_off: f64,
    seed: f64,
    args: MountainArgs,
) -> String {
    /*
    * Draw triangles at the base of the mountain.
    */
    fn foot(noise: &mut Noise, layers: &Vec<Vec<Point>>, x_off: f64, y_off: f64) -> String {
        let mut ft_layers: Vec<Vec<Point>> = Vec::new();
        let span = 10;
        let mut ni = 0;
        let loop_limit = layers.len() - 2;
        for i in 0..loop_limit {
            if i == ni {
                // ni increases at increments of 1 or 2 until it it is 2 less than the number of l
                ni = (ni + noise.rand_choice_arr(&[1, 2])).min(layers.len() - 1);
                let layer = &layers[i];
                let n_layer = &layers[ni];
                let pt_count = ((layers[i].len() as f64 / 8.).ceil() as usize).min(10);
                let mut ft_layer1 = Vec::with_capacity(pt_count + span);
                let mut ft_layer2 =Vec::with_capacity(pt_count + span);
                for j in 0..pt_count {
                    ft_layer1.push(Point {
                        x: layer[j].x + noise.noise(j as f64 * 0.1, i as f64, 0.) * 10.,
                        y: layer[j].y,
                    });
                    ft_layer2.push(Point {
                        x: layer[layer.len() - 1 - j].x
                            - noise.noise(j as f64 * 0.1, i as f64, 0.) * 10.,
                        y: layer[layer.len() - 1 - j].y,
                    });
                }

                ft_layer1.reverse();
                ft_layer2.reverse();

                for j in 0..span {
                    let p = j as f64 / span as f64;
                    // x1 y1 are the start of all pts in the corner of traingle?
                    let x1 = layer[0].x * (1. - p) + n_layer[0].x * p;
                    let mut y1 = layer[0].y * (1. - p) + n_layer[0].y * p;

                    let pt_last = layer.len() - 1;
                    let x2 = layer[pt_last].x * (1. - p) + n_layer[pt_last].x * p;
                    let mut y2 = layer[pt_last].y * (1. - p) + n_layer[pt_last].y * p;

                    let vib = -1.7 * (p - 1.) * p.powf(0.2);
                    y1 += vib * 5. + noise.noise(x_off * 0.05, i as f64, 0.) * 5.;
                    y2 += vib * 5. + noise.noise(x_off * 0.05, i as f64, 0.) * 5.;

                    ft_layer1.push(Point { x: x1, y: y1 });
                    ft_layer2.push(Point { x: x2, y: y2 });
                }
                ft_layers.push(ft_layer1);
                ft_layers.push(ft_layer2);
            }
        }
        let mut g = Group::new("foot".to_string());
        // let colors_poly = ["pink", "red", "yellow"];
        // let colors_stroke = ["chartreuse", "chocolate", "orange"];
        for layer in ft_layers.iter() {
            // these polys are roughly triangles that cover partway up texture linex
            // and reach the base
            g.add(poly(layer,
                PolyArgs {
                    x_off,
                    y_off,
                    // fil: colors_poly[i % 3].to_string(),
                    fil: white(),
                    stroke: "none".to_string(),
                    ..PolyArgs::default(Some("ft-poly".to_string()))
                },
            ));

            // Draw the bottom and lines on the foot of the mountain
            // these lines cover the base of the mountain and outline
            // the shading above.
            let r1 = noise.rand();
            g.add(stroke(
                noise,
                &layer.iter()
                    .map(|p| Point { x: p.x + x_off, y: p.y + y_off })
                    .collect(),
                StrokeArgs {
                    col: color_a(100, 100, 100, 0.1 +(r1 * 0.1)),
                    // col: colors_stroke[i % 3].to_string(),
                    width: 1.,
                    ..StrokeArgs::default("ft-tr".to_string())
                },
            ));
        }
        g.to_string()
    }

    fn vegetate(
        noise: &mut Noise,
        layers: &Vec<Vec<Point>>,
        x_off: f64,
        y_off: f64,
        seed: f64,
        h: f64,
        tree_func: fn(noise: &mut Noise, x: f64, y: f64, x_off: f64, y_off: f64, h: f64) -> String,
        growth_rule: fn(
            noise: &mut Noise,
            pts: &Vec<Vec<Point>>,
            i: usize,
            j: usize,
            seed: f64,
            h: f64,
        ) -> bool,
        proof_rule: fn(pts: &Vec<Point>, y: f64) -> bool,
    ) -> String {
        let mut veg_list: Vec<Point> = Vec::new();
        let mut g = Group::new("veg".to_string());
        // might be error in original impl here he uses len straightI
        // /*
        for layer_idx in 0..(layers.len()) {
            // same possibl error as above
            let j_lim = layers[layer_idx].len() - 1;
            for pt_idx in 0..j_lim {
                if growth_rule(noise, layers, layer_idx, pt_idx, seed, h) {
                    veg_list.push(Point {
                        x: layers[layer_idx][pt_idx].x,
                        y: layers[layer_idx][pt_idx].x,
                    });
                }
            }
        }
        if veg_list.len() > 1 {
            let veg_len = veg_list.len() - 1;
            for i in 0..veg_len {
                if proof_rule(&veg_list, i as f64) {
                    g.add(tree_func(
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
        g.to_string()
    }

    let h = args.height;
    let w = args.width;

    // Ptlist[0] is the outline of the mountain and
    // the rest of the vectors are the inner textures
    let num_layers = 10;
    let num_pts = 50;

    let mut ht_off = 0.;
    let layers = (0..num_layers).map(|layer_idx| {
            ht_off += (noise.rand() * y_off) / 100.;
            // Expansion will shrink from 1 towards 0 as we approach the final layer
            let expansion = 1. - ((layer_idx as f64) / (num_layers as f64));
            // println!("Layer {}", layer_idx);
            (0..num_pts)
                .map(|pt_idx| {
                    let tilt = (pt_idx as f64 / num_pts as f64 - 0.5) * PI;
                    let y = tilt.cos() * noise.noise(tilt + 10., layer_idx as f64 * 0.15, seed);
                    // println!("x (tilt) {} y {} expansin {}", tilt, y, expansion);
                    Point {
                        x: (tilt / PI) * w * expansion,
                        y: (-y) * h * expansion + ht_off,
                    }
                }).collect()
        }).collect();

    let mut group = Group::new("mnt".to_string());
    // Rim
    group.add(vegetate(
        noise,
        &layers,
        x_off,
        y_off,
        seed,
        h,
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
                    ..TreeArgs::default01()
                },
            )
        },
        |noise, pt_list, i, j, seed, h| {
            let ns = noise.noise(j as f64 * 0.1, seed, 0.);
            assert_ne!(h, 0.);
            i == 0 && ns * ns * ns < 0.1 && f64::abs(pt_list[i][j].y) / h > 0.2
        },
        |_veg_list, _i| true,
    ));

    // White background
    let mut white_bg_pts = layers[0].clone();
    // this point we push at the end to enclose the area of 
    // the mountain where x being 0 will return us to the origin
    // and y being the numer of layers of the mountian multiplied
    // by 4 for some reason? how does this make sense?
    white_bg_pts.push(Point { x: 0., y: num_layers as f64 * 4. });
    // println!("poly pts{:?}", poly_pts );
    group.add(poly(
        &white_bg_pts,
        PolyArgs {
            x_off,
            y_off,
            fil: "white".to_string(),
            stroke: "none".to_string(),
            width: 0.,
            name: Some("wht bg".to_string()),
        },
    ));

    // Outline
    group.add(stroke(
        noise,
        &(layers[0].iter()
            .map(|p| Point { x: p.x + x_off, y: p.y + y_off })
            .collect()),
        StrokeArgs {
            col: color_a(100, 100, 100, 0.3),
            // col: "aqua".to_string(),
            noi: 1.,
            width: 3.,
            ..StrokeArgs::default("outln-str".to_string())
        },
    ));

    // foot
    group.add(foot(noise, &layers, x_off, y_off));

    // texture
    let shading = noise.rand_choice_arrf(&[0., 0., 0., 0., 5.]);
    group.add(texture(noise,
        &layers,
        TextureArgs {
            x_off,
            y_off,
            density: args.tex_density,
            shading,
            // col: args.col,
            col: |n, _x| { color_a(100, 100, 100, n.rand()* 0.3)},
            // col: |_n, _y| "green".to_string(),
            ..TextureArgs::default()
        },
    ));

    // Top
    group.add(vegetate(
        noise,
        &layers,
        x_off,
        y_off,
        seed,
        h,
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
                    ..TreeArgs::default02()
                },
            )
        },
        |noise, pt_list, i, j, seed, h| {
            let ns = noise.noise(i as f64 * 0.1, j as f64 * 0.1, seed + 2.);
            assert_ne!(0., h);
            ns * ns * ns < 0.1 && f64::abs(pt_list[i][j].y / h) > 0.5
        },
        |_veg_list, _i| true,
    ));
    // if false {
        if args.veg {
        // middle
        group.add(vegetate(
            noise,
            &layers,
            x_off,
            y_off,
            seed,
            h,
            |noise, x, y, x_off, y_off, h| {
                // todo should be tree 02
                assert_ne!(h, 0.);
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
                        col: color_a(100, 100, 100, noise_val * 0.5 * 0.3 + 0.3),
                        ..TreeArgs::default02()
                    },
                )
            },
            |noise, pt_list, i, j, seed, h| {
                let ns = noise.noise(i as f64 * 0.2, j as f64 * 0.5, seed);
                assert_ne!(h, 0.);
                j % 2 != 0 && ns * ns * ns * ns < 0.012 && f64::abs(pt_list[i][j].y / h) < 0.3
            },
            |_veg_list, _i| true,
        ));
        // Bottom
        group.add(vegetate(
            noise,
            &layers,
            x_off,
            y_off,
            seed,
            h,
            |noise, x, y, x_off, y_off, h| -> String {
                let _ht = ((h + y) / h) * 120.;
                let ht = _ht * 0.5 + noise.rand() * _ht * 0.5;
                let args = TreeArgs {
                    height: ht,
                    col: color_a(
                        100,
                        100,
                        100,
                        noise.noise(0.01 * x, 0.01 * y, 0.) * 0.5 * 0.3 + 0.3,
                    ),
                    ..TreeArgs::default03()
                };
                tree03(noise, x + x_off, y + y_off, args)
            },
            |noise, pt_list, i, j, seed, h| {
                let ns = noise.noise(i as f64 * 0.2, j as f64 * 0.5, seed);
                assert_ne!(h, 0.);
                (j == 0 || j == pt_list[i].len() - 1) && ns * ns * ns * ns < 0.012
            },
            |_veg_list, _i| true,
        ));
    }

    // bottom arch
    // vegetate

    // top arch

    // transm

    // bott rock
    group.to_string()
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
            seed: 0.,
        }
    }
}

pub fn flat_mount(noise: &mut Noise, x_off: f64, y_off: f64, args: FlatMountArgs) -> String {

    let mut pt_list: Vec<Vec<Point>> = Vec::new();
    let mut flat: Vec<Vec<Point>> = Vec::new();
    let reso = [5, 50];
    let reso_f = [5., 50.];
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
            let p = 1. - (j_f / reso_f[0]) * 0.6;
            let nx = (x / PI) * args.width * p;
            let mut ny = (-y) * args.height * p + hoff;
            let h = 100.;
            if ny < -h * args.cho + hoff {
                ny = -h * args.cho + hoff;
                let flat_last = flat.len() - 1;
                if flat[flat_last].len() % 2 == 0 {
                    flat[flat_last].push(Point { x: nx, y: ny });
                }
            } else {
                if flat[(flat.len() - 1)].len() % 2 == 1 {
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
            pt_list[pt_last].push(Point { x: nx, y: ny });
        }
    }

    // White BG
    let end_p = Point {
        x: 0.,
        y: reso_f[0] * 4.,
    };
    let mut bg_pts = pt_list[0].clone();
    bg_pts.push(end_p);
    let mut g = Group::new("flatmnt".to_string());
    g.add(poly(
        &bg_pts,
        PolyArgs {
            x_off,
            y_off,
            fil: "white".to_string(),
            stroke: "none".to_string(),
            ..PolyArgs::default(Some("f_mnt bg".to_string()))
        },
    ));

    // Outline
    let outln_pts = pt_list[0]
        .iter()
        .map(|p| Point {
            x: p.x + x_off,
            y: p.y + y_off,
        })
        .collect();
    g.add(stroke(noise,
        &outln_pts,
        StrokeArgs {
            col: color_a(100, 100, 100, 0.3),
            noi: 1.,
            width: 3.,
            ..StrokeArgs::default("fltmnt outln-str".to_string())
        },
    ));
    g.add(texture(noise,
        &pt_list,
        TextureArgs {
            x_off,
            y_off,
            density: args.tex,
            width: 2.,
            dis: |n| {
                if n.rand() > 0.5 {
                    0.1 + 04. * n.rand()
                } else {
                    0.9 - 0.4 * n.rand()
                }
            },
            ..TextureArgs::default()
        },
    ));

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
        return g.to_string();
    }

    let mut wb = [gr_list_1[0].x, gr_list_2[0].y];
    // wb.reserve(10);
    // wb.push([gr_list_1[0][0]);
    // wb.push(gr_list_2[0][0]]);
    for i in 0..3 {
        let p = 0.8 - i as f64 * 0.2;

        gr_list_1.push_front(Point {
            x: wb[0] * p,
            y: gr_list_1[0].y - 5.,
        });
        gr_list_2.push_front(Point {
            x: wb[1] * p,
            y: gr_list_2[0].y - 5.,
        });
    }
    wb[0] = gr_list_1[gr_list_1.len() - 1].x;
    wb[1] = gr_list_2[gr_list_2.len() - 1].x;
    for i in 0..3 {
        let i_f = i as f64;
        let p = 0.6 - i_f * i_f * 0.1;

        gr_list_1.push_back(Point {
            x: wb[0] * p,
            y: gr_list_1[gr_list_1.len() - 1].y + 1.,
        });
        gr_list_2.push_back(Point {
            x: wb[1] * p,
            y: gr_list_2[gr_list_2.len() - 1].y + 1.,
        });
    }
    let d = 5.;

    gr_list_1 = div(&gr_list_1, d);
    gr_list_2 = div(&gr_list_2, d);

    let gr_list = gr_zip(&mut gr_list_1, &mut gr_list_2);

    g.add(poly(
        &gr_list,
        PolyArgs {
            x_off,
            y_off,
            fil: "white".to_string(),
            stroke: "none".to_string(),
            width: 2.,
            name: Some("sflt mnt553".to_string()),
        },
    ));
    g.add(stroke(noise,
    &gr_list.iter()
        .map(|p| Point {x: p.x + x_off, y: p.y + y_off}).collect(),
        StrokeArgs {
            width: 3.,
            col: color_a(100, 100, 100, 0.2),
            ..StrokeArgs::default("grlst str".to_string())
        },
    ));

    if gr_list.len() > 0 {
        g.add(flat_dec(noise, x_off, y_off, bound(gr_list)));
    }
    g.to_string()
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
        if x_min > p.x {
            x_min = p.x;
        }
        if x_min < p.x {
            x_max = p.x;
        }
        if y_min > p.y {
            y_min = p.y;
        }
        if y_max < p.y {
            y_max = p.y
        }
    }
    Bound {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}
#[allow(dead_code)]
pub fn flat_dec(noise: &mut Noise, x_off: f64, y_off: f64, gr_bound: Bound) -> String {
    let mut g = Group::new("flat dec".to_string());

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
        g.add(rock(noise, x_off, y_off, seed, args));
    }
    for _ in 0..(noise.rand_choice_arr(&[0, 0, 1, 2])) {
        let xr = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
        let yr = y_off + (gr_bound.y_min + gr_bound.y_max) / 2. + noise.norm_rand(-5., 5.) + 20.;
        let mut k = 0.;
        while k < 2. + (noise.rand() * 3.) {
            // add tree08
            let x = xr + noise.norm_rand(-30., 30.).max(gr_bound.x_min).max(gr_bound.x_max);
            let height = 60. + noise.rand() * 40.;
            g.add(tree08(noise, x, yr,TreeArgs { height, ..TreeArgs::default08()}));
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
            g.add(rock(
                noise,
                x,
                y,
                seed,
                RockArgs {
                    width,
                    height,
                    sha: 5.,
                    ..RockArgs::default()
                },
            ));
            j += 1.;
        }
    } else if tt == 1 {
        let p_min = noise.rand() * 0.5;
        let p_max = noise.rand() * 0.5 + 0.5;
        let _x_min = gr_bound.x_min * (1. - p_min) + (gr_bound.x_max * p_min);
        let _x_max = gr_bound.x_min * (1. - p_max) + (gr_bound.x_max * p_max);
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
            g.add(rock(noise,
                x,
                y,
                seed,
                RockArgs {
                    width,
                    height,
                    sha: 5.,
                    ..RockArgs::default()
                },
            ));
            j += 1.;
        }
    } else if tt == 2 {
        for _i in 0..(noise.rand_choice_arr(&[1, 1, 1, 1, 2, 2, 3])) {
            let xr = noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
            let yr = (gr_bound.y_min + gr_bound.y_max) / 2.;
            // add tree 04
            g.add(tree04(noise, x_off + xr, y_off + yr + 20., TreeArgs::default04()));

            let mut j = 0.;
            while j < noise.rand() * 2. {
                // createRock(noise);
                let x = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
                let y =
                    y_off + (gr_bound.y_min + gr_bound.y_max) / 2. + noise.norm_rand(-5., 5.) + 20.;
                let seed = noise.rand() * 100.;
                let width = 50. + noise.rand() * 20.;
                let height = 40. + noise.rand() * 20.;
                g.add(rock(
                    noise,
                    x,
                    y,
                    seed,
                    RockArgs {
                        width,
                        height,
                        sha: 5.,
                        ..RockArgs::default()
                    },
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
        let _x_min = gr_bound.x_min * (1. - p_min) + gr_bound.x_max * p_min;
        let _x_min = gr_bound.x_min * (1. - p_max) + gr_bound.x_max * p_min;
        // for i in 0..x_max as
        //  loop tree 07
    }

    let mut i = 0.;
    while i < 50. * noise.rand() {
        // add tree02
        let x = x_off + noise.norm_rand(gr_bound.x_min, gr_bound.x_max);
        let y = y_off + noise.norm_rand(gr_bound.y_min, gr_bound.y_max);
        g.add(tree02(noise, x, y, TreeArgs::default02())); // FIXME ( default args for tree2)

        i += 1.;
    }

    let ts = noise.rand_choice_arr(&[0, 0, 0, 0, 0, 1]);
    if ts == 1 && tt != 4 {
        // Add arch
    }
    g.to_string()
}
pub struct DistMountArgs {
    pub height: f64,
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
pub fn dist_mount(
    noise: &mut Noise,
    x_off: f64,
    y_off: f64,
    seed: f64,
    args: DistMountArgs,
) -> String {
    let mut g = Group::new("dstmnt".to_string());
    let span = 10.;

    let mut pt_list = Vec::new();

    assert_ne!(args.seg, 0.);
    let push_cnt = (args.len / span / args.seg) as usize;

    // let inner_vec_capacity = (args.seg + 1. + (args.seg /2.)+ 1. + (args.seg /2.) + 1.) as usize;
    let lower_half_of_vec_capacity = (args.seg / 2.) as usize + 1;
    let upper_half_of_vec_capacity = args.seg as usize + 1;
    let inner_vec_capacity = lower_half_of_vec_capacity + upper_half_of_vec_capacity;

    assert_ne!(args.len, 0.);

    let len_over_span = args.len / span;

    for i in 0..push_cnt {
        let i_f = i as f64;
        pt_list.push(Vec::with_capacity(inner_vec_capacity));
        let pt_last = pt_list.len() - 1;
        pt_list[pt_last].resize(inner_vec_capacity, Point { x: 0., y: 0. });
        for j in 0..upper_half_of_vec_capacity {
            let tran = |noise: &mut Noise, k: f64| {
                let n = noise.noise(k * 0.05, seed, 0.);
                let sin_k_over_lenspan = f64::abs(f64::sin(PI * k) / len_over_span);
                /*
                 * Javascript pow (-x)^2 = x^2
                 * Rust powf returns NaN for x < 0
                 */
                let pow_res = f64::powf(sin_k_over_lenspan, 0.5);
                let y = y_off - args.height * n * pow_res;
                Point {
                    x: x_off + k * span,
                    y,
                }
            };
            pt_list[pt_last][lower_half_of_vec_capacity + j] =
                tran(noise, i_f * args.seg * j as f64)
        }

        for j in 0..lower_half_of_vec_capacity {
            let tran = |noise: &mut Noise, k: f64| Point {
                x: x_off + k * span,
                y: y_off
                    + 24.
                        * noise.noise(k * 0.05, 2., seed)
                        * f64::powf(f64::sin(PI * k) / (args.len / span), 1.),
            };
            let pt_last = pt_list.len() - 1;
            pt_list[pt_last][j] = tran(noise, i_f * args.seg + j as f64 * 2.)
        }
    }

    for i in 0..(pt_list.len()) {
        let get_col = |n: &mut Noise, x, y| {
            let c = (n.noise(x * 0.02, y * 0.02, y_off) * 55. + 200.) as u8 | 0;
            color(c, c, c)
        };
        let p = pt_list[i][pt_list[i].len() - 1];
        // let p_v = &pt_list[i];
        let mut v = Vec::with_capacity(pt_list[i].len());
        for j in 0..pt_list[i].len() {
            // fix this but it works for now sigh
            v.push(pt_list[i][j]);
        }
        // for j in 0..
        // let p2: Vec<Point> = p_v.iter().collect();
        // let v = &Vec::from(p_v.iter().collect());
        g.add(poly(
            &v,
            PolyArgs {
                fil: get_col(noise, p.x, p.y),
                stroke: none_str(),
                width: 1.,
                ..PolyArgs::default(Some("dst mnt".to_string()))
            },
        ));
        //  let t = triangulate(&pt_list[i], TriangulateArgs { 
        //      area: 100.,
        //      convex: true,
        //      optimize: false,
        //      ..TriangulateArgs::default()});
        // for k in 0..(t.len()) {
        //     let m = mid_pt(&t[k]);
        //     let co = get_col(noise, m.x, m.y);
        //     g.add(poly(&t[k], PolyArgs { 
        //         fil: co.clone(),
        //         stroke: co,
        //         width: 1.,
        //         ..PolyArgs::default(Some("dstm".to_string()))}))
        // }
    }
    g.to_string()
}

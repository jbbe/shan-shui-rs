use std::collections::HashMap;
use svg::node::element::{Group, Path};
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

fn default_tree1_args() -> TreeArgs {
    TreeArgs {
        height: 50.,
        width: 3.,
        clu: 0.,
        col: "rgba(100,100,100,0.5)".to_string(),
        noi: 0.5,
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
            0.,
            0.,
            "none".to_string(),
            args.col.clone(),
            1.5,
        ))
        .add(poly(&line2, 0., 0., "none".to_string(), args.col, 1.5));
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
                x_off,
                y_off,
                "white".to_string(),
                "none".to_string(),
                0.,
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
            g = g.add(
                stroke(
                    noise,
                    &stroke_pts,
                    StrokeArgs {
                        // col: color_a(100, 100, 100, 0.1 +(_rand() * 0.1)),
                        col: green(),
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
        /*
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
        */
        g
    }

    let height = args.height;
    let width = args.width;
    // let tex = 200.;
    let veg = args.veg;

    let mut pt_list: Vec<Vec<Point>> = Vec::new();
    let reso = [10, 50];
    let mut hoff = 0.;

    let mut group = Group::new();
    // let g = usvg::Node<usvg::NodeKind::Group>;
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
    // group = group.add(vegetate(
    //     noise,
    //     &pt_list,
    //     x_off,
    //     y_off,
    //     seed,
    //     height,
    //     |noise, x, y, x_off, y_off, _| {
    //         tree01(
    //             noise,
    //             x + x_off,
    //             y + y_off - 5.,
    //             TreeArgs {
    //                 col: color_a(
    //                     100,
    //                     100,
    //                     100,
    //                     noise.noise(0.01 * x, 0.01 * y, 0.) * 0.5 * 0.3 + 0.5,
    //                 ),
    //                 ..default_tree1_args()
    //             },
    //         )
    //     },
    //     |noise, pt_list, i, j, seed, h| {
    //         let ns = noise.noise(j as f64 * 0.1, seed, 0.);
    //         i == 0 && ns * ns * ns < 0.1 && f64::abs(pt_list[i][j].y) / h > 0.2
    //     },
    //     |_veg_list, _i| true,
    // ));

    // White background
    let mut white_pg_pts = pt_list[0].clone();
    white_pg_pts.push(Point { x: 0., y: reso[0] as f64 * 4.});
    // println!("poly pts{:?}", poly_pts );
    let white_bg = poly(
        &white_pg_pts,
        x_off,
        y_off,
        "white".to_string(),
        "none".to_string(),
        0.,
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
                    // col: color_a(100, 100, 100, 0.3),
                    col: blue(),
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
    let col = color(100, 0, 0); // texture is red
    group = group.add(texture(
        noise,
        &pt_list,
        TextureArgs {
            x_off,
            y_off,
            tex: 200,
            sha,
            col,
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

struct Man {}
#[allow(dead_code)]
struct ManArgs {
    sca: f64,
    hat: fn(Point, Point, bool) -> Path,
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

        poly(
            &pts,
            0.,
            0.,
            color_a(100, 100, 100, 0.8),
            color_a(100, 100, 100, 0.8),
            0.,
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
                        // if k < app_state.plan_mtx.len() {
                            // is this determining the crest of the mountains?
                        // let val = *(app_state.plan_mtx.entry(k).or_insert(0));
                        //  app_state.get_mut(k) = val + 1;
                        *(app_state.plan_mtx.entry(k).or_insert(0)) += 1;
                        // } else {
                        //     println!(
                        //         "!! k is out of bounds idxng into plan_mtx len: {:?} k {:?}",
                        //         app_state.plan_mtx.len(),
                        //         k
                        //     );
                        //     // The desired behavior here might be to increase the vec size to upper limit
                        //     break;
                        // }
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
                // let w = water(noise, p.x, p.y - 1000., WaterArgs::default());
                g = g.add(svg_node)
                // .add(w)
                ;
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
            .add(mountain(&mut self.noise, 100., 100., seed, args))
            .to_string()
    }
}


pub fn convert(in_file: &str, out_file: &str) {
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

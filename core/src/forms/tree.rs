// use super::*;
// {Man, ManArgs, Stick, StickArgs};
use super::super::*;
use core::f64::consts::PI;

/*
* Trees
*/
pub struct TreeArgs {
    pub height: f64,
    pub width: f64,
    pub clu: f64,
    pub col: String,
    pub noi: f64,
}

impl TreeArgs {
    pub fn default01() -> Self {
        Self {
            height: 50.,
            width: 3.,
            clu: 0.,
            col: "rgba(100,100,100,0.5)".to_string(),
            noi: 0.5,
        }
    }

    pub fn default02() -> TreeArgs {
        TreeArgs {
            height: 16.,
            width: 8.,
            clu: 5.,
            col: "rgba(100,100,100,0.5)".to_string(),
            noi: 0.5,
        }
    }

    pub fn default03(noise: &mut Noise) -> TreeArgs {
        TreeArgs {
            height: 50.,
            width: 5.,
            clu: 5.,
            col: "rgba(100,100,100,0.5)".to_string(),
            noi: 0.5,
        }
    }

}
pub fn tree01(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> String {
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
                let r3 = noise.rand();
                let r4 = noise.rand();
                let r5 = noise.rand();
                let r6 = noise.rand();
                g.add(blob(
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
                ));
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
    g.add(poly(
            &line1,
            PolyArgs {
                fil: "none".to_string(),
                stroke: args.col.clone(),
                width: 1.5,
                ..PolyArgs::default(Some("tree01 p1".to_string()))
            }
            // 0.,
            // 0.,
            // "none".to_string(),
            // args.col.clone(),
            // 1.5,
        ));
    g.add(poly(
            &line2,
            PolyArgs {
                fil: "none".to_string(),
                stroke: args.col,
                width: 1.5,
                ..PolyArgs::default(Some("tree01 p2".to_string()))
            },
        ));
    g.to_string()
}

pub fn tree02(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> String {
    let clu = args.clu as u8;
    let mut g = Group::new();
    for _ in 0..clu {
        let r1 = noise.rand();
        let r2 = noise.rand();
        let rg1 = noise.rand_gauss();
        let rg2 = noise.rand_gauss();
        g.add(blob(
            noise,
            x + rg1 * args.clu * 4.,
            y + rg2 * args.clu * 4.,
            BlobArgs {
                angle: PI / 2.,
                col: color_a(100, 100, 100, 0.8),
                // col: args.col.to_string(),
                // default fun
                width: r1 * (args.width * 0.75) + (args.width * 0.5),
                len: r2 * (args.height * 0.75) + (args.height * 0.5),
                ..BlobArgs::default()
            },
        ));
    }
    g.to_string()
}



pub fn tree03(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> String {
    let bc = noise.rand() * 0.1;
    let bp = 1;
    let ben = | x: f64| -> f64 {
        // this comes from vegetate mountain bottom the only place tree03 is called
        x * bc
    };

    let mut g = Group::new();
    let reso = 10;
    let resof = 10.;
    // let ns_list = [Point; 10; Point { x: 0, y: 0} ];
    let ns_list : Vec<Point> = (0..reso).map(|i| {
        let x = noise.noise(i as f64 * 0.5, 0., 0.);
        let y = noise.noise(i as f64 * 0.5, 0.5, 0.);
        Point { x, y }
    }).collect();
    let leafcol = [100, 100, 100, 100];

    // let blobs = 
    let mut line1 = vec![];
    let mut line2 = vec![];
    for i in 0..reso {
        let i_f = i as f64;
        let nx = x + ben(i_f / resof) * 100.;
        let ny = y - (i_f * args.height) / reso as f64;
        if i >= reso / 5 {
            for j in 0..((reso - i) * 2) {
                let shape = |x| {
                    f64::log(50. * x + 1., std::f64::consts::E) / 3.95
                };
                let ox = noise.rand() * args.width * shape((resof - i_f) / resof);
                let r_choice = noise.rand_choice_arrf(&[-1., 1.]);
                let r1 = noise.rand() - 0.5 * args.width * 2.;
                let width = noise.rand() * 6. -  3.;
                let angle = (noise.rand() - 0.5) * PI / 6.;
                let r2 = noise.rand();
                g.add(blob(
                    noise,
                    nx + ox * r_choice,
                    ny + r1,
                    BlobArgs {
                        len: ox + 2.,
                        width,
                        angle,
                        col: color_a(leafcol[1], leafcol[1], leafcol[2], r2 * 0.2 ), // som
                        ..BlobArgs::default()
                    }
                ));
            }
        }
        line1.push(Point {
            x: nx + ((ns_list[0].x - 0.5) * args.width + (args.width / 2.)) * (resof - i_f) / resof,
            y: ny
        });
        line2.push(Point {
            x: nx + ((ns_list[0].y - 0.5) * args.width + (args.width / 2.)) * (resof - i_f) / resof,
            y: ny
        });
    }
    line2.reverse();
    line1.append(&mut line2);
    g.add(poly(&line1, PolyArgs {
        fil: white(),
        stroke: args.col,
        width: 1.5,
        ..PolyArgs::default(Some("tree03".to_string()))
    }));
    g.to_string()
}


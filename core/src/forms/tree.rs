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
pub fn tree01(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> Group {
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
                ..PolyArgs::default(Some("tree01 p1".to_string()))
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
            ..PolyArgs::default(Some("tree01 p2".to_string()))
        }
    ));
    g
}

pub fn default_tree2_args() -> TreeArgs {
    TreeArgs {
        height: 16.,
        width: 8.,
        clu: 5.,
        col: "rgba(100,100,100,0.5)".to_string(),
        noi: 0.5,
    }
}

pub fn tree02(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> Group {
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
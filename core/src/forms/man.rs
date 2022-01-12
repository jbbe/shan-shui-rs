use svg::node::element::{Group, Polyline};
use std::f64::consts::PI;
use super::super::Noise;
use super::super::*;
// {distance, Noise, Point};

pub struct Man {}
// #[allow(dead_code)]
pub struct ManArgs {
    pub sca: f64,
    pub hat: fn(Point, Point, bool) -> Polyline,
    pub ite: fn(&mut Noise, Point, Point) -> Group,
    pub fli: bool,
    pub angle: [f64; 9],
    pub len: [f64; 9],
}

impl ManArgs {
    pub fn default(n: &mut Noise) -> Self {
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
#[allow(dead_code)]
pub struct StickArgs {
    fli: bool,
}
impl StickArgs {
    pub fn default() -> Self {
        Self { fli: false }
    }
}

impl Man {
    pub fn man(_x_off: f64, _y_off: f64, _args: ManArgs) -> Group {
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

    pub fn hat02(p0: Point, p1: Point, fli: bool) -> Polyline {
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

    pub fn stick01(noise: &mut Noise, _p0: Point, _p2: Point, _args: StickArgs) -> Group {
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

use std::f64::consts::PI;
use super::super::Noise;
use super::super::*;
// {distance, Noise, Point};

const PI_4 : f64= PI / 4.; 
pub struct Man {}
// #[allow(dead_code)]
pub struct ManArgs {
    pub sca: f64,
    pub hat: fn(Point, Point, bool) -> String,
    pub ite: fn(&mut Noise, Point, Point) -> String,
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
                PI_4 * n.rand(),
                ((PI * 3.) / 4.) * n.rand(),
                (PI * 3.) / 4.,
                -PI / 4.,
                -PI_4 * 4. * n.rand(), // (-PI * 3.) / 4. - (PI / 4.) * n.rand(),
                -PI_4,
            ],
            ite: |_n, _p0, _p1| "".to_string(),
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
#[derive(Clone)]
struct Sct {
    obj: Vec<(usize, Option<Sct>)>,
}
impl Sct {
    pub fn keys(&self) -> Vec<usize> {
        self.obj.iter().map(|e| { e.0 }).collect()
    }

    pub fn get(&self, i: usize) -> Option<Sct> {
        for el in self.obj.iter() {
            if el.0 == i {
                let t = el.clone();
                let s = t.1;
                return match s {
                    Some(sct) => Some(sct),
                    None => None
                }
                // if el.1.is_none() {
                //     return None;
                // } else {
                //     let t = el.1
                //     return Some(el.as_ref().1.unwrap().clone());
                // }
                // return match el.1 {
                //     Some(sct) => Some(sct.clone()),
                //     None => None
                // };
            }
        }
        None
    }

    pub fn new() -> Self {
        let five = (5, Some(Sct {obj: vec![(6, None)]}));
        let seven = (7, Some(Sct {obj: vec![(8, None)]}));
        let one = (1, Some(Sct {obj: vec![(2, None), five, seven,]}));
        
        // let four = (4, None);
        let three = (3, Some(Sct { obj: vec![(4, None)] }));

        let zero = (0, Some(Sct { obj: vec![one, three] }));
        Self { 
            obj: vec![zero],
        }
    }
}

impl Man {
    pub fn man(noise: &mut Noise, x_off: f64, _y_off: f64, args: ManArgs) -> String {
        let fli = args.fli;
        let sca = args.sca;
        let hat = args.hat;
        let ite = args.ite;
        let ang = args.angle;
        let len: Vec<f64> = args.len.iter().map(|v| { v * sca }).collect();
        let mut g = Group::new("man".to_string());
        let mut y_off = _y_off;
        let sct = Sct::new();

        fn gpar(sct: &Sct, ind: usize) -> Option<Vec<usize>> {
            // so sct is a nested object, of object with the keys being ints
            let keys = sct.keys();
            for i in 0..(keys.len()) {
            if keys[i] == ind {
                return Some(vec![ind]);
            } else {
                let sub_sct = &sct.get(keys[i]);
                if sub_sct.is_some() {
                    let sct = sub_sct.clone().unwrap();
                    let r = gpar(&sct, ind);
                    if r.is_some() {
                        let mut res = vec![keys[i]];
                        res.append(&mut r.unwrap());
                        return Some(res);
                    }

                }
            }
            }
            None
        }
        let grot = |sct: &Sct, ind: usize| -> f64 {
            let par_o = gpar(sct, ind);
            let mut rot = 0.;
            if par_o.is_some() {
                let par = par_o.unwrap();
                for i in 0..(par.len()) {
                    rot += ang[par[i]];
                }
            }
            rot
        };
        //gpos i think is global pos
        let global_pos = |sct, ind: usize| -> Point {
            let par_o = gpar(sct, ind);
            let mut pos = Point { x: 0., y: 0. };
            if par_o.is_some() {
                let par = par_o.unwrap();
                for i in 0..(par.len()) {
                    let a = grot(&sct, par[i]);
                    pos.x += len[par[i]] * a.cos();
                    pos.y += len[par[i]] * a.sin();
                }
            }
            pos
        };

        let pts: Vec<Point> = (0..ang.len()).map(|i| { global_pos(&sct, i) }).collect();
    //   for (var i = 0; i < ang.length; Vi++) {
    //     pts.push(global_pos(sct, i));
    //   }
    y_off -= pts[4].y;

    let to_global = |v: &Point| {
        Point{ x: if fli { -1. }else { 1. } * v.x + x_off, y: v.y + y_off }
    };

      // canv stuff is commented in the orig
    // for i in 1..(pts.len()) {
    //     let par = gpar(sct, i);
    //     let p0 = global_pos(sct, par[par.length - 2]);
    //     let s = div([p0, pts[i]], 10);
    //     //canv += stroke(s.map(toGlobal))
    // }

    let cloth = |noise: &mut Noise, plist, fun: fn(f64, f64) -> f64| -> String {
    // let cloth = |(plist: Vec<Point>, fun: FnMut(f64)| -> String 
    // where F: FnMut(f64 )
    // {
        let mut g = Group::new("cloth".to_string());
        let tlist = bezmh(plist, Some(2.));
        let (tlist1, tlist2) = expand(noise, tlist, fun, sca);
        let mut poly1_pt_list = tlist1.clone();
        let mut tmp = tlist2.clone();
        tmp.reverse();
        poly1_pt_list.append(&mut tmp);
        g.add(poly(&poly1_pt_list.iter().map(to_global).collect(), PolyArgs{
          fil: white(), 
          ..PolyArgs::default(Some("m1".to_string()))
        }));
        g.add(stroke(noise, &tlist1.iter().map(to_global).collect(), StrokeArgs{
          width: 1.,
          col: color_a(100,100,100,0.5),
          ..StrokeArgs::default("m1".to_string())
        }));
        g.add(stroke(noise, &tlist2.iter().map(to_global).collect(), StrokeArgs {
          width: 1.,
          col: color_a(100,100,100,0.6),
          ..StrokeArgs::default("m1".to_string())
        }));
        g.to_string()
    };

        let fsleeve = |x: f64, sca| -> f64 {
          sca *
          8. *
          ((0.5 * x * PI).sin() * (x * PI).sin().powf(0.1) +
            (1. - x) * 0.4)
        };
        let fbody : fn(f64, f64) -> f64 = |x, sca| {
          sca *
          11. *
          ((0.5 * x * PI).sin() * (x * PI).sin().powf(0.1) +
            (1. - x) * 0.5)
        };
        let fhead: fn(f64, f64) -> f64 = |x, sca| {
            sca * 7. * (0.25 - (x - 0.5).powi(2)).powf(0.3)
        };

        g.add(ite(noise, to_global(&pts[8]), to_global(&pts[6])));

        g.add(cloth(noise, vec![pts[1], pts[7], pts[8]], fsleeve));
        g.add(cloth(noise, vec![pts[1], pts[0], pts[3], pts[4]], fbody));
        g.add(cloth(noise, vec![pts[1], pts[5], pts[6]], fsleeve));
        g.add(cloth(noise, vec![pts[1], pts[2]], fhead));

        let hlist = bezmh(vec![pts[1], pts[2]], Some(2.));
        let (mut hlist1, mut hlist2) = expand(noise, hlist, fhead, sca);
        hlist1.remove((hlist1.len() as f64 * 0.1).floor() as usize);
        hlist2.remove((hlist2.len() as f64 * 0.95).floor() as usize);
        let mut concated_pts = hlist1.clone();
        hlist2.reverse();
        concated_pts.append(&mut hlist2);
        g.add(poly(&concated_pts.iter().map(to_global).collect(), PolyArgs {
            fil: color_a(100,100,100,0.6), ..PolyArgs::default(Some("mans".to_string()))
        }));

        g.add(hat(to_global(&pts[1]), to_global(&pts[2]),  fli ));

        g.to_string()
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

    pub fn hat02(p0: Point, p1: Point, fli: bool) -> String {
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
                ..PolyArgs::default(Some("man".to_string()))

            }
        )
    }

    pub fn stick01(noise: &mut Noise, _p0: Point, _p2: Point, _args: StickArgs) -> String {
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
        Group::new("stick".to_string()).to_string()
    }

    fn flipper(p_list: Vec<Point>) -> Vec<Point> {
        p_list.iter().map(|p| Point { x: -p.x, y: p.y }).collect()
    }
}
fn expand(noise: &mut Noise, ptlist: Vec<Point>, wfun: fn(f64, f64) -> f64, sca: f64) -> (Vec<Point>, Vec<Point>) {
    let mut vtxlist0 = vec![Point {x: 0., y: 0.}];
    let mut vtxlist1 = vec![Point {x: 0., y: 0.}];
    //   let mut vtxlist = vec![];
    // let n0 = noise.rand() * 10.;
    for i in 1..(ptlist.len() - 1) {
        let w = wfun(i as f64 / ptlist.len() as f64, sca);
        let a1 = f64::atan2(
          ptlist[i].y - ptlist[i - 1].y,
          ptlist[i].x - ptlist[i - 1].x,
        );
        let a2 = f64::atan2(
          ptlist[i].y - ptlist[i + 1].y,
          ptlist[i].x - ptlist[i + 1].x,
        );
        let mut a = (a1 + a2) / 2.;
        if a < a2 {
          a += PI;
        }
        vtxlist0.push(Point {
          x: ptlist[i].x + w * a.cos(),
          y: ptlist[i].y + w * a.sin(),
        });
        vtxlist1.push(Point {
          x: ptlist[i].x - w * a.cos(),
          y:ptlist[i].y - w * a.sin(),
        });
      }
      let l = ptlist.len() - 1;
      let a0 =
        f64::atan2(ptlist[1].y - ptlist[0].y, ptlist[1].x - ptlist[0].x) -
        PI / 2.;
      let a1 =
        f64::atan2(
          ptlist[l].y - ptlist[l - 1].y,
          ptlist[l].x - ptlist[l - 1].x,
        ) -
        PI / 2.;
      let w0 = wfun(0., sca);
      let w1 = wfun(1., sca);
    vtxlist0[0] = Point {
        x: ptlist[0].x + w0 * a0.cos(),
        y: ptlist[0].y + w0 * a0.sin(),
      };
    vtxlist1[0] = Point {
        x: ptlist[0].x - w0 * a0.cos(),
        y: ptlist[0].y - w0 * a0.sin(),
      };
    vtxlist0.push(Point {
        x: ptlist[l].x + w1 * a1.cos(),
        y: ptlist[l].y + w1 * a1.sin(),
      });
    vtxlist1.push(Point {
        x: ptlist[l].x - w1 * a1.cos(),
        y: ptlist[l].y - w1 * a1.sin(),
    });
    (vtxlist0, vtxlist1)
}
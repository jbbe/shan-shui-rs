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

    pub fn default03() -> TreeArgs {
        TreeArgs {
            height: 50.,
            width: 5.,
            clu: 5.,
            col: "rgba(100,100,100,0.5)".to_string(),
            noi: 0.5,
        }
    }
    
    pub fn default04() -> TreeArgs {
        TreeArgs {
            height: 300.,
            width: 6.,
            clu: 5., //unused
            col: "rgba(100,100,100,0.5)".to_string(),
            noi: 0.5,
        }
    }

    pub fn default08() -> Self {
        Self {
            height: 80.,
            width: 1.,
            col: "rgba(100,100,100,0.5)".to_string(),
            noi: 0.5,
            clu: 0., // unused
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
    let mut g = Group::new("tree01".to_string());
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
                ..PolyArgs::default("tree01 p1".to_string())
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
                ..PolyArgs::default("tree01 p2".to_string())
            },
        ));
    g.to_string()
}

pub fn tree02(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> String {
    let clu = args.clu as u8;
    let mut g = Group::new("tree02".to_string());
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
    let _bp = 1;
    let ben = | x: f64| -> f64 {
        // this comes from vegetate mountain bottom the only place tree03 is called
        x * bc
    };

    let mut g = Group::new("tree03".to_string());
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
            for _ in 0..((reso - i) * 2) {
                let shape = |x| {
                    f64::log(50. * x + 1., std::f64::consts::E) / 3.95
                };
                let ox = noise.rand() * args.width * shape((resof - i_f) / resof);
                let r_choice = noise.rand_choice_arr(&[-1., 1.]);
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
        ..PolyArgs::default("tree03".to_string())
    }));
    g.to_string()
}


pub fn tree04(noise: &mut Noise, x: f64, y: f64, args: TreeArgs) -> String {

    let mut tx_canv = Group::new("tx".to_string());

    let tr_list_tuple = branch(noise, args.height, args.width,  -PI / 2., 10., PI * 0.2);
    let mut tr_0 = tr_list_tuple.0.clone();
    let mut tr_1 = tr_list_tuple.1.clone();
    tx_canv.add(barkify(noise, x, y,tr_list_tuple));
    tr_1.reverse();
    tr_0.append(&mut tr_1);
    // let mut tr_lists = branch(noise, args.height, args.width, 0., 10., -PI / 2.);
    let tr_list = tr_0;


    let mut tw_canv = Group::new("twcnv".to_string());
    let mut trmlist = vec![];
  

    let tr_len = tr_list.len();
    let len_f = tr_len as f64;
    // // let tr_1 = tr_list_tuple.1.reverse();
    // tr_list_tuple.1.reverse();
    // tr_list.0.append()
    let mut g = Group::new("tr04".to_string());
    for  i in 0..tr_len {
      let i_f = i as f64;
      if (i_f >= len_f * 0.3 &&
          i_f <= len_f * 0.7 &&
            noise.rand() < 0.1) ||
            i_f == len_f / 2. - 1. {
          let ba = PI * 0.2 - PI * 1.4 * if i as f64 > len_f / 2. { 1. } else { 0. };
          let height = args.height * (noise.rand() + 1.) * 0.3;
          let mut brlists = branch(noise,
            height,
             args.width * 0.5,
             ba,
             10.,
             PI * 0.2
          );

          brlists.0.truncate(1);
          brlists.1.truncate(1); // should be equiv splic(0, 1)
          let foff = |v: &Point| {
            Point { x: v.x + tr_list[i].x, y: v.y + tr_list[i].y }
          };
          tx_canv.add(barkify(noise, 
            x,
            y,
            (brlists.0.iter().map(foff).collect(),
            brlists.1.iter().map(foff).collect())));

          for j in 0..(brlists.0.len()) {
            if noise.rand() < 0.2 || j == brlists.0.len() - 1 {
              tw_canv.add(twig(noise,
                brlists.0[j].x + tr_list[i].x + x,
                brlists.0[j].y + tr_list[i].y + y,
                1.,
                TwigArgs {
                  wid: args.height / 300.,
                  ang: if ba > -PI / 2. { ba } else { ba + PI },
                  sca: (0.5 * args.height) / 300.,
                  dir: if ba > -PI / 2. { 1. }  else { -1. },
                  ..TwigArgs::default()
                }
              ).to_string());
            }
          }
          brlists.1.reverse();
          let mut brlist = Vec::with_capacity(brlists.0.len() + brlists.1.len());
          brlist.extend(brlists.0.clone());
          brlist.extend(brlists.1.iter());

          trmlist.extend(
            brlist.iter().map(|v| {
              Point { x: v.x + tr_list[i].x, y: v.y + tr_list[i].y }
            })
          );
        } else {
          trmlist.push(tr_list[i]);
        }
      }
      g.add(poly(&trmlist, PolyArgs { 
        x_off: x,
        y_off: y,
        fil: white(),
        stroke: args.col,
        width: 0.,
        ..PolyArgs::default("tr3".to_string())
      }));

      trmlist.truncate(1);
      trmlist.remove(trmlist.len()  - 1); // trim first and last

      let col = color_a(100,100,100, 0.4 + noise.rand() * 0.1);
      g.add(stroke(noise,
        &trmlist.iter().map(|v| {
          Point { x: v.x + x, y: v.y + y }
        }).collect(),
        StrokeArgs {
          width: 2.5,
          col,
          fun: |_x| {
            1.0_f64.sin()
          },
          noi: 0.9,
          out: 0.,
          ..StrokeArgs::default("tr03s".to_string())
        },
      ));

      g.add(tx_canv.to_string());
      g.add(tw_canv.to_string());
      g.to_string()
}
pub fn tree08(noise: &mut Noise, x: f64, y: f64, args: TreeArgs ) -> String {
    
    let ang = noise.norm_rand(-1., 1.) * PI * 0.2;

    let mut tr_lists = branch(noise, args.height, args.width, -PI / 2. + ang, args.height / 20., PI * 0.2);

    tr_lists.1.reverse();
    tr_lists.0.append(&mut tr_lists.1);
    let tr_list = tr_lists.0; 
    // let mut i = 0;
    // for tr in tr_list {
    //     // if noise.rand() < 0.2 {
    //     //     // fractree
    //     // } else if i == (tr_list.len() as f64 / 2.).floor() as usize {
    //     //     // fractree
    //     // }
    //     i += 1;
    // }

    let mut g = Group::new("tr8".to_string());
    g.add(poly(&tr_list, PolyArgs {
        x_off: x,
        y_off: y,
        fil: white(),
        stroke: args.col,
        width: 0.,
        name: "tr08".to_string(),
    }));
    let a = 0.6 + noise.rand() * 0.1;
    g.add(stroke(noise,
&tr_list.iter().map(|v| { Point { x: v.x + x, y: v.y + y } }).collect(),
        StrokeArgs {
            col: color_a(100, 100, 100, a),
            width: 2.5,
            fun: |_| { 1.0_f64.sin() },
            noi: 0.9,
            out: 0.,
            ..StrokeArgs::default("tr08st".to_string())
        }
    ));
    g.to_string()
}


fn branch(noise: &mut Noise, height: f64, width: f64, ang: f64, det: f64, ben: f64) -> (Vec<Point>, Vec<Point>) {

    let mut t_list = vec![Point { x: 0., y: 0. }];
    let mut a0 = 0.;
    let mut nx = 0.;
    let mut ny = 0.;
    let g = 3;
    for _ in 0..g {
        let r1 = noise.rand();
        a0 += (ben / 2. + (r1 * ben)  / 2.) * noise.rand_choice_arr(&[-1., 1.]);
        nx += (a0.cos() * height) / g as f64;
        ny += (a0.sin() * height) / g as f64;
        t_list.push(Point { x: nx, y: ny });
    }
    let t_last = t_list.len() - 1;
    let ta = t_list[t_last].y.atan2(t_list[t_last].x);

    for i in 0..(t_list.len()) {
        let a = t_list[i].y.atan2(t_list[i].x);
        let d = (t_list[i].x * t_list[i].x + t_list[i].y * t_list[i].y).sqrt();

        t_list[i].x = d * (a -ta + ang).cos();
        t_list[i].y = d * (a -ta + ang).sin();
    }

    let mut tr_list1 = vec![];
    let mut tr_list2 = vec![];
    let span = det;
    let tl = (t_list.len() - 1) as f64 * span;
    let mut lx = 0.;
    let mut ly = 0.;

    for i in 0..(tl.ceil() as usize) {
        let last_p = t_list[(i as f64 / span as f64).floor() as usize];
        let next_p = t_list[(i as f64 / span as f64).ceil() as usize];
        let p = (i as f64 % span) as f64 / span as f64;
        let nx = last_p.x * (1. - p) + next_p.x * p;
        let ny = last_p.y * (1. - p) + next_p.y * p;

        let ang = (ny -ly).atan2(nx - lx);
        let woff = ((noise.noise(i as f64 * 0.3, 0., 0.) - 0.5) * width * height) / 80.;

        let b = if p == 0. {
            noise.rand() * width
        } else {
            0.
        };

        let nw = width * (((tl - i as f64) / tl) * 0.5 + 0.5);

        tr_list1.push(Point {
          x: nx + (ang + PI / 2.).cos() * (nw + woff + b),
          y: ny + (ang + PI / 2.).sin() * (nw + woff + b),
        });
        tr_list2.push(Point {
          x: nx + (ang - PI / 2.).cos() * (nw - woff + b),
          y: ny + (ang - PI / 2.).sin() * (nw - woff + b),
        });
        lx = nx;
        ly = ny;
    }
    (tr_list1, tr_list2)
}

fn barkify(noise: &mut Noise, x: f64, y: f64, trlist: (Vec<Point>, Vec<Point>)) -> String {
      fn bark(noise: &mut Noise,x: f64, y: f64, wid: f64, ang: f64) -> String{
        let len = 10. + 10. * noise.rand();
        let noi = 0.5;
        let fun = |x: f64| -> f64{
          if x <= 1. { (x * PI).sin().powf(0.5) }
            else { -((x + 1.) * PI).sin().powf(0.5) } 
        };
        let reso = 20.0;
        let mut g = Group::new("brkfy".to_string());
        let mut lalist = vec![];
        for i in 0..(reso as usize + 1) {
          let p = (i as f64 / reso) * 2.;
          let xo = len / 2. - (p - 1.).abs() * len;
          let yo = (fun(p) * wid) / 2.;
          let a = yo.atan2(xo);
          let l_letter = (xo * xo + yo * yo).sqrt();
          lalist.push(Point { x: l_letter, y: a });
        }
        let mut nslist = vec![];
        let n0 = noise.rand() * 10.;
        for i in 0..(reso as usize + 1) {
          nslist.push(noise.noise(i as f64 * 0.05, n0, 0.));
        }

        noise.loop_noise(&mut nslist);
        let mut brklist = vec![];
        for i in 0..(lalist.len()) {
          let ns = nslist[i] * noi + (1. - noi);
          let nx = x + (lalist[i].y + ang).cos() * lalist[i].x * ns;
          let ny = y + (lalist[i].y + ang).sin() * lalist[i].x * ns;
          brklist.push(Point { x: nx, y: ny });
        }
        // let frsin = (x + noise.rand()).sin();
        g.add(stroke(noise, &brklist, StrokeArgs {
          width: 0.8,
          noi: 0.,
          col: "rgba(100,100,100,0.4)".to_string(),
          out: 0.,
          fun: |x| {
            // frsin * 
            PI * 3. * x.sin()
          },
          ..StrokeArgs::default("tr04s2".to_string())
        }));
        g.to_string()
      }
      let mut g = Group::new("bark".to_string());
      for i in 2..(trlist.0.len() - 1) {
        let a0 =  trlist.0[i].x - trlist.0[i - 1].y.atan2(  trlist.0[i].x - trlist.0[i - 1].x);
        let a1 = f64::atan2(
          trlist.1[i].y - trlist.1[i - 1].y,
          trlist.1[i].x - trlist.1[i - 1].x,
        );
        let p = noise.rand();
        let nx = trlist.0[i].x * (1 as f64 - p) + trlist.1[i].x * p;
        let ny = trlist.0[i].y * (1 as f64 - p) + trlist.1[i].y * p;
        if noise.rand() < 0.2 {
          g.add(blob(noise, nx + x, ny + y, BlobArgs {
            noi: 1.,
            len: 15.,
            width: 6. - (p - 0.5).abs() * 10.,
            angle: (a0 + a1) / 2.,
            col: color_a(100,100,100,0.6),
            ..BlobArgs::default()
          }));
        } else {
          g.add(bark(noise,
            nx + x,
            ny + y,
            5. - (p - 0.5).abs() * 10.,
            (a0 + a1) / 2.,
          ));
        }

        if noise.rand() < 0.05 {
          let jl = noise.rand() * 2. + 2.;
          let jceil = jl.ceil() as usize;
          let r_choice = (1. * noise.rand()).floor() as usize;
          let choices = [
            [trlist.0[i].x, trlist.0[i].y, a0],
            [trlist.1[i].x, trlist.1[i].y, a1],
          ];
          let xya = choices[r_choice];
          for j in 0..jceil {
            let len =  4. + 6. * noise.rand();
            g.add(blob(noise,
              xya[0] + x + xya[2].cos() * (j as f64 - jl / 2.) * 4.,
              xya[1] + y + xya[2].sin() * (j as f64 - jl / 2.) * 4.,
              BlobArgs {
                width: 4.,
                len,
                angle: a0 + PI / 2.,
                col: color_a(100,100,100,0.6),
                ..BlobArgs::default()
              },
            ));
          }
        }
      }
      let mut trflist = trlist.0.clone();
      let mut trlist2copy = trlist.1.clone();
      trlist2copy.reverse();
      trflist.append(&mut trlist2copy);
      let mut rglist = vec![vec![]];
      for i in 0..(trflist.len()) {
        if noise.rand() < 0.5 {
          rglist.push(vec![]);
        } else {
          let idx = rglist.len() - 1;
          rglist[idx].push(trflist[i]);
        }
  }

  for i in 0..(rglist.len()) {
    // todo improve
    let vd = VecDeque::from_iter(rglist[i].clone());
    rglist[i] = Vec::from_iter(div(&vd, 4.));
    for j in 0..(rglist[i].len()) {
          rglist[i][j].x +=
            (noise.noise(i as f64, j as f64 * 0.1, 1.) - 0.5) * (15. + 5. * noise.rand_gauss());
          rglist[i][j].y +=
            (noise.noise(i as f64, j as f64 * 0.1, 2.) - 0.5) * (15. + 5. * noise.rand_gauss());
        }
        g.add(stroke(noise,
          &rglist[i].iter().map(|v| { Point { x: v.x + x, y: v.y + y } }).collect(),
          StrokeArgs { width: 1.5, col: color_a(100,100,100,0.7), out: 0.,
            ..StrokeArgs::default("string".to_string())},
    ));
  }
  g.to_string()
}

struct TwigArgs {
  dir: f64,
  sca: f64,
  wid: f64,
  ang: f64,
  lea: (bool, f64),
}

impl TwigArgs {
  pub fn default() -> Self {
    Self {
      dir: 1.,
      sca: 1.,
      wid: 1.,
      ang: 0.,
      lea: (true, 12.),
    }
  }
}

fn twig(noise: &mut Noise, tx: f64, ty: f64, dep: f64, args: TwigArgs) -> String {
    let mut g = Group::new("twig".to_string());
    let mut twlist = vec![];
    let tl = 10;
    let hs = noise.rand() * 0.5 + 0.5;

    let a0 = ((noise.rand() * PI) / 6.) * args.dir + args.ang;
    for i in 0..tl {
        let tfun = |_x| { -1. / (i as f64 / tl as f64 + 1.).powi(5) + 1. };
        let mx = args.dir * tfun(i as f64 / tl as f64) * 50. * args.sca * hs;
        let my = -i as f64 * 5. * args.sca;

        let a = my.atan2(mx);
        let d = (mx * mx + my * my).powf(0.5);

        let nx = f64::cos(a + a0) * d;
        let ny = f64::sin(a + a0) * d;

        twlist.push(Point { x: nx + tx, y: ny + ty });
        let dir = args.dir * noise.rand_choice_arr(&[-1., 1.]);
        if (i == ((tl / 3) | 0) || i == (((tl * 2) / 3) | 0)) && dep > 0. {
          g.add(twig(noise, nx + tx, ny + ty, dep - 1., TwigArgs {
            ang: args.ang,
            sca: args.sca * 0.8,
            wid: args.wid,
            dir,
            lea: args.lea,
          }));
        }
        let lea = args.lea;
        if i == tl - 1 && args.lea.0 == true {
          for j in 0..5 {
            let dj = (j as f64 - 2.5) * 5.;
            let width = (6. + 3. * noise.rand()) * args.wid;
            let len = (15. + 12. * noise.rand()) * args.wid;
            let angle = args.ang / 2. + PI / 2. + PI * 0.2 * (noise.rand() - 0.5);
            g.add(blob(noise,
              nx + tx + args.ang.cos() * dj * args.wid,
              ny + ty + (args.ang.sin() * dj - lea.1 / (dep + 1.)) * args.wid,
              BlobArgs {
                width,
                len,
                angle,
                col: color_a(100,100,100, 0.5 + dep * 0.2),
                fun: |x| {
                  if x <= 1. {
                   ((x * PI).sin() * x).powf(0.5) }
                    else { -f64::powf(((x - 2.) * PI * (x - 2.)).sin(), 0.5) }
                },
                ..BlobArgs::default()
              },
            ));
          }
        }
      }
      g.add(stroke(noise, &twlist, StrokeArgs {
        width: 1.,
        fun: |x| {
          ((x * PI) / 2.).cos()
        },
        col: color_a(100,100,100,0.5),
        ..StrokeArgs::default("tre04".to_string())
      }));
  g.to_string()
}
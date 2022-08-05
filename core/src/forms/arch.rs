use std::f64::consts::PI;
use super::super::*;

pub struct Arch01Args {
    pub height: f64,
    pub width: f64,
    pub rot: f64,
    pub per: f64,
}

impl Arch01Args {
    pub fn default() -> Self {
        Self {
            height: 70.,
            width: 180.,
            rot: 0.7,
            per: 5.,
            
        }
    }
}



pub fn arch01(noise: &mut Noise, x_off: f64, y_off: f64, args: Arch01Args) -> String {
    let mut g = draw::Group::new("b01".to_string());
    let seed = 0;

    let p = 0.4 + noise.rand() * 0.2;
    let h0 = args.height * p;
    let h1 = args.height * (1. - p);

    //   group.add(hut(xoff, yoff - hei, { hei: h0, wid: wid }));
    g.add(draw_box(noise, x_off, y_off, BoxArgs {
        height: h1,
        width: (args.width * 2.) / 3.,
        per: args.per,
        bot: false,
        ..BoxArgs::default()
      }));

    //   group.add(rail(xoff, yoff, seed, {
    //     tra: true,
    //     fro: false,
    //     hei: 10,
    //     wid: wid,
    //     per: per * 2,
    //     seg: (3 + Math.random() * 3) | 0,
    //   }));

    //   let mcnt = noise.randChoice([0, 1, 1, 2]);
    //   if (mcnt == 1) {
    //     group.add(Man.man(xoff + normRand(-wid / 3, wid / 3), yoff, {
    //       fli: randChoice([true, false]),
    //       sca: 0.42,
    //     }));
    //   } else if (mcnt == 2) {
    //     group.add(Man.man(xoff + normRand(-wid / 4, -wid / 5), yoff, {
    //       fli: false,
    //       sca: 0.42,
    //     });
    //     group.add(Man.man(xoff + normRand(wid / 5, wid / 4), yoff, {
    //       fli: true,
    //       sca: 0.42,
    //     }));
    //   }
    //   group.add(rail(xoff, yoff, seed, {
    //     tra: false,
    //     fro: true,
    //     hei: 10,
    //     wid: wid,
    //     per: per * 2,
    //     seg: (3 + Math.random() * 3) | 0,
    //   }));


    g.to_string()
}


pub struct DecArgs {
    pub pul: (f64, f64),//[surf * wid * 0.5, -hei],
    pub pur: (f64, f64),//[mid, -hei + per],
    pub pdl: (f64, f64),//[surf * wid * 0.5, 0],
    pub pdr: (f64, f64),//[mid, per],
}

pub struct BoxArgs {
    pub height: f64,
    pub width: f64,
    pub rot: f64,
    pub per: f64,
    pub tra: bool,
    pub bot: bool,
    pub wei: f64,
    pub dec: fn(a: DecArgs) -> Vec<Point>,
}

impl BoxArgs {
    pub fn default() -> Self {
        Self {
            height: 20.,
            width: 280.,
            rot: 0.7,
            per: 5.,
            tra: true,
            bot: true,
            wei: 3.,
            dec: |_a| { vec![] }
        }
    }
}


pub fn draw_box(_noise: &mut Noise, x_off: f64, y_off: f64, args: BoxArgs) -> String {
    let mut g = draw::Group::new("bx".to_string());
    let wid = args.width;
    let rot = args.rot;
    let mid = -wid * 0.5 + wid * rot;
    let bmid = -wid * 0.5 + wid * (1. - rot);
    let per = args.per;
    let hei = args.height;
    let mut ptlist = vec![];

    ptlist.push(div(&VecDeque::from([Point { x: -wid * 0.5, y:  -hei }, Point { x: -wid * 0.5, y: 0. }]), 5.));
    ptlist.push(div(&VecDeque::from([Point { x: wid * 0.5, y:  -hei }, Point { x: wid * 0.5, y: 0. }]), 5.));
      if args.bot {
        ptlist.push(div(&VecDeque::from([Point { x: -wid * 0.5, y:  0. }, Point { x : mid, y: per }]), 5.));
        ptlist.push(div(&VecDeque::from([Point { x: wid * 0.5, y:  0. }, Point { x: mid, y: per }]), 5.));
      }
      ptlist.push(div(&VecDeque::from([Point { x: mid, y:  -hei }, Point { x: mid, y: per }]), 5.));
      if args.tra {
        if args.bot {
          ptlist.push(div(&VecDeque::from([Point { x: -wid * 0.5, y:  0. }, Point { x: bmid, y: -per }]), 5.));
          ptlist.push(div(&VecDeque::from([Point { x: wid * 0.5, y:  0. }, Point { x: bmid, y: -per }]), 5.));
        }
        ptlist.push(div(&VecDeque::from([Point { x: bmid, y:  -hei }, Point { x: bmid, y: -per }]), 5.));
      }

      let surf = if rot < 0.5 { 1. } else { 0. } * 2. - 1.;
      let xx = (args.dec)(DecArgs {
        pul: (surf * wid * 0.5, -hei),
        pur: (mid, -hei + per),
        pdl: (surf * wid * 0.5, 0.0),
        pdr: (mid, per),
      });
      ptlist.push(VecDeque::from(xx));

      let polist = vec![
        Point { x: -wid * 0.5, y: -hei },
        Point { x: wid * 0.5, y: -hei },
        Point { x: wid * 0.5, y: 0. },
        Point { x: mid, y: per },
        Point { x: -wid * 0.5, y: 0. },
      ];

      if !args.tra {
        g.add(poly(&polist, PolyArgs {
          x_off,
          y_off,
          stroke: "none".to_string(),
          fil: "white".to_string(),
          ..PolyArgs::default(Some("bx".to_string()))
        }));
      }
      // TODO fix collect
    //   for  i in 0..(ptlist.len()) {
    //     let p1 = ptlist[i];
    //     let p = p1.iter().map(|x| {
    //         Point { x: x.x + x_off, y: x.y + y_off }
    //       }).collect();
    //     g.add(stroke(noise,
    //       p,
    //       StrokeArgs {
    //         col: color_a(100, 100, 100,0.4),
    //         noi: 1.,
    //         width: args.wei,
    //         fun: |x| {
    //           1.
    //         },
    //         ..StrokeArgs::default("".to_string())
    //       },
    //     ));
    //   }
      g.to_string()

}
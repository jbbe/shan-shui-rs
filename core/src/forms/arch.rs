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
    g.add(draw_box(
        noise,
        x_off,
        y_off,
        BoxArgs {
            height: h1,
            width: (args.width * 2.) / 3.,
            per: args.per,
            bot: false,
            ..BoxArgs::default()
        },
    ));

    let seg = 3. + noise.rand() * 3.;

    g.add(rail(
        noise,
        x_off,
        y_off,
        0.0,
        RailArgs {
            tra: true,
            fro: false,
            height: 10.,
            width: args.width,
            per: args.per * 2.,
            seg,
            ..RailArgs::default()
        },
    ));

    let mcnt = noise.rand_choice_arr(&[0, 1, 1, 2]);
    if mcnt == 1 {
        let fli = noise.rand_choice_arr(&[true, false]);
        let _x_off = x_off + noise.norm_rand(-args.width / 3., args.width / 3.);
        let manArgs = ManArgs {
            fli,
            sca: 0.42,
            ..ManArgs::default(noise)
        };
        g.add(Man::man(
            noise,
            _x_off,
            y_off,
            manArgs,
        ));
    } else if mcnt == 2 {
        let _x_off1 = x_off + noise.norm_rand(-args.width / 4., -args.width / 5.);
        let args1 = ManArgs {
            fli: false,
            sca: 0.42,
            ..ManArgs::default(noise)
        };
        g.add(Man::man(
            noise,
            _x_off1,
            y_off,
            args1,
        ));
        let _x_off2 = x_off + noise.norm_rand(args.width / 5., args.width / 4.);
        let args2 = ManArgs {
            fli: true,
            sca: 0.42,
            ..ManArgs::default(noise)
        };
        g.add(Man::man(
            noise,
            _x_off2,
            y_off,
            args2,
        ));
    }
    let rail_args = RailArgs {
        tra: false,
        fro: true,
        height: 10.,
        width: args.width,
        per: args.per * 2.,
        seg: (3. + noise.rand() * 3.), // | 0
        ..RailArgs::default()
    };
    g.add(rail(
        noise,
        x_off,
        y_off,
        0.,
        rail_args,
    ));

    g.to_string()
}

pub struct DecArgs {
    pub pul: (f64, f64), //[surf * wid * 0.5, -hei],
    pub pur: (f64, f64), //[mid, -hei + per],
    pub pdl: (f64, f64), //[surf * wid * 0.5, 0],
    pub pdr: (f64, f64), //[mid, per],
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
            dec: |_a| vec![],
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
    let mut pt_list = vec![];

    pt_list.push(div(
        &VecDeque::from([
            Point {
                x: -wid * 0.5,
                y: -hei,
            },
            Point {
                x: -wid * 0.5,
                y: 0.,
            },
        ]),
        5.,
    ));
    pt_list.push(div(
        &VecDeque::from([
            Point {
                x: wid * 0.5,
                y: -hei,
            },
            Point {
                x: wid * 0.5,
                y: 0.,
            },
        ]),
        5.,
    ));
    if args.bot {
        pt_list.push(div(
            &VecDeque::from([
                Point {
                    x: -wid * 0.5,
                    y: 0.,
                },
                Point { x: mid, y: per },
            ]),
            5.,
        ));
        pt_list.push(div(
            &VecDeque::from([
                Point {
                    x: wid * 0.5,
                    y: 0.,
                },
                Point { x: mid, y: per },
            ]),
            5.,
        ));
    }
    pt_list.push(div(
        &VecDeque::from([Point { x: mid, y: -hei }, Point { x: mid, y: per }]),
        5.,
    ));
    if args.tra {
        if args.bot {
            pt_list.push(div(
                &VecDeque::from([
                    Point {
                        x: -wid * 0.5,
                        y: 0.,
                    },
                    Point { x: bmid, y: -per },
                ]),
                5.,
            ));
            pt_list.push(div(
                &VecDeque::from([
                    Point {
                        x: wid * 0.5,
                        y: 0.,
                    },
                    Point { x: bmid, y: -per },
                ]),
                5.,
            ));
        }
        pt_list.push(div(
            &VecDeque::from([Point { x: bmid, y: -hei }, Point { x: bmid, y: -per }]),
            5.,
        ));
    }

    let surf = if rot < 0.5 { 1. } else { 0. } * 2. - 1.;
    let xx = (args.dec)(DecArgs {
        pul: (surf * wid * 0.5, -hei),
        pur: (mid, -hei + per),
        pdl: (surf * wid * 0.5, 0.0),
        pdr: (mid, per),
    });
    pt_list.push(VecDeque::from(xx));

    let po_list = vec![
        Point {
            x: -wid * 0.5,
            y: -hei,
        },
        Point {
            x: wid * 0.5,
            y: -hei,
        },
        Point {
            x: wid * 0.5,
            y: 0.,
        },
        Point { x: mid, y: per },
        Point {
            x: -wid * 0.5,
            y: 0.,
        },
    ];

    if !args.tra {
        g.add(poly(
            &po_list,
            PolyArgs {
                x_off,
                y_off,
                stroke: "none".to_string(),
                fil: "white".to_string(),
                ..PolyArgs::default(Some("bx".to_string()))
            },
        ));
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

pub struct RailArgs {
    pub height: f64,
    pub width: f64,
    pub rot: f64,
    pub per: f64,
    pub seg: f64,
    pub wei: f64,
    pub tra: bool,
    pub fro: bool,
}

impl RailArgs {
    pub fn default() -> Self {
        Self {
            height: 20.,
            width: 180.,
            rot: 0.7,
            per: 4.,
            seg: 4.,
            wei: 1.,
            tra: true,
            fro: true,
        }
    }
}

fn rail(noise: &mut Noise, x_off: f64, y_off: f64, seed: f64, args: RailArgs) -> String {
    let hei = args.height;
    let wid = args.width;
    let rot = args.rot;
    let per = args.per;
    let seg = args.seg;
    let wei = args.wei;
    let tra = args.tra;
    let fro = args.fro;

    let mid = -wid * 0.5 + wid * rot;
    let bmid = -wid * 0.5 + wid * (1. - rot);
    let mut pt_list = vec![];

    if fro {
        pt_list.push(div(
            &VecDeque::from([
                Point {
                    x: -wid * 0.5,
                    y: 0.,
                },
                Point { x: mid, y: per },
            ]),
            seg,
        ));
        pt_list.push(div(
            &VecDeque::from([
                Point { x: mid, y: per },
                Point {
                    x: wid * 0.5,
                    y: 0.,
                },
            ]),
            seg,
        ));
    }
    if tra {
        pt_list.push(div(
            &VecDeque::from([
                Point {
                    x: -wid * 0.5,
                    y: 0.,
                },
                Point { x: bmid, y: -per },
            ]),
            seg,
        ));
        pt_list.push(div(
            &VecDeque::from([
                Point { x: bmid, y: -per },
                Point {
                    x: wid * 0.5,
                    y: 0.,
                },
            ]),
            seg,
        ));
    }
    if fro {
        pt_list.push(div(
            &VecDeque::from([
                Point {
                    x: -wid * 0.5,
                    y: -hei,
                },
                Point {
                    x: mid,
                    y: -hei + per,
                },
            ]),
            seg,
        ));
        pt_list.push(div(
            &VecDeque::from([
                Point {
                    x: mid,
                    y: -hei + per,
                },
                Point {
                    x: wid * 0.5,
                    y: -hei,
                },
            ]),
            seg,
        ));
    }
    if tra {
        pt_list.push(div(
            &VecDeque::from([
                Point {
                    x: -wid * 0.5,
                    y: -hei,
                },
                Point {
                    x: bmid,
                    y: -hei - per,
                },
            ]),
            seg,
        ));
        pt_list.push(div(
            &VecDeque::from([
                Point {
                    x: bmid,
                    y: -hei - per,
                },
                Point {
                    x: wid * 0.5,
                    y: -hei,
                },
            ]),
            seg,
        ));
    }
    if tra {
        let open = f64::floor(noise.rand() * pt_list.len() as f64) as usize;
        pt_list[open].pop_back();
        let len = pt_list.len();
        pt_list[(open +len ) % len].pop_back();
    }
    let mut g = Group::new("g".to_string());

    for i in 0..(pt_list.len() / 2) {
        for j in 0..(pt_list[i].len()) {
            //ptlist.push(div([ptlist[i][j],ptlist[4+i][j]],2))
            pt_list[i][j].y += (noise.noise(i as f64, j as f64 * 0.5, seed) - 0.5) * hei;

            let k = (pt_list.len() / 2 + i) % pt_list.len();
            let m = j % pt_list[(pt_list.len() / 2 + i) % pt_list.len()].len();
            pt_list[k][m].y += (noise.noise(i as f64 + 0.5, j as f64 * 0.5, seed) - 0.5) * hei;

            // let mut ln = div(VecDeque::from([
            //     pt_list[i][j],
            //     pt_list[(pt_list.len() / 2 + i) % pt_list.len()][
            //       j % pt_list[(pt_list.len() / 2 + i) % pt_list.len()].len()
            //     ],
            //   ],
            //   2,
            // );
            // ln[0].x += (noise.random() - 0.5) * hei * 0.5;
            // g.add(poly(ln, PolyArgs {
            //    x_off,
            //    y_off,
            //   fil: "none".to_string(),
            //   stroke: color_a(100,100,100,0.5),
            //   width: 2.,
            //   name: Some("rps".to_string())
            // });
        }
    }

    for i in 0..(pt_list.len()) {
        g.add(stroke(
            noise,
            &(pt_list[i]
                .iter()
                .map(|p| Point {
                    x: p.x + x_off,
                    y: p.y + y_off,
                })
                .collect()),
            StrokeArgs {
                col: color_a(100, 100, 100, 0.5),
                noi: 0.5,
                width: wei,
                fun: |x| 1.,
                ..StrokeArgs::default("rs".to_string())
            },
        ));
    }
    g.to_string()
}

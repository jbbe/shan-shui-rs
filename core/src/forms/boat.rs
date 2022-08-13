use std::f64::consts::PI;
use super::super::*;

pub struct BoatArgs {
    pub len: f64,
    pub scale: f64,
    pub fli: bool,
}

impl BoatArgs {
    pub fn default() -> Self {
        Self {
            len: 120.,
            scale: 1.,
            fli: false,
}
    }
}

pub fn boat01(noise: &mut Noise, x_off: f64, y_off: f64, args: BoatArgs) -> String {
    let mut g = draw::Group::new("b01".to_string());
    let dir = if args.fli { -1. } else { 1. };
    let man_args = ManArgs {
            ite: |n, p0, p1| Man::stick01(n, p0, p1, StickArgs::default()),
            hat: Man::hat02,
            sca: 0.5 * (args.scale),
            fli: !(args.fli),
            len: [0., 30., 20., 30., 10., 30., 30., 30., 30.],
            ..ManArgs::default(noise)
    };
    g.add(Man::man(noise,x_off + 20. * (args.scale) * dir, y_off, man_args));
    let cnt =  (args.len * args.scale) as usize;
    let mut p_list1 = vec![];
    let mut p_list2 = vec![];
    p_list1.reserve(cnt);
    p_list2.reserve(cnt);

    let fun1 = |x: f64| -> f64 {
        (x * PI).sin().powf(0.5) * 7. * args.scale
    };
    let fun2 = |x: f64| {
        (x * PI).sin().powf(0.5) * 10. * args.scale
    };

    let mut i = 0.;
    while i < args.len * args.scale {
        p_list1.push(Point { x: i * dir, y: fun1(i / args.len)});
        p_list2.push(Point { x: i * dir, y: fun2(i / args.len)});
        i += 5. * args.scale;
    }
    p_list2.reverse();
    p_list1.append(&mut p_list2);
    g.add(poly(&p_list1, PolyArgs{ x_off, y_off, fil: white(), ..PolyArgs::default("boat".to_string())}));

    g.add(stroke(noise, 
        &p_list1.iter().map(|v|  Point { x: x_off + v.x, y: y_off + v.y}).collect(),
        StrokeArgs {
            width: 1.,
            fun: |x| { (x * PI * 2.).sin()},
            col: color_a(100, 100, 100, 0.4),
            ..StrokeArgs::default("boat".to_string())
        }));
    g.to_string()
}

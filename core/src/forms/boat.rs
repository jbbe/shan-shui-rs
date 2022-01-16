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
    let mut g = draw::Group::new();
    let dir = if args.fli { -1. } else { 1. };
    g.add(Man::man(
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

    g.to_string()
}

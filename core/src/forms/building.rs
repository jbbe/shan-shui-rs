use super::super::*;

pub struct TowerArgs {
    height: f64,
    width: f64,
}

impl TowerArgs {
    pub fn default() -> Self {
        Self {
            height: 100.,
            width: 20.
        }
    }
}
fn to_global (v: &Point, x_off: f64, y_off: f64) -> Point {
    Point { x: v.x + x_off, y: v.y + y_off }
}

fn quickstroke(noise: &mut Noise, pl:&VecDeque<Point>, x_off: f64, y_off: f64 ) -> String {
    let divved = div(pl, 5.);
    let x : Vec<Point> = divved.iter().map(|p| { to_global(p, x_off, y_off) }).collect();
  stroke(noise, &x, StrokeArgs {
    width: 1.,
    fun: |_x| { 0.5 },
    col: color_a(100,100,100,0.4),
    ..StrokeArgs::default("name".to_string())
  })
}

pub fn transmission_tower(noise: &mut Noise, x_off: f64, y_off: f64, args: TowerArgs) -> String {
    let mut g = Group::new("tran-twr".to_string());
    let wid = args.width;
    let hei = args.height;

    let p00 = Point { x: -wid * 0.05, y: -hei };
    let p01 = Point { x: wid * 0.05, y: -hei };

    let p10 = Point { x: -wid * 0.1, y: -hei * 0.9 };
    let p11 = Point { x: wid * 0.1, y:-hei * 0.9 };

    let p20 = Point { x: -wid * 0.2, y: -hei * 0.5 };
    let p21 = Point { x: wid * 0.2, y: -hei * 0.5 };

    let p30 = Point { x: -wid * 0.5,y:  0. };
    let p31 = Point { x: wid * 0.5, y: 0. };

    let bch = vec![Point { x: 0.7,y: -0.85 }, Point { x: 1., y: -0.675 }, Point { x: 0.7, y: -0.5}];
//let forloop1: Vec<String> = 
    (0..3).map(|i| {
        vec![quickstroke(noise, &VecDeque::from([
            Point { x: -bch[i].x * wid, y: bch[i].y * hei },
            Point { x: bch[i].x * wid, y: bch[i].y * hei },
          ]), x_off, y_off),
          quickstroke(noise, &VecDeque::from([
            Point { x: -bch[i].x * wid, y: bch[i].y * hei },
            Point { x: 0., y: (bch[i].y - 0.05) * hei },
          ]), x_off, y_off),
          quickstroke(noise, &VecDeque::from([
            Point { x: bch[i].x * wid, y: bch[i].y * hei },
            Point { x: 0., y: (bch[i].y - 0.05) * hei },
          ]), x_off, y_off),
          quickstroke(noise, &VecDeque::from([
            Point { x: -bch[i].x * wid, y: bch[i].y * hei },
            Point { x: -bch[i].x * wid, y: (bch[i].y + 0.1) * hei },
          ]), x_off, y_off),
          quickstroke(noise, &VecDeque::from([
            Point { x: bch[i].y * wid, y: bch[i].y * hei },
            Point { x: bch[i].x * wid, y: (bch[i].y + 0.1) * hei },
          ]), x_off, y_off)]
    }).flatten().for_each(|s| { g.add(s) });

    let l10 = div(&VecDeque::from([p00, p10, p20, p30]), 5.);
    let l11 = div(&VecDeque::from([p01, p11, p21, p31]), 5.);

    for i in 0..(l10.len() - 1) {
      g.add(quickstroke(noise, &VecDeque::from([l10[i], l11[i + 1]]), x_off, y_off));
      g.add(quickstroke(noise, &VecDeque::from([l11[i], l10[i + 1]]), x_off, y_off));
    }

    g.add(quickstroke(noise, &VecDeque::from([p00, p01]), x_off, y_off));
    g.add(quickstroke(noise, &VecDeque::from([p10, p11]), x_off, y_off));
    g.add(quickstroke(noise, &VecDeque::from([p20, p21]), x_off, y_off));
    g.add(quickstroke(noise, &VecDeque::from([p00, p10, p20, p30]), x_off, y_off));
    g.add(quickstroke(noise, &VecDeque::from([p01, p11, p21, p31]), x_off, y_off));

    g.to_string()
}
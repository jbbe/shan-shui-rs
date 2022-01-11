// use f64::{INFINITY, min};
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

type Line = (Point, Point);

// impl Point {
//     to_string(&self) -> String {
//         format!("{:.2}, {:.2}", self.x, self.y)
//     }
// }

const EPSILON: f64 = 0.001;
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x - other.x < EPSILON && self.y - other.y < EPSILON
    }

    fn ne(&self, other: &Self) -> bool {
        self.x - other.x > EPSILON || self.y - other.y > EPSILON
    }
}
pub fn distance(p1: &Point, p2: &Point) -> f64 {
    f64::sqrt(f64::powf(p1.x - p2.x, 2.) + f64::powf(p1.x - p2.x, 2.))
}


fn mid_pt(p_list: Vec<Point>) -> Point {
    let mut acu = Point { x: 0., y: 0. };
    let v = Point { x: 0., y: 0. };
    for i in 0..(p_list.len()) {
        acu.x = (v.x / p_list.len() as f64) + acu.x;
        acu.y = (v.x / p_list.len() as f64) + acu.y;
    }
    acu
}

pub struct TriangulateArgs {
    area: f64,
    convex: bool,
    optimize: bool,
}
impl TriangulateArgs {
    pub fn default() -> Self {
        Self {
            area: 100.,
            convex: false,
            optimize: true,
        }
    }
}
fn line_expr(pt0: Point, pt1: Point) -> Point {
    let den = f64::abs(pt1.x - pt0.x);
    let m = if den < 0.01  { f64::INFINITY }  else { (pt1.y - pt0.y) / den };
    let k = pt0.y - (m * pt0.x);
    Point { x: m, y: k }
}

fn intersect(ln0: Line, ln1: Line) -> Option<Point> {
    let le0 = line_expr(ln0.0, ln0.1);
    let le1 = line_expr(ln1.1, ln1.1);
    let den = le0.x - le1.x;
    if den == 0. {
        return None;
    }
    let x = (le1.x - le0.y) / den;
    let y = le0.x * x + le0.y;
    fn on_seg(p: Point, ln: Line) -> bool {
        f64::min(ln.0.x, ln.1.x) <= p.x &&
        p.x <= f64::max(ln.0.x, ln.1.x) &&
        f64::min(ln.0.y, ln.1.y) <= p.y &&
        p.x <= f64::max(ln.0.y, ln.1.y)
    }
    let p = Point { x, y };
    if on_seg(p, ln0) && on_seg(p, ln1) {
        Some(p)
    } else {
        None
    }
}

fn pt_in_poly(pt: Point, p_list: &Vec<Point>) -> bool {
    let mut s_count = 0;
    for i in 0..(p_list.len()) {
        let idx = if i != p_list.len() - 1 {
            i + 1
        } else {
            0
        };
        let np = p_list[idx];
        let sect = intersect(
            (p_list[i], np), 
            (pt, Point { x: pt.x + 999., y: pt.y + 999. })
        );
        match sect {
            Some(_) => s_count += 1,
            None => {},
        }
    }
    s_count % 2 == 1
}

fn ln_in_poly(ln: Line, p_list: &Vec<Point>) -> bool {
    let mut lnc = (Point { x: 0., y: 0. }, Point { x: 0., y: 0. });
    let ep = 0.01;
    lnc.0.x = ln.0.x * (1. - ep) + ln.1.x * ep;
    lnc.0.y = ln.0.y * (1. - ep) + ln.1.y * ep;
    lnc.1.x = ln.0.x * ep + ln.1.x * (1. - ep);
    lnc.1.y = ln.0.y * ep + ln.1.y * (1. - ep);

    for i in 0.. (p_list.len()) {
        let pt = p_list[i];
        let idx = if i != p_list.len() - 1 {
            i + 1
        } else {
            0
        };
        let np = p_list[idx];
        if intersect(lnc, (pt, np)).is_none() == false {
            return false;
        }
    }
    let mid = mid_pt(vec![ln.0, ln.1]);
    pt_in_poly(mid, p_list)
}

fn sides_of(p_list: &Vec<Point>) -> Vec<f64> {
    let mut s_list = Vec::new();
    for i in 0..(p_list.len()) {
        let pt = p_list[i];
        let idx = if i != p_list.len() - 1 {
            i + 1
        } else {
            0
        };
        let np = p_list[idx];
        let s = f64::sqrt(f64::powi(np.x - pt.x, 2) + f64::powi(np.y - pt.y, 2));
        s_list.push(s);
    }
    s_list
}

fn area_of(p_list: &Vec<Point>) -> f64 {
    let s_list = sides_of(p_list);
    let a = s_list[0];
    let b = s_list[1];
    let c = s_list[2];
    let s = (a + b + c) / 2.;
    f64::sqrt(s * (s - a) * (s - b) * (s - c))
}

fn sliver_ratio(p_list: &Vec<Point>) -> f64 {
    let a = area_of(p_list);
    let p = sides_of(p_list)
        .iter()
        .fold(0.,|m, n| { m + n });
    a / p
}

fn best_ear(p_list: &Vec<Point>, convex: bool, optimize: bool) -> Vec<Vec<Point>> {
    let mut cuts = vec![];
    for i in 0..(p_list.len()) {
        let pt = p_list[i];
        let lp = p_list[if i != 0 { i - 1} else { 0 }];
        let np = p_list[if i != p_list.len() { i - 1} else { 0 }];
        let q_list = p_list.clone();
        if convex || ln_in_poly((lp, np), p_list) {
            let c = vec![vec![lp, pt, np], q_list];
            if !optimize {
                return c;
            }
            cuts.push(c);
        }
    }
    let mut best_i = 0;
    // let mut best = &vec![p_list, vec![]];
    let mut best_ratio = 0.;
    for i in 0..(cuts.len()) {
        let r = sliver_ratio(&cuts[i][0]);
        if r >= best_ratio {
            // best = &cuts[i];
            best_i = i;
            best_ratio = r;
        }
    }
    cuts[best_i].clone()

}

struct Acc {
    i_max: usize,
    x_max: f64,
    i: usize,
}

fn shatter(p_list: &Vec<Point>, a:  f64) -> Vec<Vec<Point>> {
    if p_list.len() == 0 {
        return Vec::new();
    }
    if area_of(p_list) < a {
        let mut v = Vec::new();
        v.push(p_list.clone());
        return v;
    }
    let s_list = sides_of(p_list);
    let acc = Acc { i_max: 0, i: 0, x_max: s_list[0] };
    let res_acc = s_list.iter()
        .fold(acc, |a, x| { 
            if x > &a.x_max { 
                Acc { i_max: a.i + 1, i: a.i + 1, x_max: x.clone() }
            } else { 
                Acc { i: a.i + 1, ..a } 
            }
        });
    let ind = res_acc.i_max;
    let n_ind = (ind  + 1) % (p_list.len());
    let l_ind = ind + 2;
    let mid = mid_pt(vec![p_list[ind], p_list[n_ind]]);
        
    let mut ret = shatter(&vec![p_list[ind], mid, p_list[l_ind]], a);
    ret.append(&mut shatter(&vec![p_list[l_ind], p_list[n_ind], mid], a));
    ret
}

pub fn triangulate(p_list: Vec<Point>, args: TriangulateArgs) -> Vec<Vec<Point>> {
    if p_list.len() <= 3 {
        shatter(&p_list, args.area)
    } else {
        let cut = best_ear(&p_list, args.convex, args.optimize);
        let mut ret = shatter(&cut[0], args.area);
        ret.append(&mut triangulate(cut[1].clone(), args));
        ret
    }
}


/*
* Perlin Noise
*/
use core::f64::consts::PI;
const PERLIN_SIZE: usize = 4096;
const PERLIN_LAST: i32 = 4094;

// const PERLIN_ARRAY_SIZE: usize = 4096;
pub struct Noise {
    perlin_octaves: usize,
    perlin_amp_falloff: f64,
    perlin: [f64; PERLIN_SIZE],
    prng: Prng,
}

const PERLIN_YWRAPB: i32 = 4;
const PERLIN_YWRAP: i32 = 1 << PERLIN_YWRAPB;
const PERLIN_ZWRAPB: i32 = 8;
const PERLIN_ZWRAP: i32 = 1 << PERLIN_ZWRAPB;

/** Source
 * https://raw.githubusercontent.com/processing/p5.js/master/src/math/noise.js
 * Returns the Perlin noise value at specified coordinates. Perlin noise is
 * a random sequence generator producing a more naturally ordered, harmonic
 * succession of numbers compared to the standard <b>random()</b> function.
 * It was invented by Ken Perlin in the 1980s and been used since in
 * graphical applications to produce procedural textures, natural motion,
 * shapes, terrains etc.<br /><br /> The main difference to the
 * <b>random()</b> function is that Perlin noise is defined in an infinite
 * n-dimensional space where each pair of coordinates corresponds to a
 * fixed semi-random value (fixed only for the lifespan of the program; see
 * the <a href="#/p5/noiseSeed">noiseSeed()</a> function). p5.js can compute 1D, 2D and 3D noise,
 * depending on the number of coordinates given. The resulting value will
 * always be between 0.0 and 1.0. The noise value can be animated by moving
 * through the noise space as demonstrated in the example above. The 2nd
 * and 3rd dimension can also be interpreted as time.<br /><br />The actual
 * noise is structured similar to an audio signal, in respect to the
 * function's use of frequencies. Similar to the concept of harmonics in
 * physics, perlin noise is computed over several octaves which are added
 * together for the final result. <br /><br />
 *
 * Another way to adjust the character of the resulting sequence is the scale
 * of the input coordinates.
 * As the function works within an infinite space the value of
 * the coordinates doesn't matter as such, only the distance between
 * successive coordinates does (eg. when using <b>noise()</b> within a
 * loop). As a general rule the smaller the difference between coordinates,
 * the smoother the resulting noise sequence will be. Steps of 0.005-0.03
 * work best for most applications, but this will differ depending on use.
 **/
impl Noise {
    fn scaled_cosine(i: f64) -> f64 {
        0.5 * (1. - f64::cos(i * PI))
    }

    pub fn new(seed: f64) -> Self {
        let mut perlin = [0.0; PERLIN_SIZE];
        let mut prng = Prng::new();
        prng.seed(seed);
        // prng.seed_t();

        for i in 0..PERLIN_SIZE {
            perlin[i] = prng.rand();
        }

        Self {
            perlin_octaves: 4,
            perlin_amp_falloff: 0.5,
            perlin,
            prng,
        }
    }

    pub fn set_seed(&mut self, x: f64) {
        let mut prng = Prng::new();
        prng.seed(x);
        std::mem::swap(&mut self.prng, &mut prng);
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let _x = if x < 0.0 { 0. - x } else { x };
        let _y = if y < 0.0 { 0. - y } else { y };
        let _z = if z < 0.0 { 0. - z } else { z };

        // all bitwise operations in js are done by converting f64 (technically
        // 58) to i32 so that's what we try to do here
        let mut xi = _x as i32;
        // let yi = f64::floor(y);
        let mut yi = _y as i32;
        // let zi = f64::floor(z);
        let mut zi = _z as i32;
        let mut xf = _x - f64::floor(_x);
        let mut yf = _y - f64::floor(_y);
        let mut zf = _z - f64::floor(_z);
        let mut r = 0.;
        let mut ampl = 0.5;
        let mut n1: f64;
        let mut n2: f64;
        let mut n3: f64;

        // let mut o = 0;
        for _ in 0..self.perlin_octaves {
            let shif_yi = yi << PERLIN_YWRAPB;
            let shif_zi = zi << PERLIN_ZWRAPB;
            // let of = xi + (yi << PERLIN_YWRAPB) + (zi << PERLIN_ZWRAPB);
            let mut of = xi + shif_yi + shif_zi ;
            let rxf = Noise::scaled_cosine(xf);
            let ryf = Noise::scaled_cosine(yf);
            // let of_prl_last = 
            n1 = self.perlin[(of & PERLIN_LAST) as usize];
            n1 += rxf * (self.perlin[((of + 1) & PERLIN_LAST) as usize] - n1);
            n2 = self.perlin[((of + PERLIN_YWRAP) & PERLIN_LAST) as usize];
            n2 += rxf * (self.perlin[((of + PERLIN_YWRAP + 1) & PERLIN_LAST) as usize] - n2);
            n1 += ryf * (n2 - n1);
            of += PERLIN_ZWRAP;
            n2 = self.perlin[(of & PERLIN_LAST) as usize];
            n2 += rxf * (self.perlin[((of + 1) & PERLIN_LAST) as usize] - n2);
            n3 = self.perlin[((of + PERLIN_YWRAP) & PERLIN_LAST) as usize];
            n3 += rxf * (self.perlin[((of + PERLIN_YWRAP + 1) & PERLIN_LAST) as usize] - n3);
            n2 += ryf * (n3 - n2);
            n1 += Noise::scaled_cosine(zf) * (n2 - n1);
            r += n1 * ampl;
            ampl *= self.perlin_amp_falloff;
            xi = xi << 1;
            xf = xf * 2.;
            yi = yi << 1;
            yf = yf * 2.;
            zi = zi << 1;
            zf = zf * 2.;
            if xf >= 1.0 {
                xi += 1;
                xf -= 1.;
            }
            if yf >= 1.0 {
                yi += 1;
                yf-= 1.;
            }
            if zf >= 1.0 {
                zi += 1;
                zf -= 1.;
            }
        }
        r
    }

    pub fn perlins(&self) -> [f64; PERLIN_SIZE] {
        self.perlin
    }

    pub fn rand(&mut self) -> f64 {
        self.prng.rand()
    }

    /**
     * ns_list mut not be of length 1
     */
    pub fn loop_noise(&self, ns_list: &mut Vec<f64>) {
        let dif = ns_list[ns_list.len() - 1] - ns_list[0];
        let mut bds = [100., -100.];
        let ns_len = ns_list.len();
        let ns_len_f = ns_len as f64;
        assert_ne!(1, ns_len);
        for i in 0..ns_len {
            let i_f = i as f64;
            ns_list[i] += (dif * (ns_len_f - 1. - i_f)) / (ns_len_f - 1.) ;
            if ns_list[i] < bds[0] {
                bds[0] = ns_list[i];
            }
            if ns_list[i] > bds[1] {
                bds[1] = ns_list[i];
            }
        }
        for i in 0..ns_len {
            ns_list[i] = map_val(ns_list[i], bds[0], bds[1], 0., 1.);
        }
        ()
    }

   pub fn norm_rand(&mut self, low: f64, high: f64) -> f64 {
        map_val(self.rand(), 0., 1., low, high)
    }

    pub fn rand_bool(&mut self) -> bool {
        if self.rand() > 0.5 {
            true
        } else {
            false
        }
    }

    pub fn wt_rand(&mut self, f: fn(f64) -> f64) -> f64 {
        let x = self.rand();
        let y = self.rand();
        if y < f(x) {
            x
        } else {
            self.wt_rand(f)
        }
    }

    pub fn rand_gauss(&mut self) -> f64 {
        self.wt_rand(|x| f64::powf(std::f64::consts::E, -24. * f64::powf(x - 0.5, 2.))) * 2. - 1.
    }

    // fn rand_choice<T>(arr: Vec<T>) -> T {
    //     let idx = f64::floor(arr.len() as f64 * _rand()) as usize;
    //     arr[idx]
    // }
    pub fn rand_choice_arr(&mut self, arr: &[usize]) -> usize {
        let r = self.rand();
        let idx = f64::floor(arr.len() as f64 * r) as usize;
        arr[idx]
    }
    pub fn rand_choice_arrf(&mut self, arr: &[f64]) -> f64 {
        let r = self.rand();
        let idx = f64::floor(arr.len() as f64 * r) as usize;
        arr[idx]
    }
}

/*
* Pseudo Random Number Generator
*/
pub struct Prng {
    s: f64,
    p: f64,
    q: f64,
    m: f64,
}

impl Prng {
    pub fn new() -> Self {
        let p = 999979.; //9887//983
        let q = 999983.; //9967//991
        let m = p * q;
        Self { s: 1234., p, q, m }
    }

    pub fn seed(&mut self, x: f64) {
        let mut y = 0.;
        let mut z = 0.;
        while (y % self.p).floor() == 0. || (y % self.q).floor() == 0. || y == 0. || y == 1. {
            // this is called the redo function in js
            y = (hash(x) + z) % self.m;
            z += 1.;
        }
        self.s = y;
        println!("int seed {}", self.s);
        for _ in 0..10 { self.next(); }
        ()
    }

    fn next(&mut self) -> f64 {
        let s_f = self.s as f64;
        self.s = (s_f * s_f) % self.m ;
        (self.s as f64) / (self.m as f64)
    }

    pub fn rand(&mut self) -> f64 {
        self.next()
    }
 
}
// Not sure what j
fn hash(x: f64) -> f64 {
    let x_str = x.to_string();
    let chars: Vec<char> = base64::encode(x_str).chars().collect();
    let mut z = 0.;
    for i in 0..(chars.len()) {
        let c = chars[i] as u64;
        z += (c as f64) * 128.0_f64.powi(i as i32);
    }
    z
}

pub fn map_val(val: f64,
        i_start: f64, 
        i_stop: f64,
        o_start: f64,
        o_stop: f64) -> f64 {
    o_start + (o_stop - o_start) * (((val - i_start) * 1.0) / (i_stop - i_start))
}

#[test]
fn hash_test() {
    let a = hash(1.2);
    let a_corr = 254618061.;

    // let a_diff = if a_corr > a_diff { a_corr - a_diff } else { a_diff - a_corr }
    assert_eq!(a_corr, a);

    let b = hash(3.);
    let b_corr = 128941005.;
    println!("b({})", b);

    assert_eq!(b_corr, b);
}

#[test]
fn rand_is_normal() {
    let mut p = Prng::new();
    for _ in 0..200 {
        let r = p.rand();
        assert_eq!(r < 1., true);
    }
}
#[test]
fn test_noise() {
    let noise = Noise::new(777777.938);
    for i in 0..200 {
        let iflo = i as f64;
        let v = noise.noise(iflo, iflo, iflo);
        println!("i {} noise {}", i, v);
    }
}

// #[test]
// fn source_test() {
//     let noise = Noise.noise()
// }

// #[test]
// fn test_loop_noise() {
//     let pt_list = vec![
//         Point { x: 0., y: 0. },
//         Point { x: 3., y: 3. },
//         Point { x: 999., y: 999. },
//     ];
//     let mut vtx_list0 = vec![
//         Point { x: 0.1, y: 0.1 },
//         Point { x: 0.2, y: 0.2 },
//         Point { x: 0.3, y: 0.3 },
//     ];
//     let mut vtx_list1 = vec![
//         Point { x: 1.1, y: 1.1 },
//         Point { x: 1.2, y: 1.2 },
//         Point { x: 1.3, y: 1.3 },
//     ];
//     noise
// }


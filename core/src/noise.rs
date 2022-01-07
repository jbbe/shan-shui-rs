use super::random::_rand;
/*
* Perlin Noise
*/
const PERLIN_SIZE: usize = 4096;
const PERLIN_LAST: usize = 4094;

const PI: f64 = std::f64::consts::PI;
// const PERLIN_ARRAY_SIZE: usize = 4096;
pub struct Noise {
    perlin_octaves: usize,
    perlin_amp_falloff: f64,
    perlin: [f64; PERLIN_SIZE],
}

const PERLIN_YWRAPB: usize = 4;
const PERLIN_YWRAP: usize = 1 << PERLIN_YWRAPB;
const PERLIN_ZWRAPB: usize = 8;
const PERLIN_ZWRAP: usize = 1 << PERLIN_ZWRAPB;


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

    pub fn new() -> Self {
        let mut perlin = [0.0; PERLIN_SIZE];

        for i in 0..PERLIN_SIZE {
            perlin[i] = _rand();
        }

        Self {
            perlin_octaves: 4,
            perlin_amp_falloff: 0.5,
            perlin,
        }
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let _x = if x < 0.0 { 0. - x } else { x };
        let _y = if y < 0.0 { 0. - y } else { y };
        let _z = if z < 0.0 { 0. - z } else { z };

        let mut xi = _x as usize;
        // let yi = f64::floor(y);
        let mut yi = _y as u64;
        // let zi = f64::floor(z);
        let mut zi = _z as i64;
        let mut xf = _x - f64::floor(_x);
        let mut yf = _y - f64::floor(_y);
        let mut zf = _z - f64::floor(_z);
        let mut r = 0.;
        let mut ampl = 0.5;
        let mut n1: f64;
        let mut n2: f64;
        let mut n3: f64;

        let mut o = 0;
        while o < self.perlin_octaves {
            // let of = xi + (yi << PERLIN_YWRAPB) + (zi << PERLIN_ZWRAPB);
            let mut of = xi;
            let rxf = Noise::scaled_cosine(xf);
            let ryf = Noise::scaled_cosine(yf);
            n1 = self.perlin[of & PERLIN_LAST];
            n1 += rxf * (self.perlin[(of + 1) & PERLIN_LAST] - n1);
            n2 = self.perlin[(of + PERLIN_YWRAP) & PERLIN_LAST];
            n2 += rxf * (self.perlin[(of + PERLIN_YWRAP + 1) & PERLIN_LAST] - n2);
            n1 += ryf * (n2 - n1);
            of += PERLIN_ZWRAP;
            n2 = self.perlin[of & PERLIN_LAST];
            n2 += rxf * (self.perlin[(of + 1) & PERLIN_LAST] - n2);
            n3 = self.perlin[(of + PERLIN_YWRAP) & PERLIN_LAST];
            n3 += rxf * (self.perlin[(of + PERLIN_YWRAP + 1) & PERLIN_LAST] - n3);
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
                xi = xi + 1;
                xf = xf - 1.;
            }
            if yf >= 1.0 {
                yi = yi + 1;
                yf = yf - 1.;
            }
            if zf >= 1.0 {
                zi = zi + 1;
                zf = zf - 1.;
            }
            // increment for loop
            o = o + 1;
        }
        r
    }
}

#[test]

fn test_noise() {
    let noise = Noise::new();
    for i in 0..200 {
        let iflo = i as f64;
        let v = noise.noise(iflo, iflo, iflo);
        println!("i {} noise {}", i, v);
    }
}

pub fn _rand() -> f64 {
    rand::random::<f64>()
}

pub fn map_val(val: f64, i_start: f64, i_stop: f64, o_start: f64, o_stop: f64) -> f64 {
    o_start + (o_stop - o_start) * (((val - i_start) * 1.0) / (i_stop - i_start))
}

pub fn norm_rand(little_m: f64, big_m: f64) -> f64 {
    map_val(_rand(), 0., 1., little_m, big_m)
}


pub fn rand_bool() -> bool {
    if _rand() > 0.5 {
        true
    } else {
        false
    }
}

pub fn wt_rand(f: fn(f64) -> f64) -> f64 {
    let x = _rand();
    let y = _rand();
    if y < f(x) {
        x
    } else {
        wt_rand(f)
    }
}

pub fn rand_gauss() -> f64 {
    wt_rand(|x| f64::powf(std::f64::consts::E, -24. * f64::powf(x - 0.5, 2.))) * 2. - 1.
}

// fn rand_choice<T>(arr: Vec<T>) -> T {
//     let idx = f64::floor(arr.len() as f64 * _rand()) as usize;
//     arr[idx]
// }
pub fn rand_choice_arr(arr: &[usize]) -> usize {
    let idx = f64::floor(arr.len() as f64 * _rand()) as usize;
    arr[idx]
}
pub fn rand_choice_arrf(arr: &[f64]) -> f64 {
    let idx = f64::floor(arr.len() as f64 * _rand()) as usize;
    arr[idx]
}
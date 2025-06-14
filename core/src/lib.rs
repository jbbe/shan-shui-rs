use core::f64::consts::PI;
use std::collections::{HashMap, VecDeque};

pub mod draw;
pub mod forms;
pub mod noise;
mod point;

pub use draw::*;
pub use forms::*;
pub use noise::Noise;
pub use point::*;

// struct Palette {
//     water: String,
//     mount1: String,
//     mount2: String,
// }

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tag {
    Mount,
    DistMount,
    FlatMount,
    Boat,
}

#[derive(Debug)]
struct Plan {
    tag: Tag,
    x: f64,
    y: f64,
    // h: f64, // what does this represent? usually generated by ns() func
}

impl Plan {
    fn new(
        tag: Tag,
        x: f64,
        y: f64,
        // h: f64
    ) -> Self {
        Self { tag, x, y }
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct Chunk {
    tag: Tag,
    x: f64,
    y: f64,
    canv: String,
}

#[derive(Debug)]
struct MountainMap {
    data: HashMap<i32, u32>,
    pub step: f64,
}

impl MountainMap {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            step: 5.,
        }
    }

    fn init_plan_mtx_range(&mut self, x_min: f64, x_max: f64) {
        let mut i = x_min;
        while i < x_max {
            let i1 = f64::floor(i / self.step) as i32;
            // println!("i1 {}", i1);
            self.data.entry(i1).or_insert_with_key(|_k| 0);
            i = i + self.step;
        }
    }

    fn incr_range(&mut self, x_min: f64, x_max: f64) {
        let lower_lim = f64::floor(x_min / self.step) as i32;
        let upper_lim = f64::floor(x_max / self.step) as i32;
        for k in lower_lim..upper_lim {
            // is this determining the crest of the mountains?
            *(self.data.entry(k).or_insert(0)) += 1;
        }
    }

    pub fn is_empty(&self, i: i32) -> bool {
        self.data[&i] == 0
    }
}

const SAMP: f64 = 0.03;
#[derive(Debug)]
struct State {
    // represents map of mountain range
    // maybe should call occupied_x
    // ensures that no flat_mount's center is planned at x under teh existing
    //  mountain adjusted by randomness
    pub plan_mtx: MountainMap,
    x_min: f64,
    x_max: f64,
    chunk_width: f64,
    pub chunks: VecDeque<Chunk>,
}

impl State {
    pub fn new() -> Self {
        Self {
            plan_mtx: MountainMap::new(),
            x_min: 0.,
            x_max: 0.,
            chunk_width: 512.,
            chunks: VecDeque::new(),
        }
    }

    fn add_chunk(&mut self, nch: Chunk) {
        // Our chunks are ordered by their y coordinates
        // don't add if chunks group is empty
        // todo I think the sorting by why lets the shorter mountains be rendered on top
        if self.chunks.len() == 0 {
            self.chunks.push_back(nch);
        } else if nch.y <= self.chunks[0].y {
            self.chunks.push_front(nch);
        } else if nch.y >= self.chunks[self.chunks.len() - 1].y {
            self.chunks.push_back(nch);
        } else {
            for i in 0..(self.chunks.len() - 1) {
                if self.chunks[i].y <= nch.y && nch.y <= self.chunks[i + 1].y {
                    self.chunks.insert(i, nch);
                    return;
                }
            }
        }
    }

    pub fn gen_chunks(&mut self, noise: &mut Noise, x_min: f64, x_max: f64) {
        // let mut g = Group::new();
        while x_max > self.x_max - self.chunk_width || x_min < self.x_min + self.chunk_width {
            println!("Generating new chunk...",);

            // generate new chunk
            let plans: Vec<Plan>;
            if x_max > self.x_max - self.chunk_width {
                plans = self.mount_planner(noise, self.x_max, self.x_max + self.chunk_width);
                self.x_max = self.x_max + self.chunk_width;
            } else {
                plans = self.mount_planner(noise, self.x_min - self.chunk_width, self.x_min);
                self.x_min = self.x_min - self.chunk_width;
            }

            let len = plans.len();
            println!("Generated {:?} plans", len);
            for i in 0..len {
                let p = &plans[i];
                self.add_chunk(Chunk {
                    tag: p.tag,
                    x: p.x,
                    y: p.y,
                    canv: State::gen_chunk(noise, p, i),
                });
            }
        }
    }

    fn gen_chunk(noise: &mut Noise, p: &Plan, i: usize) -> String {
        // println!("create svg for chunk {:?} {:?} {:?}", p.tag, p.x, p.y);
        if p.tag == Tag::Mount {
            let seed = (i * 2) as f64 * noise.rand();
            let args = MountainArgs::default(noise);
            let mut g = Group::new("mount".to_string());
            g.add(mountain(noise, p.x, p.y, seed, args));
            g.add(water(noise, p.x, p.y - 1000., WaterArgs::default()));
            g.to_string()
        } else if p.tag == Tag::FlatMount {
            let seed = 2. * noise.rand();
            let width = 600. + (noise.rand() * 400.);
            let cho = 0.5 + (noise.rand() * 0.2);
            let args = FlatMountArgs {
                width,
                height: 100.,
                cho,
                seed,
                ..FlatMountArgs::default(noise)
            };
            flat_mount(noise, p.x, p.y, args)
        } else if p.tag == Tag::DistMount {
            let seed = noise.rand();
            let len = noise.rand_choice_arr(&[500., 100., 1500.]);
            dist_mount(noise,p.x, p.y, seed,DistMountArgs { 
                height: 150., len, ..DistMountArgs::default() 
            })
        } else if p.tag == Tag::Boat {
            let args = BoatArgs {
                scale: p.y / 800.,
                fli: noise.rand_bool(),
                ..BoatArgs::default()
            };
            boat01(noise, p.x, p.y, args)
        } else {
            "".to_string()
        }
    }

    /*
     * Mount planner
     */
    fn mount_planner(&mut self, noise: &mut Noise, x_min: f64, x_max: f64) -> Vec<Plan> {
        println!(
            "mount_planner top level x_min {:?} x_max {:?}",
            x_min, x_max
        );
        fn chadd_mind(registry: &mut Vec<Plan>, plan: Plan, mind: f64) -> bool {
            // let len = reg.len();
            for k in 0..(registry.len()) {
                // we only add the new chunk if
                // the difference between the new plan's x
                // and any other plan's x is less than mind
                // which defaults to 10
                if f64::abs(registry[k].x - plan.x) < mind {
                    return false;
                }
            }
            println!("+");
            registry.push(plan);
            true
        }
        /*
         * returns whether plan was succesfully added.
         */
        fn chadd(registry: &mut Vec<Plan>, plan: Plan) -> bool {
            chadd_mind(registry, plan, 10.)
        }
        // REgistry ensures that no x is placed on exactly the same line
        let mut registry: Vec<Plan> = Vec::new();

        let rand_height = |noise: &mut Noise, x| noise.noise(x * 0.01, PI, 0.);

        let mnt_width = 200.;
        // line 3757
        self.plan_mtx.init_plan_mtx_range(x_min, x_max);

        /*
         * Iterate through currently generated chunk
         * put a mountain if it is a local max of the smooth perlin noise
         * mark surrounding space as occupied. once every thousand x's place
         * a distant mountain.
         */
        let mut x = x_min;
        while x < x_max {
            // max y?
            let mut y = 0.;
            while y < rand_height(noise, x) * 480. {
                if is_local_max(noise, x, y, 2.) {
                    let x_off = x + 2. * (noise.rand() - 0.5) * 500.;
                    let y_off = y + 300.;
                    let r: Plan = Plan::new(Tag::Mount, x_off, y_off);
                    if chadd(&mut registry, r) {
                        // If we add the plan then we need to increment our map of the mountains
                        self.plan_mtx
                            .incr_range(x_off - mnt_width, x_off + mnt_width);
                    }
                }

                y += 30.;
            } // while y
            if f64::abs(x) % 1000. < 4. {
                // distmount is only added when i < 4
                println!("adding distmount");
                let y = 280. - noise.rand() * 50.;
                let r = Plan::new(Tag::DistMount, x, y);
                chadd(&mut registry, r);
            } // if

            x = x + self.plan_mtx.step;
        } // while x
        println!("Xmin {:?} xmax {:?}", x_min, x_max); // 3794

        /*
         * After moutnains are generated we add a flat mountain
         * to ~10% of the unoccupied points
         */
        x = x_min;
        while x < f64::floor(x_max) {
            let idx = f64::floor(x / self.plan_mtx.step) as i32;
            // println!("Xmax {:?} i {:?} idx {:?} step {:?}", x_min, i, idx, x_step);
            if self.plan_mtx.is_empty(idx) && noise.rand() < 0.01 {
                let mut y = 0.;
                while y < (4. * noise.rand()) {
                    let r = Plan::new(
                        Tag::FlatMount,
                        x + (2. * (noise.rand() - 0.5) * 700.),
                        700. - y * 50.,
                    );
                    chadd(&mut registry, r);
                    y = y + 1.;
                } // while y
            }
            x = x + self.plan_mtx.step;
        } // while x

        /*
         * Add boats with a 20% chance
         */
        x = x_min;
        while x < x_max {
            if noise.rand() < 0.2 {
                let r = Plan::new(Tag::Boat, x, 300. + (noise.rand() * 390.));
                chadd_mind(&mut registry, r, 400.);
            }
            x = x + self.plan_mtx.step;
        }
        registry
    }
}

pub fn svg_string(seed: f64) -> String {
    Painting::new(seed).update(0., 3000.)
}

pub struct Painting {
    state: State,
    noise: Noise,
}

impl Painting {
    pub fn new(seed: f64) -> Self {
        Self {
            state: State::new(),
            noise: Noise::new(seed),
        }
    }

    pub fn chunk_render(&self, x_min: f64, x_max: f64) -> String {
        let mut canv = vec![];
        // println!("Rendering {:?} chunks", self.state.chunks.len());
        for i in 0..(self.state.chunks.len()) {
            if x_min - self.state.chunk_width < self.state.chunks[i].x
                && self.state.chunks[i].x < x_max + self.state.chunk_width
            {
                println!("pushing chunk {}", i);
                canv.push(self.state.chunks[i].canv.to_string());
            }
        }
        println!("Rendered {} chunks", canv.len());
        canv.join("")
    }

    pub fn preload(&mut self, x_min: f64, x_max: f64) {
        self.state.gen_chunks(&mut self.noise, x_min, x_max)
    }

    pub fn update(&mut self, x_min: f64, x_max: f64) -> String {
        self.state.gen_chunks(&mut self.noise, x_min, x_max);
        self.chunk_render(x_min, x_max)
    }

    pub fn full_svg(&mut self, width: f64, height: f64) -> String {
        self.state.gen_chunks(&mut self.noise, 0., width);
        Self::svg_template(width, height, 0., self.chunk_render(0., width))
    }

    pub fn svg_template(w: f64, h: f64, _x: f64, svg: String) -> String {
        vec![
            "<svg id='SVG' xmlns='http://www.w3.org/2000/svg' width='",
            &w.to_string()[..],
            "' height='",
            &h.to_string()[..],
            "' style='mix-blend-mode:multiply;' viewBox ='0 0 ",
            &w.to_string()[..],
            " ",
            &h.to_string()[..],
            "'><g id='G'>",
            &svg[..],
            "</g></svg>",
        ]
        .join("")
    }
    pub fn draw_boat(&mut self) -> String {
        let resolution = 512.;
        Painting::svg_template(
            resolution,
            resolution,
            0.,
            boat01(&mut self.noise, 256., 256., BoatArgs::default()),
        )
    }
    
    pub fn draw_transmission_tower(&mut self) -> String {
        let resolution = 512.;
        Painting::svg_template(
            resolution,
            resolution,
            0.,
            transmission_tower(&mut self.noise, 256., 256., TowerArgs::default()),
        )
    }

    pub fn draw_mount(&mut self) -> String {
        let resolution = 512.;
        let seed = (2.) * self.noise.rand();
        let args = MountainArgs::default(&mut self.noise);
        Painting::svg_template(
            resolution,
            resolution,
            0.,
            mountain(&mut self.noise, 10., 300., seed, args),
        )
    }

    pub fn draw_man(&mut self) -> String {
        let resolution = 512.;
        let args = ManArgs::default(&mut self.noise);
        Painting::svg_template(
            resolution,
            resolution,
            0.,
            Man::man(&mut self.noise, 10., 300., args),
        )
    }
   
    pub fn draw_arch01(&mut self) -> String {
        let resolution = 512.;
        let args = Arch01Args::default();
        Painting::svg_template(
            resolution,
            resolution,
            0.,
            arch01(&mut self.noise, 10., 300., args),
        )
    }
}
fn is_local_max(noise: &mut Noise, x: f64, y: f64, r: f64) -> bool {
    let f = |x: f64, _: f64| -> f64 { f64::max(noise.noise(x * SAMP, 0., 0.) - 0.55, 0.) * 2. };
    let z0 = f(x, y);
    if z0 <= 0.3 {
        return false;
    }
    let loc_min_x = x - r;
    let max_x = x + r;
    let min_y = y - r;
    let max_y = y + r;
    let mut i = loc_min_x;
    while i < max_x {
        let mut j = min_y;
        while j < max_y {
            if f(i, j) > z0 {
                return false;
            }
            j += 1.;
        }
        i += 1.;
    }
    true
}

// #[bench]
// fn first_update(b: &mut Bencher) {
//     b.iter(||{
//         let p = Painting::new(1231232.);
//         p.update(0., 3000.);
//     });
//     // println!(s);
// }

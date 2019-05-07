extern crate rand;

use std::cell::RefCell;
use std::f64::consts::PI;

use super::*;

type Canvas = web_sys::CanvasRenderingContext2d;

pub struct Universe {
    dimensions: (f64, f64),
    planets: RefCell<Vec<Planet>>,
}

impl Universe {
    pub(crate) fn new(dimensions: (f64, f64)) -> Self {
        Universe {
            dimensions,
            planets: RefCell::new(vec![]),
        }
    }

    pub(crate) fn init_random(&mut self) {
        //let mut rng = rand::thread_rng();
        self.planets.borrow_mut().push(Planet::new(
            self.dimensions.0 / 2.0,
            self.dimensions.1 / 2.0,
            6_000.0,
            20.0,
            Point {
                //x: rng.gen_range(-0.1, 0.5),
                //y: rng.gen_range(-0.1, 0.5),
                x: 0.0,
                y: 0.0,
            },
        ));

        for _i in 0..super::NO_OF_PLANETS {
            self.planets
                .borrow_mut()
                .push(Planet::new_rng(self.dimensions));
        }

        //self.planets
        //    .push(Planet::new(400.0, 300.0, 5_000.0, 6_000_000.0));
        //self.planets
        //    .push(Planet::new(450.0, 350.0, 3_000.0, 1_000_000.0));
    }

    #[allow(dead_code)]
    pub(crate) fn log(&self, val: &JsValue) {
        web_sys::console::log_1(&val);
    }

    pub(crate) fn add_planet(&mut self, x: f64, y: f64) {
        self.planets.borrow_mut().push(Planet::new_semi_rng(x, y));
    }

    /// The main computation of the universe. In a nested loop, we look at each planet,
    /// calculate the gravitational force in relation to each other planet, then sum up the
    /// forces, scale them to make them more visible and set the net acceleration.
    /// Since we're only holding references to our planets, when one gets eaten, we initially set
    /// it to `dead` and remove it from the `planets` vector after the loop is finished.
    #[allow(non_snake_case)]
    pub(crate) fn tick<'a>(&self) {
        let G = 6.67 * 10_f64.powf(-11.0);

        // Some parameters to make the universe more enjoyable to look at.
        let scale_f = 10_f64.powf(4.0);
        let eating_force = 400.0;

        for (i, p) in self.planets.borrow().iter().enumerate() {
            if p.dead() {
                continue;
            }

            let mut forces: Vec<Point> = vec![];
            for (j, other_p) in self.planets.borrow().iter().enumerate() {
                if p.dead() {
                    continue;
                };

                if i != j {
                    let direction = other_p.pos() - p.pos();
                    let d = direction.mag();
                    let F = (G * p.mass() * other_p.mass()) / (d * d);

                    if direction.mag() <= p.radius() && F > eating_force {
                        //self.log(&F.into());
                        p.eat(&other_p);
                        other_p.die();
                        continue;
                    } else {
                        // The gravitational force between two bodies will always be the same for
                        // both. Note that although I am applying the same gravitational force to
                        // Earth as it is to me, the acceleration happening is a very one-sided
                        // affair. That's because Earth probably ate a few more planets than I did
                        // and can throw all her weight in the ring, or, in Newton's words:
                        // F = a/m
                        let acc = direction.norm() * (F / p.mass());
                        forces.push(acc);
                    }
                }
            }

            // We start with a force of (0, 0) and apply each previously calculated gravitational
            // force in turn.
            let mut net_force = Point { x: 0.0, y: 0.0 };
            net_force = forces.into_iter().fold(net_force, |acc, curr| acc + curr);

            // We need to scale F for now to have something actually happening on the screen.
            p.accelerate(net_force * scale_f);

            // Let's have Sun stay in the middle of the universe.
            if i > 0 {
                p.update(self.dimensions);
            }
        }

        let planets: Vec<Planet> = self
            .planets
            .borrow_mut()
            .iter()
            .filter(|p| !p.dead())
            .map(|p| p.clone())
            .collect();

        self.planets.replace(planets);
    }

    #[allow(non_snake_case)]
    pub(crate) fn draw<'a>(&self, ctx: &'a Canvas) -> &'a Canvas {
        ctx.clear_rect(0.0, 0.0, self.dimensions.0, self.dimensions.1);
        ctx.set_stroke_style(&"magenta".into());
        ctx.set_fill_style(&"black".into());
        ctx.set_line_width(4.0);

        for p in self.planets.borrow().iter() {
            ctx.begin_path();
            ctx.set_font(&"16px Mono");

            ctx.arc(p.pos().x, p.pos().y, p.radius(), 0.0, PI * 2.0)
                .unwrap();

            ctx.stroke();
            ctx.fill();
            ctx.set_stroke_style(&"white".into());
            ctx.set_fill_style(&"gray".into());
        }

        ctx
    }
}

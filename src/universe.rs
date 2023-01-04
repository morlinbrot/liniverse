extern crate rand;

use std::cell::RefCell;
use std::f64::consts::PI;

use super::*;

type Canvas = web_sys::CanvasRenderingContext2d;

pub struct Universe {
    dimensions: (f64, f64),
    planets: Vec<Rc<RefCell<Planet>>>,
}

impl Universe {
    pub fn new(dimensions: (f64, f64)) -> Self {
        Self {
            dimensions,
            planets: Vec::new(),
        }
    }

    pub fn init_random(&mut self) {
        // let mut rng = rand::thread_rng();
        self.planets.push(Rc::new(RefCell::new(Planet::new_sun(
            self.dimensions.0 / 2.0,
            self.dimensions.1 / 2.0,
        ))));

        for _i in 0..NO_OF_PLANETS {
            let planet = Rc::new(RefCell::new(Planet::new_rng(self.dimensions)));
            self.planets.push(planet);
        }
    }

    #[allow(dead_code)]
    pub fn log(&self, val: &JsValue) {
        web_sys::console::log_1(&val);
    }

    pub fn add_planet(&mut self, x: f64, y: f64) {
        let p = Rc::new(RefCell::new(Planet::new_semi_rng(x, y)));
        self.planets.push(p);
    }

    pub fn tick_n_draw<'a>(&self, ctx: &'a Canvas, time: f64) -> &'a Canvas {
        let mut qtree = self.init_quad_tree();
        for planet in &self.planets {
            qtree.insert(planet.clone()).unwrap();
        }

        let mut ctx = self.refresh_canvas(ctx);
        for planet in &self.planets {
            qtree.update_body(planet.clone(), time).unwrap();
            ctx = self.draw_planet(ctx, planet.clone());
        }

        ctx
    }

    /// The main computation of the universe. In a nested loop, we look at each planet,
    /// calculate the gravitational force in relation to each other planet, then sum up the
    /// forces, scale them to make them more visible and set the net acceleration.
    /// Since we're only holding references to our planets, when one gets eaten, we initially set
    /// it to `dead` and remove it from the `planets` vector after the loop is finished.
    #[allow(non_snake_case)]
    pub fn tick_n_draw_brute<'a>(&mut self, ctx: &'a Canvas, _time: f64) -> &'a Canvas {
        let G = 6.67 * 10_f64.powf(-11.0);

        for (i, p) in self.planets.iter().enumerate() {
            let p = p.borrow();
            if p.dead() {
                continue;
            }

            let mut forces: Vec<Point> = vec![];
            for (j, other_p) in self.planets.iter().enumerate() {
                let other_p = other_p.borrow();
                if p.dead() {
                    continue;
                };

                if i != j {
                    let direction = other_p.pos() - p.pos();
                    let d = direction.mag();
                    let F = (G * p.mass() * other_p.mass()) / (d * d);

                    if direction.mag() <= p.radius() && F > EATING_FORCE {
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
            let mut net_force = Point::default();
            net_force = forces.into_iter().fold(net_force, |acc, curr| acc + curr);

            // We need to scale F for now to have something actually happening on the screen.
            p.accelerate(net_force * SCALE_F);

            // Let's have Sun stay in the middle of the universe.
            if i > 0 {
                p.update(self.dimensions);
            }
        }

        let planets = self
            .planets
            .iter()
            .filter(|p| !p.borrow_mut().dead())
            .map(|p| p.clone())
            .collect();

        self.planets = planets;

        let mut ctx = self.refresh_canvas(ctx);
        for p in self.planets.iter() {
            ctx = self.draw_planet(ctx, p.clone());
        }

        ctx
    }

    fn init_quad_tree(&self) -> QuadNode {
        let width = self.dimensions.0;
        let height = self.dimensions.1;
        let bounds = Rect::new(width / 2.0, height / 2.0, width, height);

        let cfg = Rc::new(QuadConfig {
            capacity: 1,
            theta: 0.5,
        });

        QuadNode::new(cfg, bounds)
    }

    fn refresh_canvas<'a>(&self, ctx: &'a Canvas) -> &'a Canvas {
        ctx.clear_rect(0.0, 0.0, self.dimensions.0, self.dimensions.1);
        ctx.set_stroke_style(&"magenta".into());
        ctx.set_fill_style(&"black".into());
        ctx.set_line_width(4.0);
        ctx
    }

    fn draw_planet<'a>(&self, ctx: &'a Canvas, planet: Rc<RefCell<Planet>>) -> &'a Canvas {
        let planet = planet.borrow();
        let pos = planet.pos();
        ctx.begin_path();
        ctx.arc(pos.x, pos.y, planet.radius(), 0.0, PI * 2.0)
            .unwrap();
        ctx.stroke();
        ctx.fill();
        ctx.set_stroke_style(&"white".into());
        ctx.set_fill_style(&"gray".into());
        ctx
    }
}

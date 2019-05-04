extern crate math;
extern crate rand;

//use math::round;
use std::f64::consts::PI;
//use wasm_bindgen::prelude::*;

use super::{Planet, Point};

type Canvas = web_sys::CanvasRenderingContext2d;

/// F = m*a
/// 1 Newton is the force F required to accelerate a mass of 1kg by 1m/s² in the
/// direction of the force F:

pub(crate) struct Universe {
    dimensions: (f64, f64),
    planets: Vec<Planet>,
}

impl Universe {
    pub(crate) fn new(width: f64, height: f64) -> Self {
        Universe {
            dimensions: (width, height),
            planets: vec![],
        }
    }

    pub(crate) fn init_random(&mut self) {
        self.planets.push(Planet::new(400.0, 300.0, 8000.0, 50.0));

        for _i in 0..super::NO_OF_PLANETS {
            self.planets.push(Planet::new_rng());
        }

        //self.planets
        //    .push(Planet::new(400.0, 300.0, 5_000.0, 6_000_000.0));
        //self.planets
        //    .push(Planet::new(450.0, 350.0, 3_000.0, 1_000_000.0));
    }

    #[allow(non_snake_case)]
    pub(crate) fn tick<'a>(&self) {
        let G = 6.67 * 10_f64.powf(-11.0);

        let scale_f = 10_f64.powf(4.0);

        for (i, p) in self.planets.iter().enumerate() {
            let mut forces: Vec<Point> = vec![];

            for (j, other_p) in self.planets.iter().enumerate() {
                if i != j {
                    let direction = other_p.pos() - p.pos();

                    if direction.mag() <= p.radius {
                        continue;
                    } else {
                        let d = direction.mag();
                        let F = (G * p.mass() * other_p.mass()) / (d * d);

                        let acc = direction.norm() * (F / p.mass());

                        forces.push(acc);
                    }
                }
            }

            let mut net_force = Point { x: 0.0, y: 0.0 };
            for f in forces.into_iter() {
                net_force = net_force + f;
            }

            p.accelerate(net_force * scale_f);
            p.mv(self.dimensions.0, self.dimensions.1);
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn draw<'a>(&self, ctx: &'a Canvas) -> &'a Canvas {
        ctx.set_stroke_style(&"hotpink".into());
        ctx.set_line_width(2.0);

        for p in (&self.planets).iter() {
            ctx.begin_path();
            ctx.set_font(&"16px Mono");

            // Draw the planet itself.
            ctx.arc(p.pos().x, p.pos().y, p.radius, 0.0, PI * 2.0)
                .unwrap();

            let target_x = p.pos().x + p.velocity.get().x;
            let target_y = p.pos().y + p.velocity.get().y;

            ctx.move_to(p.pos().x, p.pos().y);
            ctx.line_to(target_x, target_y);

            //let text = format!("acc: {}", acc.x);
            //ctx.fill_text(&text, p.pos().x, p.pos().y).unwrap();

            //let text = format!("velocity: {}", p.velocity.get().x);
            //ctx.fill_text(&text, p.pos().x, p.pos().y + 20.0).unwrap();
            //let val: JsValue = dist.mag().into();
            //web_sys::console::log_1(&val);

            ctx.stroke();
            ctx.set_stroke_style(&"black".into());
        }

        ctx
    }
}

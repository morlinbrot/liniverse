extern crate math;
extern crate rand;

//use math::round;
use std::f64::consts::PI;
//use wasm_bindgen::prelude::*;

use super::Planet;

type Canvas = web_sys::CanvasRenderingContext2d;

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
        for _i in 0..super::NO_OF_PLANETS {
            self.planets.push(Planet::new_rng());
        }

        //self.planets
        //    .push(Planet::new(400.0, 300.0, 5_000.0, 6_000_000.0));
        //self.planets
        //    .push(Planet::new(450.0, 350.0, 3_000.0, 1_000_000.0));
    }

    pub(crate) fn tick(&mut self) {
        for p in &mut self.planets {
            p.mv(self.dimensions.0, self.dimensions.1);
        }
    }

    pub(crate) fn draw<'a>(&self, ctx: &'a Canvas) -> &'a Canvas {
        let G = 6.67 * 10_f64.powf(-11.0);
        //let scale = 10_f64.powf(-1.0);
        let scale = 3.0 * 10_f64.powf(-6.0);
        let scale_f = 2.0 * 10_f64.powf(-48.0);

        let min_force = 10_f64.powf(-20.0);

        ctx.set_stroke_style(&"hotpink".into());

        for (i, p) in (&self.planets).iter().enumerate() {
            ctx.begin_path();
            ctx.set_font(&"16px Mono");

            // Draw the planet itself.
            ctx.arc(p.pos().x, p.pos().y, p.radius * scale, 0.0, PI * 2.0)
                .unwrap();

            for (j, other_p) in (&self.planets).iter().enumerate() {
                if j != i {
                    let direction = other_p.pos() - p.pos();
                    let mut F = (p.mass() * other_p.mass() / direction.mag().powf(2.0));
                    F = F * scale_f;
                    let acc = direction.norm() * F;

                    if F > min_force {
                        p.accelerate(acc);
                        //let target_x = p.pos().x + acc.norm().x * F;
                        //let target_y = p.pos().y + acc.norm().y * F;
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
                    }
                }
            }

            ctx.stroke();
            ctx.set_stroke_style(&"black".into());
        }

        ctx
    }
}

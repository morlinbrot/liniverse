extern crate math;
extern crate rand;

use math::round;
use rand::Rng;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

use super::{Planet, Point};

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

    pub(crate) fn init_random(&mut self, no_of_planets: usize) {
        //let (x1, y1, x2, y2) = (200.0, 300.0, 400.0, 300.0);

        //self.planets.push(Planet::new(x1, y1));
        //self.planets.push(Planet::new(x2, y2));

        for _i in 0..no_of_planets {
            let mut rng = rand::thread_rng();

            let x = self.dimensions.0 / 2.0 + rng.gen_range(-150.0, 150.0);
            let y = self.dimensions.1 / 2.0 + rng.gen_range(-150.0, 150.0);

            self.planets.push(Planet::new(x, y));
        }
    }

    pub(crate) fn tick(&mut self) {
        for p in &mut self.planets {
            p.mv(self.dimensions.0, self.dimensions.1);
        }
    }

    pub(crate) fn draw<'a>(&self, ctx: &'a Canvas) -> &'a Canvas {
        let G = 6.67 * 10_f64.powf(-11.0);
        //let scale = 10_f64.powf(-1.0);
        let scale = 0.005;
        let scale_f = 10_f64.powf(-34.0);

        let min_force = 10_f64.powf(-20.0);

        for (i, p) in (&self.planets).iter().enumerate() {
            ctx.begin_path();
            let p_pos = p.get_pos();
            // Draw circle.
            ctx.arc(p_pos.x, p_pos.y, p.radius * scale, 0.0, PI * 2.0)
                .unwrap();

            for j in 0..i {
                let other_p = &self.planets[j];
                let other_pos = other_p.get_pos();

                let dist = other_pos - p_pos;

                let F = G * (p.m * other_p.m / dist.mag().powf(2.0));

                if F > min_force {
                    let target_x = p_pos.x + dist.norm().x * (F * scale_f);
                    let target_y = p_pos.y + dist.norm().y * (F * scale_f);

                    ctx.move_to(p_pos.x, p_pos.y);
                    ctx.line_to(target_x, target_y);

                    ctx.set_font(&"16px Mono");
                    //let text = format!("F: {}", F);
                    //ctx.fill_text(&text, p_pos.x, p_pos.y).unwrap();
                    //let val: JsValue = dist.mag().into();
                    //web_sys::console::log_1(&val);
                }
            }

            for k in i + 1..self.planets.len() {
                let other_p = &self.planets[k];
                let other_pos = other_p.get_pos();

                let dist = other_pos - p_pos;

                let F = G * (p.m * other_p.m / dist.mag().powf(2.0));

                if F > min_force {
                    let target_x = p_pos.x + dist.norm().x * scale_f;
                    let target_y = p_pos.y + dist.norm().y * scale_f;

                    ctx.move_to(p_pos.x, p_pos.y);
                    ctx.line_to(target_x, target_y);

                    ctx.set_font(&"16px Mono");
                    //let text = format!("F: {}", F);
                    //ctx.fill_text(&text, p_pos.x, p_pos.y).unwrap();
                    //let val: JsValue = dist.mag().into();
                    //web_sys::console::log_1(&val);
                }
            }

            ctx.stroke();
        }

        ctx
    }
}

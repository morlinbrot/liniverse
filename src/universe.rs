extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

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

    pub(crate) fn init_random(&mut self, no_of_planets: usize) {
        for _i in 0..no_of_planets {
            let mut rng = rand::thread_rng();

            let x = self.dimensions.0 / 2.0 + rng.gen_range(-150.0, 150.0);
            let y = self.dimensions.1 / 2.0 + rng.gen_range(-150.0, 150.0);

            self.planets.push(Planet::new(x, y));
        }
    }

    pub(crate) fn tick(&mut self) {
        for p in &mut self.planets {
            p.mv();
        }
    }

    pub(crate) fn draw<'a>(&self, ctx: &'a Canvas) -> &'a Canvas {
        for p in &self.planets {
            ctx.begin_path();
            let (x, y) = p.get_pos();
            ctx.arc(x, y, 20.0, 0.0, PI * 2.0).unwrap();
            ctx.stroke();
        }

        ctx
    }
}

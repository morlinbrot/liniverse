extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

use super::Point;

type Canvas = web_sys::CanvasRenderingContext2d;

#[allow(dead_code)]
pub(crate) struct Planet {
    pub(crate) pos: Point,
    dir: Point,
    // Mass in kg
    pub(crate) m: f64,
    // Radius in km
    pub(crate) radius: f64,
    // Volume
    V: f64,
    // Velocity
    v: f64,
    // Density in kg/m³
    p: f64,
}

impl Planet {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        // Earth
        let radius = 6_371.0;
        let density = 5513.0;
        //let mass = 5.9722 * 10_f64.powf(24);

        // Sun
        //let radius = 695_510.0;
        //let density = 1409.0;
        //let mass = 7.9897 * 10_f64.powf(30);

        //// Moon
        //let radius = 1737.0;
        //let density = 3344.0;
        //let mass = 7.348 * 10_f64.powf(22);

        //// Mars
        //let radius = 3389.5;
        //let density = 3934;
        //let mass = 6.419 * 10_f64.powf(23);

        let volume = 4.0 / 3.0 * PI * (radius * 1_000 as f64).powf(3.0);
        let mass = density * volume;

        Planet {
            radius,
            pos: Point { x, y },
            dir: Point {
                x: rng.gen_range(-5.0, 5.0),
                y: rng.gen_range(-5.0, 5.0),
            },
            V: volume,
            p: density,
            m: mass,
            v: 1.0,
        }
    }

    pub(crate) fn get_pos(&self) -> Point {
        self.pos
    }

    pub(crate) fn mv(&mut self, max_x: f64, max_y: f64) {
        let mut x = self.pos.x + self.dir.x * self.v;
        let mut y = self.pos.y - self.dir.y * self.v;

        if x > max_x {
            x = x - max_x;
        } else if x < 0.0 {
            x = x + max_x;
        }

        if y > max_y {
            y = y - max_y;
        } else if y < 0.0 {
            y = y + max_y;
        }

        self.pos = Point { x, y };
    }

    //pub(crate) fn draw<'a>(&self, ctx: &'a Canvas) -> &'a Canvas {
    //    ctx.begin_path();
    //    let (x, y) = (self.pos.x, self.pos.y);
    //    // Draw circle.
    //    ctx.arc(x, y, 20.0, 0.0, PI * 2.0).unwrap();

    //    ctx.move_to(x, y);
    //    ctx.line_to(x + 10.0, y + 10.0);

    //    ctx.stroke();
    //    ctx
    //}
}

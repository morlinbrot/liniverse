extern crate rand;

use rand::Rng;

use super::Point;
use super::Universe;

type Canvas = web_sys::CanvasRenderingContext2d;

#[allow(dead_code)]
pub(crate) struct Planet {
    pub(crate) pos: Point,
    dir: Point,
    // Mass in kg
    pub(crate) m: f64,
    pub(crate) radius: f64,
    // Velocity
    v: f64,
}

impl Planet {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();

        Planet {
            pos: Point { x, y },
            dir: Point {
                x: rng.gen_range(-5.0, 5.0),
                y: rng.gen_range(-5.0, 5.0),
            },
            m: 5.927 * 10_f64.powf(24.0),
            radius: 6_371.0,
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
        }

        if y > max_y {
            y = y - max_y;
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

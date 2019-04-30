extern crate rand;

use rand::Rng;

struct Point {
    x: f64,
    y: f64,
}

pub(crate) struct Planet {
    pos: Point,
    dir: Point,
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
            v: 1.0,
        }
    }

    pub(crate) fn get_pos(&self) -> (f64, f64) {
        (self.pos.x, self.pos.y)
    }

    pub(crate) fn mv(&mut self) {
        let x = self.pos.x + self.dir.x * self.v;
        let y = self.pos.y - self.dir.y * self.v;

        self.pos = Point { x, y };
    }
}

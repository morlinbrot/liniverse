extern crate rand;

use rand::Rng;
use std::cell::Cell;
use std::f64::consts::PI;

use super::Point;

type Canvas = web_sys::CanvasRenderingContext2d;

// Earth
//let density = 5513.0;
//let radius = 6_371.0;
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

#[allow(dead_code)]
pub(crate) struct Planet {
    pos: Point,
    dir: Point,
    // Density D in kg/m³
    pub(crate) density: f64,
    // Radius r in m
    pub(crate) radius: f64,
    // Speed s in m/s
    pub(crate) speed: Cell<f64>,

    pub(crate) velocity: Cell<Point>,
}

#[allow(dead_code)]
impl Planet {
    pub(crate) fn new(x: f64, y: f64, density: f64, radius: f64) -> Self {
        let dir = Point { x: 5.0, y: 5.0 };
        let speed = 1.0;
        Planet {
            density,
            dir: dir.norm(),
            pos: Point { x, y },
            radius,
            speed: Cell::new(speed),
            velocity: Cell::new(dir.norm() * speed),
        }
    }

    pub(crate) fn new_rng() -> Self {
        let mut rng = rand::thread_rng();

        let density = 5513.0;
        let radius = rng.gen_range(3_000_000.0, 8_000_000.0);

        let dir = Point {
            x: rng.gen_range(-5.0, 5.0),
            y: rng.gen_range(-5.0, 5.0),
        };
        let pos = Point {
            x: rng.gen_range(0.0, 800.0),
            y: rng.gen_range(0.0, 600.0),
        };
        let speed = rng.gen_range(1.0, 4.0);

        Planet {
            density,
            dir: dir.norm(),
            pos,
            radius,
            speed: Cell::new(speed),

            velocity: Cell::new(dir.norm() * speed),
        }
    }

    // Mass m in kg/m³
    pub(crate) fn mass(&self) -> f64 {
        self.density * self.volume()
    }

    // Volume V in m³
    pub(crate) fn volume(&self) -> f64 {
        4.0 / 3.0 * PI * (self.radius as f64).powf(3.0)
    }

    pub(crate) fn accelerate(&self, acc: Point) {
        self.velocity.set(self.velocity.get() + acc.norm());
        self.speed.set(self.speed.get() + acc.mag() / self.mass());
    }

    pub(crate) fn pos(&self) -> Point {
        self.pos
    }

    pub(crate) fn mv(&mut self, max_x: f64, max_y: f64) {
        //let mut x = self.pos.x + self.dir.x * self.speed;
        //let mut y = self.pos.y - self.dir.y * self.speed;
        let mut x = self.pos.x + self.velocity.get().x;
        let mut y = self.pos.y + self.velocity.get().y;

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
}

impl std::fmt::Debug for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "X: {}", self.pos().x)?;
        write!(f, "\nY")?;

        Ok(())
    }
}

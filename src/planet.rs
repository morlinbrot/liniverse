extern crate rand;

use rand::Rng;
use std::cell::Cell;
use std::f64::consts::PI;

use super::*;

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
#[derive(Clone)]
pub(crate) struct Planet {
    pos: Cell<Point>,
    // Density D in kg/m³
    density: f64,
    // Radius r in m
    pub(crate) radius: f64,
    pub(crate) velocity: Cell<Point>,
    pub(crate) dead: Cell<bool>,
}

#[allow(dead_code)]
impl Planet {
    pub(crate) fn new(x: f64, y: f64, density: f64, radius: f64) -> Self {
        let velocity = Point { x: 1.0, y: 1.0 };

        Planet {
            density,
            radius,
            pos: Cell::new(Point { x, y }),
            velocity: Cell::new(velocity),
            dead: Cell::new(false),
        }
    }

    pub(crate) fn new_rng() -> Self {
        let mut rng = rand::thread_rng();

        let density = 5513.0;
        let radius = rng.gen_range(5.0, 10.0);

        let pos = Point {
            x: rng.gen_range(0.0, DIMENSIONS.0),
            y: rng.gen_range(0.0, DIMENSIONS.1),
        };

        let velocity = Point {
            x: rng.gen_range(-3.0, 3.0),
            y: rng.gen_range(-3.0, 3.0),
        };

        Planet {
            density,
            radius,
            pos: Cell::new(pos),
            velocity: Cell::new(velocity),
            dead: Cell::new(false),
        }
    }

    pub(crate) fn pos(&self) -> Point {
        self.pos.get()
    }

    pub(crate) fn mass(&self) -> f64 {
        self.density * self.volume()
    }

    pub(crate) fn volume(&self) -> f64 {
        4.0 / 3.0 * PI * (self.radius as f64).powf(3.0)
    }

    pub(crate) fn accelerate(&self, acc: Point) {
        self.velocity.set(self.velocity.get() + acc);
    }

    pub(crate) fn eat(&self, _other_p: &Planet) {
        unimplemented!();
    }

    pub(crate) fn update(&self) {
        let max_x = DIMENSIONS.0;
        let max_y = DIMENSIONS.1;

        let mut x = self.pos().x + self.velocity.get().x;
        let mut y = self.pos().y + self.velocity.get().y;

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

        self.pos.set(Point { x, y });
    }
}

//use std::iter::FromIterator;
//impl<'a> FromIterator<&'a Planet> for Planet {
//    fn from_iter<I: IntoIterator<Item = &'a Planet>>(iter: I) -> Self {
//        Planet::new_rng()
//    }
//}

impl std::fmt::Debug for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "X: {}", self.pos().x)?;
        write!(f, "\nY")?;

        Ok(())
    }
}

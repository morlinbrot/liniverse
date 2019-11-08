extern crate rand;

use rand::Rng;
use std::cell::Cell;
use std::f64::consts::PI;
use uuid::Uuid;

use super::*;

// Earth
//let density = 5513.0;
//let radius = 6_371.0;
//let mass = 5.9722 * 10_f64.powf(24);

// Sun
//let radius = 695_510.0;
//let density = 1409.0;
//let mass = 7.9897 * 10_f64.powf(30);

// Moon
//let radius = 1737.0;
//let density = 3344.0;
//let mass = 7.348 * 10_f64.powf(22);

// Mars
//let radius = 3389.5;
//let density = 3934;
//let mass = 6.419 * 10_f64.powf(23);

/// A full blown planet living inside our [`Universe`](./universe/struct.Universe.html).
///
/// We're using `Cell` and the interior mutability pattern to be able to loop over immutable
/// references to `Planet`s on each [`tick`](./universe/struct.Universe.html#method.tick) and still be able to mutate the fields.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Planet {
    id: Uuid,
    /// Vector of the planet's current coordinates.
    pos: Cell<Point>,
    /// Density D in kg/m³.
    density: Cell<f64>,
    /// Radius r in m.
    radius: Cell<f64>,
    /// The vector at which the planet will travel on the next
    /// [`update`](../planet/struct.Planet.html#method.update).
    velocity: Cell<Point>,
    /// Marks the planet to be removed from [`Universe`](../universe/struct.Universe.html)'s `planets`.
    dead: Cell<bool>,
}

#[allow(dead_code, non_snake_case)]
impl Planet {
    /// Create a `Planet` with given parameters.
    pub fn new(x: f64, y: f64, density: f64, radius: f64, velocity: Point) -> Self {
        Planet {
            id: Uuid::new_v4(),
            density: Cell::new(density),
            radius: Cell::new(radius),
            pos: Cell::new(Point { x, y }),
            velocity: Cell::new(velocity),
            dead: Cell::new(false),
        }
    }

    /// Create a `Planet` with randomly generated parameters.
    pub fn new_rng(dimensions: (f64, f64)) -> Self {
        let mut rng = rand::thread_rng();

        let density = 5513.0;
        let radius = rng.gen_range(5.0, 10.0);

        let pos = Point {
            x: rng.gen_range(0.0, dimensions.0),
            y: rng.gen_range(0.0, dimensions.1),
        };

        let velocity = Point {
            x: rng.gen_range(-0.2, 1.5),
            y: rng.gen_range(-0.2, 1.5),
        };

        Planet {
            id: Uuid::new_v4(),
            density: Cell::new(density),
            radius: Cell::new(radius),
            pos: Cell::new(pos),
            velocity: Cell::new(velocity),
            dead: Cell::new(false),
        }
    }

    /// Create a `Planet` with randomly generated parameters at a specified position.
    pub fn new_semi_rng(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();

        let density = 5_000.0;
        let radius = rng.gen_range(10.0, 12.0);

        let velocity = Point {
            x: rng.gen_range(-0.2, 1.5),
            y: rng.gen_range(-0.2, 1.5),
        };

        Planet {
            id: Uuid::new_v4(),
            density: Cell::new(density),
            radius: Cell::new(radius),
            pos: Cell::new(Point { x, y }),
            velocity: Cell::new(velocity),
            dead: Cell::new(false),
        }
    }

    /// Update the planet's position by adding its velocity to its current position.
    /// If it moves out of the universe's dimensions, it's inserted back in on the other side.
    pub fn update(&self, dimensions: (f64, f64)) {
        let max_x = dimensions.0;
        let max_y = dimensions.1;

        let mut x = self.pos().x + self.velocity().x;
        let mut y = self.pos().y + self.velocity().y;

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

    /// Add two masses together, calculate the new volume and derive a new radius.  
    /// V = m/D  
    /// V = 4 / 3 * π * radius³  
    /// r³ = V / (4 / 3 * π)  
    pub fn eat(&self, other_p: &Planet) {
        let m = self.mass() + other_p.mass();
        let D = (self.density() + other_p.density()) / 2.0;
        let V = m / D;

        let r = (V / (4.0 / 3.0 * PI)).cbrt();
        self.radius.set(r);
    }

    /// Add a given acceleration to the planet's velocity. The acceleration vector
    /// should represent the single net force to be applied each tick.
    pub fn accelerate(&self, acc: Point) {
        self.velocity.set(self.velocity.get() + acc);
    }

    pub fn pos(&self) -> Point {
        self.pos.get()
    }

    pub fn dead(&self) -> bool {
        self.dead.get()
    }

    pub fn die(&self) {
        self.dead.set(true)
    }

    pub fn radius(&self) -> f64 {
        self.radius.get()
    }

    pub fn mass(&self) -> f64 {
        self.density() * self.volume()
    }

    fn density(&self) -> f64 {
        self.density.get()
    }

    fn velocity(&self) -> Point {
        self.velocity.get()
    }

    fn volume(&self) -> f64 {
        4.0 / 3.0 * PI * (self.radius() as f64).powf(3.0)
    }
}

impl std::fmt::Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{:>13}", "PLANET")?;
        writeln!(f, "{:>12}: x: {}, y: {}", "Pos", self.pos().x, self.pos().y)?;
        writeln!(f, "{:>12}: {}", "Density", self.density())?;
        writeln!(f, "{:>12}: {}", "Radius", self.radius())?;
        writeln!(
            f,
            "{:>12}: x: {}, y: {}",
            "Velocity",
            self.velocity().x,
            self.velocity().y
        )?;
        writeln!(f, "{:>12}: {}", "Magnitude", self.velocity().mag())?;
        writeln!(f, "{:>12}: {}", "Dead", self.dead())?;
        writeln!(f, "{:>12}: {}", "Volume", self.volume())?;
        writeln!(f, "{:>12}: {}", "Mass", self.mass())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let v = Point::new(1.0, 1.0);
        let p1 = Planet::new(10.0, 10.0, 1000.0, 10.0, v);
        let _p2 = Planet::new(20.0, 20.0, 1000.0, 10.0, v);
        let dimensions = (800.0, 600.0);

        assert_eq!(4188790.204786391, p1.mass());
        assert_eq!(4188.790204786391, p1.volume());

        p1.accelerate(Point::new(1.0, 1.0));
        p1.update(dimensions);
        assert_eq!(Point::new(12.0, 12.0), p1.pos());

        p1.accelerate(Point::new(dimensions.0, dimensions.1));
        p1.update(dimensions);
        assert_eq!(Point::new(14.0, 14.0), p1.pos());

        // Run `cargo test -- --nocapture` to see `println!` output.
        //println!("{}", p1);
    }
}

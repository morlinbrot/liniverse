use uuid::Uuid;

use super::*;

/// An example implementation of [`Newtonian`](./trait.Newtonian.html).
pub struct Body {
    center: Point,
    id: Uuid,
    mass: f64,
    velocity: Point,
}

impl Body {
    pub fn new(id: Uuid, center: Point, mass: f64) -> Self {
        Self {
            center,
            id,
            mass,
            velocity: Point::new(0.0, 0.0),
        }
    }
}

impl Newtonian for Body {
    fn id(&self) -> Uuid {
        self.id
    }

    fn mass(&self) -> f64 {
        self.mass
    }

    fn position(&self) -> Point {
        self.center
    }

    fn velocity(&self) -> Point {
        self.velocity
    }

    fn set_position(&mut self, new_position: Point) {
        self.center = new_position;
    }

    fn set_velocity(&mut self, new_velocity: Point) {
        self.velocity = new_velocity;
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let p = self.position();
        let v = self.velocity();
        writeln!(f, "{:>13}", "BODY")?;
        writeln!(f, "{:>12}: {}", "Id", self.id())?;
        writeln!(f, "{:>12}: {}", "Mass", self.mass())?;
        writeln!(f, "{:>12}: x: {}, y: {}", "Pos", p.x, p.y)?;
        writeln!(f, "{:>12}: x: {}, y: {}", "Velocity", v.x, v.y)?;
        Ok(())
    }
}

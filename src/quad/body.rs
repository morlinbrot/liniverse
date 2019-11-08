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
    #[allow(non_snake_case)]
    fn apply_force(&mut self, body: QuadBody, delta_time: f64) -> Result<(), std::io::Error> {
        let G = 6.67 * 10_f64.powf(-11.0);

        // TODO: Get time right.
        let dt = delta_time;

        let body = body.borrow();

        // Distance r between the two bodies.
        let dx = (body.center().x - self.center().x).powf(2.0);
        let dy = (body.center().y - self.center().y).powf(2.0);
        let r = (dx + dy).sqrt();

        // Net force being exerted on the body.
        let F = (G * self.mass() * body.mass()) / r.powf(2.0);
        let Fx = F * dx / r;
        let Fy = F * dy / r;

        // Compute acceleration.
        let ax = Fx / self.mass();
        let ay = Fy / self.mass();

        let vx = self.velocity().x + dt * ax;
        let vy = self.velocity().y + dt * ay;

        // Compute new position.
        let px = self.center().x + dt * vx;
        let py = self.center().y + dt * vy;

        self.center = Point { x: px, y: py };
        Ok(())
    }

    fn center(&self) -> Point {
        self.center
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn mass(&self) -> f64 {
        self.mass
    }

    fn update_position(&mut self, new_position: Point) {
        self.center = new_position;
    }

    fn velocity(&self) -> Point {
        self.velocity
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{:>13}", "BODY")?;
        let x = self.center().x;
        let y = self.center().y;
        writeln!(f, "{:>12}: x: {}, y: {}", "Pos", x, y)?;
        writeln!(f, "{:>12}: {}", "Mass", self.mass())?;
        Ok(())
    }
}

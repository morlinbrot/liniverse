// Implementation following
// http://arborjs.org/docs/barnes-hut
use super::{
    point::Point,
    rect::{Cardinal, Rect},
};
use std::rc::Rc;
use uuid::Uuid;

pub trait QuadNodeBody {
    fn calc_f(&mut self, body: Rc<dyn QuadNodeBody>, delta_time: f64);
    fn center(&self) -> Point;
    fn id(&self) -> Uuid;
    fn mass(&self) -> f64;
    fn velocity(&self) -> Point;
}

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

impl QuadNodeBody for Body {
    fn calc_f(&mut self, body: Rc<dyn QuadNodeBody>, delta_time: f64) {
        let G = 6.67 * 10_f64.powf(-11.0);

        // TODO: Get time right.
        let dt = delta_time;

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

    fn velocity(&self) -> Point {
        self.velocity
    }
}

pub struct QuadNode {
    capacity: usize,
    center_of_mass: Point,
    bodies: Vec<Rc<dyn QuadNodeBody>>,
    mass: Option<f64>,
    nodes: Option<Box<[Self; 4]>>,
    rect: Rect,
}

impl QuadNode {
    pub fn new(capacity: usize, rect: Rect) -> Self {
        let center_of_mass = rect.center();
        Self {
            capacity,
            center_of_mass,
            rect,
            bodies: Vec::new(),
            mass: None,
            nodes: None,
        }
    }

    pub fn insert(&mut self, body: Rc<dyn QuadNodeBody>) -> Result<(), std::io::Error> {
        self.update_aggregation(body.clone());

        if self.bodies.len() < self.capacity {
            self.bodies.push(body);
            return Ok(());
        }

        if self.nodes.is_none() {
            self.subdivide();
        }

        let mut bodies = vec![body];
        bodies.append(&mut self.bodies);

        for body in bodies {
            for node in self.nodes.as_mut().unwrap().iter_mut() {
                if node.rect.contains(&body.center()) {
                    node.insert(body)?;
                    break;
                }
            }
        }

        Ok(())
    }

    // Calculations based on
    // https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/nbody.html
    #[allow(non_snake_case)]
    pub fn calc_f(&self, target_body: Rc<dyn QuadNodeBody>, F: f64) -> f64 {
        // Means we're an external node. apply all contained bodie's forces.
        if self.nodes.is_none() {
            for body in &self.bodies {
                let target = target_body.clone();
                //let _: () = target;
                target.get_mut().calc_f(body.clone(), 0.5);
            }
        }

        1.0
    }

    fn subdivide(&mut self) {
        self.nodes = Some(Box::new([
            Self::new(self.capacity, self.rect.split_rect(Cardinal::NW)),
            Self::new(self.capacity, self.rect.split_rect(Cardinal::NE)),
            Self::new(self.capacity, self.rect.split_rect(Cardinal::SE)),
            Self::new(self.capacity, self.rect.split_rect(Cardinal::SW)),
        ]));
    }

    fn update_aggregation(&mut self, b: Rc<dyn QuadNodeBody>) {
        let (new_mass, new_x, new_y) = match self.mass {
            Some(mass) => {
                let new_mass = mass + b.mass();
                (
                    new_mass,
                    self.center_of_mass.x * mass + b.center().x * b.mass() / new_mass,
                    self.center_of_mass.y * mass + b.center().y * b.mass() / new_mass,
                )
            }
            None => (b.mass(), b.center().x, b.center().y),
        };

        self.mass = Some(new_mass);
        self.center_of_mass = Point::new(new_x, new_y);
    }
}

#[cfg(test)]

mod test {
    use super::*;

    fn setup() -> (QuadNode, Vec<Rc<Body>>) {
        let width = 100.00;
        let height = 100.0;
        let bounds = Rect::new(width / 2.0, height / 2.0, width, height);
        let bodies = vec![
            Rc::new(Body::new(Uuid::new_v4(), Point::new(2.0, 2.0), 10.0)),
            Rc::new(Body::new(Uuid::new_v4(), Point::new(3.0, 3.0), 10.0)),
        ];

        (QuadNode::new(1, bounds), bodies)
    }

    #[test]
    fn insert() {
        let (mut qnode, bodies) = setup();
        qnode.insert(bodies[0].clone()).unwrap();
        assert_eq!(qnode.bodies.len(), 1);
        assert_eq!(qnode.mass, Some(10.0));
    }

    #[test]
    fn insert_and_subdivide() {
        let (mut qnode, bodies) = setup();
        qnode.insert(bodies[0].clone()).unwrap();
        qnode.insert(bodies[1].clone()).unwrap();
        assert_eq!(qnode.bodies.len(), 0);
        assert_eq!(qnode.mass, Some(20.0));
        assert!(qnode.nodes.is_some());
    }
}

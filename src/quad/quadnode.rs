// Implementation following
// http://arborjs.org/docs/barnes-hut
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use super::*;

/// The trait that any struct must implement to be inserted as a body into a
/// [`QuadNode`](./struct.QuadNode.html).
pub trait Newtonian {
    fn id(&self) -> Uuid;
    fn mass(&self) -> f64;
    fn position(&self) -> Point;
    fn velocity(&self) -> Point;
    fn set_position(&mut self, new_position: Point);
    fn set_velocity(&mut self, new_velocity: Point);
}

// Used for testing.
// pub trait DisplayNewtonian: Newtonian + std::fmt::Display {}
// impl DisplayNewtonian for Body {}
// type QuadBody = Rc<RefCell<dyn DisplayNewtonian>>;

pub type QuadBody = Rc<RefCell<dyn Newtonian>>;

/// Shared config that applies to all nodes in the tree.
pub struct QuadConfig {
    capacity: usize,
    theta: f64,
}

/// Used to construct a quad tree. Ether holds a vector of bodies up until its capacity or aggregates the mass and
/// center of mass of all the bodies that may be held by nodes further down the tree.
///
/// Any struct implementing [`Newtonian`](./trait.Newtonian.html) may be inserted
/// into the tree. When passed to [`apply_forces`](./struct.QuadNode.html#method.apply_forces),
/// gravitational forces of all the bodies in the tree will be applied.
///
/// The [`QuadConfig`](./struct.QuadConfig.html)'s `theta` value sets the threshhold at which
/// a node's aggregated values will be applied instead of an individual body's ones.
pub struct QuadNode {
    /// Config shared across nodes.
    cfg: Rc<QuadConfig>,
    /// The center of mass of the node, aggregated across all contained bodies.
    com: Point,
    /// All the bodies currently held by the node. Empty if node is internal.
    bodies: Vec<QuadBody>,
    /// Aggregated mass of all contained bodies.
    mass: Option<f64>,
    /// An array of four sub-nodes, splitting this node into its quadrants. `None` if node is
    /// external.
    nodes: Option<Box<[Self; 4]>>,
    /// A [`Rect`](../rect/struct.Rect.html) representing the Cartesian plane this node covers.
    rect: Rect,
}

impl QuadNode {
    pub fn new(cfg: Rc<QuadConfig>, rect: Rect) -> Self {
        let com = rect.center();
        Self {
            cfg,
            com,
            rect,
            bodies: Vec::new(),
            mass: None,
            nodes: None,
        }
    }

    pub fn insert(&mut self, body: QuadBody) -> Result<(), std::io::Error> {
        self.aggregate(body.clone());

        if self.bodies.len() < self.cfg.capacity {
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
                if node.rect.contains(&body.borrow().position()) {
                    node.insert(body)?;
                    break;
                }
            }
        }

        Ok(())
    }

    #[allow(non_snake_case)]
    fn calculate_f(a: QuadBody, b: QuadBody, delta: f64) -> Result<(), std::io::Error> {
        let mut a = a.borrow_mut();
        let b = b.borrow();

        let G = 6.67 * 10_f64.powf(-11.0);

        // Distance r between the two bodies.
        let dx = (b.position().x - a.position().x).powf(2.0);
        let dy = (b.position().y - a.position().y).powf(2.0);
        let r = (dx + dy).sqrt();

        // Net force being exerted on the body.
        let F = (G * a.mass() * b.mass()) / r.powf(2.0);
        let Fx = F * dx / r;
        let Fy = F * dy / r;

        // Compute acceleration.
        let ax = Fx / a.mass();
        let ay = Fy / a.mass();

        let vx = a.velocity().x + delta * ax;
        let vy = a.velocity().y + delta * ay;

        // Compute new position.
        let px = a.position().x + delta * vx;
        let py = a.position().y + delta * vy;

        a.set_position(Point { x: px, y: py });
        Ok(())
    }

    // Calculations based on
    // https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/nbody.html
    pub fn apply_forces(&self, target_body: QuadBody, delta: f64) -> Result<(), std::io::Error> {
        match &self.nodes {
            // Otherwise.
            Some(nodes) => {
                let s = (self.rect.width() + self.rect.height()) / 2.0;
                let d = self.com.distance_to(target_body.borrow().position());
                let ratio = s / d;

                // We are far away from the body and simply apply the aggregated values.
                if self.cfg.theta < ratio {
                    let aggregation = Rc::new(RefCell::new(Body::new(
                        Uuid::new_v4(),
                        self.com,
                        self.mass.unwrap(),
                    )));
                    QuadNode::calculate_f(target_body.clone(), aggregation.clone(), delta)?;
                }

                // We keep going recursively.
                for node in nodes.iter() {
                    node.apply_forces(target_body.clone(), delta)?;
                }
            }
            // External node.
            None => {
                for body in &self.bodies {
                    if target_body.borrow().id() != body.borrow().id() {
                        QuadNode::calculate_f(target_body.clone(), body.clone(), delta)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn subdivide(&mut self) {
        self.nodes = Some(Box::new([
            Self::new(self.cfg.clone(), self.rect.split_rect(Cardinal::NW)),
            Self::new(self.cfg.clone(), self.rect.split_rect(Cardinal::NE)),
            Self::new(self.cfg.clone(), self.rect.split_rect(Cardinal::SE)),
            Self::new(self.cfg.clone(), self.rect.split_rect(Cardinal::SW)),
        ]));
    }

    fn aggregate(&mut self, body: QuadBody) {
        let body = body.borrow();
        let (new_mass, new_x, new_y) = match self.mass {
            Some(mass) => {
                let new_mass = mass + body.mass();
                (
                    new_mass,
                    (self.com.x * mass + body.position().x * body.mass()) / new_mass,
                    (self.com.y * mass + body.position().y * body.mass()) / new_mass,
                )
            }
            None => (body.mass(), body.position().x, body.position().y),
        };

        self.mass = Some(new_mass);
        self.com = Point::new(new_x, new_y);
    }
}

#[cfg(test)]

mod test {
    use super::*;

    fn setup() -> (QuadNode, Vec<Rc<RefCell<Body>>>) {
        let width = 10.00;
        let height = 10.0;
        let bounds = Rect::new(width / 2.0, height / 2.0, width, height);

        let cfg = Rc::new(QuadConfig {
            capacity: 1,
            theta: 0.5,
        });

        let mut bodies = vec![];
        let mass = 1.0 * 10_f64.powf(6.0);
        let positions = vec![
            Point::new(4.0, 6.0),
            Point::new(6.0, 6.0),
            Point::new(7.0, 7.0),
            Point::new(4.0, 4.0),
        ];
        for pos in positions {
            let id = Uuid::new_v4();
            bodies.push(Rc::new(RefCell::new(Body::new(id, pos, mass))));
        }

        (QuadNode::new(cfg, bounds), bodies)
    }

    #[test]
    fn insert_and_aggregate() {
        let (mut qnode, bodies) = setup();
        let b1 = &bodies[0];
        qnode.insert(b1.clone()).unwrap();
        assert_eq!(qnode.bodies.len(), 1);
        // Node should have agg. mass of the inserted body.
        assert_eq!(qnode.mass, Some(b1.borrow().mass()));
    }

    #[test]
    fn insert_and_subdivide() {
        let (mut qnode, bodies) = setup();
        let b1 = &bodies[0];
        let b2 = &bodies[1];
        qnode.insert(b1.clone()).unwrap();
        qnode.insert(b2.clone()).unwrap();
        assert_eq!(qnode.bodies.len(), 0);
        // Node should have agg. mass of two inserted bodies.
        let agg_m = b1.borrow().mass() + b2.borrow().mass();
        assert_eq!(qnode.mass, Some(agg_m));
        assert!(qnode.nodes.is_some());
    }

    #[test]
    fn update_forces() {
        // TODO: Test calculate_f with actual values.
        let (mut qnode, bodies) = setup();
        for b in &bodies {
            qnode.insert(b.clone()).unwrap();
        }

        for b in &bodies {
            qnode.apply_forces(b.clone(), 1.0).unwrap();
            println!("{}", b.borrow().position());
        }
    }
}

// Implementation following
// http://arborjs.org/docs/barnes-hut
use super::{
    point::Point,
    rect::{Cardinal, Rect},
};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

/// The trait that any struct must implement to be inserted as a body into a
/// [`QuadNode`](../quadnode/struct.QuadNode.html).
pub trait Liniversable {
    fn apply_force(&mut self, body: QuadNodeBody, delta_time: f64) -> Result<(), std::io::Error>;
    fn center(&self) -> Point;
    fn id(&self) -> Uuid;
    fn mass(&self) -> f64;
    fn velocity(&self) -> Point;
    fn update_position(&mut self, new_position: Point);
}

type QuadNodeBody = Rc<RefCell<dyn Liniversable>>;

/// An example struct implementing [`Liniversable`](./trait.Liniversable.html).
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

impl Liniversable for Body {
    #[allow(non_snake_case)]
    fn apply_force(&mut self, body: QuadNodeBody, delta_time: f64) -> Result<(), std::io::Error> {
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

/// Shared config that applies to all nodes in the tree.
pub struct QuadConfig {
    capacity: usize,
    theta: f64,
}

/// Used to construct a quad tree. Ether holds a vector of bodies up until its capacity or aggregates the mass and
/// center of mass of all the bodies that may be held by nodes further down the tree.
///
/// Any struct implementing [`Liniversable`](../quadnode/trait.Liniversable.html) may be inserted
/// into the tree. When passed to [`apply_forces`](./struct.QuadNode.html#method.apply_forces),
/// gravitational forces of all the bodies in the tree will be applied.
///
/// The [`QuadConfig`](../quadnode/struct.QuadConfig.html)'s `theta` value sets the threshhold at which
/// a node's aggregated values will be applied instead of an individual body's ones.
pub struct QuadNode {
    /// Config shared across nodes.
    cfg: Rc<QuadConfig>,
    /// The center of mass of the node, aggregated across all contained bodies.
    com: Point,
    /// All the bodies currently held by the node. Empty if node is internal.
    bodies: Vec<QuadNodeBody>,
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

    pub fn insert(&mut self, body: QuadNodeBody) -> Result<(), std::io::Error> {
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
                if node.rect.contains(&body.borrow().center()) {
                    node.insert(body)?;
                    break;
                }
            }
        }

        Ok(())
    }

    // Calculations based on
    // https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/nbody.html
    pub fn apply_forces(&self, target_body: QuadNodeBody) -> Result<(), std::io::Error> {
        let time = 0.5;
        match &self.nodes {
            // Otherwise.
            Some(nodes) => {
                let s = (self.rect.width() + self.rect.height()) / 2.0;
                let d = self.com.distance_to(target_body.borrow().center());
                let ratio = s / d;

                // We are far away from the body and simply apply the aggregated values.
                if self.cfg.theta < ratio {
                    let self_as_body = Rc::new(RefCell::new(Body::new(
                        Uuid::new_v4(),
                        self.com,
                        self.mass.unwrap(),
                    )));
                    target_body.borrow_mut().apply_force(self_as_body, time)?;
                }

                // We keep going recursively.
                for node in nodes.iter() {
                    node.apply_forces(target_body.clone())?;
                }
            }
            // External node.
            None => {
                for body in &self.bodies {
                    if target_body.borrow().id() != body.borrow().id() {
                        target_body.borrow_mut().apply_force(body.clone(), time)?;
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

    fn aggregate(&mut self, body: QuadNodeBody) {
        let body = body.borrow();
        let (new_mass, new_x, new_y) = match self.mass {
            Some(mass) => {
                let new_mass = mass + body.mass();
                (
                    new_mass,
                    (self.com.x * mass + body.center().x * body.mass()) / new_mass,
                    (self.com.y * mass + body.center().y * body.mass()) / new_mass,
                )
            }
            None => (body.mass(), body.center().x, body.center().y),
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
        let bodies = vec![
            Rc::new(RefCell::new(Body::new(
                Uuid::new_v4(),
                // Spawn in NW quadrant.
                Point::new(4.0, 6.0),
                10.0,
            ))),
            Rc::new(RefCell::new(Body::new(
                Uuid::new_v4(),
                // Spawn in NE quadrant.
                Point::new(6.0, 6.0),
                10.0,
            ))),
            Rc::new(RefCell::new(Body::new(
                Uuid::new_v4(),
                // Spawn another point in NE quadrant.
                Point::new(7.0, 7.0),
                10.0,
            ))),
            Rc::new(RefCell::new(Body::new(
                Uuid::new_v4(),
                // Spawn in SW quadrant.
                Point::new(4.0, 4.0),
                10.0,
            ))),
        ];

        (QuadNode::new(cfg, bounds), bodies)
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

    #[test]
    fn update_forces() {
        // TODO: Not yet working!
        let (mut qnode, bodies) = setup();
        let b1 = &bodies[0];
        let b2 = &bodies[1];
        let b3 = &bodies[2];
        let b4 = &bodies[3];
        qnode.insert(b1.clone()).unwrap();
        qnode.insert(b2.clone()).unwrap();
        qnode.insert(b3.clone()).unwrap();
        qnode.insert(b4.clone()).unwrap();

        qnode.apply_forces(b1.clone()).unwrap();

        println!("b1: {}", b1.borrow().center());
        println!("b2: {}", b2.borrow().center());
    }
}

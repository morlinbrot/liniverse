// Implementation following
// http://arborjs.org/docs/barnes-hut and
// https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/nbody.html
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
    pub capacity: usize,
    pub theta: f64,
}

/// Used to construct a quad tree. Ether holds a vector of bodies up until its capacity or aggregates the mass and
/// center of mass of all the bodies that may be held by nodes further down the tree.
///
/// Any struct implementing [`Newtonian`](./trait.Newtonian.html) may be inserted
/// into the tree. When passed to [`sum_up_force`](./struct.QuadNode.html#method.sum_up_force),
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

    /// Insert a [`Newtonian`](./trait.Newtonian.html) body into the tree.
    pub fn insert(&mut self, body: QuadBody) -> Result<(), std::io::Error> {
        self.aggregate(body.clone());

        // If we're still an external node and we're not at capacity yet, we insert and return.
        if self.nodes.is_none() && self.bodies.len() < self.cfg.capacity {
            self.bodies.push(body);
            return Ok(());
        }

        // Capacity has been reached but we don't have sub-nodes yet.
        if self.nodes.is_none() {
            self.subdivide();
        }

        // We're at capacity and have sub-nodes.
        // On reaching capacity for the first time, we need to make sure to pass on any already
        // contained bodies.
        let mut bodies = vec![body];
        bodies.append(&mut self.bodies);

        // All bodies are then recursively passed on to our sub-nodes.
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

    /// Update a [`Newtonian`](./trait.Newtonian.html) body with the net graviational force being exerted on
    /// it by calling [`set_velocity`](./struct.Body.html#method.set_velocity) and [`set_position`](./struct.Body.html#method.set_position) with the updated values.
    pub fn sum_up_force(&self, target_body: QuadBody, delta: f64, net_v: Point) -> Point {
        println!("Starting with: {}", net_v.x);
        let mut net_v = net_v;
        match &self.nodes {
            // Internal node.
            Some(nodes) => {
                let s = (self.rect.width() + self.rect.height()) / 2.0;
                let d = target_body.borrow().position().distance_to(self.com);
                //let d = self.com.distance_to(target_body.borrow().position());
                let ratio = s / d;

                // We are far away from the body and simply apply the aggregated values.
                if ratio < self.cfg.theta {
                    //if self.cfg.theta < ratio {
                    let aggregation = Rc::new(RefCell::new(Body::new(
                        Uuid::new_v4(),
                        self.com,
                        self.mass.unwrap(),
                    )));
                    let f = QuadNode::calc_force(target_body.clone(), aggregation);
                    let v = QuadNode::calc_velocity(target_body.clone(), f, delta);
                    net_v += v;
                    println!("Returning from agg: {}", &net_v);
                    return net_v;
                }

                // We keep going recursively.
                for node in nodes.iter() {
                    return node.sum_up_force(target_body.clone(), delta, net_v);
                }
            }
            // External node.
            None => {
                for body in &self.bodies {
                    if target_body.borrow().id() != body.borrow().id() {
                        let f = QuadNode::calc_force(target_body.clone(), body.clone());
                        let v = QuadNode::calc_velocity(target_body.clone(), f, delta);
                        net_v += v;
                        println!("Returning from ext: {}", &net_v);
                        return net_v;
                    }
                }
            }
        }

        net_v
    }

    pub fn update_body(&self, target_body: QuadBody, delta: f64) -> Result<(), std::io::Error> {
        let net_v = self.sum_up_force(target_body.clone(), delta, Point::new(0.0, 0.0));

        // Update velocity to be able to use it in the next tick.
        target_body.borrow_mut().set_velocity(net_v);

        let max_x = self.rect.width();
        let max_y = self.rect.height();

        let mut x = target_body.borrow().position().x + delta * net_v.x;
        let mut y = target_body.borrow().position().y + delta * net_v.y;

        // Make sure bodies don't leave the visible area.
        if x > max_x {
            x -= max_x;
        } else if x < 0.0 {
            x += max_x;
        }

        if y > max_y {
            y -= max_y;
        } else if y < 0.0 {
            y += max_y;
        }

        // Apply net velocity to compute new position.
        target_body.borrow_mut().set_position(Point::new(x, y));

        Ok(())
    }
    #[allow(non_snake_case)]
    fn calc_force(a: QuadBody, b: QuadBody) -> Point {
        let a = a.borrow();
        let b = b.borrow();

        let G = 6.6726 * 10_f64.powf(-11.0);

        // Distance r between the two bodies.
        let dx = b.position().x - a.position().x;
        let dy = b.position().y - a.position().y;
        let r = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

        // Net force being exerted onto the body.
        let F = (G * a.mass() * b.mass()) / r.powf(2.0);
        let Fx = F * (dx / r);
        let Fy = F * (dy / r);

        Point { x: Fx, y: Fy }
    }

    #[allow(non_snake_case)]
    fn calc_velocity(a: QuadBody, F: Point, delta: f64) -> Point {
        let a = a.borrow();

        // Compute acceleration at time t.
        let ax = F.x / a.mass();
        let ay = F.y / a.mass();

        // Compute velocity at time dt / 2.
        let vx = a.velocity().x + delta * ax;
        let vy = a.velocity().y + delta * ay;

        Point { x: vx, y: vy }
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

    fn setupdate_body() -> (QuadNode, Vec<Rc<RefCell<Body>>>) {
        let width = 10.00;
        let height = 10.0;
        let bounds = Rect::new(width / 2.0, height / 2.0, width, height);

        let cfg = Rc::new(QuadConfig {
            capacity: 1,
            theta: 0.5,
        });

        let mut bodies = vec![];
        let mass = 100.0;
        let positions = vec![
            Point::new(4.0, 6.0),
            Point::new(6.0, 6.0),
            Point::new(8.0, 8.0),
            Point::new(4.0, 4.0),
        ];
        for pos in positions {
            let id = Uuid::new_v4();
            bodies.push(Rc::new(RefCell::new(Body::new(id, pos, mass))));
        }

        (QuadNode::new(cfg, bounds), bodies)
    }

    #[test]
    fn subdivide_and_aggregate() {
        let (mut qnode, bodies) = setupdate_body();
        let b1 = &bodies[0];
        let b2 = &bodies[1];
        let b3 = &bodies[2];

        qnode.insert(b1.clone()).unwrap();
        // Node should be external and have agg. mass of the inserted body.
        assert_eq!(qnode.bodies.len(), 1);
        assert_eq!(qnode.mass, Some(b1.borrow().mass()));

        qnode.insert(b2.clone()).unwrap();
        // Node should have subdivide (be internal) and have agg. mass of both bodies.
        let agg_m = b1.borrow().mass() + b2.borrow().mass();
        assert_eq!(qnode.bodies.len(), 0);
        assert_eq!(qnode.mass, Some(agg_m));
        assert!(qnode.nodes.is_some());

        qnode.insert(b3.clone()).unwrap();
        // NE quadrant should have further subdivided.
        let nodes = qnode.nodes.unwrap();
        let l1_ne = &nodes[1];
        assert_eq!(l1_ne.bodies.len(), 0);
        assert_eq!(l1_ne.nodes.is_some(), true);

        // b2 & b3 should have been moved to NE and SW quadrants respectively.
        let l2_ne = &l1_ne.nodes.as_ref().unwrap()[1];
        let l2_sw = &l1_ne.nodes.as_ref().unwrap()[3];
        assert_eq!(l2_ne.bodies.len(), 1);
        assert_eq!(l2_sw.bodies.len(), 1);
    }

    #[test]
    fn rule_of_laws() {
        // Newton's first law states that:
        // F = Gm₁m₂ / r²
        // Let's say we have two bodies with a mass of 100kg, 1m apart from each other.
        // Following the above formula, we expect a force of 0.00000066726N to be exerted on b1.
        let res_f = (6.6726 * 10_f64.powf(-11.0) * 100.0 * 100.0) / 1.0;

        let delta = 1.0;
        let mass = 100.0;
        let positions = vec![Point::new(1.0, 1.0), Point::new(1.0, 2.0)];
        let mut bodies = vec![];
        for pos in positions {
            let id = Uuid::new_v4();
            bodies.push(Rc::new(RefCell::new(Body::new(id, pos, mass))));
        }
        let b1 = &bodies[0];
        let b2 = &bodies[1];

        let f = QuadNode::calc_force(b1.clone(), b2.clone());
        // It's only exerted on y since both bodies are on the same x plane.
        assert_eq!(f.y, res_f);

        // With Newton's second law stating that:
        // a = F / m
        // the velocity v₁ of our body at time t₁ can be calculated as
        // v₁ = v₀ + a * dt
        // Since in our example, v₀ = 0 and dt = t₁ - t₀ = 1, we expect
        let res_v = 0.0 + f.y / mass * delta;
        let v = QuadNode::calc_velocity(b1.clone(), f, delta);
        assert_eq!(v.y, res_v);
    }

    #[test]
    fn update_body() {
        let (mut qnode, bodies) = setupdate_body();
        for b in &bodies {
            qnode.insert(b.clone()).unwrap();
        }

        for body in &bodies {
            qnode.update_body(body.clone(), 1.0).unwrap();
            // Use `cargo test -- --nocapture` to see the output.
            //println!("{}", body.borrow());
        }

        //let res_p1 = Point::new(4.000000015094995, 5.999999998903067);
        //assert_eq!(bodies[0].borrow().position(), res_p1);
    }
}

// Implementation following
// http://arborjs.org/docs/barnes-hut
use super::{planet::Planet, point::Point, rect::Rect};

pub struct Body {
    center: Point,
    mass: f64,
}

impl QuadNodeBody for Body {
    fn center(&self) -> Point {
        self.center
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}

pub trait QuadNodeBody {
    fn center(&self) -> Point;
    fn mass(&self) -> f64;
}

pub struct QuadNode<T> {
    capacity: usize,
    center_of_mass: Point,
    bodies: Vec<T>,
    mass: Option<f64>,
    nodes: Option<Box<Vec<Self>>>,
    rect: Rect,
}

impl<T: QuadNodeBody> QuadNode<T> {
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

    pub fn insert(&mut self, body: T) -> Result<(), std::io::Error> {
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
            // TODO: Update center of mass and total mass
            self.update_mass(&body);

            for node in self.nodes.as_mut().unwrap().iter_mut() {
                if node.rect.contains(&body.center()) {
                    node.insert(body)?;
                    break;
                }
            }
        }

        Ok(())
    }

    fn subdivide(&self) {
        unimplemented!();
    }

    fn update_mass(&mut self, b: &T) {
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

    fn init() -> QuadNode<Body> {
        let dimensions = (100.0, 100.0);
        let bounds = Rect::new(1.0, 1.0, dimensions.0, dimensions.1);
        QuadNode::new(1, bounds)
    }

    #[test]
    fn insert() {
        let mut qnode = init();
        let b1 = Body {
            center: Point::new(2.0, 2.0),
            mass: 10.0,
        };
        qnode.insert(b1).unwrap();
        assert_eq!(qnode.bodies.len(), 1);
    }

    #[test]
    fn insert_and_subdivide() {
        //let mut qnode = init();
        //let b1 = Body {
        //    center: Point::new(2.0, 2.0),
        //    mass: 10.0,
        //};
        //let b2 = Body {
        //    center: Point::new(3.0, 3.0),
        //    mass: 10.0,
        //};
        //qnode.insert(b1).unwrap();
        //qnode.insert(b2).unwrap();
        //assert_eq!(qnode.bodies.len(), 0);
        //assert!(qnode.nodes.is_some());
    }
}

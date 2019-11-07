// Implementation following
// http://arborjs.org/docs/barnes-hut
use super::{
    point::Point,
    rect::{Cardinal, Rect},
};
use std::rc::Rc;
use uuid::Uuid;

pub trait QuadNodeBody {
    fn center(&self) -> Point;
    fn id(&self) -> Uuid;
    fn mass(&self) -> f64;
}

pub struct Body {
    center: Point,
    id: Uuid,
    mass: f64,
}

impl Body {
    pub fn new(id: Uuid, center: Point, mass: f64) -> Self {
        Self { center, id, mass }
    }
}

impl QuadNodeBody for Body {
    fn center(&self) -> Point {
        self.center
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}

pub struct QuadNode<T> {
    capacity: usize,
    center_of_mass: Point,
    bodies: Vec<Rc<T>>,
    mass: Option<f64>,
    nodes: Option<Box<[Self; 4]>>,
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

    pub fn insert(&mut self, body: Rc<T>) -> Result<(), std::io::Error> {
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

    fn subdivide(&mut self) {
        self.nodes = Some(Box::new([
            Self::new(self.capacity, self.rect.split_rect(Cardinal::NW)),
            Self::new(self.capacity, self.rect.split_rect(Cardinal::NE)),
            Self::new(self.capacity, self.rect.split_rect(Cardinal::SE)),
            Self::new(self.capacity, self.rect.split_rect(Cardinal::SW)),
        ]));
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

    fn setup() -> (QuadNode<Body>, Vec<Rc<Body>>) {
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
    }

    #[test]
    fn insert_and_subdivide() {
        let (mut qnode, bodies) = setup();
        qnode.insert(bodies[0].clone()).unwrap();
        qnode.insert(bodies[1].clone()).unwrap();
        assert_eq!(qnode.bodies.len(), 0);
        assert!(qnode.nodes.is_some());
    }
}

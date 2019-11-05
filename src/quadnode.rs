// Implementation following
// http://arborjs.org/docs/barnes-hut
use super::{planet::Planet, point::Point, rect::Rect};

pub struct Body {
    pos: Point,
    mass: f64,
}

pub struct QuadNode {
    capacity: usize,
    center_of_mass: Point,
    bodies: Vec<Body>,
    mass: Option<f64>,
    nodes: Option<Box<Vec<Self>>>,
    rect: Rect,
}

impl QuadNode {
    pub(crate) fn new(capacity: usize, rect: Rect) -> Self {
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

    pub(crate) fn insert(&mut self, body: Body) -> Result<(), std::io::Error> {
        if self.bodies.len() <= self.capacity {
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

            for node in self.nodes.as_mut().unwrap().iter_mut() {
                if node.rect.contains(&body.pos) {
                    node.insert(body)?;
                    break;
                }
            }
        }

        //if let Some(nodes) = &mut self.nodes {
        //    let mut bodies = vec![body];
        //    bodies.append(&mut self.bodies);

        //    for b in bodies {
        //        for node in nodes.as_mut() {
        //            if node.rect.contains(&b.pos) {
        //                //let _: () = node;
        //                node.insert(b);
        //                break;
        //            }
        //        }
        //    }
        //}
        Ok(())
    }

    fn subdivide(&self) {
        unimplemented!();
    }
}

extern crate rand;

use std::cell::RefCell;
use std::f64::consts::PI;

use super::*;

type Canvas = web_sys::CanvasRenderingContext2d;

pub struct Universe {
    dimensions: (f64, f64),
    planets: Vec<Rc<RefCell<Planet>>>,
    qtree: QuadNode,
}

impl Universe {
    pub fn new(dimensions: (f64, f64)) -> Self {
        let width = dimensions.0;
        let height = dimensions.1;
        let bounds = Rect::new(width / 2.0, height / 2.0, width, height);

        let cfg = Rc::new(QuadConfig {
            capacity: 1,
            theta: 0.5,
        });

        let qtree = QuadNode::new(cfg, bounds);

        Self {
            dimensions,
            planets: Vec::new(),
            qtree,
        }
    }

    pub fn init_random(&mut self) {
        //let mut rng = rand::thread_rng();
        self.planets.push(Rc::new(RefCell::new(Planet::new(
            self.dimensions.0 / 2.0,
            self.dimensions.1 / 2.0,
            6_000.0,
            20.0,
            Point::new(0.0, 0.0),
        ))));

        for _i in 0..super::NO_OF_PLANETS {
            let planet = Rc::new(RefCell::new(Planet::new_rng(self.dimensions)));
            self.qtree.insert(planet.clone()).unwrap();
            self.planets.push(planet);
        }
    }

    #[allow(dead_code)]
    pub fn log(&self, val: &JsValue) {
        web_sys::console::log_1(&val);
    }

    pub fn add_planet(&mut self, x: f64, y: f64) {
        let p = Rc::new(RefCell::new(Planet::new_semi_rng(x, y)));
        self.qtree.insert(p.clone()).unwrap();
        self.planets.push(p);
    }

    pub fn tick_n_draw<'a>(&self, ctx: &'a Canvas, time: f64) -> &'a Canvas {
        ctx.clear_rect(0.0, 0.0, self.dimensions.0, self.dimensions.1);
        ctx.set_stroke_style(&"magenta".into());
        ctx.set_fill_style(&"black".into());
        ctx.set_line_width(4.0);

        for planet in &self.planets {
            self.qtree.update_body(planet.clone(), time).unwrap();

            let planet = planet.borrow();
            ctx.begin_path();

            ctx.arc(planet.pos().x, planet.pos().y, planet.radius(), 0.0, PI * 2.0)
                .unwrap();

            ctx.stroke();
            ctx.fill();
            ctx.set_stroke_style(&"white".into());
            ctx.set_fill_style(&"gray".into());
        }

        ctx
    }
}

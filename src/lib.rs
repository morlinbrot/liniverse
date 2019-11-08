//! Simulation of a 2-dimensional galaxy of planets and the forces acting on them.
//!
//! We use [`wasm_bindgen`](https://github.com/rustwasm/wasm-bindgen) to access JavaScript events
//! and use HTML elements like canvas.
//!
//! On each [`tick`](./universe/struct.Universe.html#method.tick) of the
//! [`Universe`](./universe/struct.Universe.html), we compute the forces at play, update a [`Planet`](./planet/struct.Planet.html)'s
//! position and [`draw`](./universe/struct.Universe.html#method.draw) everything out onto the canvas
//! inside our [`RenderLoop`](./renderloop/struct.RenderLoop.html).
//! 
//! To be able to efficiently render a large amount of planets, we reduce computations by constructing a [`quad`](./quad/index.html)
//! tree which will aggregate the gravitational forces of far away planets.
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod planet;
pub use planet::Planet;

pub mod geo;
pub use geo::{Cardinal, Point, Rect};
pub mod quad;
pub use quad::{Body, Newtonian, QuadConfig, QuadNode};

mod renderloop;
pub use renderloop::RenderLoop;

mod universe;
pub use universe::Universe;

const NO_OF_PLANETS: usize = 100;

fn get_dimensions(canvas: &web_sys::HtmlCanvasElement) -> (f64, f64) {
    let bounding_rect = (canvas.as_ref() as &web_sys::Element).get_bounding_client_rect();
    (bounding_rect.width(), bounding_rect.height())
}

#[allow(dead_code)]
#[wasm_bindgen]
pub struct ModuleHandler {
    render_loop: Rc<RefCell<RenderLoop>>,
    closures: Vec<Box<dyn Drop>>,
}

#[wasm_bindgen]
pub fn main(
    canvas: web_sys::HtmlCanvasElement,
    restart_btn: web_sys::HtmlElement,
    play_pause_btn: web_sys::HtmlElement,
) -> Result<ModuleHandler, JsValue> {
    let window = web_sys::window().expect("No window object.");
    let dimensions = get_dimensions(&canvas);
    let universe = Rc::new(RefCell::new(Universe::new(dimensions)));
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut closures: Vec<Box<dyn Drop>> = Vec::new();

    let render_loop = Rc::new(RefCell::new(RenderLoop::new(
        universe.clone(),
        window.clone(),
        play_pause_btn.clone(),
        context,
    )));

    render_loop.borrow_mut().closure = Some({
        let render_loop = render_loop.clone();
        Closure::wrap(Box::new(move || {
            render_loop.borrow_mut().render_loop();
        }))
    });

    {
        let closure: Closure<dyn Fn() -> _> = {
            let render_loop = render_loop.clone();
            Closure::wrap(Box::new(move || -> Result<(), JsValue> {
                render_loop.borrow_mut().play_pause()?;
                Ok(())
            }))
        };
        (play_pause_btn.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closures.push(Box::new(closure));
    }

    {
        let closure: Closure<dyn Fn() -> _> = {
            let render_loop = render_loop.clone();
            Closure::wrap(Box::new(move || -> Result<(), JsValue> {
                if render_loop.borrow().is_running() {
                    render_loop.borrow_mut().pause()?;
                }
                let mut universe = Universe::new(dimensions);
                universe.init_random();
                render_loop.borrow_mut().replace_universe(universe);
                render_loop.borrow_mut().play()?;
                Ok(())
            }))
        };
        (restart_btn.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closures.push(Box::new(closure));
    }

    {
        let closure: Closure<dyn Fn(_)> = {
            let universe = universe.clone();
            let canvas = canvas.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let bounding_rect =
                    (canvas.as_ref() as &web_sys::Element).get_bounding_client_rect();
                let x = event.client_x() as f64 - bounding_rect.left();
                let y = event.client_y() as f64 - bounding_rect.top();

                universe.borrow_mut().add_planet(x, y);
            }))
        };
        (canvas.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closures.push(Box::new(closure));
    }

    universe.borrow_mut().init_random();
    render_loop.borrow_mut().play()?;

    Ok(ModuleHandler {
        render_loop,
        closures,
    })
}

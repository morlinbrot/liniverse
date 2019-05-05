use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod point;
use point::Point;

pub mod planet;
use planet::Planet;

mod renderloop;
use renderloop::RenderLoop;

mod universe;
use universe::Universe;

const DIMENSIONS: (f64, f64) = (1080.0, 700.0);
const NO_OF_PLANETS: usize = 100;
const NO_OF_ITERATIONS: usize = 5_000;

#[allow(dead_code)]
#[wasm_bindgen]
pub struct ModuleHandler {
    render_loop: Rc<RefCell<RenderLoop>>,
    closures: Vec<Box<Drop>>,
}

#[wasm_bindgen]
pub fn main(
    canvas: web_sys::HtmlCanvasElement,
    start_btn: web_sys::HtmlElement,
    stop_btn: web_sys::HtmlElement,
) -> Result<ModuleHandler, JsValue> {
    let window = web_sys::window().expect("No window object.");
    let universe = Rc::new(RefCell::new(Universe::new()));
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut closures: Vec<Box<Drop>> = Vec::new();

    let render_loop = Rc::new(RefCell::new(RenderLoop::new(
        universe.clone(),
        window.clone(),
        start_btn.clone(),
        stop_btn.clone(),
        context,
    )));

    render_loop.borrow_mut().closure = Some({
        let render_loop = render_loop.clone();
        Closure::wrap(Box::new(move || {
            render_loop.borrow_mut().render_loop();
        }))
    });
    
    {
        let closure: Closure<Fn() -> _> = {
            let render_loop = render_loop.clone();
            Closure::wrap(Box::new(move || -> Result<(), JsValue> {
                render_loop.borrow_mut().play()?;
                Ok(())
            }))
        };
        (start_btn.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closures.push(Box::new(closure));
    }

    {
        let closure: Closure<Fn() -> _> = {
            let render_loop = render_loop.clone();
            Closure::wrap(Box::new(move || -> Result<(), JsValue> {
                render_loop.borrow_mut().pause()?;
                Ok(())
            }))
        };
        (stop_btn.as_ref() as &web_sys::EventTarget)
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

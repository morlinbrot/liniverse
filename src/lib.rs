use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod point;
use point::Point;

pub mod planet;
use planet::Planet;

mod universe;
use universe::Universe;

const NO_OF_PLANETS: usize = 20;
const NO_OF_ITERATIONS: usize = 100;

fn window() -> web_sys::Window {
    web_sys::window().expect("Can't instantiate window object")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Can't register requestAnimationFrame");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Can't instantiate document object")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("Can't instantiate body")
}

fn canvas() -> web_sys::HtmlCanvasElement {
    let c = document()
        .get_element_by_id("canvas")
        .expect("Can't instantiate canvas element");

    let canvas: web_sys::HtmlCanvasElement = c
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let dimensions = (800.0, 600.0);

    let body = body();

    let context = canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let status_div = document().create_element("div")?;
    status_div.set_id("status");
    body.append_child(&status_div)?;

    let mut universe = Universe::new(dimensions.0, dimensions.1);
    universe.init_random(NO_OF_PLANETS);

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i >= NO_OF_ITERATIONS {
            let _ = f.borrow_mut().take();
            return;
        }

        context.clear_rect(0.0, 0.0, dimensions.0, dimensions.1);
        status_div.set_inner_html(&format!("Frame: {}", i));

        universe.draw(&context);

        universe.tick();

        i += 1;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

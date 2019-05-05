use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::Universe;

pub struct RenderLoop {
    animation_id: Option<i32>,
    universe: Rc<RefCell<Universe>>,
    window: web_sys::Window,
    start_btn: web_sys::HtmlElement,
    stop_btn: web_sys::HtmlElement,
    pub closure: Option<Closure<Fn()>>,

    context: web_sys::CanvasRenderingContext2d,
}

impl RenderLoop {
    pub fn new(
        universe: Rc<RefCell<Universe>>,
        window: web_sys::Window,
        start_btn: web_sys::HtmlElement,
        stop_btn: web_sys::HtmlElement,

        context: web_sys::CanvasRenderingContext2d,
    ) -> Self {
        RenderLoop {
            universe,
            window,
            start_btn,
            stop_btn,
            animation_id: None,
            closure: None,

            context,
        }
    }
}

impl RenderLoop {
    pub fn render_loop(&mut self) {
        self.universe.borrow().tick();

        // TODO: Move this to a `Renderer` struct.
        self.universe.borrow().draw(&self.context);

        self.animation_id = if let Some(ref closure) = self.closure {
            Some(
                self.window
                    .request_animation_frame(closure.as_ref().unchecked_ref())
                    .expect("cannot set animation frame"),
            )
        } else {
            None
        }
    }

    pub fn play(&mut self) -> Result<(), JsValue> {
        self.render_loop();
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), JsValue> {
        if let Some(id) = self.animation_id {
            self.window.cancel_animation_frame(id)?;
            self.animation_id = None;
        }
        Ok(())
    }
}

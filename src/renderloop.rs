use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::*;

pub struct RenderLoop {
    pub closure: Option<Closure<dyn Fn()>>,
    animation_id: Option<i32>,
    context: web_sys::CanvasRenderingContext2d,
    fps: Vec<f64>,
    play_pause_btn: web_sys::HtmlElement,
    prev_timestamp: f64,
    universe: Rc<RefCell<Universe>>,
    window: web_sys::Window,
}

impl RenderLoop {
    pub fn new(
        universe: Rc<RefCell<Universe>>,
        window: web_sys::Window,
        play_pause_btn: web_sys::HtmlElement,

        context: web_sys::CanvasRenderingContext2d,
    ) -> Self {
        Self {
            animation_id: None,
            closure: None,
            context,
            fps: Vec::new(),
            play_pause_btn,
            prev_timestamp: 0.0,
            universe,
            window,
        }
    }
}

impl RenderLoop {
    pub fn render_loop(&mut self) {
        let perf = self
            .window
            .performance()
            .expect("performance should be available");

        //let _timer = Timer::new("Universe::tick");

        let now = perf.now();
        let delta = now - self.prev_timestamp;
        self.prev_timestamp = now;

        //let fps = 1.0 / delta * 1000.0;
        //self.fps.push(fps);
        //if self.fps.len() > 100 {
        //    self.fps.remove(0);
        //}
        //let mean = self.fps.iter().fold(0.0, |acc, curr| acc + curr);

        self.universe
            .borrow()
            .tick_n_draw(&self.context, delta / 10.0);

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

    pub fn replace_universe(&mut self, universe: Universe) {
        let _ = self.universe.replace(universe);
    }

    pub fn is_running(&self) -> bool {
        self.animation_id.is_some()
    }

    pub fn play(&mut self) -> Result<(), JsValue> {
        (self.play_pause_btn.as_ref() as &web_sys::Node).set_text_content(Some("⏸"));
        self.render_loop();
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), JsValue> {
        (self.play_pause_btn.as_ref() as &web_sys::Node).set_text_content(Some("▶"));
        if let Some(id) = self.animation_id {
            self.window.cancel_animation_frame(id)?;
            self.animation_id = None;
        }
        Ok(())
    }

    pub fn play_pause(&mut self) -> Result<(), JsValue> {
        if self.is_running() {
            self.pause()
        } else {
            self.play()
        }
    }
}

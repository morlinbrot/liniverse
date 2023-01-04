use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::*;

struct FpsStats {
    min: f64,
    max: f64,
    mean: f64,
}

pub struct RenderLoop {
    universe: Rc<RefCell<Universe>>,
    window: web_sys::Window,
    document: web_sys::Document,
    context: web_sys::CanvasRenderingContext2d,

    animation_id: Option<i32>,
    pub closure: Option<Closure<dyn Fn()>>,
    frames: Vec<f64>,
    play_pause_btn: web_sys::HtmlElement,
    prev_timestamp: f64,
}

impl RenderLoop {
    pub fn new(
        universe: Rc<RefCell<Universe>>,
        window: web_sys::Window,
        document: web_sys::Document,
        context: web_sys::CanvasRenderingContext2d,
        play_pause_btn: web_sys::HtmlElement,
    ) -> Self {
        Self {
            universe,
            window,
            document,
            context,
            play_pause_btn,

            animation_id: None,
            closure: None,
            frames: Vec::new(),
            prev_timestamp: 0.0,
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

        let target_fps = 60.0;

        let now = perf.now();
        let delta = now - self.prev_timestamp;

        // We only render when we have to to keep up with target_fps.
        if delta > 1000.0 / target_fps {
            self.prev_timestamp = now;

            let stats = self.calc_fps_stats(delta);

            if let Some(fps_display) = self.document.get_element_by_id("fps") {
                fps_display.set_inner_html(&format!(
                    "FPS min: {}, max: {}, mean: {}",
                    stats.min, stats.max, stats.mean
                ));
            }

            self.universe
                .borrow_mut()
                .tick_n_draw_brute(&self.context, delta);
        }
        //let mean = self.fps.iter().fold(0.0, |acc, curr| acc + curr);

        self.universe
            .borrow_mut()
            .tick_n_draw_brute(&self.context, delta / 50.0);

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

    fn calc_fps_stats(&mut self, delta: f64) -> FpsStats {
        let fps = 1.0 / delta * 1000.0;

        self.frames.push(fps);
        if self.frames.len() > 100 {
            self.frames.remove(0);
        }

        let mut min = std::f64::INFINITY;
        let mut max = std::f64::NEG_INFINITY;
        let mut sum = 0.0;
        for f in &self.frames {
            min = f.min(min);
            max = f.max(max);
            sum += f;
        }
        let mean = sum / self.frames.len() as f64;

        FpsStats {
            min: min.floor(),
            max: max.floor(),
            mean: mean.floor(),
        }
    }
}

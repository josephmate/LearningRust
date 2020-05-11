mod utils;

use std::f64;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// velocity is pixels per second
// positions are pixels
// 0,0 is top left
// positive x is towards the right
// positive y is towards the bottom
struct Drawable {
    x_vel: f64,
    y_vel: f64,
    x_posn: f64,
    y_posn: f64,
}

#[wasm_bindgen]
pub struct Renderer {
    // canvas for drawing
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,

    // the time that we last drew
    last_draw: u32,

    circle: Drawable,

    square: Drawable,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new(canvas_id: &str, start_time: u32) -> Renderer {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        
        let width = canvas.width();
        let height = canvas.height();
        
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Renderer {
            canvas: canvas,
            context: context,
            last_draw: start_time,

            // All data related to drawing and moving the circle
            circle: Drawable {
                x_vel: 10.0,
                y_vel: 10.0,
                x_posn: 0.0,
                y_posn: 0.0,
            },

            // All the data related to drawing and moving the square
            square: Drawable {
                x_vel: -10.0,
                y_vel: -10.0,
                x_posn: width as f64,
                y_posn: height as f64,
            },
        }
    }
    
    pub fn render(&mut self, current_time: u32) {
        let elapsed_time = current_time - self.last_draw;
        
        self.circle.update_posn(elapsed_time);
        self.square.update_posn(elapsed_time);

        self.draw();

        self.last_draw = current_time;
    }

    fn draw(&mut self) {
        self.context.clear_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);

        self.context.begin_path();
        self.context.rect(self.square.x_posn, self.square.y_posn, 10.0, 10.0);
        self.context.stroke();

        // 2 Pi Radians is 360 degrees
        self.context.begin_path();
        self.context.arc(self.circle.x_posn, self.circle.y_posn, 10.0, 0.0, f64::consts::PI * 2.0).unwrap();
        self.context.stroke();
    }
}

impl Drawable {
    fn update_posn(&mut self, elapsed_time: u32) {
        self.x_posn = self.x_posn + self.x_vel * elapsed_time as f64 / 1000.0;
        self.y_posn = self.y_posn + self.y_vel * elapsed_time as f64 / 1000.0;
    }
}
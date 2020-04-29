extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod app;

use app::{App, GraphicsConfig};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};


fn main() {
    // create a new game and run it
    let mut app = App::new(GraphicsConfig::new(String::from("Survive"), 960.0, 768.0));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

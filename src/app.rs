use piston::input::{RenderArgs, UpdateArgs};
use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;

pub struct GraphicsConfig {
  title: String,
  width: f64,
  height: f64
}

impl GraphicsConfig {
  pub fn new(title: String, width: f64, height: f64) -> Self {
    GraphicsConfig {
      title,
      width,
      height
    }
  }
}
pub struct App {
  gc: GraphicsConfig,
  gl: GlGraphics, // OpenGl Draw backend
  rotation: f64,  // rotation for square
  window: Window
}

impl App {
  pub fn new(gc: GraphicsConfig) -> Self {
    let opengl = OpenGL::V3_2;
    App {
      gc,
      gl: GlGraphics::new(opengl),
      rotation: 0.0,
      window: WindowSettings::new(gc.title, [gc.width, gc.height])
      .graphics_api(opengl)
      .exit_on_esc(true)
      .build()
      .unwrap()
    }
  }
  
  pub fn render(&mut self, args: &RenderArgs) {
      use graphics::*;
      const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
      const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

      let square = rectangle::square(0.0, 0.0, 50.0);
      let rotation = self.rotation;
      let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

      self.gl.draw(args.viewport(), |c, gl| {
          // Clear the screen
          clear(GREEN, gl);
          let transform = c
              .transform
              .trans(x, y)
              .rot_rad(rotation)
              .trans(-25.0, -25.0);

          // Draw a box rotating around the middle of the screen.
          rectangle(RED, square, transform, gl);
      });
  }

  pub fn update(&mut self, args: &UpdateArgs) {
      // Rotate 2 radians per second.
      self.rotation += 2.0 * args.dt;
  }
}
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate nalgebra;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use nalgebra::*;

#[derive(Copy, Clone)]
enum Direction {
    Up = -1,
    Down = 1,
}

//impl Direction {
    //fn to_i8(&self) -> i8 {
        //*self as i8
    //}
//}

struct Paddle {
    up: Key,
    down: Key,
    position: Vector2<f64>, // top left corner
    direction: Option<Direction>,
}

const PADDLE_SPEED: f64 = 150.0;
const PADDLE_WIDTH: f64 = 10.0;
const PADDLE_HEIGHT: f64 = 50.0;

impl Paddle {
    fn y_velocity(&self) -> f64 {
        let rv = match self.direction {
            Some(dir) => dir as i8,
            None => (0 as i8),
        };
        (rv as f64) * PADDLE_SPEED
    }

    fn on_key_press(&mut self, key: Key) {
        if key == self.up {
            self.direction = Some(Direction::Up)
        } else if key == self.down {
            self.direction = Some(Direction::Down)
        }
    }

    fn on_key_release(&mut self, key: Key) {
        match self.direction {
            Some(Direction::Up) if key == self.up => self.direction = None,
            Some(Direction::Down) if key == self.down => self.direction = None,
            _ => (),
        }
    }

    fn update_position(&mut self, args: &UpdateArgs) {
        self.position.y += self.y_velocity() * args.dt;
        self.position.y = self.position.y.min(WORLD_HEIGHT - PADDLE_HEIGHT).max(0.0);
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    paddles: [Paddle; 2],
}

const WORLD_WIDTH: f64 = 200.0;
const WORLD_HEIGHT: f64 = 200.0;

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let scale = Vector2::new((args.width as f64) / WORLD_WIDTH, (args.height as f64) / WORLD_HEIGHT);
        let paddles = self.paddles.iter().map( |p|
            Vector2::new(p.position.x * scale.x, p.position.y * scale.y)
        );
        let rect: types::Rectangle = [0.0, 0.0, PADDLE_WIDTH * scale.x, PADDLE_HEIGHT * scale.y];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            for p in paddles {
                let transform = c.transform.trans(p.x, p.y);
                rectangle(WHITE, rect, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for p in self.paddles.iter_mut() {
            p.update_position(args)
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("pong", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let starting_y: f64 = (WORLD_HEIGHT / 2 as f64) - (PADDLE_HEIGHT / 2 as f64);

    let mut app = App {
        gl: GlGraphics::new(opengl),
        paddles: [
            Paddle {
                up: Key::W,
                down: Key::S,
                position: Vector2::new(0.0, starting_y),
                direction: None,
            },
            Paddle {
                up: Key::I,
                down: Key::K,
                position: Vector2::new(WORLD_WIDTH - PADDLE_WIDTH, starting_y),
                direction: None,
            },
        ]
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            for p in app.paddles.iter_mut() {
                p.on_key_press(key);
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            for p in app.paddles.iter_mut() {
                p.on_key_release(key);
            }
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}

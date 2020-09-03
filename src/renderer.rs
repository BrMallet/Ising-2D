use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct Renderer {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64  // Rotation for the square.
}

impl Renderer {
    fn render(&mut self, args: &RenderArgs, lines: &Vec<(f64,f64,f64,f64)>) {
        use graphics::*;

        //Cores?

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        //Cria sistema de coordenada x e y , e a variavels rotação

        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        //Desenha a linha

        let mut lines_to_draw = Vec::new();

        for (x1,y1,x2,y2) in lines {
            let from = [x1 * args.window_size[0] / 2.0, y1 * (- args.window_size[1] / 2.0)];
            let to = [x2 * args.window_size[0] / 2.0, y2 * ( args.window_size[1] / 2.0)];
            lines_to_draw.push((from, to, WHITE));
        }     

        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-0.0, -0.0);

            for (f, t, color) in lines_to_draw {
                line_from_to(color, 0.5, f, t, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation +=  2.0 * args.dt;
    }
}

pub fn setup_renderer() -> (Window, Renderer) {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let window: Window = WindowSettings::new("spinning-square", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let app = Renderer {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    (window, app)
}

//Loop de animação

pub fn render_in_a_loop(renderer: &mut Renderer, window: &mut Window) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            renderer.render(&args, &vec![(0.0,0.0,1.0,1.0)]);
        }

        if let Some(args) = e.update_args() {
            renderer.update(&args);
        }
    }
}

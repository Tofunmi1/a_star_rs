extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

// use a_star::colors::{BLACK, WHITE};
use a_star::{
    colors::*,
    config::{HEIGHT, SPACING, WIDTH},
};
use glutin_window::{GlutinWindow as Window, OpenGL};
use graphics::{rectangle, Context, Rectangle};
use opengl_graphics::GlGraphics;
use piston::{window::WindowSettings, EventSettings, Events, RenderEvent};

pub struct Cell<'a> {
    _parent_i: i32,
    _parent_j: i32,
    _f: f32,
    _g: f32,
    _h: f32,
    _rect: &'a Rectangle,
    xy: Vec<i32>,
}

impl<'a> Cell<'a> {
    fn _init_cell<'b>(&self, rect: &'b Rectangle) -> Cell<'b> {
        Cell {
            _parent_i: 0,
            _parent_j: 0,
            _f: 0.0,
            _g: 0.0,
            _h: 0.1,
            _rect: rect,
            xy: vec![0, 0],
        }
    }
}

pub struct Grid<'a> {
    cells: Vec<Cell<'a>>,
}

impl<'a> Grid<'a> {
    pub fn render(&self, c: Context, g: &mut GlGraphics) {
        let cells: Vec<Cell>;
        for y in 0..WIDTH {
            for x in 0..HEIGHT {
                let sq = rectangle::square(x as f64 * SPACING, y as f64 * SPACING, 12.0);
                let mut cell = Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 0.5);

                cell = cell.color([1.0, 1.0, 1.0, 1.0]);
                cell.draw(sq, &Default::default(), c.transform, g);
            }
        }
    }
}

//handle grid rendering etc
pub struct App {}

impl App {
    fn _render(&self, window: &mut Window, gl: &mut GlGraphics, grid: &Grid) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(window) {
            use graphics::*;

            if let Some(args) = e.render_args() {
                gl.draw(args.viewport(), |c, g| {
                    clear([1.0, 1.0, 1.0, 1.0], g);
                    grid.render(c, g);
                });
            }
        }
    }
}

//closed list -> explored nodes
//open list
//heuristic function to estiamte the best path

//test with custom starting points first
//test with custom obstacles too ?
fn main() {
    let opengl = OpenGL::V3_2;
    let height = 500;
    let width = 500;
    let spacing: f64 = 10.0;
    let mut window: Window = WindowSettings::new("a * path finding", [height, width])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    // events
    // cell classification
    // cell array ?
}

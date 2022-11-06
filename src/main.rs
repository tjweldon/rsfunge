mod funge;

use std::{env, fs};

#[derive(Debug)]
pub struct Cli {
    pub target: String,
    pub stop_after: usize,
    pub visual: bool,
}

impl Cli {

    // from_env is a named constructor for the Cli struct that populates the fields
    // from the command line args.
    pub fn from_env() -> Cli {
        let mut args: Vec<String> = env::args().collect();

        // reverse the args so we can consume them like a stack
        args.reverse();

        // pop off the args[0] which is the name of this executable
        args.pop();

        // target is the befunge code file
        let target: String = match args.pop() {
            Some(t) => t,
            None => "./test.b98".to_string(),
        };

        // stop after is the max number of vm cycles to execute
        let stop_after: usize = match args.pop() {
            Some(t) => match t.parse::<usize>() {
                Ok(i) => i,
                Err(_) => funge::Vm::FOREVER,
            },
            None => funge::Vm::FOREVER,
        };

        Cli { target, stop_after, visual: true }
    }
}

#[allow(dead_code)]
fn main() {
    let cli = Cli::from_env();
    //println!("{:?}", cli);
    //if cli.stop_after == funge::Vm::FOREVER {
    //    return;
    //}

    let code = load_code(cli.target.as_str());

    let mut fvm = funge::Vm::new(code);

    let ran_for = match fvm.run_for(cli.stop_after) {
        Ok(x) => x,
        Err(_) => 0,
    };
    println!("\nRan for {}", ran_for);

    visual::run();
}

fn load_code(path: &str) -> String {
    fs::read_to_string(path).expect("Error loading code")
}

mod visual {
    use nannou::prelude::*;
    use nannou::geom::Vec2;
    use core::ops::Add;
    use crate::load_code;
    use std::{thread, time};

    use super::funge;
    const C_WIDTH: f32 = 100.0;
    const C_HEIGHT: f32 = 100.0;

    struct Rect {
        pub top: Vec2,
        pub btm: Vec2,
        pub left: Vec2,
        pub right: Vec2,
        pub w: u32,
        pub h: u32,
    }

    impl Rect {
        fn new(vm: &funge::Vm) -> Self {
            let (w, h) = canvas_size(vm);
            let (top, btm, right, left) = (
                pt2(0.0, h as f32 / 2.0),
                pt2(0.0, -(h as f32) / 2.0),
                pt2(w as f32 / 2.0, 0.0),
                pt2(-(w as f32) / 2.0, 0.0),
            );

            Rect { top, btm, left, right, w, h }
        }
    }


    #[allow(dead_code)]
    impl funge::Stack<usize> {
        pub fn draw_at(&self, draw: &Draw) {}
    }


    struct Model {
        _window: window::Id,
        vm: funge::Vm,
    }

    fn canvas_size(vm: &funge::Vm) -> (u32, u32) {
        let (cols, rows) = vm.space.dims();
        (
            (cols as f32 * C_WIDTH) as u32, 
            (rows as f32 * C_HEIGHT) as u32,
        )
    }

    fn view(app: &App, _model: &Model, frame: Frame) {
        let draw = app.draw();
        draw.background().color(WHITE);

        // setting up a bunch of convenient shorthands
        let (cols, rows) = _model.vm.space.dims();

        let c_rect = Rect::new(&_model.vm);

        let grid_weight = 2.0;
        


        // iterate over the row index and draw a horizontal line at the bottom of each cell
        for row_idx in 0..rows {
            let y: f32 = row_idx as f32 * C_HEIGHT + c_rect.btm.y;
            let vertical = pt2(0.0, y);
            
            let (start, end) = (
                c_rect.left.add(vertical), 
                c_rect.right.add(vertical),
            );
            
            draw.line().start(start).end(end).weight(grid_weight);
        }

        // iterate over the column index and draw a vertical line on the left hand side of the cell
        for col_idx in 0..cols {
            let x: f32 = col_idx as f32 * C_WIDTH + c_rect.left.x;
            let horizontal = pt2(x, 0.0);
            
            let (start, end) = (
                c_rect.top.add(horizontal),
                c_rect.btm.add(horizontal),
            );

            draw.line().start(start).end(end).weight(grid_weight);
        }

        // draw the current instruction pointer location
        let ip_location = _model.vm.get_location();
        
        // for converting from funge::Location in funge::Space to nannou::geom::Vec2 in canvas
        // space
        let to_canvas_coords = |loc: funge::Location| -> Vec2 {
            pt2(
                loc.0 as f32 * C_WIDTH,
                (rows - 1 - loc.1 as usize) as f32 * C_HEIGHT,
                )
                .add(c_rect.btm)
                .add(c_rect.left)
                .add(pt2(C_WIDTH/2.0, C_HEIGHT/2.0))
        };

        let ip_vec = to_canvas_coords(ip_location);
        draw
            .rect()
            .w_h(C_WIDTH, C_HEIGHT)
            .color(GREEN)
            .xy(ip_vec);
        

        // this loop writes the code into each cell
        // char_offset is the offset of each character from the bottom left of the cell 
        let char_offset = pt2(C_WIDTH/2.0, C_HEIGHT/2.0);
        // loop over rows
        for y_idx in 0..rows {
            
            // the funge space coords start at the top left, but nannou likes to
            // use a more formal maths-esque style with the (0,0) point being the center
            // this means we need a reflection/translation to get from one to the other,
            // hence the (rows - 1 - y_idx)
            let y = (rows - 1 - y_idx) as f32 * C_HEIGHT + c_rect.btm.y + char_offset.y;
            
            // loop over cells in each row
            for x_idx in 0..cols {
                // x starts at the left so we just need to scale it and offset correctly
                let x = x_idx as f32 * C_WIDTH + c_rect.left.x + char_offset.x;
                
                // derive the cell to be drawn in from the indices
                let location = funge::Location(x_idx as i64, y_idx as i64);
                
                // get the string representation of the code in that cell
                let character = format!("{}", _model.vm.space.get(&location) as u8 as char);
                
                // draw the character
                draw.text(&character.to_owned())
                    .x(x).y(y)
                    .font_size((0.75*C_HEIGHT) as u32)
                    .color(BLACK);
            }
        }

        draw.to_frame(app, &frame).unwrap();
    }

    pub fn run() {
        let model2 = |app: &App| -> Model {
            let vm = funge::Vm::new(load_code("./test.b98"));
            let c_rect = Rect::new(&vm); 

            let _window = app.new_window()
                .size(c_rect.w, c_rect.h)
                .view(view)
                .build()
                .unwrap();
            Model { _window, vm }
        };

        let update = |_app: &App, _model: &mut Model, _update: Update| {
            _ = _model.vm.run_for(1);
            thread::sleep(time::Duration::from_millis(100));
        };
        

        nannou::app(model2)
            .update(update)
            .run();
    }
}

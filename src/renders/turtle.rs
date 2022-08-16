use std::collections::LinkedList;
use rand::{
    Rng,
    rngs::ThreadRng
};
use turtle::{
    Turtle,
    Color,
    rand::random
};

use crate::models::render_model::Render;
use crate::state::{
    Angle,
    State,
    Side,
    Pos,
    ScreenPosition
};

pub struct Size {
    w: f64,
    h: f64
}

pub struct TurtleRender {
    cursor: Turtle,
    stack: LinkedList<State>,
    rng: ThreadRng,
    pub size: Size
}

impl TurtleRender {
    pub fn new(w: u32, h: u32) -> Self {
        let mut turtle = Turtle::new();
        
        turtle.set_speed("instant");

        Self {
            cursor: turtle,
            stack: LinkedList::new(),
            rng: rand::thread_rng(),
            size: Size {
                w: w as f64,
                h: h as f64
            }
        }
    }

    fn turn(&mut self, angle: Angle) {
        match angle.side {
            Side::Left => self.turn_left(angle.value),
            Side::Right => self.turn_right(angle.value),
        }
    }
}

impl Render for TurtleRender {
    fn step_forward(&mut self, distance: f64) {
        self.cursor.forward(distance);
    }

    fn step_backward(&mut self, distance: f64) {
        self.cursor.backward(distance);
    }

    fn turn_left(&mut self, angle: f64) {
        self.cursor.left(angle);
    }

    fn turn_right(&mut self, angle: f64) {
        self.cursor.right(angle);
    }

    fn turn_random(&mut self) {
        // Random angle
        let angle = self.rng.gen::<f64>() * 360.;

        // Random side value
        let side = self.rng.gen::<u8>() % 2;

        // Turn and get turn side to save the angle
        match side {
            0 => self.turn_left(angle),
            _ => self.turn_right(angle)
        };
    }

    fn pen_up(&mut self) {
        self.cursor.pen_up();
    }

    fn pen_down(&mut self) {
        self.cursor.pen_down()
    }

    fn color_random(&mut self) {
        let color = random::<Color>().opaque();
    
        self.cursor.set_pen_color(color);
    }

    fn save_state(&mut self) {
        // Turtle position
        let pos = self.cursor.position();

        let state = State {
            position: Pos {
                x: pos.x,
                y: pos.y
            },
            angle: self.cursor.heading()
        };

        self.stack.push_back(state);
    }

    fn restore_state(&mut self) {
        self.pen_up();

        // Last state
        let state = self.stack.pop_back();

        self.cursor.set_heading(0.);

        match state {
            Some(value) => {
                let pos = value.position;
                let angle = value.angle;
                
                self.cursor.go_to([pos.x, pos.y]);
                self.cursor.set_heading(angle);
            },
            None => return
        };

        self.pen_down();
    }

    fn set_pen_size(&mut self, size: f64) {
        self.cursor.set_pen_size(size);
    }

    fn save_svg(&mut self, filename: &str) {
        let (w, h) = (self.size.w as u32, self.size.h as u32);

        self.cursor.drawing_mut().set_size((w, h));
        self.cursor.drawing().save_svg(filename);
    }

    fn set_pos(&mut self, pos: ScreenPosition) {
        let pen_size = self.cursor.pen_size();
        let dm = self.cursor.drawing_mut();
        let w_mid = self.size.w / 2. - pen_size; 
        let h_mid = self.size.h / 2.; 

        match pos {
            ScreenPosition::Coord(x, y) => dm.set_center((x, y)),
            ScreenPosition::Center => dm.reset_center(),
            ScreenPosition::TopLeft => dm.set_center((-w_mid, h_mid)),
            ScreenPosition::TopRight => dm.set_center((w_mid, h_mid)),
            ScreenPosition::BottomLeft => dm.set_center((w_mid, -h_mid)),
            ScreenPosition::BottomRight => dm.set_center((w_mid, -h_mid)),
        }
    }

    fn save_state_and_turn(&mut self, angle: Angle) {
        // Save
        self.save_state();

        // Turn
        self.turn(angle);
    }

    fn restore_state_and_turn(&mut self, angle: Angle) {
        // Restore
        self.restore_state();

        // Turn
        self.turn(angle);
    }
}

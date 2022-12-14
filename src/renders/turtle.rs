use std::collections::LinkedList;
use rand::{
    Rng,
    rngs::ThreadRng
};

use turtle::Point;
use turtle::{
    Turtle,
    Color,
    rand::random
};

use crate::models::render_model::Render;
use crate::square::Square;
use crate::state::{
    Angle,
    State,
    Side,
    Pos,
    ScreenPosition,
    SizeType
};

impl Into<Pos> for Point {
    fn into(self) -> Pos {
        Pos {
            x: self.x,
            y: self.y,
        }
    }
}

pub struct TurtleRender {
    cursor: Turtle,
    stack: LinkedList<State>,
    rng: ThreadRng,
    /// SVG size type
    pub size: SizeType,
    figure: Square,
    position: ScreenPosition
}

impl TurtleRender {
    pub fn new(size_type: SizeType) -> Self {
        // Turtle setup
        let mut turtle = Turtle::new();

        // Figure area default point
        let point = turtle.position();
        let pos = Pos {
            x: point.x,
            y: point.y
        };
        
        // Setup turtle graphic details
        turtle.set_speed("instant");
        turtle.set_heading(0.);

        Self {
            cursor: turtle,
            stack: LinkedList::new(),
            rng: rand::thread_rng(),
            size: size_type,
            figure: Square {
                top_left: pos,
                bottom_right: pos
            },
            position: ScreenPosition::default()
        }
    }

    fn turn(&mut self, angle: Angle) {
        match angle.side {
            Side::Left => self.turn_left(angle.value),
            Side::Right => self.turn_right(angle.value),
        }
    }

    /// Return the SVG size
    fn get_size(&mut self) -> (f64, f64) {
        match self.size {
            SizeType::Custom(w, h) => (w, h),
            
            // Figure size
            SizeType::Auto => {
                let figure_size = self.figure.size();
        
                (figure_size.w, figure_size.h)
            }
        }
    }

    /// Return the final figure position (top left angle)
    fn get_position(&mut self) -> Point {
        let svg_size = self.get_size();
        let pen_offset = self.cursor.pen_size();

        let figure_size = self.figure.size();

        // Offset including the pen line size
        let x_offset = (svg_size.0 - figure_size.w - pen_offset) / 2.;
        let y_offset = (svg_size.1 - figure_size.h - pen_offset) / 2.;

        match self.position {
            ScreenPosition::Coord(x, y) => (x, y),
            ScreenPosition::Center => (-self.figure.top_left.x, -self.figure.top_left.y),
            ScreenPosition::TopLeft =>(-self.figure.top_left.x - x_offset, -self.figure.top_left.y + y_offset),
            ScreenPosition::TopRight => (-self.figure.top_left.x + x_offset, -self.figure.top_left.y + y_offset),
            ScreenPosition::BottomLeft => (-self.figure.top_left.x - x_offset, -self.figure.top_left.y - y_offset),
            ScreenPosition::BottomRight => (-self.figure.top_left.x + x_offset, -self.figure.top_left.y - y_offset),
        }.into()
    }
}

impl Render for TurtleRender {
    fn step_forward(&mut self, distance: f64) {
        self.cursor.forward(distance);

        // Updating area max size
        self.figure.update_max_area(self.cursor.position().into());
    }

    fn step_backward(&mut self, distance: f64) {
        self.cursor.backward(distance);

        // Updating area max size
        self.figure.update_max_area(self.cursor.position().into());
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
        // SVG size
        let (w, h) = self.get_size();
        // Figure pos / size
        let mut fig_pos = self.get_position();
        let fig_size = self.figure.size();
        
        // Square center
        fig_pos.x -= fig_size.w / 2.;
        fig_pos.y += fig_size.h / 2.;

        self.cursor.drawing_mut().set_size((w as u32, h as u32));
        self.cursor.drawing_mut().set_center(fig_pos);
        self.cursor.drawing().save_svg(filename);
    }

    fn set_figure_pos(&mut self, pos: ScreenPosition) {
        self.position = pos;
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

    fn set_pen_color(&mut self, r: f64, g: f64, b: f64) {
        let color = Color::rgb(r, g, b);
        
        self.cursor.set_pen_color(color);
    }

    fn set_bg(&mut self, r: f64, g: f64, b: f64) {
        let color = Color::rgb(r, g, b);

        self.cursor.drawing_mut().set_background_color(color);
    }

    fn reset(&mut self) {
        self.cursor.reset();

        self.cursor.set_speed("instant");
        self.cursor.set_heading(0.);
    }
}

use crate::{
    models::render_model::Render,
    state::{ScreenPosition, Angle, Pos, Side}
};
use std::collections::LinkedList;
use rand::{
    rngs::ThreadRng, Rng
};

use crate::square::Square;
use crate::state::{
    State,
    SizeType
};

use turtle_svg::{
    turtle::TurtleSvg,
    draw::pen::PenPos, color::{Color}
};

impl Into<Pos> for PenPos {
    fn into(self) -> Pos {
        Pos {
            x: self.0,
            y: self.1
        }
    }
}

pub struct TurtleHeadless {
    cursor: TurtleSvg,
    stack: LinkedList<State>,
    rng: ThreadRng,
    /// SVG size type
    pub size: SizeType,
    figure: Square,
    position: ScreenPosition
}

impl TurtleHeadless {
    pub fn new(size_type: SizeType) -> Self {
        let point = Pos {
            x: 0.,
            y: 0.
        };

        Self {
            cursor: TurtleSvg::new(),
            stack: LinkedList::new(),
            rng: rand::thread_rng(),
            size: size_type,
            figure: Square {
                top_left: point,
                bottom_right: point
            },
            position: ScreenPosition::default()
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

    fn turn(&mut self, angle: Angle) {
        match angle.side {
            Side::Left => self.turn_left(angle.value),
            Side::Right => self.turn_right(angle.value),
        }
    }

    /// Return the final figure position (top left angle)
    fn get_position(&mut self) -> PenPos {
        let svg_size = self.get_size();
        let figure_size = self.figure.size();

        // Offset including the pen line size
        let x_offset = (svg_size.0 - figure_size.w) / 2.;
        let y_offset = (svg_size.1 - figure_size.h) / 2.;

        match self.position {
            ScreenPosition::Coord(x, y) => (x, y),
            ScreenPosition::Center => (self.figure.top_left.x, self.figure.top_left.y),
            ScreenPosition::TopLeft =>(self.figure.top_left.x - x_offset, self.figure.top_left.y + y_offset),
            ScreenPosition::TopRight => (self.figure.top_left.x + x_offset, self.figure.top_left.y + y_offset),
            ScreenPosition::BottomLeft => (self.figure.top_left.x - x_offset, self.figure.top_left.y - y_offset),
            ScreenPosition::BottomRight => (self.figure.top_left.x + x_offset, self.figure.top_left.y - y_offset),
        }.into()
    }
}

impl Render for TurtleHeadless {
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
        self.cursor.pen_down();
    }

    fn color_random(&mut self) {
        let color: Color = (
            self.rng.gen::<u8>() % 255,
            self.rng.gen::<u8>() % 255,
            self.rng.gen::<u8>() % 255,
            255
        ).into();
    
        self.cursor.set_pen_color(color)
    }

    fn save_state(&mut self) {
        // Pen position
        let pos = self.cursor.position();

        let state = State {
            position: pos.into(),
            angle: self.cursor.heading().degrees(),
        };

        self.stack.push_back(state);
    }

    fn restore_state(&mut self) {
        let state = self.stack.pop_back();

        self.cursor.set_heading(0.);

        match state {
            Some(value) => {
                let pos = value.position;
                let angle = value.angle;
                
                self.cursor.go_to((pos.x, pos.y));
                self.cursor.set_heading(angle);
            },
            None => return
        };
    }

    fn set_pen_size(&mut self, size: f64) {
        self.cursor.set_pen_size(size);
    }

    fn save_svg(&mut self, filename: &str) {
        // SVG size
        let svg_size = self.get_size();
        let mut fig_pos = self.get_position();
        let fig_size = self.figure.size();

        // Square center
        fig_pos.0 += fig_size.w / 2.;
        fig_pos.1 -= fig_size.h / 2.;

        self.cursor.drawing_mut().set_size(svg_size);
        // Centering figure on drawing
        self.cursor.drawing_mut().set_center(fig_pos);
        self.cursor.drawing_mut().save_svg(filename);
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
        let color: Color = (
            r as u8,
            g as u8,
            b as u8,
            255
        ).into();

        self.cursor.set_pen_color(color)
    }

    fn set_bg(&mut self, r: f64, g: f64, b: f64) {
        let color: Color = (
            r as u8,
            g as u8,
            b as u8,
            255
        ).into();

        self.cursor.set_background_color(color)
    }

    fn reset(&mut self) {
        self.cursor.reset();
    }
}

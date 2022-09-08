use crate::{models::render_model::Render, state::SizeType};

use super::turtle::TurtleRender;
use super::turtle_headless::TurtleHeadless;

pub enum Renderer {
    TurtleNormal(SizeType),
    TurtleHeadless(SizeType),
}

impl Renderer {
    pub fn get_render_obj(self) -> Box<dyn Render> {
        match self {
            Renderer::TurtleNormal(s) => Box::new(TurtleRender::new(s)),
            Renderer::TurtleHeadless(s) => Box::new(TurtleHeadless::new(s))
        }
    }
}

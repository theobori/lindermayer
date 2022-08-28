use crate::{models::render_model::Render, state::SizeType};

use super::turtle::{TurtleRender};

pub enum Renderer {
    TurtleNormal(SizeType),
    TurtleHeadless(SizeType),
}

impl Renderer {
    pub fn get_render_obj(self) -> Box<dyn Render> {
        let render = match self {
            Renderer::TurtleNormal(s) => TurtleRender::new(s),
            Renderer::TurtleHeadless(_) => todo!(),
        };

        Box::new(render)
    }
}

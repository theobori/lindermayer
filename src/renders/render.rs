use crate::{models::render_model::Render, state::SizeType};

use super::turtle::{TurtleRender};

pub enum RenderType {
    TurtleNormal(SizeType),
    TurtleHeadless(SizeType),
}

impl RenderType {
    pub fn get_render(self) -> Box<dyn Render> {
        let render = match self {
            RenderType::TurtleNormal(s) => TurtleRender::new(s),
            RenderType::TurtleHeadless(_) => todo!(),
        };

        Box::new(render)
    }
}

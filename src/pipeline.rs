use bevy::{
    prelude::{Component, Handle},
    reflect::TypeUuid,
    render::camera::RenderTarget,
};

use crate::prelude::VFXPipe;

#[derive(TypeUuid)]
#[uuid = "df501d89-a02d-4a8c-88ba-2d3033583527"]
pub struct VFXPipeline {
    pipes: Vec<Box<dyn VFXPipe>>,
    target: RenderTarget,
}

impl VFXPipeline {
    pub fn new(pipes: Vec<Box<dyn VFXPipe>>, target: RenderTarget) -> Self {
        VFXPipeline { pipes, target }
    }

    pub fn render_target(&self) -> RenderTarget {
        self.target.clone() // TODO: when there is a active pipe before then pipe through it
    }
}

#[derive(Component)]
pub struct PipelineComponent(pub Handle<VFXPipeline>);

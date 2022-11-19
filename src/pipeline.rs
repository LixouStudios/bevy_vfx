use std::ops::{Deref, DerefMut};

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

    pub fn get_pipe<T: VFXPipe + 'static>(&self, index: usize) -> Option<&T> {
        if let Some(pipe) = self.pipes.get(index) {
            pipe.as_any().downcast_ref::<T>()
        } else {
            None
        }
    }

    pub fn get_pipe_mut<T: VFXPipe + 'static>(&mut self, index: usize) -> Option<&mut T> {
        if let Some(pipe) = self.pipes.get_mut(index) {
            pipe.as_any_mut().downcast_mut::<T>()
        } else {
            None
        }
    }

    pub fn render_target(&self) -> RenderTarget {
        if let Some(first_pipe) = self.pipes.get(0) {
            if first_pipe.is_enabled() {
                RenderTarget::Image(first_pipe.image())
            } else {
                self.target.clone() // TODO: iterate over every pipe until end or found
            }
        } else {
            self.target.clone()
        }
    }
}

#[derive(Component)]
pub struct PipelineComponent(pub Handle<VFXPipeline>);

impl Deref for PipelineComponent {
    type Target = Handle<VFXPipeline>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PipelineComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

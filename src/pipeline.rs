use bevy::{prelude::Component, render::camera::RenderTarget};

use crate::prelude::VFXPipe;

#[derive(Component)]
pub struct VFXPipeline {
    pub(crate) pipes: Vec<Box<dyn VFXPipe>>,
    pub(crate) target: RenderTarget,
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
        let mut index = 0;
        loop {
            if let Some(pipe) = self.pipes.get(index) {
                if pipe.is_enabled() {
                    return RenderTarget::Image(pipe.image());
                } else {
                    index += 1;
                    continue;
                }
            } else {
                return self.target.clone();
            }
        }
    }
}

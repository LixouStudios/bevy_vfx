use std::any::Any;

use bevy::prelude::{Handle, Image};

pub trait VFXPipe: Sync + Send {
    fn image(&self) -> Handle<Image>;

    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn toggle_enabled(&mut self) {
        self.set_enabled(!self.is_enabled());
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

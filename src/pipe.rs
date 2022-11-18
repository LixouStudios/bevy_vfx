use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

pub struct VFXPipe {
    pub data: Box<dyn VFXPipeData>,
}

impl Deref for VFXPipe {
    type Target = Box<dyn VFXPipeData>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for VFXPipe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub trait VFXPipeData: Sync + Send {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

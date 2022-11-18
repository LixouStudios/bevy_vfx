use bevy::prelude::AddAsset;
use bevy::prelude::Plugin;

pub use crate::pipe::*;
pub use crate::pipeline::*;

pub struct VFXPlugin;

impl Plugin for VFXPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<VFXPipeline>();
    }
}

use bevy::prelude::AssetEvent;
use bevy::prelude::Assets;
use bevy::prelude::CoreStage;
use bevy::prelude::EventReader;
use bevy::prelude::Handle;
use bevy::prelude::Image;
use bevy::prelude::Plugin;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::Extent3d;
use bevy::utils::HashSet;
use bevy::window::WindowCreated;
use bevy::window::WindowId;
use bevy::window::WindowResized;
use bevy::window::Windows;

pub use crate::pipe::*;
pub use crate::pipeline::*;

pub struct VFXPlugin;

impl Plugin for VFXPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_to_stage(CoreStage::PostUpdate, resize_vfx_pipes);
    }
}

fn resize_vfx_pipes(
    mut window_resized_events: EventReader<WindowResized>,
    mut window_created_events: EventReader<WindowCreated>,
    mut image_asset_events: EventReader<AssetEvent<Image>>,
    windows: Res<Windows>,
    mut images: ResMut<Assets<Image>>,
    mut queue: Query<&mut VFXPipeline>,
) {
    let mut changed_window_ids = Vec::new();

    // Collect all unique window IDs of changed windows by inspecting created windows
    for event in window_created_events.iter() {
        if changed_window_ids.contains(&event.id) {
            continue;
        }

        changed_window_ids.push(event.id);
    }

    // Collect all unique window IDs of changed windows by inspecting resized windows
    for event in window_resized_events.iter() {
        if changed_window_ids.contains(&event.id) {
            continue;
        }

        changed_window_ids.push(event.id);
    }

    let changed_image_handles: HashSet<&Handle<Image>> = image_asset_events
        .iter()
        .filter_map(|event| {
            if let AssetEvent::Modified { handle } = event {
                Some(handle)
            } else {
                None
            }
        })
        .collect();

    for vfx in &mut queue {
        if is_changed(&vfx.target, &changed_window_ids, &changed_image_handles) {
            let info = vfx
                .target
                .get_render_target_info(&windows, &images)
                .unwrap();

            let logical_size = (info.physical_size.as_dvec2() / info.scale_factor).as_vec2();

            for pipe in &vfx.pipes {
                images.get_mut(&pipe.image()).unwrap().resize(Extent3d {
                    width: info.physical_size.x,
                    height: info.physical_size.y,
                    ..Default::default()
                });
            }
        }
    }
}

// the function exists like that in bevy but is private so i had to do my own impl
fn is_changed(
    target: &RenderTarget,
    changed_window_ids: &[WindowId],
    changed_image_handles: &HashSet<&Handle<Image>>,
) -> bool {
    match target {
        RenderTarget::Window(window_id) => changed_window_ids.contains(window_id),
        RenderTarget::Image(image_handle) => changed_image_handles.contains(&image_handle),
    }
}

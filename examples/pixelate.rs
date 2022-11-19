use bevy::{
    prelude::{
        default, shape, App, Assets, Camera, Camera2dBundle, Color, Commands, Mesh, Quat, ResMut,
        Transform, Vec3,
    },
    render::camera::RenderTarget,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    DefaultPlugins,
};
use bevy_vfx::prelude::{PipelineComponent, VFXPipe, VFXPipeline, VFXPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(VFXPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut vfx_pipes: ResMut<Assets<VFXPipeline>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default()
            .with_scale(Vec3::splat(128.))
            .with_rotation(Quat::from_rotation_z(23.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });

    let pipeline = VFXPipeline::new(
        vec![Pixelate::create(4)],
        RenderTarget::Window(Default::default()),
    );

    // some test code to edit data.
    let mut x = Pixelate::create(4);
    x.size = 4;

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                target: pipeline.render_target(),
                ..default()
            },
            ..default()
        },
        PipelineComponent(vfx_pipes.add(pipeline)), // this makes us access the pipeline and also let it drop when there is no more camera using it :)
    ));
}

struct Pixelate {
    pub size: u8,
}

// this is a mess! make a proc macro
impl VFXPipe for Pixelate {}

impl Pixelate {
    pub fn create(size: u8) -> Box<Pixelate> {
        Box::new(Pixelate { size })
    }
}

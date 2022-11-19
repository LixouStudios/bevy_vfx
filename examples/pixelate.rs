use bevy::{
    prelude::{
        default, shape, App, Assets, Camera, Camera2dBundle, Color, Commands, Handle, Image, Input,
        KeyCode, Mesh, Quat, Query, Res, ResMut, Transform, Vec3,
    },
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    DefaultPlugins,
};
use bevy_vfx::prelude::{VFXPipe, VFXPipeline, VFXPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(VFXPlugin)
        .add_startup_system(setup)
        .add_system(toggle_vfx)
        .run();
}

fn toggle_vfx(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Camera, &mut VFXPipeline)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut cam, mut pipeline) in &mut query {
            pipeline
                .get_pipe_mut::<Pixelate>(0)
                .unwrap()
                .toggle_enabled();

            cam.target = pipeline.render_target();
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
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
        vec![Pixelate::create(
            4,
            images.add(make_image(1280, 720, "name")),
        )],
        RenderTarget::Window(Default::default()),
    );

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                target: pipeline.render_target(),
                ..default()
            },
            ..default()
        },
        pipeline,
    ));
}

#[derive(VFXPipe)]
struct Pixelate {
    pub size: u8,
    image: Handle<Image>,
    enabled: bool,
}

impl Pixelate {
    pub fn create(size: u8, image: Handle<Image>) -> Box<Pixelate> {
        Box::new(Pixelate {
            size,
            image,
            enabled: true,
        })
    }
}

fn make_image(width: u32, height: u32, name: &'static str) -> Image {
    let size = Extent3d {
        width,
        height,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some(name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };
    image.resize(size);
    image
}

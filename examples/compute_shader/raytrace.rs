use bevy::{prelude::*, reflect::*, render::{pipeline::{
            ComputePipelineDescriptor
        }, render_graph::{AssetRenderResourcesNode, ComputePassNode, RenderGraph, base::{self, camera::CAMERA_3D}}, renderer::{RenderResources}, shader::{ComputeShaderStages, ShaderStage}, texture::{Extent3d, TextureFormat}}};

const COMPUTE_SHADER: &str = r#"
#version 450

layout(local_size_x = 1) in;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

void main() {
    mat4 my_mat = ViewProj;
}
"#;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_asset::<ComputeResource>()
        .add_startup_system(setup.system())
        .run();
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct ComputeResource {
    pub color: Color,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut pipelines: ResMut<Assets<ComputePipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut compute_resources: ResMut<Assets<ComputeResource>>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());

    // Setup compute pipeline
    let pipeline_handle = pipelines.add(ComputePipelineDescriptor::new(ComputeShaderStages {
        compute: shaders.add(Shader::from_glsl(ShaderStage::Compute, COMPUTE_SHADER))
    }));

    // Create texture.
    let size = Extent3d::new(256, 256, 1);
    let format = TextureFormat::R16Float;
    let texture = Texture::new(size, bevy::render::texture::TextureDimension::D2, vec![0; size.volume() * format.pixel_size()], format);
    let texture_handle = textures.add(texture);

    // Create compute resource
    let compute_resource = ComputeResource {
        color: Color::default(),
    };
    compute_resources.add(compute_resource);


    // Create compute pass node.
    let mut compute_pass_node = ComputePassNode::new(pipeline_handle, UVec3::new(1, 1, 1));
    compute_pass_node.add_camera(CAMERA_3D);

    render_graph.add_node("COMPUTE_NODE", compute_pass_node);
    render_graph
        .add_node_edge("COMPUTE_NODE", base::node::MAIN_PASS)
        .unwrap();

    // Add compute resource for bind groups.
    render_graph.add_system_node(
        "compute_resource",
        AssetRenderResourcesNode::<ComputeResource>::new(true),
    );
    render_graph
        .add_node_edge("compute_resource", "COMPUTE_NODE")
        .unwrap();

    let mut transform = Transform::from_xyz(0.0, 0.0, -10.0);
    transform.scale = Vec3::splat(1.0 / 256.0);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        transform,
        ..Default::default()
    });
}

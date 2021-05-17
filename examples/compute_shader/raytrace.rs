use bevy::{prelude::*, render::{pipeline::{ComputePipelineDescriptor}, render_graph::{ComputePassNode, RenderGraph, base::{self, camera::CAMERA_3D}}, shader::{ComputeShaderStages, ShaderStage}}};

const COMPUTE_SHADER: &str = r#"
#version 450

layout(local_size_x = 1) in;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

void main() {

}
"#;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut pipelines: ResMut<Assets<ComputePipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Setup compute pipeline
    let pipeline_handle = pipelines.add(ComputePipelineDescriptor::new(ComputeShaderStages {
        compute: shaders.add(Shader::from_glsl(ShaderStage::Compute, COMPUTE_SHADER))
    }));

    let mut compute_pass_node = ComputePassNode::new(pipeline_handle, UVec3::new(1, 1, 1));
    compute_pass_node.add_camera(CAMERA_3D);

    render_graph.add_node("COMPUTE_NODE", compute_pass_node);
    render_graph
        .add_node_edge("COMPUTE_NODE", base::node::MAIN_PASS)
        .unwrap();

    let texture_handle = asset_server.load("branding/icon.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });
}

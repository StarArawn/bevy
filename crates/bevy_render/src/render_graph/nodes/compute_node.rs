use bevy_asset::{Assets, Handle};
use bevy_ecs::{prelude::{Mut, World}, world::WorldCell};
use bevy_math::UVec3;

use crate::{camera::ActiveCameras, pipeline::{
        BindGroupDescriptorId, PipelineCompiler, ComputePipelineDescriptor, ComputePipelineSpecialization,
    }, render_graph::Node, renderer::{
        BindGroupId, RenderResourceBindings, RenderResourceContext,
    }, shader::Shader};

#[derive(Debug)]
struct SetBindGroupCommand {
    index: u32,
    descriptor_id: BindGroupDescriptorId,
    bind_group: BindGroupId,
}

/// This node can be used to run a fullscreen pass with a custom pipeline
/// taking optional render textures and samples from previous passes as input.
#[derive(Debug)]
pub struct ComputePassNode {
    /// Shader pipeline that will be used by this fullscreen pass
    pipeline_handle: Handle<ComputePipelineDescriptor>,
    /// Handle to the compiled pipeline specialization used for rendering
    specialized_pipeline_handle: Option<Handle<ComputePipelineDescriptor>>,
    /// Internal render resource bindings for the additional inputs to this pass
    render_resource_bindings: RenderResourceBindings,
    /// SetBindGroupCommands for this frame, collected during prepare and update
    bind_groups: Vec<SetBindGroupCommand>,
    /// Denotes the number of work groups to dispatch in each dimension.
    work_groups: UVec3,
    /// A list of cameras
    cameras: Vec<String>,
}

impl ComputePassNode {
    pub fn new(
        pipeline_handle: Handle<ComputePipelineDescriptor>,
        work_groups: UVec3,
    ) -> Self {
        Self {
            pipeline_handle,
            specialized_pipeline_handle: None,
            render_resource_bindings: RenderResourceBindings::default(),
            bind_groups: Vec::new(),
            work_groups,
            cameras: Vec::new(),
        }
    }

    pub fn add_camera(&mut self, camera_name: &str) {
        self.cameras.push(camera_name.to_string());
    }

    /// Set up and compile the specialized pipeline to use
    fn setup_specialized_pipeline(&mut self, world: &mut WorldCell) {
        // Get all the necessary resources
        let mut pipeline_descriptors = world
            .get_resource_mut::<Assets<ComputePipelineDescriptor>>()
            .unwrap();

        let mut pipeline_compiler = world.get_resource_mut::<PipelineCompiler>().unwrap();
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();

        let render_resource_context = world
            .get_resource::<Box<dyn RenderResourceContext>>()
            .unwrap();

        let pipeline_descriptor = pipeline_descriptors
            .get(&self.pipeline_handle)
            .unwrap()
            .clone();

        let pipeline_specialization = ComputePipelineSpecialization {
            ..Default::default()
        };

        let specialized_pipeline_handle = if let Some(specialized_pipeline) = pipeline_compiler
            .get_specialized_compute_pipeline(&self.pipeline_handle, &pipeline_specialization)
        {
            specialized_pipeline
        } else {
            pipeline_compiler.compile_compute_pipeline(
                &**render_resource_context,
                &mut pipeline_descriptors,
                &mut shaders,
                &self.pipeline_handle,
                &pipeline_specialization,
            )
        };

        render_resource_context.create_compute_pipeline(
            specialized_pipeline_handle.clone(),
            &pipeline_descriptor,
            &*shaders,
        );

        self.specialized_pipeline_handle
            .replace(specialized_pipeline_handle);
    }
}

// Update bind groups and collect SetBindGroupCommands in Vec
fn update_bind_groups(
    render_resource_bindings: &mut RenderResourceBindings,
    pipeline_descriptor: &ComputePipelineDescriptor,
    render_resource_context: &dyn RenderResourceContext,
    set_bind_group_commands: &mut Vec<SetBindGroupCommand>,
) {
    // Try to set up the bind group for each descriptor in the pipeline layout
    // Some will be set up later, during update
    for bind_group_descriptor in &pipeline_descriptor.layout.as_ref().unwrap().bind_groups {
        if let Some(bind_group) = render_resource_bindings
            .update_bind_group(bind_group_descriptor, render_resource_context)
        {
            set_bind_group_commands.push(SetBindGroupCommand {
                index: bind_group_descriptor.index,
                descriptor_id: bind_group_descriptor.id,
                bind_group: bind_group.id,
            })
        }
    }
}

impl  Node for ComputePassNode {
    fn prepare(&mut self, world: &mut World) {
        // Clear previous frame's bind groups
        self.bind_groups.clear();

        world.resource_scope(|world, mut active_cameras: Mut<ActiveCameras>| {
            let pipeline_descriptor = {
                let mut world_cell = world.cell();

                // Compile the specialized pipeline
                if self.specialized_pipeline_handle.is_none() {
                    self.setup_specialized_pipeline(&mut world_cell);
                }

                // Prepare bind groups
                // Get the necessary resources
                let mut render_resource_bindings =
                world_cell.get_resource_mut::<RenderResourceBindings>().unwrap();

                let pipeline_descriptors = world_cell.get_resource::<Assets<ComputePipelineDescriptor>>().unwrap();

                let render_resource_context = world_cell
                    .get_resource::<Box<dyn RenderResourceContext>>()
                    .unwrap();

                let pipeline_descriptor = pipeline_descriptors
                    .get(self.specialized_pipeline_handle.as_ref().unwrap())
                    .unwrap();

                // Do the update
                update_bind_groups(
                    &mut render_resource_bindings,
                    pipeline_descriptor,
                    &**render_resource_context,
                    &mut self.bind_groups,
                );

                pipeline_descriptor.clone()
            };

            let render_resource_context = &**world
                .get_resource::<Box<dyn RenderResourceContext>>()
                .unwrap();
            
            for camera_name in self.cameras.iter() {
                let active_camera = if let Some(active_camera) = active_cameras.get_mut(camera_name)
                {
                    active_camera
                } else {
                    continue;
                }; 

                 let layout = pipeline_descriptor.get_layout().unwrap();
                for bind_group_descriptor in layout.bind_groups.iter() {
                    if let Some(bind_group) =
                        active_camera.bindings.update_bind_group(
                            bind_group_descriptor,
                            render_resource_context,
                        )
                    {
                        self.bind_groups.push(SetBindGroupCommand {
                            index: bind_group_descriptor.index,
                            descriptor_id: bind_group_descriptor.id,
                            bind_group: bind_group.id,
                        });
                    }
                }
                
            }
        });
    }

    fn update(
        &mut self,
        world: &bevy_ecs::prelude::World,
        render_context: &mut dyn crate::renderer::RenderContext,
        _input: &crate::render_graph::ResourceSlots,
        _output: &mut crate::render_graph::ResourceSlots,
    ) {
        // Prepare bind groups
        // Get the necessary resources
        let pipeline_descriptors = world.get_resource::<Assets<ComputePipelineDescriptor>>().unwrap();
        let pipeline_descriptor = pipeline_descriptors
            .get(self.specialized_pipeline_handle.as_ref().unwrap())
            .unwrap();
        let render_resource_context = render_context.resources_mut();

        // Do the update
        update_bind_groups(
            &mut self.render_resource_bindings,
            pipeline_descriptor,
            render_resource_context,
            &mut self.bind_groups,
        );

        // Check if all bindings are set, will get WGPU error otherwise
        if self.bind_groups.len()
            != pipeline_descriptor
                .layout
                .as_ref()
                .unwrap()
                .bind_groups
                .len()
        {
            panic!("Failed to set all bind groups");
        }

        // Begin actual render pass
        render_context.begin_compute_pass(
            &mut |compute_pass| {
                // Set pipeline
                compute_pass.set_pipeline(self.specialized_pipeline_handle.as_ref().unwrap());

                // Set all prepared bind groups
                self.bind_groups.iter().for_each(|command| {
                    compute_pass.set_bind_group(
                        command.index,
                        command.descriptor_id,
                        command.bind_group,
                        // Never needed, because no per-object bindings
                        None,
                    );
                });

                // Draw a single triangle without the need for buffers
                // see fullscreen.vert
                compute_pass.dispatch(self.work_groups.x, self.work_groups.y, self.work_groups.z);
            },
        );
    }
}
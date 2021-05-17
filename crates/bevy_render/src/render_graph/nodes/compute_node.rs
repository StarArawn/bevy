use bevy_asset::{Assets, Handle};
use bevy_ecs::{prelude::{Mut, World}, world::{WorldBorrowMut, WorldCell}};
use bevy_math::UVec3;
use crate::{camera::ActiveCameras, draw::DrawError, pipeline::{BindGroupDescriptorId, ComputePipelineDescriptor, ComputePipelineSpecialization, PipelineCompiler}, render_graph::Node, renderer::{AssetRenderResourceBindings, BindGroupId, RenderResourceBindings, RenderResourceContext}, shader::Shader};

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

    fn set_asset_bind_groups(
        render_resource_context: &dyn RenderResourceContext,
        asset_render_resource_bindings: &mut WorldBorrowMut<AssetRenderResourceBindings>,
        compute_pipelines_descriptor: &ComputePipelineDescriptor,
        render_resource_bindings: &mut [&mut RenderResourceBindings],
    ) -> Vec<SetBindGroupCommand> {
        let mut bind_groups = Vec::new();

        let layout = compute_pipelines_descriptor
            .get_layout()
            .ok_or(DrawError::PipelineHasNoLayout).unwrap();
        
        'bind_group_descriptors: for bind_group_descriptor in layout.bind_groups.iter() {
            for bindings in render_resource_bindings.iter_mut() {
                if let Some(bind_group) =
                    bindings.update_bind_group(bind_group_descriptor, render_resource_context)
                {
                    dbg!(&bind_group_descriptor);
                    bind_groups.push(SetBindGroupCommand {
                        index: bind_group_descriptor.index,
                        descriptor_id: bind_group_descriptor.id,
                        bind_group: bind_group.id,
                    });
                    continue 'bind_group_descriptors;
                }
            }

            for bindings in render_resource_bindings.iter_mut() {
                for (asset_handle, _) in bindings.iter_assets() {
                    let asset_bindings = if let Some(asset_bindings) =
                        asset_render_resource_bindings.get_mut_untyped(asset_handle)
                    {
                        asset_bindings
                    } else {
                        continue;
                    };

                    if let Some(bind_group) = asset_bindings
                        .update_bind_group(bind_group_descriptor, render_resource_context)
                    {
                        dbg!(&bind_group_descriptor);
                        bind_groups.push(SetBindGroupCommand {
                            index: bind_group_descriptor.index,
                            descriptor_id: bind_group_descriptor.id,
                            bind_group: bind_group.id,
                        });
                        continue 'bind_group_descriptors;
                    }
                }
            }
        }
        bind_groups
    }
}

impl Node for ComputePassNode {
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

                let mut asset_render_resource_bindings = world_cell.get_resource_mut::<AssetRenderResourceBindings>().unwrap();

                let render_resource_bindings = &mut [
                    &mut render_resource_bindings,
                    &mut self.render_resource_bindings,
                ];

                // Update bind groups for render resource assets
                let bind_groups = Self::set_asset_bind_groups(
                    &**render_resource_context,
                    &mut asset_render_resource_bindings,
                    pipeline_descriptor, 
                    render_resource_bindings
                );

                self.bind_groups.extend(bind_groups);
                
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
        _world: &bevy_ecs::prelude::World,
        render_context: &mut dyn crate::renderer::RenderContext,
        _input: &crate::render_graph::ResourceSlots,
        _output: &mut crate::render_graph::ResourceSlots,
    ) {
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

                // Dispatch compute shader.
                compute_pass.dispatch(self.work_groups.x, self.work_groups.y, self.work_groups.z);
            },
        );
    }
}
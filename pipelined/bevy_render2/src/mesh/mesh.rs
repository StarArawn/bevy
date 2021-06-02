mod conversions;
mod mesh_resource_provider;

pub use mesh_resource_provider::*;

use crate::{
    pipeline::{
        IndexFormat, InputStepMode, PrimitiveTopology, VertexAttribute, VertexBufferLayout,
        VertexFormat,
    },
    render_resource::BufferId,
};
use bevy_core::cast_slice;
use bevy_math::*;
use bevy_reflect::TypeUuid;
use bevy_utils::EnumVariantMeta;
use std::{borrow::Cow, collections::BTreeMap};

pub const INDEX_BUFFER_ASSET_INDEX: u64 = 0;
pub const VERTEX_ATTRIBUTE_BUFFER_ID: u64 = 10;

/// An array where each entry describes a property of a single vertex.
#[derive(Clone, Debug, EnumVariantMeta)]
pub enum VertexAttributeValues {
    Float32(Vec<f32>),
    Sint32(Vec<i32>),
    Uint32(Vec<u32>),
    Float32x2(Vec<[f32; 2]>),
    Sint32x2(Vec<[i32; 2]>),
    Uint32x2(Vec<[u32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    Sint32x3(Vec<[i32; 3]>),
    Uint32x3(Vec<[u32; 3]>),
    Float32x4(Vec<[f32; 4]>),
    Sint32x4(Vec<[i32; 4]>),
    Uint32x4(Vec<[u32; 4]>),
    Sint16x2(Vec<[i16; 2]>),
    Snorm16x2(Vec<[i16; 2]>),
    Uint16x2(Vec<[u16; 2]>),
    Unorm16x2(Vec<[u16; 2]>),
    Sint16x4(Vec<[i16; 4]>),
    Snorm16x4(Vec<[i16; 4]>),
    Uint16x4(Vec<[u16; 4]>),
    Unorm16x4(Vec<[u16; 4]>),
    Sint8x2(Vec<[i8; 2]>),
    Snorm8x2(Vec<[i8; 2]>),
    Uint8x2(Vec<[u8; 2]>),
    Unorm8x2(Vec<[u8; 2]>),
    Sint8x4(Vec<[i8; 4]>),
    Snorm8x4(Vec<[i8; 4]>),
    Uint8x4(Vec<[u8; 4]>),
    Unorm8x4(Vec<[u8; 4]>),
}

impl VertexAttributeValues {
    /// Returns the number of vertices in this VertexAttribute. For a single
    /// mesh, all of the VertexAttributeValues must have the same length.
    pub fn len(&self) -> usize {
        match *self {
            VertexAttributeValues::Float32(ref values) => values.len(),
            VertexAttributeValues::Sint32(ref values) => values.len(),
            VertexAttributeValues::Uint32(ref values) => values.len(),
            VertexAttributeValues::Float32x2(ref values) => values.len(),
            VertexAttributeValues::Sint32x2(ref values) => values.len(),
            VertexAttributeValues::Uint32x2(ref values) => values.len(),
            VertexAttributeValues::Float32x3(ref values) => values.len(),
            VertexAttributeValues::Sint32x3(ref values) => values.len(),
            VertexAttributeValues::Uint32x3(ref values) => values.len(),
            VertexAttributeValues::Float32x4(ref values) => values.len(),
            VertexAttributeValues::Sint32x4(ref values) => values.len(),
            VertexAttributeValues::Uint32x4(ref values) => values.len(),
            VertexAttributeValues::Sint16x2(ref values) => values.len(),
            VertexAttributeValues::Snorm16x2(ref values) => values.len(),
            VertexAttributeValues::Uint16x2(ref values) => values.len(),
            VertexAttributeValues::Unorm16x2(ref values) => values.len(),
            VertexAttributeValues::Sint16x4(ref values) => values.len(),
            VertexAttributeValues::Snorm16x4(ref values) => values.len(),
            VertexAttributeValues::Uint16x4(ref values) => values.len(),
            VertexAttributeValues::Unorm16x4(ref values) => values.len(),
            VertexAttributeValues::Sint8x2(ref values) => values.len(),
            VertexAttributeValues::Snorm8x2(ref values) => values.len(),
            VertexAttributeValues::Uint8x2(ref values) => values.len(),
            VertexAttributeValues::Unorm8x2(ref values) => values.len(),
            VertexAttributeValues::Sint8x4(ref values) => values.len(),
            VertexAttributeValues::Snorm8x4(ref values) => values.len(),
            VertexAttributeValues::Uint8x4(ref values) => values.len(),
            VertexAttributeValues::Unorm8x4(ref values) => values.len(),
        }
    }

    /// Returns `true` if there are no vertices in this VertexAttributeValue
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn as_float3(&self) -> Option<&[[f32; 3]]> {
        match self {
            VertexAttributeValues::Float32x3(values) => Some(values),
            _ => None,
        }
    }

    // TODO: add vertex format as parameter here and perform type conversions
    /// Flattens the VertexAttributeArray into a sequence of bytes. This is
    /// useful for serialization and sending to the GPU.
    pub fn get_bytes(&self) -> &[u8] {
        match self {
            VertexAttributeValues::Float32(values) => cast_slice(&values[..]),
            VertexAttributeValues::Sint32(values) => cast_slice(&values[..]),
            VertexAttributeValues::Uint32(values) => cast_slice(&values[..]),
            VertexAttributeValues::Float32x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Sint32x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Uint32x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Float32x3(values) => cast_slice(&values[..]),
            VertexAttributeValues::Sint32x3(values) => cast_slice(&values[..]),
            VertexAttributeValues::Uint32x3(values) => cast_slice(&values[..]),
            VertexAttributeValues::Float32x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Sint32x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Uint32x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Sint16x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Snorm16x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Uint16x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Unorm16x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Sint16x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Snorm16x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Uint16x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Unorm16x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Sint8x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Snorm8x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Uint8x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Unorm8x2(values) => cast_slice(&values[..]),
            VertexAttributeValues::Sint8x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Snorm8x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Uint8x4(values) => cast_slice(&values[..]),
            VertexAttributeValues::Unorm8x4(values) => cast_slice(&values[..]),
        }
    }
}

impl From<&VertexAttributeValues> for VertexFormat {
    fn from(values: &VertexAttributeValues) -> Self {
        match values {
            VertexAttributeValues::Float32(_) => VertexFormat::Float32,
            VertexAttributeValues::Sint32(_) => VertexFormat::Sint32,
            VertexAttributeValues::Uint32(_) => VertexFormat::Uint32,
            VertexAttributeValues::Float32x2(_) => VertexFormat::Float32x2,
            VertexAttributeValues::Sint32x2(_) => VertexFormat::Sint32x2,
            VertexAttributeValues::Uint32x2(_) => VertexFormat::Uint32x2,
            VertexAttributeValues::Float32x3(_) => VertexFormat::Float32x3,
            VertexAttributeValues::Sint32x3(_) => VertexFormat::Sint32x3,
            VertexAttributeValues::Uint32x3(_) => VertexFormat::Uint32x3,
            VertexAttributeValues::Float32x4(_) => VertexFormat::Float32x4,
            VertexAttributeValues::Sint32x4(_) => VertexFormat::Sint32x4,
            VertexAttributeValues::Uint32x4(_) => VertexFormat::Uint32x4,
            VertexAttributeValues::Sint16x2(_) => VertexFormat::Sint16x2,
            VertexAttributeValues::Snorm16x2(_) => VertexFormat::Snorm16x2,
            VertexAttributeValues::Uint16x2(_) => VertexFormat::Uint16x2,
            VertexAttributeValues::Unorm16x2(_) => VertexFormat::Unorm16x2,
            VertexAttributeValues::Sint16x4(_) => VertexFormat::Sint16x4,
            VertexAttributeValues::Snorm16x4(_) => VertexFormat::Snorm16x4,
            VertexAttributeValues::Uint16x4(_) => VertexFormat::Uint16x4,
            VertexAttributeValues::Unorm16x4(_) => VertexFormat::Unorm16x4,
            VertexAttributeValues::Sint8x2(_) => VertexFormat::Sint8x2,
            VertexAttributeValues::Snorm8x2(_) => VertexFormat::Snorm8x2,
            VertexAttributeValues::Uint8x2(_) => VertexFormat::Uint8x2,
            VertexAttributeValues::Unorm8x2(_) => VertexFormat::Unorm8x2,
            VertexAttributeValues::Sint8x4(_) => VertexFormat::Sint8x4,
            VertexAttributeValues::Snorm8x4(_) => VertexFormat::Snorm8x4,
            VertexAttributeValues::Uint8x4(_) => VertexFormat::Uint8x4,
            VertexAttributeValues::Unorm8x4(_) => VertexFormat::Unorm8x4,
        }
    }
}

/// An array of indices into the VertexAttributeValues for a mesh.
///
/// It describes the order in which the vertex attributes should be joined into faces.
#[derive(Debug, Clone)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Indices {
    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        match self {
            Indices::U16(vec) => IndicesIter::U16(vec.iter()),
            Indices::U32(vec) => IndicesIter::U32(vec.iter()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Indices::U16(vec) => vec.len(),
            Indices::U32(vec) => vec.len(),
        }
    }
}
enum IndicesIter<'a> {
    U16(std::slice::Iter<'a, u16>),
    U32(std::slice::Iter<'a, u32>),
}
impl Iterator for IndicesIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IndicesIter::U16(iter) => iter.next().map(|val| *val as usize),
            IndicesIter::U32(iter) => iter.next().map(|val| *val as usize),
        }
    }
}

impl From<&Indices> for IndexFormat {
    fn from(indices: &Indices) -> Self {
        match indices {
            Indices::U16(_) => IndexFormat::Uint16,
            Indices::U32(_) => IndexFormat::Uint32,
        }
    }
}

// TODO: this shouldn't live in the Mesh type
#[derive(Debug, Clone)]
pub struct MeshGpuData {
    pub vertex_buffer: BufferId,
    pub index_buffer: Option<BufferId>,
}

// TODO: allow values to be unloaded after been submitting to the GPU to conserve memory
#[derive(Debug, TypeUuid, Clone)]
#[uuid = "8ecbac0f-f545-4473-ad43-e1f4243af51e"]
pub struct Mesh {
    primitive_topology: PrimitiveTopology,
    /// `std::collections::BTreeMap` with all defined vertex attributes (Positions, Normals, ...)
    /// for this mesh. Attribute name maps to attribute values.
    /// Uses a BTreeMap because, unlike HashMap, it has a defined iteration order,
    /// which allows easy stable VertexBuffers (i.e. same buffer order)
    attributes: BTreeMap<Cow<'static, str>, VertexAttributeValues>,
    indices: Option<Indices>,
    gpu_data: Option<MeshGpuData>,
}

/// Contains geometry in the form of a mesh.
///
/// Often meshes are automatically generated by bevy's asset loaders or primitives, such as
/// [`crate::shape::Cube`] or [`crate::shape::Box`], but you can also construct
/// one yourself.
///
/// Example of constructing a mesh:
/// ```
/// # use bevy_render::mesh::{Mesh, Indices};
/// # use bevy_render::pipeline::PrimitiveTopology;
/// fn create_triangle() -> Mesh {
///     let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
///     mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
///     mesh.set_indices(Some(Indices::U32(vec![0,1,2])));
///     mesh
/// }
/// ```
impl Mesh {
    /// Per vertex coloring. Use in conjunction with [`Mesh::set_attribute`]
    pub const ATTRIBUTE_COLOR: &'static str = "Vertex_Color";
    /// The direction the vertex normal is facing in.
    /// Use in conjunction with [`Mesh::set_attribute`]
    pub const ATTRIBUTE_NORMAL: &'static str = "Vertex_Normal";
    /// The direction of the vertex tangent. Used for normal mapping
    pub const ATTRIBUTE_TANGENT: &'static str = "Vertex_Tangent";

    /// Where the vertex is located in space. Use in conjunction with [`Mesh::set_attribute`]
    pub const ATTRIBUTE_POSITION: &'static str = "Vertex_Position";
    /// Texture coordinates for the vertex. Use in conjunction with [`Mesh::set_attribute`]
    pub const ATTRIBUTE_UV_0: &'static str = "Vertex_Uv";

    /// Per vertex joint transform matrix weight. Use in conjunction with [`Mesh::set_attribute`]
    pub const ATTRIBUTE_JOINT_WEIGHT: &'static str = "Vertex_JointWeight";
    /// Per vertex joint transform matrix index. Use in conjunction with [`Mesh::set_attribute`]
    pub const ATTRIBUTE_JOINT_INDEX: &'static str = "Vertex_JointIndex";

    /// Construct a new mesh. You need to provide a PrimitiveTopology so that the
    /// renderer knows how to treat the vertex data. Most of the time this will be
    /// `PrimitiveTopology::TriangleList`.
    pub fn new(primitive_topology: PrimitiveTopology) -> Self {
        Mesh {
            primitive_topology,
            attributes: Default::default(),
            indices: None,
            gpu_data: None,
        }
    }

    pub fn primitive_topology(&self) -> PrimitiveTopology {
        self.primitive_topology
    }

    pub fn gpu_data(&self) -> Option<&MeshGpuData> {
        self.gpu_data.as_ref()
    }

    /// Sets the data for a vertex attribute (position, normal etc.). The name will
    /// often be one of the associated constants such as [`Mesh::ATTRIBUTE_POSITION`]
    pub fn set_attribute(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        values: impl Into<VertexAttributeValues>,
    ) {
        let values: VertexAttributeValues = values.into();
        self.attributes.insert(name.into(), values);
    }

    /// Retrieve the data currently set behind a vertex attribute.
    pub fn attribute(&self, name: impl Into<Cow<'static, str>>) -> Option<&VertexAttributeValues> {
        self.attributes.get(&name.into())
    }

    pub fn attribute_mut(
        &mut self,
        name: impl Into<Cow<'static, str>>,
    ) -> Option<&mut VertexAttributeValues> {
        self.attributes.get_mut(&name.into())
    }

    /// Indices describe how triangles are constructed out of the vertex attributes.
    /// They are only useful for the [`crate::pipeline::PrimitiveTopology`] variants that use
    /// triangles
    pub fn set_indices(&mut self, indices: Option<Indices>) {
        self.indices = indices;
    }

    pub fn indices(&self) -> Option<&Indices> {
        self.indices.as_ref()
    }

    pub fn indices_mut(&mut self) -> Option<&mut Indices> {
        self.indices.as_mut()
    }

    pub fn get_index_buffer_bytes(&self) -> Option<&[u8]> {
        self.indices.as_ref().map(|indices| match &indices {
            Indices::U16(indices) => cast_slice(&indices[..]),
            Indices::U32(indices) => cast_slice(&indices[..]),
        })
    }

    pub fn get_vertex_buffer_layout(&self) -> VertexBufferLayout {
        let mut attributes = Vec::new();
        let mut accumulated_offset = 0;
        for (attribute_name, attribute_values) in self.attributes.iter() {
            let vertex_format = VertexFormat::from(attribute_values);
            attributes.push(VertexAttribute {
                name: attribute_name.clone(),
                offset: accumulated_offset,
                format: vertex_format,
                shader_location: 0,
            });
            accumulated_offset += vertex_format.get_size();
        }

        VertexBufferLayout {
            name: Default::default(),
            stride: accumulated_offset,
            step_mode: InputStepMode::Vertex,
            attributes,
        }
    }

    pub fn count_vertices(&self) -> usize {
        let mut vertex_count: Option<usize> = None;
        for (attribute_name, attribute_data) in self.attributes.iter() {
            let attribute_len = attribute_data.len();
            if let Some(previous_vertex_count) = vertex_count {
                assert_eq!(previous_vertex_count, attribute_len,
                        "Attribute {} has a different vertex count ({}) than other attributes ({}) in this mesh.", attribute_name, attribute_len, previous_vertex_count);
            }
            vertex_count = Some(attribute_len);
        }

        vertex_count.unwrap_or(0)
    }

    pub fn get_vertex_buffer_data(&self) -> Vec<u8> {
        let mut vertex_size = 0;
        for attribute_values in self.attributes.values() {
            let vertex_format = VertexFormat::from(attribute_values);
            vertex_size += vertex_format.get_size() as usize;
        }

        let vertex_count = self.count_vertices();
        let mut attributes_interleaved_buffer = vec![0; vertex_count * vertex_size];
        // bundle into interleaved buffers
        let mut attribute_offset = 0;
        for attribute_values in self.attributes.values() {
            let vertex_format = VertexFormat::from(attribute_values);
            let attribute_size = vertex_format.get_size() as usize;
            let attributes_bytes = attribute_values.get_bytes();
            for (vertex_index, attribute_bytes) in
                attributes_bytes.chunks_exact(attribute_size).enumerate()
            {
                let offset = vertex_index * vertex_size + attribute_offset;
                attributes_interleaved_buffer[offset..offset + attribute_size]
                    .copy_from_slice(attribute_bytes);
            }

            attribute_offset += attribute_size;
        }

        attributes_interleaved_buffer
    }

    /// Duplicates the vertex attributes so that no vertices are shared.
    ///
    /// This can dramatically increase the vertex count, so make sure this is what you want.
    /// Does nothing if no [Indices] are set.
    pub fn duplicate_vertices(&mut self) {
        fn duplicate<T: Copy>(values: &[T], indices: impl Iterator<Item = usize>) -> Vec<T> {
            indices.map(|i| values[i]).collect()
        }

        assert!(
            matches!(self.primitive_topology, PrimitiveTopology::TriangleList),
            "can only duplicate vertices for `TriangleList`s"
        );

        let indices = match self.indices.take() {
            Some(indices) => indices,
            None => return,
        };
        for (_, attributes) in self.attributes.iter_mut() {
            let indices = indices.iter();
            match attributes {
                VertexAttributeValues::Float32(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Sint32(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Uint32(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Float32x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Sint32x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Uint32x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Float32x3(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Sint32x3(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Uint32x3(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Sint32x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Uint32x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Float32x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Sint16x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Snorm16x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Uint16x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Unorm16x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Sint16x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Snorm16x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Uint16x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Unorm16x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Sint8x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Snorm8x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Uint8x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Unorm8x2(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Sint8x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Snorm8x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Uint8x4(vec) => *vec = duplicate(&vec, indices),
                VertexAttributeValues::Unorm8x4(vec) => *vec = duplicate(&vec, indices),
            }
        }
    }

    /// Calculates the [`Mesh::ATTRIBUTE_NORMAL`] of a mesh.
    ///
    /// Panics if [`Indices`] are set.
    /// Consider calling [Mesh::duplicate_vertices] or export your mesh with normal attributes.
    pub fn compute_flat_normals(&mut self) {
        if self.indices().is_some() {
            panic!("`compute_flat_normals` can't work on indexed geometry. Consider calling `Mesh::duplicate_vertices`.");
        }

        let positions = self
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .expect("`Mesh::ATTRIBUTE_POSITION` vertex attributes should be of type `float3`");

        let normals: Vec<_> = positions
            .chunks_exact(3)
            .map(|p| face_normal(p[0], p[1], p[2]))
            .flat_map(|normal| std::array::IntoIter::new([normal, normal, normal]))
            .collect();

        self.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    }
}

fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
    (b - a).cross(c - a).normalize().into()
}

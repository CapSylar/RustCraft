use crate::engine::{chunk::{Chunk}, geometry::{voxel_vertex::VoxelVertex, mesh::Mesh, chunk_mesh::Face}};

#[derive(PartialEq)]
pub enum MeshingOption
{
    Opaque,
    Transparent,
}

pub trait ChunkMesher
{
    /// Generate the mesh for the chunk
    /// 
    /// Genrated Mesh is placed in mesh
    fn generate_mesh(chunk: &Chunk, mesh: &mut Mesh<VoxelVertex>, trans_faces: &mut Vec<Face>);
}

pub const VOXEL_SIZE: f32 = 1.0;

pub const VOXEL_FACE_VALUES : [(i32,i32,i32);6] = 
[
    (0,1,0),
    (0,-1,0),
    (0,0,1),
    (0,0,-1),
    (1,0,0),
    (-1,0,0)
];
    
#[derive(Clone, Copy)]
pub enum Direction // order is important, since the indices are used to index the normal table in the shader
{
    Posx,Posy,Posz,Negx,Negy,Negz
}

impl Direction
{
    pub fn from_index(index: usize) -> Direction
    {
        match index
        {
            0 => Direction::Posx,
            1 => Direction::Posy,
            2 => Direction::Posz,
            3 => Direction::Negx,
            4 => Direction::Negy,
            5 => Direction::Negz,
            _ => Direction::Posx,
        }
    }

    pub fn opposite(&self) -> Direction
    {
        Direction::from_index(*self as usize + 3)
    }
}

pub enum UVs
{
    LowerLeft, LowerRight, UpperLeft, UpperRight
}
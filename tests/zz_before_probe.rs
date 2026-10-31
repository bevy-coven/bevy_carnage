use bevy::asset::RenderAssetUsages;
use bevy::math::{Mat4, Vec3};
use bevy::mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues};
use bevy_carnage::{JointGroup, fracture_skeleton};

fn skinned(topology: PrimitiveTopology, joints: Vec<[u16; 4]>, weights: Vec<[f32; 4]>, indices: Vec<u32>) -> Mesh {
    let positions: Vec<[f32; 3]> = (0..joints.len()).map(|i| [i as f32, 0.0, 0.0]).collect();
    let mut mesh = Mesh::new(topology, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_INDEX, VertexAttributeValues::Uint16x4(joints));
    mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, weights);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
fn show(label: &str, r: Result<bevy_carnage::SkeletalFracture, bevy_carnage::SkeletalRefusal>) {
    match r {
        Ok(f) => {
            let tris: Vec<usize> = f.pieces.iter().flat_map(|p| p.meshes.iter().flatten().map(|m| m.indices().map_or(0, |i| i.len()))).collect();
            println!("PROBE {label}: Ok, piece index counts {tris:?}");
        }
        Err(e) => println!("PROBE {label}: Err({e:?})"),
    }
}
#[test]
fn probe() {
    let binds = vec![Mat4::IDENTITY, Mat4::from_translation(Vec3::new(-2.0, 0.0, 0.0))];
    let one = vec![JointGroup { joints: vec![0], anchor: 0 }];
    let two = vec![JointGroup { joints: vec![0], anchor: 0 }, JointGroup { joints: vec![1], anchor: 1 }];
    let r = || vec![[1.0, 0.0, 0.0, 0.0]; 4];
    let strip = skinned(PrimitiveTopology::TriangleStrip, vec![[0; 4]; 4], r(), vec![0, 1, 2, 3]);
    show("strip [0,1,2,3] (2 triangles)", fracture_skeleton(&[&strip], &binds, &one, &[]));
    let lines = skinned(PrimitiveTopology::LineList, vec![[0; 4]; 4], r(), vec![0, 1, 1, 2, 2, 3]);
    show("line list, 6 indices", fracture_skeleton(&[&lines], &binds, &one, &[]));
    let ragged = skinned(PrimitiveTopology::TriangleList, vec![[0; 4]; 4], r(), vec![0, 1, 2, 3]);
    show("triangle list, 4 indices", fracture_skeleton(&[&ragged], &binds, &one, &[]));
    let bad = skinned(PrimitiveTopology::TriangleList, vec![[0; 4]; 3], r()[..3].to_vec(), vec![0, 1, 7]);
    show("index 7 of 3 vertices", fracture_skeleton(&[&bad], &binds, &one, &[]));
    let soft = skinned(PrimitiveTopology::TriangleList, vec![[0, 1, 0, 0]; 3], vec![[0.6, 0.4, 0.0, 0.0]; 3], vec![0, 1, 2]);
    show("0.6/0.4 across two groups", fracture_skeleton(&[&soft], &binds, &two, &[]));
}

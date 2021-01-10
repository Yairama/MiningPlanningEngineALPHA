
use crate::components::CSVNodes;
use amethyst::core::ecs::{Entities, WriteStorage, Read, ReadStorage};
use amethyst::{
    assets::{Format as AssetFormat, Handle, Loader},
    core::{math::Vector3, Transform, TransformBundle},
    ecs::{World, WorldExt},
    error::Error,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        camera::Camera,
        light::{Light, PointLight},
        mtl::{Material, MaterialDefaults},
        palette::{Srgb, Srgba},
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        rendy::{
            mesh::{MeshBuilder, Normal, Position, TexCoord},
            texture::palette::load_from_srgba,
        },
        types::{DefaultBackend, Mesh, MeshData},
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst::core::ecs::shred::Resource;
use amethyst::shred::ReadExpect;
use amethyst::assets::AssetStorage;
use amethyst::renderer::Texture;
use std::fs;
use simple_csv::{SimpleCsvReader, SimpleCsvReaderOptions};
use amethyst::core::math::Point3;


#[derive(Clone, Debug)]
struct Cube;

impl AssetFormat<MeshData> for Cube {
    fn name(&self) -> &'static str {
        "CUSTOM_CUBE"
    }

    /// Reads the given bytes and produces asset data.
    fn import_simple(&self, bytes: Vec<u8>) -> Result<MeshData, Error> {
        let data: String = String::from_utf8(bytes)?;
        let trimmed: Vec<&str> = data.lines().filter(|line| !line.is_empty()).collect();

        let mut pos = Vec::with_capacity(trimmed.len() * 3);
        let mut norm = Vec::with_capacity(trimmed.len() * 3);
        let mut tex = Vec::with_capacity(trimmed.len() * 3);

        for line in trimmed {
            let nums: Vec<&str> = line.split_whitespace().collect();
            pos.push(Position([
                nums[0].parse::<f32>().unwrap(),
                nums[1].parse::<f32>().unwrap(),
                nums[2].parse::<f32>().unwrap(),
            ]));
            norm.push(Normal([
                nums[3].parse::<f32>().unwrap(),
                nums[4].parse::<f32>().unwrap(),
                nums[5].parse::<f32>().unwrap(),
            ]));
            tex.push(TexCoord([0.0, 0.0]))
        }
        Ok(MeshBuilder::new()
            .with_vertices(pos)
            .with_vertices(norm)
            .with_vertices(tex)
            .into())
    }
}


pub fn draw_cube_map(dir: String, entities: &Entities, cube_map_nodes: &mut WriteStorage<CSVNodes>, mat_defaults: &mut ReadExpect<MaterialDefaults>, loader: &mut ReadExpect<Loader>,
                     handle_mesh: &mut WriteStorage<Handle<Mesh>>, handle_material: &mut WriteStorage<Handle<Material>>, transform: &mut WriteStorage<Transform>,
                     read_mesh_storage: &ReadExpect<AssetStorage<Mesh>>,read_textures_storage: &ReadExpect<AssetStorage<Texture>>,read_materials_storage: &ReadExpect<AssetStorage<Material>> ){

    let contents = fs::read_to_string(dir)
        .expect("Something went wrong reading the file");
    let bytes = contents.into_bytes();
    let test_csv_reader = &*bytes;
    let mut csv_options: SimpleCsvReaderOptions = Default::default();
    csv_options.delimiter = ';';
    let mut reader = SimpleCsvReader::with_options(test_csv_reader,csv_options);
    let mut points: Vec<Point3<f32>> = Vec::new();
    for a in reader {
        let vec = a.unwrap();
        let x: f32 = vec[0].clone().replace("\u{feff}","").parse().unwrap();
        let y: f32 = vec[1].clone().replace("\u{feff}","").parse().unwrap();
        let z: f32 = vec[2].clone().replace("\u{feff}","").parse().unwrap();
        points.push(Point3::new(x,y,z));
    }

    let (mesh, mtl) = {

        let meshes = read_mesh_storage;
        let textures = read_textures_storage;
        let materials = read_materials_storage;

        let mesh: Handle<Mesh> = loader.load("custom/meshes/cube.custom", Cube, (), meshes);
        let albedo = loader.load_from_data(
            load_from_srgba(Srgba::new(0.1, 0.5, 0.3, 1.0)).into(),
            (),
            textures,
        );
        let mat: Handle<Material> = loader.load_from_data(
            Material {
                albedo,
                ..mat_defaults.0.clone()
            },
            (),
            materials,
        );

        (mesh, mat)
    };

    for point in points {

        let mut trans = Transform::default();
        trans.set_translation_xyz(point.x-13700.0, point.z-3500.0, point.y-27400.0);
        trans.set_scale(Vector3::new(1.0, 1.0, 1.0));

        let new_entity = entities.create();
        handle_mesh.insert(new_entity,mesh.clone());
        handle_material.insert(new_entity, mtl.clone());
        transform.insert(new_entity, trans);

    }

    println!("END");


}
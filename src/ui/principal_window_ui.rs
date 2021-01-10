
#[path = "../features/dxf/draw_dxf.rs"]
mod draw_dxf;
#[path = "../features/csv/draw_cube_map.rs"]
mod draw_cube_map;

use amethyst_imgui::imgui::{im_str, Condition, Window, MenuItem, Ui};
use std::fs;
use amethyst::DataInit;
use crate::components::{DXFNodes, CSVNodes};
use amethyst::renderer::debug_drawing::DebugLinesComponent;
use amethyst::renderer::types::Mesh;
use amethyst::renderer::mtl::Material;
use amethyst::core::ecs::{Entities, WriteStorage, Read, ReadStorage};
use amethyst::renderer::{MaterialDefaults, Texture};
use amethyst::assets::{Handle, Loader, AssetStorage};
use amethyst::core::Transform;
use amethyst::core::ecs::shred::{Fetch, Resource};
use amethyst::shred::ReadExpect;


fn build_gui(ui: &Ui, entities: Entities,mut dxf_nodes: WriteStorage<DXFNodes>, mut debug_lines_component: WriteStorage<DebugLinesComponent>,
             mut csv_nodes: WriteStorage<CSVNodes> , mut materials_default: ReadExpect<MaterialDefaults>, mut loader: ReadExpect<Loader>, mut handle_mesh: WriteStorage<Handle<Mesh>>,
            mut handle_material:WriteStorage<Handle<Material>>, mut transform: WriteStorage<Transform>, read_mesh_storage: ReadExpect<AssetStorage<Mesh>>,
             read_texture_storage: ReadExpect<AssetStorage<Texture>>, read_material_storage: ReadExpect<AssetStorage<Material>>){



    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
            MenuItem::new(im_str!("Open"))
                .shortcut(im_str!("Ctrl+O"))
                .build(ui);
            MenuItem::new(im_str!("Close"))
                .build(ui);
            menu.end(ui);

        }
        if let Some(menu) = ui.begin_menu(im_str!("Help"), true) {
            MenuItem::new(im_str!("About"))
                .shortcut(im_str!("F1"))
                .build(ui);
            menu.end(ui);
        }
        menu_bar.end(ui);
    }

    Window::new(im_str!("Project Explorer"))
        .size([300.0, 150.0], Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .position([10.0,500.0],Condition::Always)
        .build(ui,||{

            let dir = String::from("projects/");
            let paths = fs::read_dir(&dir).unwrap();
            for path in paths{
                let item = path.unwrap().path().display().to_string().replace(&dir,"");
                if let Some(menu) = ui.begin_menu(&im_str!("{}",item), true) {

                   if item.to_ascii_lowercase().contains(".dxf") {
                       if MenuItem::new(&im_str!("Draw DXF")).build(ui) {

                           let mut a = dir.clone();
                           a.push_str(&item);
                           draw_dxf::draw_dxf(a,&entities, &mut dxf_nodes, &mut debug_lines_component);
                       }
                       if MenuItem::new(&im_str!("Generate 3D Surface")).build(ui){

                       }
                   }

                    if item.to_ascii_lowercase().contains(".csv") {
                        if MenuItem::new(&im_str!("Generate Cube Maps")).build(ui) {

                            let mut a = dir.clone();
                            a.push_str(&item);
                            draw_cube_map::draw_cube_map(a, &entities, &mut csv_nodes, &mut materials_default, &mut loader,
                            &mut handle_mesh, &mut handle_material, &mut transform, &read_mesh_storage,&read_texture_storage, &read_material_storage);
                            //draw_dxf::draw_dxf(&a,&entities, &mut dxf_nodes, &mut debug_lines_component);
                        }
                        if MenuItem::new(&im_str!("Generate point cloud")).build(ui){

                        }
                    }
                    menu.end(ui);
                }



                ui.separator();
            }
        });

    Window::new(im_str!("Nodes Tree"))
        .size([300.0,400.0], Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .position([10.0,20.0],Condition::Always)
        .build(ui,||{



            ui.separator();
        })
}

#[derive(Default, Clone, Copy)]
pub struct UIPlanningEngine;
impl<'s> amethyst::ecs::System<'s> for UIPlanningEngine {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, DXFNodes>,
        WriteStorage<'s, DebugLinesComponent>,
        WriteStorage<'s, CSVNodes>,
        ReadExpect<'s, MaterialDefaults>,
        ReadExpect<'s, Loader>,
        WriteStorage<'s, Handle<Mesh>>,
        WriteStorage<'s, Handle<Material>>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, AssetStorage<Mesh>>,
        ReadExpect<'s, AssetStorage<Texture>>,
        ReadExpect<'s, AssetStorage<Material>>
    );

    fn run(&mut self, (entities, mut dxf_nodes, mut debug_lines_component, mut csv_nodes, mut material_defaults, mut loader,
        mut handle_mesh, mut handle_material, mut transform, read_mesh_storage, read_texture_storage, read_material_storage): Self::SystemData) {

        amethyst_imgui::with(|ui| build_gui(ui,entities, dxf_nodes, debug_lines_component, csv_nodes, material_defaults, loader,handle_mesh, handle_material,
        transform, read_mesh_storage, read_texture_storage, read_material_storage));

    }

}
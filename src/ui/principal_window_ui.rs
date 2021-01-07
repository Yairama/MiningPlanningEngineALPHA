
#[path = "../features/dxf/draw_dxf.rs"]
mod draw_dxf;

use amethyst_imgui::imgui::{im_str, Condition, Window, MenuItem, Ui};
use std::fs;
use amethyst::DataInit;
use specs::{Entities, WriteStorage};
use crate::components::DXFNodes;
use amethyst::renderer::debug_drawing::DebugLinesComponent;


fn build_gui(ui: &Ui, entities: Entities,mut dxf_nodes: WriteStorage<DXFNodes>, mut debug_lines_component: WriteStorage<DebugLinesComponent>){



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

                    if MenuItem::new(&im_str!("Draw DXF")).build(ui) {

                        let mut a = dir.clone();
                        a.push_str(&item);
                        draw_dxf::draw_dxf(&a,&entities, &mut dxf_nodes, &mut debug_lines_component);
                    }
                    if MenuItem::new(&im_str!("Generate 3D Surface")).build(ui){

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
        WriteStorage<'s, DebugLinesComponent>
    );

    fn run(&mut self, (entities, mut dxf_nodes, mut debug_lines_component): Self::SystemData) {

        amethyst_imgui::with(|ui| build_gui(ui,entities, dxf_nodes, debug_lines_component ));

    }

}
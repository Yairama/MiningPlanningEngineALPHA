
use amethyst_imgui::imgui::{im_str, Condition, Window, MenuItem};
use std::fs;

#[derive(Default, Clone, Copy)]
pub struct UIPlanningEngine;
impl<'s> amethyst::ecs::System<'s> for UIPlanningEngine {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        amethyst_imgui::with(|ui| {

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

            Window::new(im_str!("Project Explorer")).
                size([300.0, 500.0], Condition::FirstUseEver)
                .movable(false)
                .position([0.0,40.0],Condition::Always)
                .build(ui,||{
                    let dir = "projects/";
                    let paths = fs::read_dir(dir).unwrap();
                    for path in paths{
                        let item = path.unwrap().path().display().to_string().replace(dir,"");
                        ui.text(im_str!("{}",item));
                    }
                });


        });



    }

}

#[path = "./config/config.rs"]
mod config;
#[path = "./ui/principal_window_ui.rs"]
mod principal_window_ui;
#[path = "./world_resources/camera_controller.rs"]
mod camera_controller;
#[path = "./world_resources/debug_lines.rs"]
mod debug_lines;
#[path = "./components/components.rs"]
mod components;
use crate::components::{DXFNodes, CSVNodes};

use amethyst::{
    controls::{FlyControlBundle,HideCursor},
    core::TransformBundle,
    input::{is_close_requested, is_key_down, InputBundle, StringBindings,InputEvent},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderSkybox, RenderToWindow, RenderShaded3D},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    winit::{VirtualKeyCode, MouseButton},
};
use amethyst::{
    assets::{Format as AssetFormat, Handle, Loader},
    core::{math::Vector3, Transform},
    ecs::{World, WorldExt},
    error::Error,
    prelude::*,
    renderer::{
        camera::Camera,
        light::{Light, PointLight},
        mtl::{Material, MaterialDefaults},
        palette::{Srgb, Srgba},
        rendy::{
            mesh::{MeshBuilder, Normal, Position, TexCoord},
            texture::palette::load_from_srgba,
        },

    },
};

use amethyst_imgui::RenderImgui;
use amethyst::winit::{Window, EventsLoop};
use amethyst::renderer::debug_drawing::{DebugLines, DebugLinesComponent};
use amethyst::window::ScreenDimensions;
use amethyst::renderer::{Mesh, RenderBase3D};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use std::fs::File;
use amethyst::renderer::pod::DirectionalLight;


struct PlanningCore;
impl SimpleState for PlanningCore {
    fn on_start(&mut self, mut data: StateData<'_, GameData>) {
        let StateData { world, .. } = data;
        world.write_resource::<HideCursor>().hide = false;
        world.write_resource::<Window>().set_maximized(true);
        world.register::<DXFNodes>();
        world.register::<CSVNodes>();

        let light: Light = PointLight {
            intensity: 1000000.0,
            radius: 1000.0,
            color: Srgb::new(1.0, 1.0, 1.0),
            ..Default::default()
        }
            .into();

        let mut transform = Transform::default();
        transform.set_translation_xyz(5.0, 1000.0, 15.0);

        // Add point light.
        world.create_entity().with(light).with(transform).build();

        debug_lines::set_debug_lines(world);
        camera_controller::set_up_camera(world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
        //world.write_resource::<HideCursor>().hide = false;
        match event {



            StateEvent::Window(event) => {
                // Exit if the user hits escape
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                }

            }

            StateEvent::Input(event)=>{

                match event  {
                    InputEvent::MouseButtonPressed(MouseButton::Right) => {
                        world.write_resource::<HideCursor>().hide = true;
                    }

                    InputEvent::MouseButtonReleased(MouseButton::Right) => {
                        world.write_resource::<HideCursor>().hide = false;
                    }

                    _ => {}
                }


            }

            StateEvent::Ui(ui_event) => {
                log::info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );
            }
            _=>(),
        }

        Trans::None
    }


}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("src/config/display.ron");
    let key_bindings_path = app_root.join("src/config/input.ron");
    let assets_dir = app_root.join("src/assets/");
    let fly_control_bundle = FlyControlBundle::<StringBindings>::new(
        Some(String::from("move_x")),
        Some(String::from("move_y")),
        Some(String::from("move_z")),

    ).with_sensitivity(0.4, 0.4).with_speed(100.0);

/*    let mut game_data = GameDataBuilder::default()
        .with_barrier()
        .with( ui::principal_window_ui::UIPlanningEngine::default(), "imgui_use", &[])
        .with_bundle(InputBundle::<StringBindings>::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.36, 0.52, 1.0]))
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?;*/

    let mut game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with(debug_lines::ExampleLinesSystem, "example_lines_system", &[])
        .with_bundle(fly_control_bundle)?
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement"]))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?)
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderSkybox::default())
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?.with( principal_window_ui::UIPlanningEngine::default(), "imgui_use", &[]);

    let mut game = Application::build(assets_dir, PlanningCore)?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 99999)
        .build(game_data)?;
    game.run();
    Ok(())
}
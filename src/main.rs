
pub mod config;
pub mod ui;

use amethyst::{
    controls::{FlyControlBundle, FlyControlTag},
    core::{
        math::{Point3, Vector3},
        transform::{Transform, TransformBundle},
        Time,
    },
    derive::SystemDesc,
    ecs::{Read, System, SystemData, WorldExt, Write},
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        camera::Camera,
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
        plugins::{RenderDebugLines, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    winit::VirtualKeyCode,
};


use amethyst_imgui::RenderImgui;
use amethyst::core::SystemBundle;


#[derive(SystemDesc)]
struct ExampleLinesSystem;

impl<'s> System<'s> for ExampleLinesSystem {
    type SystemData = (
        Write<'s, DebugLines>, // Request DebugLines resource
        Read<'s, Time>,
    );

    fn run(&mut self, (mut debug_lines_resource, time): Self::SystemData) {
        // Drawing debug lines as a resource
        let t = (time.absolute_time_seconds() as f32).cos();

        debug_lines_resource.draw_direction(
            Point3::new(t, 0.0, 0.5),
            Vector3::new(0.0, 0.3, 0.0),
            Srgba::new(0.5, 0.05, 0.65, 1.0),
        );

        debug_lines_resource.draw_line(
            Point3::new(t, 0.0, 0.5),
            Point3::new(0.0, 0.0, 0.2),
            Srgba::new(0.5, 0.05, 0.65, 1.0),
        );
    }
}

struct PlanningCore;
impl SimpleState for PlanningCore {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        // Setup debug lines as a resource
        data.world.insert(DebugLines::new());
        // Configure width of lines. Optional step
        data.world.insert(DebugLinesParams { line_width: 2.0 });

        // Setup debug lines as a component and add lines to render axes & grid
        let mut debug_lines_component = DebugLinesComponent::with_capacity(100);

        // X-axis (red)
        debug_lines_component.add_direction(
            Point3::new(0.0, 0.0001, 0.0),
            Vector3::new(0.2, 0.0, 0.0),
            Srgba::new(1.0, 0.0, 0.23, 1.0),
        );

        // Y-axis (yellowish-green)
        debug_lines_component.add_direction(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.2, 0.0),
            Srgba::new(0.5, 0.85, 0.1, 1.0),
        );

        // Z-axis (blue)
        debug_lines_component.add_direction(
            Point3::new(0.0, 0.0001, 0.0),
            Vector3::new(0.0, 0.0, 0.2),
            Srgba::new(0.2, 0.75, 0.93, 1.0),
        );

        let width: u32 = 10;
        let depth: u32 = 10;
        let main_color = Srgba::new(0.4, 0.4, 0.4, 1.0);

        // Grid lines in X-axis
        for x in 0..=width {
            let (x, width, depth) = (x as f32, width as f32, depth as f32);

            let position = Point3::new(x - width / 2.0, 0.0, -depth / 2.0);
            let direction = Vector3::new(0.0, 0.0, depth);

            debug_lines_component.add_direction(position, direction, main_color);

            // Sub-grid lines
            if (x - width).abs() < 0.0001 {
                for sub_x in 1..10 {
                    let sub_offset = Vector3::new((1.0 / 10.0) * sub_x as f32, -0.001, 0.0);

                    debug_lines_component.add_direction(
                        &position + sub_offset,
                        direction,
                        Srgba::new(0.1, 0.1, 0.1, 0.2),
                    );
                }
            }
        }

        // Grid lines in Z-axis
        for z in 0..=depth {
            let (z, width, depth) = (z as f32, width as f32, depth as f32);

            let position = Point3::new(-width / 2.0, 0.0, z - depth / 2.0);
            let direction = Vector3::new(width, 0.0, 0.0);

            debug_lines_component.add_direction(position, direction, main_color);

            // Sub-grid lines
            if (z - depth).abs() < 0.0001 {
                for sub_z in 1..10 {
                    let sub_offset = Vector3::new(0.0, -0.001, (1.0 / 10.0) * sub_z as f32);

                    debug_lines_component.add_direction(
                        &position + sub_offset,
                        direction,
                        Srgba::new(0.1, 0.1, 0.1, 0.2),
                    );
                }
            }
        }

        // Debug lines are automatically rendered by including the debug lines
        // rendering plugin
        data.world.register::<DebugLinesComponent>();
        data.world
            .create_entity()
            .with(debug_lines_component)
            .build();

        // Setup camera
        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(0.0, 0.5, 2.0);
        data.world
            .create_entity()
            .with(FlyControlTag)
            .with(Camera::perspective(
                1.33333,
                std::f32::consts::FRAC_PI_2,
                0.1,
            ))
            .with(local_transform)
            .build();
    }

    fn handle_event(&mut self, _: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
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
    )
        .with_sensitivity(0.1, 0.1)

        ;

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
        .with(ExampleLinesSystem, "example_lines_system", &[])
        .with( ui::principal_window_ui::UIPlanningEngine::default(), "imgui_use", &[])
        .with_bundle(fly_control_bundle)?
        .with_bundle(TransformBundle::new().with_dep(&["mouse_focus"]))?

        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?)
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderSkybox::default())
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?;

    let mut game = Application::build(assets_dir, PlanningCore)?.build(game_data)?;
    game.run();
    Ok(())
}
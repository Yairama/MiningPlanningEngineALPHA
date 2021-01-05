use amethyst::shred::World;
use amethyst::core::Transform;
use amethyst::core::ecs::{WorldExt, Builder};
use amethyst::controls::FlyControlTag;
use amethyst::renderer::Camera;

pub fn set_up_camera(world: &mut World){
    // Setup camera
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(0.0, 0.5, 2.0);
    world
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
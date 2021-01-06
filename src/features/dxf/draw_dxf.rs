use amethyst::shred::World;

use dxf::{
    Drawing, entities::*
};

pub fn draw_dxf(dir: &String){

    let dxf_file= Drawing::load_file(dir).unwrap();

    for e in dxf_file.entities {
        println!("found entity on layer {}", e.common.layer);
        match e.specific {

            EntityType::Circle(ref circle) => {

            },
            EntityType::Line(ref line) => {
                // do something with the line
            },
            _ => (),
        }
    }

}
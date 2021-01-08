use amethyst::shred::World;

use dxf::{
    Drawing, entities::*
};
use amethyst::core::math::{Point3, Point};
use crate::components::DXFNodes;
use amethyst::renderer::debug_drawing::DebugLinesComponent;
use amethyst::renderer::palette::Srgba;
use std::cmp;
use amethyst::core::ecs::{Entities, WriteStorage};

pub fn draw_dxf(dir: &String, entities: &Entities, dxf_nodes: &mut WriteStorage<DXFNodes>, debug_lines_component: &mut WriteStorage<DebugLinesComponent>){

    let dxf_file= Drawing::load_file(&dir).unwrap();
    let mut p_entities: Vec<Vec<Point3<f32>>> = Vec::new();
    let mut x_min: f32 = f32::MAX;
    let mut y_min: f32 = f32::MAX;
    let mut z_min: f32 = f32::MAX;
    for e in dxf_file.entities {

        match e.specific {

            EntityType::Circle(ref circle) => {

            },
            EntityType::LwPolyline(ref lw_p_line) => {
                let mut line: Vec<Point3<f32>> = Vec::new();
                let ver = &lw_p_line.vertices;
                for v in ver {
                    line.push(Point3::new(v.x as f32,v.bulge as f32,v.y as f32));
                    x_min = f32::min(x_min,v.x as f32);
                    y_min = f32::min(y_min, v.y as f32);
                    z_min = f32::min(z_min,v.bulge as f32);
                    //println!("lengs {} {} {}",v.x,v.y,v.bulge);
                }
                p_entities.push(line);
            },
            EntityType::Polyline(ref p_line) => {
                let mut line: Vec<Point3<f32>> = Vec::new();
                let ver = &p_line.vertices;
                for v in ver {
                    line.push(Point3::new(v.location.x as f32,v.location.z as f32,v.location.y as f32));
                    x_min = f32::min(x_min,v.location.x as f32);
                    y_min = f32::min(y_min, v.location.y as f32);
                    z_min = f32::min(z_min,v.location.z as f32);
                    //println!("lengs {} {} {}",v.location.x,v.location.y,v.location.z);
                }
                p_entities.push(line);
            },
            _ => (println!("Some entities are not supported")),
        }

    }
    println!("awa de uwu {}",dxf_nodes.count());


    let mut lines = DebugLinesComponent::with_capacity(1000000);

    for layer in &p_entities{
        let mut count =0usize;

        loop{
            let v1 = &layer[&count.clone()+0];
            let v2 = &layer[&count.clone()+1];
            //println!("lengs {} {} {}",v1.x,v1.y,v1.z);
            lines.add_line(
                Point3::new(v1.x - x_min, v1.y-z_min, v1.z-y_min),
                Point3::new(v2.x-x_min, v2.y-z_min, v2.z-y_min),
                Srgba::new(1.0, 0.0, 0.23, 1.0),
            );
            count +=1;
            if count == layer.len()-1{
                break;
            }
        }

    }
    println!("lengs {} {} {}",x_min,y_min,z_min);

    let dxf_node = DXFNodes::new(dir.to_string(), p_entities.clone(),Point3::new(x_min,y_min,z_min));
    let new_entity = entities.create();
    debug_lines_component.insert(new_entity,lines);
    dxf_nodes.insert(new_entity,dxf_node);
    println!("awa de uwu {}",dxf_nodes.count());

}
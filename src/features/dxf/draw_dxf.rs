use amethyst::shred::World;

use dxf::{
    Drawing, entities::*
};
use amethyst::core::math::{Point3, Point};
use specs::{Entities, WriteStorage};
use crate::components::DXFNodes;
use amethyst::renderer::debug_drawing::DebugLinesComponent;
use amethyst::renderer::palette::Srgba;

pub fn draw_dxf(dir: &String, entities: &Entities, dxf_nodes: &mut WriteStorage<DXFNodes>, debug_lines_component: &mut WriteStorage<DebugLinesComponent>){

    let dxf_file= Drawing::load_file(&dir).unwrap();
    let mut p_entities: Vec<Vec<Point3<f64>>> = Vec::new();
    for e in dxf_file.entities {

        match e.specific {

            EntityType::Circle(ref circle) => {

            },
            EntityType::LwPolyline(ref lw_p_line) => {
                let mut line: Vec<Point3<f64>> = Vec::new();
                let ver = &lw_p_line.vertices;
                for v in ver {
                    line.push(Point3::new(v.x-17400.0,v.bulge-4200.0,v.y-28200.0));
                    println!("lengs {} {} {}",v.x,v.y,v.bulge);
                }
                p_entities.push(line);
            },
            EntityType::Polyline(ref p_line) => {
                let mut line: Vec<Point3<f64>> = Vec::new();
                let ver = &p_line.vertices;
                for v in ver {
                    line.push(Point3::new(v.location.x-17400.0,v.location.z-4200.0,v.location.y-28200.0));
                    println!("lengs {} {} {}",v.location.x,v.location.y,v.location.z);
                }
                p_entities.push(line);
            },
            _ => (println!("Some entities are not supported")),
        }

    }
    println!("awa de uwu {}",dxf_nodes.count());
    let dxf_node = DXFNodes::new(dir.to_string(), p_entities.clone());
    let new_entity = entities.create();
    dxf_nodes.insert(new_entity,dxf_node);

    let mut lines = DebugLinesComponent::with_capacity(1000000);

    for layer in p_entities{
        let mut count =0usize;

        loop{
            let v1 = &layer[&count.clone()+0];
            let v2 = &layer[&count.clone()+1];
            lines.add_line(
                Point3::new(v1.x as f32, v1.y as f32, v1.z as f32),
                Point3::new(v2.x as f32, v2.y as f32, v2.z as f32),
                Srgba::new(1.0, 0.0, 0.23, 1.0),
            );
            count +=1;
            if count == layer.len()-1{
                println!("dasdasdasdas");
                break;
            }
        }


    }

    lines.add_line(
        Point3::new(-50.0, 0.0001, 0.0),
        Point3::new(50.0, 90.0, 30.0),
        Srgba::new(1.0, 0.0, 0.23, 1.0),
    );

    let ent2 = entities.create();
    debug_lines_component.insert(ent2,lines);

    println!("awa de uwu {}",dxf_nodes.count());

}
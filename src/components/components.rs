use amethyst::core::math::Point3;
use specs::{Component, DenseVecStorage};

pub struct DXFNodes {
    pub node_name: String,
    pub p_entities: Vec<Vec<Point3<f64>>>
}

impl DXFNodes{
    pub fn new(node_name: String, p_entities: Vec<Vec<Point3<f64>>>) -> DXFNodes {
        DXFNodes{
            node_name,p_entities
        }
    }
}

impl Component for DXFNodes{
    type Storage = DenseVecStorage<Self>;
}
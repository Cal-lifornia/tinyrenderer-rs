use crate::vec3::Point3;

pub struct BoundingBox {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

impl BoundingBox {
    pub fn new(p1: &Point3, p2: &Point3, p3: &Point3) -> Self {
        Self {
            min_x: p1.x().min(p2.x()).min(p3.x()) as usize,
            max_x: p1.x().max(p2.x()).max(p3.x()) as usize,
            min_y: p1.y().min(p2.y()).min(p3.y()) as usize,
            max_y: p1.y().max(p2.y()).max(p3.y()) as usize,
        }
    }
}

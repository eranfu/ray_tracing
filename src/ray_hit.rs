use crate::*;

#[derive(Default, Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub position: Vector3,
    pub normal: Vector3,
    pub material: Option<&'a Box<Material>>,
}

pub trait Hittable {
    fn hit<'a, 'b: 'a>(
        &'b self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord<'a>,
    ) -> bool;
}

impl<THittable> Hittable for Vec<Box<THittable>>
where
    THittable: Hittable,
{
    fn hit<'a, 'b: 'a>(
        &'b self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord<'a>,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hit_ele in self {
            if hit_ele.hit(ray, t_min, closest_so_far, hit_record) {
                closest_so_far = hit_record.t;
                hit_anything = true;
            }
        }
        hit_anything
    }
}
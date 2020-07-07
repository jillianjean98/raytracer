pub mod hittable_list {
    use crate::objects;
    use crate::utils::ray::Ray;

    //#[derive(Copy, Clone)]
    pub struct HittableList {
        objects: Vec<Box<dyn objects::hittable::Hittable>>
    }

    impl HittableList {
        pub fn new() -> HittableList {
            HittableList {
                objects: Vec::new()
            }
        }

        pub fn new_init(objects: Vec<Box<dyn objects::hittable::Hittable>>) -> HittableList {
            HittableList {
                objects
            }
        }

        pub fn clear(&mut self) {
            self.objects.clear();
        }

        pub fn add(&mut self, obj: Box<dyn objects::hittable::Hittable>) {
            self.objects.push(obj);
        }
    }

    impl objects::hittable::Hittable for HittableList {
        fn hit(&self, r : Ray, t_min: f64, t_max: f64, mut rec: &mut objects::hittable::HitRecord) -> bool {
            let mut tmp_rec = objects::hittable::HitRecord::new_default();
            let mut hit_anything = false;
            let mut closest_so_far = t_max;
            let mut curr_obj = &self.objects[0];
            for obj in &self.objects {
                if obj.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                    hit_anything = true;
                    closest_so_far = tmp_rec.get_t();
                    curr_obj = obj;
                }
            }
            curr_obj.hit(r, t_min, closest_so_far+1.0, &mut rec);
            hit_anything
        }
    }
}
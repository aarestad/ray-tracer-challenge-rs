use ray_tracer_challenge_rs::camera::Camera;
use ray_tracer_challenge_rs::canvas::{Canvas, Ppm};
use ray_tracer_challenge_rs::color::Color;
use ray_tracer_challenge_rs::intersection::{Intersection, Intersections};
use ray_tracer_challenge_rs::light::PointLight;
use ray_tracer_challenge_rs::material::Material;
use ray_tracer_challenge_rs::objects::Object;
use ray_tracer_challenge_rs::patterns::Pattern;
use ray_tracer_challenge_rs::precompute::Precompute;
use ray_tracer_challenge_rs::ray::Ray;
use ray_tracer_challenge_rs::transforms::Transform;
use ray_tracer_challenge_rs::tuple::{Point, Tuple, Vector};
use ray_tracer_challenge_rs::world::World;

use std::collections::HashMap;
use std::rc::Rc;

use nalgebra::DMatrix;

use crate::RayTracerFloat;

#[derive(Debug, Default, cucumber::World)]
pub struct RayTracerWorld {
    pub canvases: HashMap<String, Canvas>,
    pub colors: HashMap<String, Color>,
    pub ppms: HashMap<String, Ppm>,
    pub objects: HashMap<String, Rc<dyn Object>>,
    pub intersections: HashMap<String, Rc<Intersection>>,
    // lol
    pub intersectionses: HashMap<String, Rc<Intersections>>,
    pub matrices: HashMap<String, DMatrix<RayTracerFloat>>,
    pub tuples: HashMap<String, Tuple>,
    pub rays: HashMap<String, Ray>,
    pub transforms: HashMap<String, Transform>,
    pub lights: HashMap<String, PointLight>,
    pub materials: HashMap<String, Rc<Material>>,
    pub worlds: HashMap<String, Rc<World>>,
    pub precomps: HashMap<String, Precompute>,
    pub cameras: HashMap<String, Camera>,
    pub patterns: HashMap<String, Rc<dyn Pattern>>,
}

// TODO this seems like a job for... a macro!
impl RayTracerWorld {
    pub fn get_canvas_or_panic(&self, canvas_name: &String) -> &Canvas {
        self.canvases
            .get(canvas_name)
            .unwrap_or_else(|| panic!("missing canvas named {}", canvas_name))
    }

    pub fn get_mut_canvas_or_panic(&mut self, canvas_name: &String) -> &mut Canvas {
        self.canvases
            .get_mut(canvas_name)
            .unwrap_or_else(|| panic!("missing canvas named {}", canvas_name))
    }

    pub fn get_color_or_panic(&self, color_name: &String) -> &Color {
        self.colors
            .get(color_name)
            .unwrap_or_else(|| panic!("missing color named {}", color_name))
    }

    pub fn get_ppm_or_panic(&self, ppm_name: &String) -> &Ppm {
        self.ppms
            .get(ppm_name)
            .unwrap_or_else(|| panic!("missing PPM named {}", ppm_name))
    }

    pub fn get_object_or_panic(&self, object_name: &String) -> &Rc<dyn Object> {
        self.objects
            .get(object_name)
            .unwrap_or_else(|| panic!("missing object named {}", object_name))
    }

    pub fn get_optional_int(&self, int_name: &String) -> Option<&Rc<Intersection>> {
        self.intersections.get(int_name)
    }

    pub fn get_ints_or_panic(&self, ints_name: &String) -> &Rc<Intersections> {
        self.intersectionses
            .get(ints_name)
            .unwrap_or_else(|| panic!("missing intersections {}", ints_name))
    }

    pub fn get_matrix_or_panic(&self, matrix_name: &String) -> &DMatrix<RayTracerFloat> {
        self.matrices
            .get(matrix_name)
            .unwrap_or_else(|| panic!("missing array {}", matrix_name))
    }

    pub fn get_tuple_or_panic(&self, v: &String) -> &Tuple {
        self.tuples
            .get(v)
            .unwrap_or_else(|| panic!("missing vector {}", v))
    }

    pub fn get_point_or_panic(&self, p: &String) -> &Point {
        self.tuples
            .get(p)
            .unwrap_or_else(|| panic!("missing point {}", p))
    }

    pub fn get_vector_or_panic(&self, v: &String) -> &Vector {
        self.tuples
            .get(v)
            .unwrap_or_else(|| panic!("missing vector {}", v))
    }

    pub fn get_ray_or_panic(&self, ray_name: &String) -> &Ray {
        self.rays
            .get(ray_name)
            .unwrap_or_else(|| panic!("missing ray {}", ray_name))
    }

    pub fn get_transform_or_panic(&self, matrix_name: &String) -> &Transform {
        self.transforms
            .get(matrix_name)
            .unwrap_or_else(|| panic!("missing transform {}", matrix_name))
    }

    pub fn get_light_or_panic(&self, light_name: &String) -> &PointLight {
        self.lights
            .get(light_name)
            .unwrap_or_else(|| panic!("missing light {}", light_name))
    }

    pub fn get_material_or_panic(&self, material_name: &String) -> &Rc<Material> {
        self.materials
            .get(material_name)
            .unwrap_or_else(|| panic!("missing material {}", material_name))
    }

    pub fn get_world_or_panic(&self, world_name: &String) -> &Rc<World> {
        self.worlds
            .get(world_name)
            .unwrap_or_else(|| panic!("missing world {}", world_name))
    }

    pub fn get_precomp_or_panic(&self, precomp_name: &String) -> &Precompute {
        self.precomps
            .get(precomp_name)
            .unwrap_or_else(|| panic!("missing precompute {}", precomp_name))
    }

    pub fn get_camera_or_panic(&self, camera_name: &String) -> &Camera {
        self.cameras
            .get(camera_name)
            .unwrap_or_else(|| panic!("missing camera named {}", camera_name))
    }

    pub fn get_pattern_or_panic(&self, pattern_name: &String) -> &Rc<dyn Pattern> {
        self.patterns
            .get(pattern_name)
            .unwrap_or_else(|| panic!("missing pattern named {}", pattern_name))
    }
}

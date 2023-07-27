use ray_tracer_challenge_rs::canvas::Canvas;
use ray_tracer_challenge_rs::color::Color;
use ray_tracer_challenge_rs::intersection::{Intersection, Intersections};
use ray_tracer_challenge_rs::light::PointLight;
use ray_tracer_challenge_rs::material::Material;
use ray_tracer_challenge_rs::objects::Sphere;
use ray_tracer_challenge_rs::ppm::Ppm;
use ray_tracer_challenge_rs::ray::Ray;
use ray_tracer_challenge_rs::world::World;
use std::collections::HashMap;

use nalgebra::{DMatrix, Matrix4};
use ray_tracer_challenge_rs::tuple::Tuple;

#[derive(Debug, Default, cucumber::World)]
pub struct RayTracerWorld {
    pub canvases: HashMap<String, Canvas>,
    pub colors: HashMap<String, Color>,
    pub ppms: HashMap<String, Ppm>,
    pub spheres: HashMap<String, Sphere>,
    pub intersections: HashMap<String, Intersection>,
    // lol
    pub intersectionses: HashMap<String, Intersections>,
    pub matrices: HashMap<String, DMatrix<f32>>,
    pub tuples: HashMap<String, Tuple>,
    pub rays: HashMap<String, Ray>,
    pub transforms: HashMap<String, Matrix4<f32>>,
    pub lights: HashMap<String, PointLight>,
    pub materials: HashMap<String, Material>,
    pub worlds: HashMap<String, World>,
}

// TODO this seems like a job for... a macro!
impl RayTracerWorld {
    pub fn get_canvas_or_panic(&self, canvas_name: &String) -> &Canvas {
        self.canvases
            .get(canvas_name)
            .expect(format!("missing canvas named {}", canvas_name).as_str())
    }

    pub fn get_mut_canvas_or_panic(&mut self, canvas_name: &String) -> &mut Canvas {
        self.canvases
            .get_mut(canvas_name)
            .expect(format!("missing canvas named {}", canvas_name).as_str())
    }

    pub fn get_color_or_panic(&self, color_name: &String) -> &Color {
        self.colors
            .get(color_name)
            .expect(format!("missing color named {}", color_name).as_str())
    }

    pub fn get_ppm_or_panic(&self, ppm_name: &String) -> &Ppm {
        self.ppms
            .get(ppm_name)
            .expect(format!("missing PPM named {}", ppm_name).as_str())
    }

    pub fn get_sphere_or_panic(&self, sphere_name: &String) -> &Sphere {
        self.spheres
            .get(sphere_name)
            .expect(format!("missing sphere named {}", sphere_name).as_str())
    }

    pub fn get_optional_int(&self, int_name: &String) -> Option<&Intersection> {
        self.intersections.get(int_name)
    }

    pub fn get_ints_or_panic(&self, ints_name: &String) -> &Intersections {
        self.intersectionses
            .get(ints_name)
            .expect(format!("missing intersections {}", ints_name).as_str())
    }

    pub fn get_matrix_or_panic(&self, matrix_name: &String) -> &DMatrix<f32> {
        self.matrices
            .get(matrix_name)
            .expect(format!("missing array {}", matrix_name).as_str())
    }

    pub fn get_tuple_or_panic(&self, tuple_name: &String) -> &Tuple {
        self.tuples
            .get(tuple_name)
            .expect(format!("missing tuple {}", tuple_name).as_str())
    }

    pub fn get_ray_or_panic(&self, ray_name: &String) -> &Ray {
        self.rays
            .get(ray_name)
            .expect(format!("missing ray {}", ray_name).as_str())
    }

    pub fn get_transform_or_panic(&self, matrix_name: &String) -> &Matrix4<f32> {
        self.transforms
            .get(matrix_name)
            .expect(format!("missing transform {}", matrix_name).as_str())
    }

    pub fn get_light_or_panic(&self, light_name: &String) -> &PointLight {
        self.lights
            .get(light_name)
            .expect(format!("missing light {}", light_name).as_str())
    }

    pub fn get_material_or_panic(&self, material_name: &String) -> &Material {
        self.materials
            .get(material_name)
            .expect(format!("missing material {}", material_name).as_str())
    }

    pub fn get_world_or_panic(&self, world_name: &String) -> &World {
        self.worlds
            .get(world_name)
            .expect(format!("missing world {}", world_name).as_str())
    }
}

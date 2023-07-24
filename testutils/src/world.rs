use cucumber::World;
use std::collections::HashMap;

use ray_tracer_challenge_rs::canvas::Canvas;
use ray_tracer_challenge_rs::color::Color;
use ray_tracer_challenge_rs::intersection::{Intersection, Intersections};
use ray_tracer_challenge_rs::objects::Sphere;
use ray_tracer_challenge_rs::ppm::Ppm;
use ray_tracer_challenge_rs::ray::Ray;

use nalgebra::{DMatrix, Matrix4};
use ray_tracer_challenge_rs::tuple::Tuple;

#[derive(Debug, Default, World)]
pub struct RayTracerWorld {
    pub canvases: HashMap<String, Canvas>,
    pub colors: HashMap<String, Color>,
    pub ppms: HashMap<String, Ppm>,
    pub spheres: HashMap<String, Sphere>,
    pub intersections: HashMap<String, Option<Intersection>>,
    // lol
    pub intersectionses: HashMap<String, Intersections>,
    pub matrices: HashMap<String, DMatrix<f32>>,
    pub tuples: HashMap<String, Tuple>,
    pub rays: HashMap<String, Ray>,
    pub transforms: HashMap<String, Matrix4<f32>>,
}

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

    pub fn get_int_or_panic(&self, int_name: &String) -> Option<Intersection> {
        let int = self
            .intersections
            .get(int_name)
            .expect(format!("missing intersection named {}", int_name).as_str());

        if let Some(i) = int {
            Some(Intersection::new(i.t, i.object.clone()))
        } else {
            None
        }
    }

    pub fn get_ints_or_panic(&self, ints_name: &String) -> &Intersections {
        self.intersectionses
            .get(ints_name)
            .expect(format!("missing intersections named {}", ints_name).as_str())
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
            .expect(format!("missing ray named {}", ray_name).as_str())
    }

    pub fn get_transform_or_panic(&self, matrix_name: &String) -> &Matrix4<f32> {
        self.transforms
            .get(matrix_name)
            .expect(format!("missing transform named {}", matrix_name).as_str())
    }
}

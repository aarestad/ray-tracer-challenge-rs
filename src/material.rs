use approx::{abs_diff_eq, AbsDiffEq};

use crate::{
    color::{Color, BLACK},
    light::PointLight,
    pattern::{Pattern, Solid},
    tuple::{Point, Vector},
    util::RayTracerFloat,
};

use crate::util::EPSILON;

pub struct MaterialBuilder {
    pattern: Solid,
    ambient: RayTracerFloat,
    diffuse: RayTracerFloat,
    specular: RayTracerFloat,
    shininess: RayTracerFloat,
}

impl Default for MaterialBuilder {
    fn default() -> Self {
        Self {
            pattern: Solid::new(Color::new(1., 1., 1.)),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }
}

impl MaterialBuilder {
    pub fn color(mut self, c: Color) -> Self {
        self.pattern = Solid::new(c);
        self
    }

    pub fn ambient(mut self, a: RayTracerFloat) -> Self {
        self.ambient = a;
        self
    }

    pub fn diffuse(mut self, d: RayTracerFloat) -> Self {
        self.diffuse = d;
        self
    }

    pub fn specular(mut self, sp: RayTracerFloat) -> Self {
        self.specular = sp;
        self
    }

    pub fn shininess(mut self, sh: RayTracerFloat) -> Self {
        self.shininess = sh;
        self
    }

    pub fn build(&self) -> Material {
        Material {
            pattern: self.pattern,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub pattern: Solid,
    pub ambient: RayTracerFloat,
    pub diffuse: RayTracerFloat,
    pub specular: RayTracerFloat,
    pub shininess: RayTracerFloat,
}

impl Default for Material {
    fn default() -> Self {
        MaterialBuilder::default().build()
    }
}

impl AbsDiffEq for Material {
    type Epsilon = RayTracerFloat;

    fn default_epsilon() -> Self::Epsilon {
        EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.pattern, other.pattern, epsilon = epsilon)
            && abs_diff_eq!(self.ambient, other.ambient, epsilon = epsilon)
            && abs_diff_eq!(self.diffuse, other.diffuse, epsilon = epsilon)
            && abs_diff_eq!(self.specular, other.specular, epsilon = epsilon)
            && abs_diff_eq!(self.shininess, other.shininess, epsilon = epsilon)
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = self.pattern.color_at(&point) * light.intensity;

        // find the direction to the light source
        let lightv = (light.position - point).normalize();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot(&normalv);

        let (diffuse, specular) = if light_dot_normal < 0. || in_shadow {
            (BLACK, BLACK)
        } else {
            // compute the diffuse contribution
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            // ceflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = -lightv.reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            let specular = if reflect_dot_eye <= 0. {
                BLACK
            } else {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        // # Add the three contributions together to get the final shading
        ambient + diffuse + specular
    }
}

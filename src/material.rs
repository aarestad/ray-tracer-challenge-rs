use std::rc::Rc;

use crate::{
    color::{Color, BLACK},
    light::PointLight,
    objects::Object,
    patterns::Pattern,
    tuple::{Point, Vector},
    util::RayTracerFloat,
};

const REFRAC_VACUUM: RayTracerFloat = 1.0;
const REFRAC_AIR: RayTracerFloat = 1.00029;
const REFRAC_WATER: RayTracerFloat = 1.333;
const REFRAC_GLASS: RayTracerFloat = 1.52;
const REFRAC_DIAMOND: RayTracerFloat = 2.417;

pub struct MaterialBuilder {
    pattern: Pattern,
    ambient: RayTracerFloat,
    diffuse: RayTracerFloat,
    specular: RayTracerFloat,
    shininess: RayTracerFloat,
    reflective: RayTracerFloat,
    transparency: RayTracerFloat,
    refractive: RayTracerFloat,
}

impl Default for MaterialBuilder {
    fn default() -> Self {
        Self {
            pattern: Pattern::Solid(Color::new(1., 1., 1.)),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            reflective: 0.,
            transparency: 0.,
            refractive: 1.,
        }
    }
}

impl MaterialBuilder {
    pub fn color(mut self, c: Color) -> Self {
        self.pattern = Pattern::Solid(c);
        self
    }

    pub fn pattern(mut self, p: Pattern) -> Self {
        self.pattern = p;
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

    pub fn reflective(mut self, re: RayTracerFloat) -> Self {
        self.reflective = re;
        self
    }

    pub fn transparency(mut self, tr: RayTracerFloat) -> Self {
        self.transparency = tr;
        self
    }

    pub fn refractive(mut self, ri: RayTracerFloat) -> Self {
        self.refractive = ri;
        self
    }

    pub fn build(self) -> Material {
        Material {
            pattern: self.pattern,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
            reflective: self.reflective,
            transparency: self.transparency,
            refractive: self.refractive,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: RayTracerFloat,
    pub diffuse: RayTracerFloat,
    pub specular: RayTracerFloat,
    pub shininess: RayTracerFloat,
    pub reflective: RayTracerFloat,
    pub transparency: RayTracerFloat,
    pub refractive: RayTracerFloat,
}

impl Default for Material {
    fn default() -> Self {
        MaterialBuilder::default().build()
    }
}

impl Material {
    pub fn from(other: &Material) -> Self {
        Self {
            pattern: other.pattern,
            ambient: other.ambient,
            diffuse: other.diffuse,
            specular: other.specular,
            shininess: other.shininess,
            reflective: other.reflective,
            transparency: other.transparency,
            refractive: other.refractive,
        }
    }

    pub fn lighting(
        &self,
        object: &Object,
        light: PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = self.pattern.color_at(object, &point) * light.intensity;

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

#[cfg(test)]
mod test {
    use super::MaterialBuilder;

    #[test]
    fn material_defaults() {
        let m = MaterialBuilder::default().build();
        assert_eq!(m.reflective, 0.);
        assert_eq!(m.transparency, 0.);
        assert_eq!(m.refractive, 1.);
    }
}

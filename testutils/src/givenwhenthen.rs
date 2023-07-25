use crate::{
    parameters::{Axis, SingleValue},
    world::RayTracerWorld,
};
use cucumber::{given, then, when};
use nalgebra::Matrix4;
use ray_tracer_challenge_rs::{
    canvas::Canvas,
    color::Color,
    intersection::Intersectable,
    light::PointLight,
    material::Material,
    objects::Sphere,
    ray::Ray,
    transforms::{rotation, scaling, translation},
    tuple::Tuple,
};

#[given(regex = r"^(\w+)\s*←\s*((tuple|point|vector)\(.+)$")]
fn given_a_tuple(world: &mut RayTracerWorld, tuple_name: String, tuple: Tuple) {
    world.tuples.insert(tuple_name, tuple);
}

#[given(expr = r"{word} ← canvas\({int}, {int}\)")]
fn given_a_canvas(world: &mut RayTracerWorld, name: String, width: usize, height: usize) {
    world.canvases.insert(name, Canvas::new(width, height));
}

#[given(expr = r"{word} ← color\({float}, {float}, {float}\)")]
fn given_a_color(world: &mut RayTracerWorld, name: String, r: f32, g: f32, b: f32) {
    world.colors.insert(name, Color::new(r, g, b));
}

#[given(expr = r"{word} ← material\(\)")]
fn given_default_material(world: &mut RayTracerWorld, name: String) {
    world.materials.insert(name, Material::default());
}

#[given(
    expr = r"{word} ← ray\(point\({float}, {float}, {float}\), vector\({float}, {float}, {float}\)\)"
)]
fn given_a_ray(
    world: &mut RayTracerWorld,
    ray_name: String,
    ox: f32,
    oy: f32,
    oz: f32,
    dx: f32,
    dy: f32,
    dz: f32,
) {
    world.rays.insert(
        ray_name,
        Ray::new(Tuple::point(ox, oy, oz), Tuple::vector(dx, dy, dz)),
    );
}

#[given(expr = r"{word} ← sphere\(\)")]
fn given_a_default_sphere(world: &mut RayTracerWorld, sphere_name: String) {
    world.spheres.insert(sphere_name, Sphere::default());
}

#[given(expr = r"{word} ← sphere\({word}\)")]
fn given_a_sphere_with_transform(
    world: &mut RayTracerWorld,
    sphere_name: String,
    transform_name: String,
) {
    let trans = world.get_transform_or_panic(&transform_name);
    world
        .spheres
        .insert(sphere_name, Sphere::new(*trans, Default::default()));
}

#[given(expr = r"{word} ← sphere\(default, {word}\)")]
fn given_a_sphere_with_default_transform_and_material(
    world: &mut RayTracerWorld,
    sphere_name: String,
    material_name: String,
) {
    world.spheres.insert(
        sphere_name,
        Sphere::new(Matrix4::identity(), Default::default()),
    );
}

#[given(expr = r"{word} ← translation\({float}, {float}, {float}\)")]
fn given_a_translation(world: &mut RayTracerWorld, trans_name: String, x: f32, y: f32, z: f32) {
    world.transforms.insert(trans_name, translation(x, y, z));
}

#[given(expr = r"{word} ← scaling\({float}, {float}, {float}\)")]
fn given_a_scaling(world: &mut RayTracerWorld, trans_name: String, x: f32, y: f32, z: f32) {
    world.transforms.insert(trans_name, scaling(x, y, z));
}

#[given(expr = r"{word} ← scaling\({float}, {float}, {float}\) * rotation_{axis}\({float}\)")]
fn given_rotation_scaling(
    world: &mut RayTracerWorld,
    trans_name: String,
    sx: f32,
    sy: f32,
    sz: f32,
    axis: Axis,
    rot: f32,
) {
    let s = scaling(sx, sy, sz);
    let r = rotation(axis.val(), rot);
    world.transforms.insert(trans_name, s * r);
}

#[given(expr = r"{word} ← intersect\({word}, {word}\)")]
#[when(expr = r"{word} ← intersect\({word}, {word}\)")]
fn when_ray_intersects_sphere(
    world: &mut RayTracerWorld,
    int_name: String,
    sphere_name: String,
    ray_name: String,
) {
    let sphere = world.get_sphere_or_panic(&sphere_name);
    let ray = world.get_ray_or_panic(&ray_name);
    world
        .intersectionses
        .insert(int_name, sphere.intersections(ray));
}

#[when(expr = r"{word} ← normal_at\({word}, point\({float}, {float}, {float}\)\)")]
fn when_sphere_normal_at(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere_name: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let s = world.get_sphere_or_panic(&sphere_name);
    let p = Tuple::point(x, y, z);
    world.tuples.insert(normal_name, s.normal_at(p));
}

#[when(expr = r"{word} ← reflect\({word}, {word}\)")]
fn when_reflection(
    world: &mut RayTracerWorld,
    reflection_name: String,
    vec_name: String,
    norm_name: String,
) {
    let v = world.get_tuple_or_panic(&vec_name);
    let n = world.get_tuple_or_panic(&norm_name);
    world.tuples.insert(reflection_name, v.reflect(n));
}

#[when(expr = r"{word} ← point_light\({word}, {word}\)")]
fn when_light_created(
    world: &mut RayTracerWorld,
    light_name: String,
    pos_name: String,
    intensity_name: String,
) {
    let p = world.get_tuple_or_panic(&pos_name);
    let i = world.get_color_or_panic(&intensity_name);
    world.lights.insert(light_name, PointLight::new(*p, *i));
}

#[when(expr = r"{word} ← {word}.material")]
fn when_material_from_sphere(world: &mut RayTracerWorld, mat_name: String, sphere_name: String) {
    let s = world.get_sphere_or_panic(&sphere_name);
    world.materials.insert(mat_name, s.material);
}

#[then(regex = r"^(\w+) = vector\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn assert_vector(world: &mut RayTracerWorld, vector_name: String, x: f32, y: f32, z: f32) {
    let actual = world.get_tuple_or_panic(&vector_name);
    let expected = Tuple::vector(x, y, z);

    assert!(
        actual.approx_eq(&expected),
        "expected {}, was {}",
        actual,
        expected
    );
}

#[then(expr = r"{word}.position = {word}")]
fn assert_light_position(world: &mut RayTracerWorld, light_name: String, pos_name: String) {
    let l = world.get_light_or_panic(&light_name);
    let p = world.get_tuple_or_panic(&pos_name);
    assert_eq!(&l.position, p);
}

#[then(expr = r"{word}.intensity = {word}")]
fn assert_light_intensity(world: &mut RayTracerWorld, light_name: String, color_name: String) {
    let l = world.get_light_or_panic(&light_name);
    let c = world.get_color_or_panic(&color_name);
    assert_eq!(&l.intensity, c);
}

#[then(expr = r"{word} = normalize\({word}\)")]
fn assert_vector_normalized(world: &mut RayTracerWorld, lhs_name: String, rhs_name: String) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&rhs_name);

    assert!(lhs.approx_eq(&rhs.normalize()));
}

#[then(expr = r"{word} = material\(\)")]
fn assert_default_material(world: &mut RayTracerWorld, mat_name: String) {
    let m = world.get_material_or_panic(&mat_name);
    assert_eq!(*m, Material::default());
}

#[then(expr = r"{word}.color = color\({float}, {float}, {float}\)")]
fn assert_mat_color(world: &mut RayTracerWorld, mat_name: String, r: f32, g: f32, b: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_eq!(m.color, Color::new(r, g, b));
}

#[then(expr = r"{word}.ambient = {float}")]
fn assert_mat_ambient(world: &mut RayTracerWorld, mat_name: String, expected: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_eq!(m.ambient, expected);
}

#[then(expr = r"{word}.diffuse = {float}")]
fn assert_mat_diffuse(world: &mut RayTracerWorld, mat_name: String, expected: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_eq!(m.diffuse, expected);
}

#[then(expr = r"{word}.specular = {float}")]
fn assert_mat_specular(world: &mut RayTracerWorld, mat_name: String, expected: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_eq!(m.specular, expected);
}

#[then(expr = r"{word}.shininess = {float}")]
fn assert_mat_shininess(world: &mut RayTracerWorld, mat_name: String, expected: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_eq!(m.shininess, expected);
}

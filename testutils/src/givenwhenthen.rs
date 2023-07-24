use crate::{
    parameters::{MathExpr, SingleValue},
    world::RayTracerWorld,
};
use cucumber::{given, then, when};
use ray_tracer_challenge_rs::{
    canvas::Canvas,
    color::Color,
    intersection::Intersectable,
    objects::Sphere,
    ray::Ray,
    transforms::{scaling, translation},
    tuple::Tuple,
};

#[given(expr = r"{word} ← canvas\({int}, {int}\)")]
fn given_a_canvas(world: &mut RayTracerWorld, name: String, width: usize, height: usize) {
    world.canvases.insert(name, Canvas::new(width, height));
}

#[given(expr = r"{word} ← color\({float}, {float}, {float}\)")]
fn given_a_color(world: &mut RayTracerWorld, name: String, r: f32, g: f32, b: f32) {
    world.colors.insert(name, Color::new(r, g, b));
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
    world.spheres.insert(sphere_name, Sphere::new(*trans));
}

#[given(expr = r"{word} ← translation\({float}, {float}, {float}\)")]
fn given_a_translation(world: &mut RayTracerWorld, trans_name: String, x: f32, y: f32, z: f32) {
    world.transforms.insert(trans_name, translation(x, y, z));
}

#[given(expr = r"{word} ← scaling\({float}, {float}, {float}\)")]
fn given_a_scaling(world: &mut RayTracerWorld, trans_name: String, x: f32, y: f32, z: f32) {
    world.transforms.insert(trans_name, scaling(x, y, z));
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

#[when(expr = r"{word} ← normal_at\({word}, point\({mathexpr}, {mathexpr}, {mathexpr}\)\)")]
fn when_sphere_normal_at(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere_name: String,
    x: MathExpr,
    y: MathExpr,
    z: MathExpr,
) {
    let s = world.get_sphere_or_panic(&sphere_name);
    let p = Tuple::point(x.val(), y.val(), z.val());
    world.tuples.insert(normal_name, s.normal_at(p));
}

#[then(expr = r"{word} = vector\({mathexpr}, {mathexpr}, {mathexpr}\)")]
fn assert_vector(
    world: &mut RayTracerWorld,
    vector_name: String,
    x: MathExpr,
    y: MathExpr,
    z: MathExpr,
) {
    let actual = world.get_tuple_or_panic(&vector_name);
    let expected = Tuple::vector(x.val(), y.val(), z.val());

    assert!(actual.approx_eq(&expected));
}

#[then(expr = r"{word} = normalize\({word}\)")]
fn assert_vector_normalized(world: &mut RayTracerWorld, lhs_name: String, rhs_name: String) {
    let lhs = world.get_tuple_or_panic(&lhs_name);
    let rhs = world.get_tuple_or_panic(&lhs_name);

    assert!(lhs.approx_eq(&rhs.normalize()));
}

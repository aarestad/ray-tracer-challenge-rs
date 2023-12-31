use crate::{step::get_4x4_matrix_from_step, world::RayTracerWorld, RayTracerFloat, EPSILON};
use cucumber::{gherkin::Step, then};
use ray_tracer_challenge_rs::{
    color::Color,
    material::Material,
    objects::default_sphere,
    patterns::Pattern,
    transforms::{identity, scaling, translation},
    tuple::{Point, Vector},
};

use std::str::FromStr;

use approx::assert_abs_diff_eq;

#[then(regex = r"^(\w+) = vector\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn assert_vector(
    world: &mut RayTracerWorld,
    vector_name: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let actual = world.get_vector_or_panic(&vector_name);
    let expected = Vector::vector(x, y, z);

    assert_abs_diff_eq!(actual, &expected);
}

#[then(expr = r"{word}.position = {word}")]
fn assert_light_position(world: &mut RayTracerWorld, light_name: String, pos_name: String) {
    let l = world.get_light_or_panic(&light_name);
    let p = world.get_point_or_panic(&pos_name);
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
    let lhs = world.get_vector_or_panic(&lhs_name);
    let rhs = world.get_vector_or_panic(&rhs_name);

    assert_abs_diff_eq!(lhs, &rhs.normalize());
}

#[then(regex = r"^(\w+) = material\(\)")]
fn assert_default_material(world: &mut RayTracerWorld, mat_name: String) {
    let m = world.get_material_or_panic(&mat_name);
    assert_eq!(m, &Material::default());
}

#[then(regex = r"^(\w+) = color\((.+), (.+), (.+)\)")]
fn assert_color(
    world: &mut RayTracerWorld,
    color: String,
    r: RayTracerFloat,
    g: RayTracerFloat,
    b: RayTracerFloat,
) {
    let c = world.get_color_or_panic(&color);
    assert_abs_diff_eq!(*c, Color::new(r, g, b));
}

#[then(regex = r"^(\w+).color = color\((.+), (.+), (.+)\)")]
fn assert_mat_color(
    world: &mut RayTracerWorld,
    mat_name: String,
    r: RayTracerFloat,
    g: RayTracerFloat,
    b: RayTracerFloat,
) {
    let m = world.get_material_or_panic(&mat_name);
    assert_eq!(m.pattern, Pattern::Solid(Color::new(r, g, b)));
}

#[then(expr = r"{word}.ambient = {float}")]
fn assert_mat_ambient(world: &mut RayTracerWorld, mat_name: String, expected: RayTracerFloat) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(m.ambient, expected);
}

#[then(expr = r"{word}.diffuse = {float}")]
fn assert_mat_diffuse(world: &mut RayTracerWorld, mat_name: String, expected: RayTracerFloat) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(m.diffuse, expected);
}

#[then(expr = r"{word}.specular = {float}")]
fn assert_mat_specular(world: &mut RayTracerWorld, mat_name: String, expected: RayTracerFloat) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(m.specular, expected);
}

#[then(expr = r"{word}.shininess = {float}")]
fn assert_mat_shininess(world: &mut RayTracerWorld, mat_name: String, expected: RayTracerFloat) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(m.shininess, expected);
}

// very specific re for this
#[then(regex = r"^w.light = light$")]
fn assert_world_light(world: &mut RayTracerWorld) {
    assert_eq!(
        &world.get_world_or_panic(&"w".into()).light_source,
        world.get_light_or_panic(&"light".into()),
    );
}

#[then(expr = r"{word} contains {word}")]
fn assert_world_contains_sphere(world: &mut RayTracerWorld, w: String, s: String) {
    let render_world = world.get_world_or_panic(&w);
    let sphere = world.get_object_or_panic(&s);

    assert!(render_world.objects.iter().any(|o| { o == sphere }));
}

#[then(regex = r"^(\w+)\.count = (\d+)$")]
fn assert_intersection_count(world: &mut RayTracerWorld, int_name: String, expected: usize) {
    let intersects = world.get_ints_or_panic(&int_name);

    assert_eq!(intersects.ints().len(), expected)
}

#[then(regex = r"^comps\.(\w+) = (.+)$")]
fn assert_precompute_property(world: &mut RayTracerWorld, prop_name: String, prop_expr: String) {
    let pc = world.get_precomp_or_panic(&"comps".to_string());

    match prop_name.as_str() {
        "t" => {
            let i_name = prop_expr.split('.').next().unwrap();
            let i = world.get_optional_int(&i_name.to_string()).unwrap();
            assert_eq!(pc.t, i.t);
        }
        "object" => {
            let i_name = prop_expr.split('.').next().unwrap();
            let i = world.get_optional_int(&i_name.to_string()).unwrap();
            assert_eq!(pc.object, i.object);
        }
        "point" => {
            let p = Point::from_str(prop_expr.as_str()).unwrap();
            assert_eq!(pc.point, p);
        }
        "eyev" | "normalv" => {
            let v = Vector::from_str(prop_expr.as_str()).unwrap();

            if prop_name == "eyev" {
                assert_eq!(pc.eyev, v);
            } else {
                assert_eq!(pc.normalv, v);
            }
        }
        "inside" => {
            let expected = bool::from_str(prop_expr.as_str()).unwrap();
            assert_eq!(pc.inside, expected);
        }
        _ => panic!("bad prop name {}", prop_name),
    }
}

#[then(expr = r"{word} = {word}.material.color")]
fn assert_sphere_color(world: &mut RayTracerWorld, c: String, s: String) {
    let color = world.get_color_or_panic(&c);
    let sphere = world.get_object_or_panic(&s);

    assert_eq!(Pattern::Solid(*color), sphere.material.pattern);
}

#[then(regex = r"^(\w+) = identity_matrix$")]
fn assert_transform_identity(world: &mut RayTracerWorld, t: String) {
    let transform = world.get_transform_or_panic(&t);
    assert_eq!(*transform, identity());
}

#[then(expr = r"{word} = scaling\({float}, {float}, {float}\)")]
fn assert_transform_scaling(
    world: &mut RayTracerWorld,
    t: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let transform = world.get_transform_or_panic(&t);
    assert_eq!(*transform, scaling(x, y, z));
}

#[then(regex = r"^(\w+) = translation\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)")]
fn assert_transform_translation(
    world: &mut RayTracerWorld,
    t: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let transform = world.get_transform_or_panic(&t);
    assert_eq!(*transform, translation(x, y, z));
}

#[then(regex = r"^(\w+) is the following 4x4 matrix:")]
fn assert_transform_arbitrary(world: &mut RayTracerWorld, step: &Step, t: String) {
    let transform = world.get_transform_or_panic(&t);
    let expected = get_4x4_matrix_from_step(step);
    assert_abs_diff_eq!(*transform, expected, epsilon = EPSILON);
}

#[then(regex = r"^c.hsize = (\d+)")]
fn assert_camera_hsize(world: &mut RayTracerWorld, hsize: usize) {
    let c = world.get_camera_or_panic(&"c".to_string());
    assert_eq!(c.hsize, hsize);
}

#[then(regex = r"^c.vsize = (\d+)")]
fn assert_camera_vsize(world: &mut RayTracerWorld, vsize: usize) {
    let c = world.get_camera_or_panic(&"c".to_string());
    assert_eq!(c.vsize, vsize);
}

#[then(regex = r"^c.field_of_view = (\d+\.\d+)")]
fn assert_camera_fov(world: &mut RayTracerWorld, fov: RayTracerFloat) {
    let c = world.get_camera_or_panic(&"c".to_string());
    assert_eq!(c.field_of_view, fov);
}

#[then(regex = r"^c\.transform = identity_matrix$")]
fn assert_camera_transform(world: &mut RayTracerWorld) {
    let c = world.get_camera_or_panic(&"c".to_string());
    assert_eq!(c.transform, identity());
}

#[then(regex = r"^c.pixel_size = (-?\d+(?:\.\d+)?)")]
fn assert_camera_pixel_size(world: &mut RayTracerWorld, pixel_size: RayTracerFloat) {
    let c = world.get_camera_or_panic(&"c".to_string());
    assert_abs_diff_eq!(c.pixel_size, pixel_size, epsilon = EPSILON);
}

#[then(regex = r"^(\w+)\.origin = point\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)")]
fn assert_ray_origin(
    world: &mut RayTracerWorld,
    r: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let ray = world.get_ray_or_panic(&r);
    assert_abs_diff_eq!(ray.origin, Point::point(x, y, z));
}

#[then(
    regex = r"^(\w+)\.direction = vector\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)"
)]
fn assert_ray_direction(
    world: &mut RayTracerWorld,
    r: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let ray = world.get_ray_or_panic(&r);
    assert_abs_diff_eq!(ray.direction, Vector::vector(x, y, z));
}

#[then(
    regex = r"^pixel_at\((\w+), (\d+), (\d+)\) = color\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)"
)]
fn assert_pixel_at(
    world: &mut RayTracerWorld,
    canvas_name: String,
    x: usize,
    y: usize,
    r: RayTracerFloat,
    g: RayTracerFloat,
    b: RayTracerFloat,
) {
    let canvas = world.get_canvas_or_panic(&canvas_name);
    assert_abs_diff_eq!(Color::new(r, g, b), canvas.pixel_at(x, y));
}

#[then(expr = r"is_shadowed\({word}, {word}\) is {word}")]
fn assert_is_shadowed(world: &mut RayTracerWorld, w: String, p: String, is_shadowed: String) {
    let rt_world = world.get_world_or_panic(&w);
    let point = world.get_point_or_panic(&p);
    let expected = is_shadowed == "true";
    let actual = rt_world.is_shadowed(point);

    assert!(if expected { actual } else { !actual });
}

#[then(expr = r"{word}.over_point.z < -EPSILON\/2")]
fn assert_over_point_small(world: &mut RayTracerWorld, c: String) {
    let comps = world.get_precomp_or_panic(&c);
    assert!(comps.over_point.z() < -EPSILON / 2.);
}

#[then(expr = r"{word}.point.z > {word}.over_point.z")]
fn assert_point_z_gt_over_point(world: &mut RayTracerWorld, c: String) {
    let comps = world.get_precomp_or_panic(&c);
    assert!(comps.point.z() > comps.over_point.z());
}

#[then(regex = r"^s\.transform = (\w+)$")]
fn assert_object_transform_name(world: &mut RayTracerWorld, trans_name: String) {
    let o = world.get_object_or_panic(&"s".to_string());

    let t = if trans_name == "identity_matrix" {
        identity()
    } else {
        *world.get_transform_or_panic(&trans_name)
    };

    assert_eq!(o.transform, t)
}

#[then(
    regex = r"^s\.transform = translation\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)"
)]
fn assert_object_transform_translation(
    world: &mut RayTracerWorld,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let o = world.get_object_or_panic(&"s".to_string());

    let t = translation(x, y, z);

    assert_eq!(o.transform, t)
}

#[then(regex = r"^s\.material = material\(\)")]
fn assert_object_default_material(world: &mut RayTracerWorld) {
    let object = world.get_object_or_panic(&"s".to_string());
    assert_eq!(object.material, Material::default());
}

#[then(regex = r"^s\.material = (\w+)$")]
fn assert_object_named_material(world: &mut RayTracerWorld, m: String) {
    let object = world.get_object_or_panic(&"s".to_string());
    let material = world.get_material_or_panic(&m);
    assert_eq!(object.material, *material);
}

#[then(expr = r"{word} is empty")]
fn assert_empty_intersections(world: &mut RayTracerWorld, xs: String) {
    let ints = world.get_ints_or_panic(&xs);
    assert!(ints.ints().is_empty());
}

#[then(expr = r"{word}[{int}] = {float}")]
fn assert_nth_intersection(
    world: &mut RayTracerWorld,
    int_name: String,
    nth: usize,
    expected: RayTracerFloat,
) {
    let ints = world.get_ints_or_panic(&int_name);

    let actual = &ints.ints()[nth];

    assert_eq!(actual.t, expected);
}

#[then(regex = r"^(\w+)\[(\d)\]\.t = (.+)")]
fn assert_nth_intersection_t(
    world: &mut RayTracerWorld,
    int_name: String,
    nth: usize,
    expected: RayTracerFloat,
) {
    let ints = world.get_ints_or_panic(&int_name);

    let actual = &ints.ints()[nth];

    assert_eq!(actual.t, expected);
}

#[then(regex = r"^(\w+)\[(\d)\]\.object = (.+)")]
fn assert_nth_intersection_object(
    world: &mut RayTracerWorld,
    int_name: String,
    nth: usize,
    o: String,
) {
    let ints = world.get_ints_or_panic(&int_name);

    let actual = &ints.ints()[nth].object;
    let expected = world.get_object_or_panic(&o);

    assert_eq!(actual, expected);
}

#[then(expr = r"stripe_at\({word}, point\({float}, {float}, {float}\)\) = {word}")]
fn assert_stripe_color_at_point(
    world: &mut RayTracerWorld,
    p: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
    c: String,
) {
    let pattern = world.get_pattern_or_panic(&p);
    let point = Point::point(x, y, z);
    let color = world.get_color_or_panic(&c);
    assert_eq!(&pattern.color_at(&default_sphere(), &point), color);
}

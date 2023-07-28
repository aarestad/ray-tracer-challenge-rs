use crate::{
    parameters::{Axis, SingleValue},
    world::RayTracerWorld,
};
use cucumber::{gherkin::Step, given, then, when};
use nalgebra::Matrix4;
use ray_tracer_challenge_rs::{
    canvas::Canvas,
    color::Color,
    intersection::Intersection,
    light::PointLight,
    material::{Material, MaterialBuilder},
    objects::{Object, Sphere},
    ray::Ray,
    transforms::{identity, rotation, scaling, translation},
    tuple::Tuple,
    world::World,
};

use std::{rc::Rc, str::FromStr};

use regex::Regex;

use approx::assert_abs_diff_eq;

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
#[allow(clippy::too_many_arguments)]
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

#[given(expr = r"{word} ← intersection\({float}, {word}\)")]
#[when(expr = r"{word} ← intersection\({float}, {word}\)")]
fn when_intersection_created(
    world: &mut RayTracerWorld,
    int_name: String,
    t: f32,
    object_name: String,
) {
    let o = world.get_sphere_or_panic(&object_name);
    world
        .intersections
        .insert(int_name, Intersection::new(t, Rc::new(*o)));
}

#[given(expr = r"{word} ← sphere\(\)")]
fn given_a_default_sphere(world: &mut RayTracerWorld, sphere_name: String) {
    world.spheres.insert(sphere_name, Sphere::default());
}

fn parse_three_args(s: &str) -> (f32, f32, f32) {
    let three_args_re = Regex::new(r"\((.+), (.+), (.+)\)").unwrap();

    let args: Vec<f32> = three_args_re
        .captures(s)
        .unwrap()
        .iter()
        .skip(1)
        .map(|c| f32::from_str(c.unwrap().as_str()).unwrap())
        .collect();

    (args[0], args[1], args[2])
}

#[given(expr = r"{word} ← sphere\(\) with:")]
fn given_a_sphere(world: &mut RayTracerWorld, step: &Step, sphere_name: String) {
    let table = step.table().unwrap();

    let mut material_builder = MaterialBuilder::default();
    let mut transform = identity();

    let prop_re = Regex::new(r"\.(\w+)").unwrap();

    for row in table.rows.iter() {
        if let [prop, val] = &row[0..2] {
            match prop {
                _ if prop == "transform" => {
                    let args = parse_three_args(val);

                    match val {
                        _ if val.starts_with("scaling") => {
                            transform = scaling(args.0, args.1, args.2);
                        }
                        _ if val.starts_with("translation") => {
                            transform = translation(args.0, args.1, args.2);
                        }
                        _ => panic!("bad transform val: {}", val),
                    }
                }
                _ if prop.starts_with("material") => {
                    let mat_prop_name = &prop_re.captures(prop).unwrap()[1];

                    match mat_prop_name {
                        "color" => {
                            let args = parse_three_args(val);

                            material_builder =
                                material_builder.color(Color::new(args.0, args.1, args.2));
                        }
                        "diffuse" => {
                            material_builder =
                                material_builder.diffuse(f32::from_str(val).unwrap());
                        }
                        "specular" => {
                            material_builder =
                                material_builder.specular(f32::from_str(val).unwrap());
                        }
                        _ => panic!("bad mat prop: {}", mat_prop_name),
                    }
                }
                _ => panic!("bad prop: {}", prop),
            }
        } else {
            panic!("row too short");
        }
    }

    world.spheres.insert(
        sphere_name,
        Sphere::new(transform, material_builder.build()),
    );
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

#[given(
    expr = r"{word} ← point_light\(point\({float}, {float}, {float}\), color\({float}, {float}, {float}\)\)"
)]
#[allow(clippy::too_many_arguments)]
fn given_point_light(
    world: &mut RayTracerWorld,
    light: String,
    x: f32,
    y: f32,
    z: f32,
    r: f32,
    g: f32,
    b: f32,
) {
    let l = PointLight::new(Tuple::point(x, y, z), Color::new(r, g, b));

    if !light.contains('.') {
        world.lights.insert(light, l);
    } else {
        // if so, we're setting the light on a world
        let world_name = light.split('.').next().unwrap();
        let mut ray_world = world.get_mut_world_or_panic(&world_name.to_string());
        ray_world.light_source = l;
    }
}

#[given(expr = r"{word} ← the {word} object in {word}")]
fn given_nth_shape_in_world(world: &mut RayTracerWorld, name: String, nth: String, wn: String) {
    let ray_world = world.get_world_or_panic(&wn);

    let n = match nth.as_str() {
        "first" => 0,
        "second" => 1,
        _ => panic!("bad nth: {}", nth),
    };

    world
        .spheres
        .insert(name, *ray_world.objects[n].as_sphere());
}

#[given(expr = r"{word}.material.ambient ← {float}")]
fn given_sphere_ambient_val(world: &mut RayTracerWorld, s: String, ambient: f32) {
    let sphere = world.get_mut_sphere_or_panic(&s);
    sphere.material_mut().ambient = ambient;
}

#[given(expr = r"{word} ← default_world_with_objects\({word}, {word}\)")]
fn given_world_with_objects(world: &mut RayTracerWorld, w: String, s1: String, s2: String) {
    let o1 = world.get_sphere_or_panic(&s1);
    let o2 = world.get_sphere_or_panic(&s2);
    world.worlds.insert(
        w,
        World::default_world_with_objects(vec![Rc::new(*o1), Rc::new(*o2)]),
    );
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
    world.materials.insert(mat_name, *s.material());
}

#[when(expr = r"{word} ← lighting\({word}, {word}, {word}, {word}, {word}\)")]
fn when_lighting_material(
    world: &mut RayTracerWorld,
    result: String,
    mat: String,
    light: String,
    position: String,
    eyev: String,
    normalv: String,
) {
    let m = world.get_material_or_panic(&mat);
    let l = *world.get_light_or_panic(&light);
    let p = *world.get_tuple_or_panic(&position);
    let e = *world.get_tuple_or_panic(&eyev);
    let n = *world.get_tuple_or_panic(&normalv);

    world.colors.insert(result, m.lighting(l, p, e, n));
}

#[given(expr = r"{word} ← default_world\(\)")]
#[when(expr = r"{word} ← default_world\(\)")]
fn given_or_when_default_world(world: &mut RayTracerWorld, world_name: String) {
    world.worlds.insert(world_name, World::default_world());
}

#[given(expr = r"{word} ← intersect_world\({word}, {word}\)")]
#[when(expr = r"{word} ← intersect_world\({word}, {word}\)")]
fn when_ray_intersects_world(world: &mut RayTracerWorld, ints: String, w: String, r: String) {
    let rt_world = world.get_world_or_panic(&w);
    let ray = world.get_ray_or_panic(&r);
    world
        .intersectionses
        .insert(ints, rt_world.intersects_with(ray));
}

#[given(expr = r"{word} ← prepare_computations\({word}, {word}\)")]
#[when(expr = r"{word} ← prepare_computations\({word}, {word}\)")]
fn when_precomputing(world: &mut RayTracerWorld, pc: String, i: String, r: String) {
    let int = world.get_optional_int(&i).unwrap();
    let ray = world.get_ray_or_panic(&r);
    world.precomps.insert(pc, int.precompute_with(ray));
}

#[given(expr = r"{word} ← shade_hit\({word}, {word}\)")]
#[when(expr = r"{word} ← shade_hit\({word}, {word}\)")]
fn when_shade_hit(world: &mut RayTracerWorld, c: String, w: String, pc: String) {
    let ray_world = world.get_world_or_panic(&w);
    let precompute = world.get_precomp_or_panic(&pc);
    world.colors.insert(c, ray_world.shade_hit(precompute));
}

#[given(expr = r"{word} ← color_at\({word}, {word}\)")]
#[when(expr = r"{word} ← color_at\({word}, {word}\)")]
fn when_color_at(world: &mut RayTracerWorld, c: String, w: String, r: String) {
    let ray_world = world.get_world_or_panic(&w);
    let ray = world.get_ray_or_panic(&r);
    world.colors.insert(c, ray_world.color_at(ray));
}

#[given(expr = r"{word} ← hit\({word}\)")]
#[when(expr = r"{word} ← hit\({word}\)")]
fn when_hit_queried(world: &mut RayTracerWorld, hit_name: String, ints_name: String) {
    let i = world.get_ints_or_panic(&ints_name);
    let maybe_hit = i.hit();

    if let Some(i) = maybe_hit {
        world
            .intersections
            .insert(hit_name, Intersection::new(i.t, i.object.clone()));
    }
}

#[then(regex = r"^(\w+) = vector\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn assert_vector(world: &mut RayTracerWorld, vector_name: String, x: f32, y: f32, z: f32) {
    let actual = world.get_tuple_or_panic(&vector_name);
    let expected = Tuple::vector(x, y, z);

    assert_abs_diff_eq!(actual, &expected);
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

    assert_abs_diff_eq!(lhs, &rhs.normalize());
}

#[then(expr = r"{word} = material\(\)")]
fn assert_default_material(world: &mut RayTracerWorld, mat_name: String) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(*m, Material::default());
}

#[then(regex = r"^(\w+) = color\((.+), (.+), (.+)\)")]
fn assert_color(world: &mut RayTracerWorld, color: String, r: f32, g: f32, b: f32) {
    let c = world.get_color_or_panic(&color);
    assert_abs_diff_eq!(*c, Color::new(r, g, b));
}

#[then(regex = r"^(\w+).color = color\((.+), (.+), (.+)\)")]
fn assert_mat_color(world: &mut RayTracerWorld, mat_name: String, r: f32, g: f32, b: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(m.color, Color::new(r, g, b));
}

#[then(expr = r"{word}.ambient = {float}")]
fn assert_mat_ambient(world: &mut RayTracerWorld, mat_name: String, expected: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(m.ambient, expected);
}

#[then(expr = r"{word}.diffuse = {float}")]
fn assert_mat_diffuse(world: &mut RayTracerWorld, mat_name: String, expected: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(m.diffuse, expected);
}

#[then(expr = r"{word}.specular = {float}")]
fn assert_mat_specular(world: &mut RayTracerWorld, mat_name: String, expected: f32) {
    let m = world.get_material_or_panic(&mat_name);
    assert_abs_diff_eq!(m.specular, expected);
}

#[then(expr = r"{word}.shininess = {float}")]
fn assert_mat_shininess(world: &mut RayTracerWorld, mat_name: String, expected: f32) {
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
    let sphere = world.get_sphere_or_panic(&s);

    assert!(render_world
        .objects
        .iter()
        .any(|o| { o.as_sphere() == sphere }));
}

#[then(regex = r"^(\w+)\.count = (\d+)$")]
fn assert_intersection_count(world: &mut RayTracerWorld, int_name: String, expected: usize) {
    let intersects = world.get_ints_or_panic(&int_name);

    assert_eq!(intersects.ints().len(), expected)
}

#[then(regex = r"^(\w+)\[(\d)\]\.t = (.+)")]
fn assert_nth_intersection_t(
    world: &mut RayTracerWorld,
    int_name: String,
    nth: usize,
    expected: f32,
) {
    let ints = world.get_ints_or_panic(&int_name);

    let actual = &ints.ints()[nth];

    assert_eq!(actual.t, expected);
}

#[then(regex = r"^comps\.(\w+) = (.+)$")]
fn assert_precompute_property(world: &mut RayTracerWorld, prop_name: String, prop_expr: String) {
    let pc = world.get_precomp_or_panic(&"comps".to_string());

    match prop_name.as_str() {
        "t" => {
            let i_name = prop_expr.split('.').next().unwrap();
            let i = world.get_optional_int(&i_name.to_string()).unwrap();
            assert_eq!(pc.intersection.t, i.t);
        }
        "object" => {
            let i_name = prop_expr.split('.').next().unwrap();
            let i = world.get_optional_int(&i_name.to_string()).unwrap();
            assert_eq!(pc.intersection.object.as_sphere(), i.object.as_sphere());
        }
        "point" => {
            let p = Tuple::from_str(prop_expr.as_str()).unwrap();
            assert_eq!(pc.point, p);
        }
        "eyev" | "normalv" => {
            let v = Tuple::from_str(prop_expr.as_str()).unwrap();

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
    let sphere = world.get_sphere_or_panic(&s);

    assert_abs_diff_eq!(color, &sphere.material().color);
}

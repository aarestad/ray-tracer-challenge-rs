use crate::{
    parameters::{Axis, SingleValue},
    world::RayTracerWorld,
    RayTracerFloat,
};
use cucumber::{gherkin::Step, given, when};
use ray_tracer_challenge_rs::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    intersection::Intersection,
    light::PointLight,
    material::{Material, MaterialBuilder},
    objects::{Object, Sphere, TestShape},
    ray::Ray,
    transforms::{identity, rotation, scaling, translation},
    tuple::Tuple,
    world::World,
};

use std::{rc::Rc, str::FromStr};

use regex::Regex;

#[given(regex = r"^(\w+)\s*←\s*((tuple|point|vector)\(.+)$")]
fn given_a_tuple(world: &mut RayTracerWorld, tuple_name: String, tuple: Tuple) {
    world.tuples.insert(tuple_name, tuple);
}

#[given(expr = r"{word} ← canvas\({int}, {int}\)")]
fn given_a_canvas(world: &mut RayTracerWorld, name: String, width: usize, height: usize) {
    world.canvases.insert(name, Canvas::new(width, height));
}

#[given(expr = r"{word} ← color\({float}, {float}, {float}\)")]
fn given_a_color(
    world: &mut RayTracerWorld,
    name: String,
    r: RayTracerFloat,
    g: RayTracerFloat,
    b: RayTracerFloat,
) {
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
    ox: RayTracerFloat,
    oy: RayTracerFloat,
    oz: RayTracerFloat,
    dx: RayTracerFloat,
    dy: RayTracerFloat,
    dz: RayTracerFloat,
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
    t: RayTracerFloat,
    object_name: String,
) {
    let o = world.get_object_or_panic(&object_name);
    world
        .intersections
        .insert(int_name, Intersection::new(t, Rc::clone(o)));
}

#[given(expr = r"{word} ← sphere\(\)")]
fn given_a_default_sphere(world: &mut RayTracerWorld, sphere_name: String) {
    world
        .objects
        .insert(sphere_name, Rc::new(Sphere::default()));
}

fn parse_three_args(s: &str) -> (RayTracerFloat, RayTracerFloat, RayTracerFloat) {
    let three_args_re = Regex::new(r"\((.+), (.+), (.+)\)").unwrap();

    let args: Vec<RayTracerFloat> = three_args_re
        .captures(s)
        .unwrap()
        .iter()
        .skip(1)
        .map(|c| RayTracerFloat::from_str(c.unwrap().as_str()).unwrap())
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
                                material_builder.diffuse(RayTracerFloat::from_str(val).unwrap());
                        }
                        "specular" => {
                            material_builder =
                                material_builder.specular(RayTracerFloat::from_str(val).unwrap());
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

    world.objects.insert(
        sphere_name,
        Rc::new(Sphere::new(transform, material_builder.build())),
    );
}

#[given(expr = r"{word} ← sphere\({word}\)")]
fn given_a_sphere_with_transform(
    world: &mut RayTracerWorld,
    sphere_name: String,
    transform_name: String,
) {
    let trans = world.get_transform_or_panic(&transform_name);
    world.objects.insert(
        sphere_name,
        Rc::new(Sphere::new(*trans, Default::default())),
    );
}

#[given(expr = r"{word} ← sphere\(default, {word}\)")]
fn given_a_sphere_with_default_transform_and_material(
    world: &mut RayTracerWorld,
    sphere_name: String,
) {
    world.objects.insert(
        sphere_name,
        Rc::new(Sphere::new(identity(), Default::default())),
    );
}

#[given(expr = r"{word} ← translation\({float}, {float}, {float}\)")]
fn given_a_translation(
    world: &mut RayTracerWorld,
    trans_name: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    world.transforms.insert(trans_name, translation(x, y, z));
}

#[given(expr = r"{word} ← scaling\({float}, {float}, {float}\)")]
fn given_a_scaling(
    world: &mut RayTracerWorld,
    trans_name: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    world.transforms.insert(trans_name, scaling(x, y, z));
}

#[given(expr = r"{word} ← scaling\({float}, {float}, {float}\) * rotation_{axis}\({float}\)")]
fn given_rotation_scaling(
    world: &mut RayTracerWorld,
    trans_name: String,
    sx: RayTracerFloat,
    sy: RayTracerFloat,
    sz: RayTracerFloat,
    axis: Axis,
    rot: RayTracerFloat,
) {
    let s = scaling(sx, sy, sz);
    let r = rotation(axis.val(), rot);
    world.transforms.insert(trans_name, s * r);
}

#[given(expr = r"{word} ← rotation_{axis}\({float}\) * translation\({float}, {float}, {float}\)")]
fn given_rotation_translation(
    world: &mut RayTracerWorld,
    trans_name: String,
    axis: Axis,
    rot: RayTracerFloat,
    tx: RayTracerFloat,
    ty: RayTracerFloat,
    tz: RayTracerFloat,
) {
    let t = translation(tx, ty, tz);
    let r = rotation(axis.val(), rot);
    world.transforms.insert(trans_name, r * t);
}

#[given(
    expr = r"{word} ← point_light\(point\({float}, {float}, {float}\), color\({float}, {float}, {float}\)\)"
)]
#[allow(clippy::too_many_arguments)]
fn given_point_light(
    world: &mut RayTracerWorld,
    light: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
    r: RayTracerFloat,
    g: RayTracerFloat,
    b: RayTracerFloat,
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
fn given_nth_sphere_in_world(world: &mut RayTracerWorld, name: String, nth: String, wn: String) {
    let ray_world = world.get_world_or_panic(&wn);

    let n = match nth.as_str() {
        "first" => 0,
        "second" => 1,
        _ => panic!("bad nth: {}", nth),
    };

    world
        .objects
        .insert(name, Rc::new(*ray_world.objects[n].as_sphere()));
}

#[given(expr = r"{word}.material.ambient ← {float}")]
fn given_sphere_ambient_val(world: &mut RayTracerWorld, s: String, ambient: RayTracerFloat) {
    let sphere = world.get_object_or_panic(&s);
    let mut mat = sphere.material().clone();
    mat.ambient = ambient;

    let new_sphere = Sphere::new(*sphere.as_ref().transform(), mat);
    world.objects.insert(s, Rc::new(new_sphere));
}

#[given(expr = r"{word} ← default_world_with_objects\({word}, {word}\)")]
fn given_world_with_objects(world: &mut RayTracerWorld, w: String, s1: String, s2: String) {
    let o1 = world.get_object_or_panic(&s1);
    let o2 = world.get_object_or_panic(&s2);
    world.worlds.insert(
        w,
        World::default_world_with_objects(vec![Rc::clone(o1), Rc::clone(o2)]),
    );
}

#[given(expr = r"{word} ← camera\({int}, {int}, {float}\)")]
fn given_a_camera_identity(
    world: &mut RayTracerWorld,
    c: String,
    hsize: usize,
    vsize: usize,
    fov: RayTracerFloat,
) {
    world
        .cameras
        .insert(c, Camera::new(hsize, vsize, fov, identity()));
}

#[given(expr = r"{word} ← camera\({int}, {int}, {float}, {word}\)")]
fn given_a_camera_transform(
    world: &mut RayTracerWorld,
    c: String,
    hsize: usize,
    vsize: usize,
    fov: RayTracerFloat,
    t: String,
) {
    let transform = world.get_transform_or_panic(&t);

    world
        .cameras
        .insert(c, Camera::new(hsize, vsize, fov, *transform));
}

#[given(expr = r"{word} ← intersect\({word}, {word}\)")]
#[when(expr = r"{word} ← intersect\({word}, {word}\)")]
fn when_ray_intersects_sphere(
    world: &mut RayTracerWorld,
    int_name: String,
    sphere_name: String,
    ray_name: String,
) {
    let sphere = world.get_object_or_panic(&sphere_name);
    let ray = world.get_ray_or_panic(&ray_name);
    world
        .intersectionses
        .insert(int_name, sphere.intersections(ray));
}

#[when(expr = r"{word} ← ray_for_pixel\({word}, {int}, {int}\)")]
fn when_ray_for_pixel(world: &mut RayTracerWorld, r: String, c: String, x: usize, y: usize) {
    let camera = world.get_camera_or_panic(&c);
    world.rays.insert(r, camera.ray_for_pixel(x, y));
}

#[when(expr = r"{word} ← normal_at\({word}, point\({float}, {float}, {float}\)\)")]
fn when_sphere_normal_at(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere_name: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let s = world.get_object_or_panic(&sphere_name);
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
    let v = world.get_vector_or_panic(&vec_name);
    let n = world.get_vector_or_panic(&norm_name);
    world.tuples.insert(reflection_name, v.reflect(n));
}

#[when(expr = r"{word} ← point_light\({word}, {word}\)")]
fn when_light_created(
    world: &mut RayTracerWorld,
    light_name: String,
    pos_name: String,
    intensity_name: String,
) {
    let p = world.get_point_or_panic(&pos_name);
    let i = world.get_color_or_panic(&intensity_name);
    world.lights.insert(light_name, PointLight::new(*p, *i));
}

#[when(expr = r"{word} ← {word}.material")]
fn when_material_from_sphere(world: &mut RayTracerWorld, mat_name: String, sphere_name: String) {
    let s = world.get_object_or_panic(&sphere_name);
    world.materials.insert(mat_name, *s.material());
}

#[when(expr = r"{word} ← lighting\({word}, {word}, {word}, {word}, {word}\)")]
fn when_lighting_material_not_in_shadow(
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
    let p = *world.get_point_or_panic(&position);
    let e = *world.get_vector_or_panic(&eyev);
    let n = *world.get_vector_or_panic(&normalv);

    world.colors.insert(result, m.lighting(l, p, e, n, false));
}

#[when(expr = r"{word} ← lighting\({word}, {word}, {word}, {word}, {word}, {word}\)")]
fn when_lighting_material_possibly_in_shadow(
    world: &mut RayTracerWorld,
    result: String,
    mat: String,
    light: String,
    position: String,
    eyev: String,
    normalv: String,
    in_shadow: String,
) {
    let m = world.get_material_or_panic(&mat);
    let l = *world.get_light_or_panic(&light);
    let p = *world.get_point_or_panic(&position);
    let e = *world.get_vector_or_panic(&eyev);
    let n = *world.get_vector_or_panic(&normalv);

    world
        .colors
        .insert(result, m.lighting(l, p, e, n, in_shadow == "true"));
}

#[given(expr = r"{word} ← default_world\(\)")]
#[when(expr = r"{word} ← default_world\(\)")]
fn given_or_when_default_world(world: &mut RayTracerWorld, world_name: String) {
    world.worlds.insert(world_name, World::default_world());
}

#[given(expr = r"{word} ← world\([{}], {word}\)")]
fn given_arbitrary_world(world: &mut RayTracerWorld, w: String, objects: String, l: String) {
    let obj_names: Vec<_> = objects.split(",").collect();

    let mut objs: Vec<Rc<dyn Object>> = vec![];

    for obj in obj_names {
        objs.push(Rc::clone(
            world.get_object_or_panic(&obj.trim().to_string()),
        ));
    }

    let light = world.get_light_or_panic(&l);

    world.worlds.insert(w, World::new(objs, *light));
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

#[given(expr = r"{word} ← view_transform\({word}, {word}, {word}\)")]
#[when(expr = r"{word} ← view_transform\({word}, {word}, {word}\)")]
fn when_view_transform(
    world: &mut RayTracerWorld,
    transform_name: String,
    from_p: String,
    to_p: String,
    up_v: String,
) {
    let from = world.get_point_or_panic(&from_p);
    let to = world.get_point_or_panic(&to_p);
    let up = world.get_vector_or_panic(&up_v);

    world
        .transforms
        .insert(transform_name, from.view_transform(to, up));
}

#[when(expr = r"{word} ← render\({word}, {word}\)")]
fn when_rendering(world: &mut RayTracerWorld, i: String, c: String, w: String) {
    let camera = world.get_camera_or_panic(&c);
    let render_world = world.get_world_or_panic(&w);
    world.canvases.insert(i, camera.render(&render_world));
}

#[given(expr = r"{word} ← test_shape\(\)")]
fn given_default_test_shape(world: &mut RayTracerWorld, s: String) {
    world.objects.insert(s, Rc::new(TestShape::default()));
}

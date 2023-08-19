use crate::{
    parameters::{Axis, SingleValue},
    world::RayTracerWorld,
    RayTracerFloat,
};
use cucumber::{gherkin::Step, given, when};
use ray_tracer_challenge_rs::{
    camera::Camera,
    canvas::Canvas,
    color::{Color, BLACK, WHITE},
    intersection::{Intersection, Intersections},
    light::PointLight,
    material::{Material, MaterialBuilder},
    objects::{default_plane, default_sphere, default_test_shape, Object},
    patterns::Pattern,
    ray::Ray,
    transforms::{identity, rotation, scaling, translation, RotationAxis, Transform},
    tuple::{Point, Tuple},
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
        .insert(int_name, Rc::new(Intersection::new(t, Rc::clone(o))));
}

#[given(expr = r"{word} ← sphere\(\)")]
fn given_a_default_sphere(world: &mut RayTracerWorld, sphere_name: String) {
    world.objects.insert(sphere_name, Rc::new(default_sphere()));
}

fn parse_three_args(s: &str) -> (RayTracerFloat, RayTracerFloat, RayTracerFloat) {
    let three_args_re = Regex::new(r"\((.+), (.+), ([^\)]+)").unwrap();

    let args: Vec<RayTracerFloat> = three_args_re
        .captures(s)
        .unwrap()
        .iter()
        .skip(1) // skips group "0", i.e. the whole match
        .map(|g| g.unwrap().as_str())
        .map(|s| RayTracerFloat::from_str(s).expect(&format!("bad number: {}", s)))
        .collect();

    (args[0], args[1], args[2])
}

fn transform_for_args(expr: &str) -> Transform {
    let args = parse_three_args(expr);

    match expr {
        _ if expr.starts_with("scaling") => scaling(args.0, args.1, args.2),
        _ if expr.starts_with("translation") => translation(args.0, args.1, args.2),
        _ => panic!("bad transform name: {}", expr),
    }
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
                    transform = transform_for_args(val);
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
        Rc::new(Object::Sphere(transform, material_builder.build())),
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
        Rc::new(Object::Sphere(*trans, Material::default().into())),
    );
}

#[given(expr = r"{word} ← sphere\(default, {word}\)")]
fn given_a_sphere_with_default_transform_and_material(
    world: &mut RayTracerWorld,
    sphere_name: String,
) {
    world.objects.insert(
        sphere_name,
        Rc::new(Object::Sphere(identity(), Default::default())),
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
        let ray_world = world.get_world_or_panic(&world_name.to_string());
        world.worlds.insert(
            world_name.to_string(),
            Rc::new(World::new(ray_world.objects.clone(), l)),
        );
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

    world.objects.insert(name, ray_world.objects[n].clone());
}

#[given(regex = r"^(\w+)\.material\.ambient ← (.+)")]
fn given_sphere_ambient_val(world: &mut RayTracerWorld, s: String, ambient: RayTracerFloat) {
    let sphere = world.get_object_or_panic(&s);
    let mut mat = Material::from(sphere.material());
    mat.ambient = ambient;

    let new_sphere = Object::Sphere(*sphere.as_ref().transform(), mat);
    world.objects.insert(s, Rc::new(new_sphere));
}

#[given(regex = r"^(\w+).pattern ← stripe_pattern\(color\(1, 1, 1\), color\(0, 0, 0\)\)")]
fn given_material_stripe_pattern(world: &mut RayTracerWorld, m: String) {
    let material = world.get_material_or_panic(&m);
    let mut new_mat = Material::from(material);
    new_mat.pattern = Pattern::Stripe {
        even: WHITE,
        odd: BLACK,
        transform: identity(),
    };
    world.materials.insert(m, new_mat);
}

#[given(regex = r"^(\w+)\.ambient ← (.+)")]
fn given_material_ambient_val(world: &mut RayTracerWorld, m: String, ambient: RayTracerFloat) {
    let material = world.get_material_or_panic(&m);
    let mut new_mat = Material::from(material);
    new_mat.ambient = ambient;
    world.materials.insert(m, new_mat);
}

#[given(regex = r"^(\w+)\.diffuse ← (.+)")]
fn given_material_diffuse_val(world: &mut RayTracerWorld, m: String, diffuse: RayTracerFloat) {
    let material = world.get_material_or_panic(&m);
    let mut new_mat = Material::from(material);
    new_mat.diffuse = diffuse;
    world.materials.insert(m, new_mat);
}

#[given(regex = r"^(\w+)\.specular ← (.+)")]
fn given_material_specular_val(world: &mut RayTracerWorld, m: String, specular: RayTracerFloat) {
    let material = world.get_material_or_panic(&m);
    let mut new_mat = Material::from(material);
    new_mat.specular = specular;
    world.materials.insert(m, new_mat);
}

#[given(expr = r"{word} ← default_world_with_objects\({word}, {word}\)")]
fn given_world_with_objects(world: &mut RayTracerWorld, w: String, s1: String, s2: String) {
    let o1 = world.get_object_or_panic(&s1);
    let o2 = world.get_object_or_panic(&s2);
    world.worlds.insert(
        w,
        Rc::new(World::default_world_with_objects(vec![
            Rc::clone(o1),
            Rc::clone(o2),
        ])),
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
        .insert(int_name, Rc::new(sphere.clone().intersections(ray)));
}

#[when(expr = r"{word} ← ray_for_pixel\({word}, {int}, {int}\)")]
fn when_ray_for_pixel(world: &mut RayTracerWorld, r: String, c: String, x: usize, y: usize) {
    let camera = world.get_camera_or_panic(&c);
    world.rays.insert(r, camera.ray_for_pixel(x, y));
}

#[when(expr = r"{word} ← normal_at\({word}, point\({float}, {float}, {float}\)\)")]
fn when_object_normal_at(
    world: &mut RayTracerWorld,
    normal_name: String,
    o: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    let obj = world.get_object_or_panic(&o);
    let p = Tuple::point(x, y, z);
    world.tuples.insert(normal_name, obj.normal_at(p));
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

    world
        .colors
        .insert(result, m.lighting(&default_sphere(), l, p, e, n, false));
}

#[allow(clippy::too_many_arguments)]
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

    world.colors.insert(
        result,
        m.lighting(&default_sphere(), l, p, e, n, in_shadow == "true"),
    );
}

#[when(
    expr = r"{word} ← lighting\({word}, {word}, point\({float}, {float}, {float}\), {word}, {word}, {word}\)"
)]
fn when_lighting_color(
    world: &mut RayTracerWorld,
    c: String,
    m: String,
    l: String,
    px: RayTracerFloat,
    py: RayTracerFloat,
    pz: RayTracerFloat,
    ev: String,
    nv: String,
    shad: String,
) {
    let material = world.get_material_or_panic(&m);
    let light = world.get_light_or_panic(&l);
    let point = Point::point(px, py, pz);
    let eyev = world.get_vector_or_panic(&ev);
    let normalv = world.get_vector_or_panic(&nv);
    let in_shadow = shad == "true";

    world.colors.insert(
        c,
        material.lighting(&default_sphere(), *light, point, *eyev, *normalv, in_shadow),
    );
}

#[given(expr = r"{word} ← default_world\(\)")]
#[when(expr = r"{word} ← default_world\(\)")]
fn given_or_when_default_world(world: &mut RayTracerWorld, world_name: String) {
    world
        .worlds
        .insert(world_name, Rc::new(World::default_world()));
}

#[given(expr = r"{word} ← world\([{}], {word}\)")]
fn given_arbitrary_world(world: &mut RayTracerWorld, w: String, objects: String, l: String) {
    let obj_names: Vec<_> = objects.split(',').collect();

    let mut objs: Vec<Rc<Object>> = vec![];

    for obj in obj_names {
        objs.push(Rc::clone(
            world.get_object_or_panic(&obj.trim().to_string()),
        ));
    }

    let light = world.get_light_or_panic(&l);

    world.worlds.insert(w, Rc::new(World::new(objs, *light)));
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
    world.precomps.insert(
        pc,
        int.clone()
            .precompute_with(ray, Rc::new(Intersections::new(vec![int.clone()]))),
    );
}

#[given(expr = r"{word} ← shade_hit\({word}, {word}\)")]
#[when(expr = r"{word} ← shade_hit\({word}, {word}\)")]
fn when_shade_hit(world: &mut RayTracerWorld, c: String, w: String, pc: String) {
    let ray_world = world.get_world_or_panic(&w);
    let precompute = world.get_precomp_or_panic(&pc);
    world.colors.insert(c, ray_world.shade_hit(precompute, 1));
}

#[given(expr = r"{word} ← color_at\({word}, {word}\)")]
#[when(expr = r"{word} ← color_at\({word}, {word}\)")]
fn when_color_at(world: &mut RayTracerWorld, c: String, w: String, r: String) {
    let ray_world = world.get_world_or_panic(&w);
    let ray = world.get_ray_or_panic(&r);
    world.colors.insert(c, ray_world.color_at(ray, 1));
}

#[given(expr = r"{word} ← hit\({word}\)")]
#[when(expr = r"{word} ← hit\({word}\)")]
fn when_hit_queried(world: &mut RayTracerWorld, hit_name: String, ints_name: String) {
    let i = world.get_ints_or_panic(&ints_name);
    let maybe_hit = Rc::clone(&i).hit();

    if let Some(i) = maybe_hit {
        world
            .intersections
            .insert(hit_name, Rc::new(Intersection::new(i.t, i.object.clone())));
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
    world.objects.insert(s, Rc::new(default_test_shape()));
}

#[given(expr = r"{word} ← test_shape\(translation\({float}, {float}, {float}\), material\(\)\)")]
#[when(expr = r"{word} ← test_shape\(translation\({float}, {float}, {float}), material\(\)\)")]
fn given_a_test_shape_translation(
    world: &mut RayTracerWorld,
    s: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    world.objects.insert(
        s,
        Rc::new(Object::TestShape(translation(x, y, z), Material::default())),
    );
}

#[given(expr = r"{word} ← test_shape\(scaling\({float}, {float}, {float}\), material\(\)\)")]
fn given_a_test_shape_scaling(
    world: &mut RayTracerWorld,
    s: String,
    x: RayTracerFloat,
    y: RayTracerFloat,
    z: RayTracerFloat,
) {
    world.objects.insert(
        s,
        Rc::new(Object::TestShape(scaling(x, y, z), Material::default())),
    );
}

#[given(expr = r"s ← test_shape\(scaling\(1, 0.5, 1) * rotation_z\(0.628318), material\())")]
fn given_arbitrary_test_shape(world: &mut RayTracerWorld) {
    world.objects.insert(
        "s".to_string(),
        Rc::new(Object::TestShape(
            scaling(1., 0.5, 1.) * rotation(RotationAxis::Z, 0.628318),
            Material::default(),
        )),
    );
}

#[when(expr = r"{word} ← test_shape\(identity\(\), {word}\)")]
fn given_a_test_shape_named_mat(world: &mut RayTracerWorld, s: String, m: String) {
    let material = world.get_material_or_panic(&m);

    world
        .objects
        .insert(s, Rc::new(Object::TestShape(identity(), material.clone())));
}

#[given(expr = r"{word} ← plane\(\)")]
fn given_default_plane(world: &mut RayTracerWorld, p: String) {
    world.objects.insert(p, Rc::new(default_plane()));
}

#[given(expr = r"{word} ← stripe_pattern\({word}, {word}\)")]
fn given_stripe_pattern(world: &mut RayTracerWorld, p: String, c1: String, c2: String) {
    let even = world.get_color_or_panic(&c1);
    let odd = world.get_color_or_panic(&c2);
    world.patterns.insert(
        p,
        Pattern::Stripe {
            even: *even,
            odd: *odd,
            transform: identity(),
        }
        .into(),
    );
}

#[given(expr = r"set_transform\({word}, {}")]
fn given_sphere_with_transform(w: &mut RayTracerWorld, s: String, t: String) {
    let sphere = w.get_object_or_panic(&s);
    let transform = transform_for_args(&t);

    w.objects.insert(
        s,
        Rc::new(Object::Sphere(transform, sphere.material().clone())),
    );
}

#[given(expr = r"set_pattern_transform\(pattern, {word}\)\)")]
fn given_stripe_with_transform(w: &mut RayTracerWorld, p: String, t: String) {
    let stripe = w.get_pattern_or_panic(&p);
    let transform = transform_for_args(&t);

    let (even, odd) = match stripe.as_ref() {
        Pattern::Stripe {
            transform: _,
            even,
            odd,
        } => (*even, *odd),
        _ => panic!("not a stripe!"),
    };

    w.patterns.insert(
        p,
        Pattern::Stripe {
            transform,
            even,
            odd,
        }
        .into(),
    );
}

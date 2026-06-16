mod math;
mod objects;
mod scene;
mod renderer;

use math::{Vec3, Point3, Color};
use objects::{Sphere, Plane, Cube, Cylinder, Material};
use scene::{Scene, Camera, PointLight};
use renderer::{render, save_ppm};

fn scene_sphere() -> (Scene, Camera) {
    let mut scene = Scene::new();

    let sphere_mat = Material::new(Color::new(1.0, 0.5, 0.8));
    scene.add(Sphere::new(Point3::new(0.0, 0.5, 0.0), 1.0, sphere_mat));

    let ground_mat = Material::new(Color::new(0.5, 0.5, 0.5));
    scene.add(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_mat,
    ));

    scene.add_light(PointLight::new(Point3::new(5.0, 10.0, 5.0), 1.0));

    let camera = Camera::new(
        Point3::new(0.0, 2.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        800.0 / 600.0,
    );

    (scene, camera)
}

fn scene_plane_cube() -> (Scene, Camera) {
    let mut scene = Scene::new();

    // object
    // Yellow cube
    let cube_mat = Material::new(Color::new(1.0, 1.0, 0.0));
    scene.add(Cube::new(Point3::new(0.0, 0.5, 0.0), 1.5, cube_mat));

    // Green ground plane
    let ground_mat = Material::new(Color::new(0.4, 0.6, 0.3));
    scene.add(Plane::new(
        Point3::new(0.0, -0.25, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_mat,
    ));

    scene.add_light(PointLight::new(Point3::new(5.0, 10.0, 5.0), 0.6));
    scene.ambient = 0.05;

    let camera = Camera::new(
        Point3::new(3.0, 3.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        800.0 / 600.0,
    );

    (scene, camera)
}

fn scene_all_objects() -> (Scene, Camera) {
    let mut scene = Scene::new();

    let sphere_mat = Material::new(Color::new(1.0, 0.5, 0.8))
        .with_reflectivity(0.3);
    scene.add(Sphere::new(Point3::new(-2.0, 0.5, 0.0), 1.0, sphere_mat));

    let cube_mat = Material::new(Color::new(1.0, 1.0, 0.0));
    scene.add(Cube::new(Point3::new(2.0, 0.5, 0.0), 1.5, cube_mat));

    let cyl_mat = Material::new(Color::new(0.0, 1.0, 1.0));
    scene.add(Cylinder::new(
        Point3::new(0.0, 0.75, -2.0),
        0.5,
        1.5,
        cyl_mat,
    ));

    let ground_mat = Material::new(Color::new(0.5, 0.5, 0.5));
    scene.add(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_mat,
    ));

    scene.add_light(PointLight::new(Point3::new(5.0, 10.0, 5.0), 1.0));

    let camera = Camera::new(
        Point3::new(0.0, 3.0, 8.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        800.0 / 600.0,
    );

    (scene, camera)
}

fn scene_all_objects_alt_camera() -> (Scene, Camera) {
    let mut scene = Scene::new();

    let sphere_mat = Material::new(Color::new(1.0, 0.5, 0.8))
        .with_reflectivity(0.3);
    scene.add(Sphere::new(Point3::new(-2.0, 0.5, 0.0), 1.0, sphere_mat));

    let cube_mat = Material::new(Color::new(1.0, 1.0, 0.0));
    scene.add(Cube::new(Point3::new(2.0, 0.5, 0.0), 1.5, cube_mat));

    let cyl_mat = Material::new(Color::new(0.0, 1.0, 1.0));
    scene.add(Cylinder::new(
        Point3::new(0.0, 0.75, -2.0),
        0.5,
        1.5,
        cyl_mat,
    ));

    let ground_mat = Material::new(Color::new(0.5, 0.5, 0.5));
    scene.add(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_mat,
    ));

    scene.add_light(PointLight::new(Point3::new(5.0, 10.0, 5.0), 1.0));

    let camera = Camera::new(
        Point3::new(6.0, 5.0, 4.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        800.0 / 600.0,
    );

    (scene, camera)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let width: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(800);
    let height: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(600);

    eprintln!("Rendering at {}x{}", width, height);

    eprintln!("Rendering scene 1: sphere...");
    let (scene, camera) = scene_sphere();
    let pixels = render(&scene, &camera, width, height);
    save_ppm("scene1_sphere.ppm", &pixels, width, height)
        .expect("Failed to save scene 1");
    eprintln!("Saved scene1_sphere.ppm");

    eprintln!("Rendering scene 2: plane and cube...");
    let (scene, camera) = scene_plane_cube();
    let pixels = render(&scene, &camera, width, height);
    save_ppm("scene2_plane_cube.ppm", &pixels, width, height)
        .expect("Failed to save scene 2");
    eprintln!("Saved scene2_plane_cube.ppm");

    eprintln!("Rendering scene 3: all objects...");
    let (scene, camera) = scene_all_objects();
    let pixels = render(&scene, &camera, width, height);
    save_ppm("scene3_all_objects.ppm", &pixels, width, height)
        .expect("Failed to save scene 3");
    eprintln!("Saved scene3_all_objects.ppm");

    eprintln!("Rendering scene 4: all objects (different camera)...");
    let (scene, camera) = scene_all_objects_alt_camera();
    let pixels = render(&scene, &camera, width, height);
    save_ppm("scene4_all_objects_alt.ppm", &pixels, width, height)
        .expect("Failed to save scene 4");
    eprintln!("Saved scene4_all_objects_alt.ppm");

    eprintln!("Done! All 4 images rendered.");
}

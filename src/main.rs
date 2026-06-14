use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, Write};
use std::ops::{Add, Div, Mul, Neg, Sub};

// ============ Vec3 ============
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            self
        }
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;

// ============ Ray ============
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

// ============ Material ============
#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflectivity: f64,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Material {
            color,
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.3,
            shininess: 50.0,
            reflectivity: 0.0,
        }
    }

    pub fn with_reflectivity(mut self, r: f64) -> Self {
        self.reflectivity = r;
        self
    }
}

// ============ Hit Record ============
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Material,
}

// ============ Objects ============
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// Sphere
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord {
            point,
            normal,
            t: root,
            material: self.material,
        })
    }
}

// Plane
pub struct Plane {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, material: Material) -> Self {
        Plane { point, normal: normal.normalize(), material }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = (self.point - ray.origin).dot(self.normal) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let normal = if denom < 0.0 { self.normal } else { -self.normal };

        Some(HitRecord {
            point: ray.at(t),
            normal,
            t,
            material: self.material,
        })
    }
}

// Cube (Axis-Aligned Bounding Box)
pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Point3, size: f64, material: Material) -> Self {
        let half = size / 2.0;
        Cube {
            min: Point3::new(center.x - half, center.y - half, center.z - half),
            max: Point3::new(center.x + half, center.y + half, center.z + half),
            material,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let inv_d = Vec3::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);

        let t0x = (self.min.x - ray.origin.x) * inv_d.x;
        let t1x = (self.max.x - ray.origin.x) * inv_d.x;
        let t0y = (self.min.y - ray.origin.y) * inv_d.y;
        let t1y = (self.max.y - ray.origin.y) * inv_d.y;
        let t0z = (self.min.z - ray.origin.z) * inv_d.z;
        let t1z = (self.max.z - ray.origin.z) * inv_d.z;

        let (t0x, t1x) = if inv_d.x < 0.0 { (t1x, t0x) } else { (t0x, t1x) };
        let (t0y, t1y) = if inv_d.y < 0.0 { (t1y, t0y) } else { (t0y, t1y) };
        let (t0z, t1z) = if inv_d.z < 0.0 { (t1z, t0z) } else { (t0z, t1z) };

        let mut t_near = t0x;
        let mut axis = 0;
        if t0y > t_near {
            t_near = t0y;
            axis = 1;
        }
        if t0z > t_near {
            t_near = t0z;
            axis = 2;
        }

        let t_far = t1x.min(t1y).min(t1z);

        if t_near > t_far || t_far < t_min {
            return None;
        }

        let t = if t_near < t_min { t_far } else { t_near };
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let mut normal = Vec3::zero();
        match axis {
            0 => normal.x = if ray.direction.x < 0.0 { 1.0 } else { -1.0 },
            1 => normal.y = if ray.direction.y < 0.0 { 1.0 } else { -1.0 },
            _ => normal.z = if ray.direction.z < 0.0 { 1.0 } else { -1.0 },
        }

        Some(HitRecord {
            point,
            normal,
            t,
            material: self.material,
        })
    }
}

// Cylinder
pub struct Cylinder {
    pub center: Point3,
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

impl Cylinder {
    pub fn new(center: Point3, radius: f64, height: f64, material: Material) -> Self {
        Cylinder { center, radius, height, material }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let half_h = self.height / 2.0;
        let y_min = self.center.y - half_h;
        let y_max = self.center.y + half_h;

        // Check cylinder body (infinite cylinder along Y)
        let dx = ray.direction.x;
        let dz = ray.direction.z;
        let ox = ray.origin.x - self.center.x;
        let oz = ray.origin.z - self.center.z;

        let a = dx * dx + dz * dz;
        let b = 2.0 * (ox * dx + oz * dz);
        let c = ox * ox + oz * oz - self.radius * self.radius;

        let mut closest_t = f64::MAX;
        let mut closest_normal = Vec3::zero();

        if a.abs() > 1e-6 {
            let discriminant = b * b - 4.0 * a * c;
            if discriminant >= 0.0 {
                let sqrtd = discriminant.sqrt();
                for &t in &[(-b - sqrtd) / (2.0 * a), (-b + sqrtd) / (2.0 * a)] {
                    if t >= t_min && t <= t_max && t < closest_t {
                        let p = ray.at(t);
                        if p.y >= y_min && p.y <= y_max {
                            closest_t = t;
                            closest_normal = Vec3::new(p.x - self.center.x, 0.0, p.z - self.center.z).normalize();
                        }
                    }
                }
            }
        }

        // Check top and bottom caps
        if ray.direction.y.abs() > 1e-6 {
            for &cap_y in &[y_min, y_max] {
                let t = (cap_y - ray.origin.y) / ray.direction.y;
                if t >= t_min && t <= t_max && t < closest_t {
                    let p = ray.at(t);
                    let dx = p.x - self.center.x;
                    let dz = p.z - self.center.z;
                    if dx * dx + dz * dz <= self.radius * self.radius {
                        closest_t = t;
                        closest_normal = if cap_y == y_max {
                            Vec3::new(0.0, 1.0, 0.0)
                        } else {
                            Vec3::new(0.0, -1.0, 0.0)
                        };
                    }
                }
            }
        }

        if closest_t < f64::MAX {
            Some(HitRecord {
                point: ray.at(closest_t),
                normal: closest_normal,
                t: closest_t,
                material: self.material,
            })
        } else {
            None
        }
    }
}

// ============ Light ============
pub struct PointLight {
    pub position: Point3,
    pub intensity: f64,
    pub color: Color,
}

impl PointLight {
    pub fn new(position: Point3, intensity: f64) -> Self {
        PointLight {
            position,
            intensity,
            color: Color::new(1.0, 1.0, 1.0),
        }
    }
}

// ============ Camera ============
pub struct Camera {
    pub position: Point3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub fov: f64,
    pub aspect: f64,
}

impl Camera {
    pub fn new(position: Point3, look_at: Point3, up: Vec3, fov: f64, aspect: f64) -> Self {
        let forward = (look_at - position).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward).normalize();

        Camera {
            position,
            forward,
            right,
            up,
            fov,
            aspect,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let half_height = (self.fov * PI / 360.0).tan();
        let half_width = half_height * self.aspect;

        let x = (2.0 * u - 1.0) * half_width;
        let y = (2.0 * v - 1.0) * half_height;

        let direction = (self.forward + self.right * x + self.up * y).normalize();
        Ray::new(self.position, direction)
    }
}

// ============ Scene ============
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<PointLight>,
    pub ambient: f64,
    pub background: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            ambient: 0.1,
            background: Color::new(0.5, 0.7, 1.0),
        }
    }

    pub fn add<T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn set_brightness(&mut self, brightness: f64) {
        self.ambient = 0.1 * brightness;
        for light in &mut self.lights {
            light.intensity = brightness;
        }
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray, t_min, closest) {
                closest = hit.t;
                result = Some(hit);
            }
        }
        result
    }

    fn is_shadowed(&self, point: Point3, light_pos: Point3) -> bool {
        let to_light = light_pos - point;
        let distance = to_light.length();
        let shadow_ray = Ray::new(point, to_light.normalize());
        self.hit(&shadow_ray, 0.001, distance - 0.001).is_some()
    }

    pub fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::zero();
        }

        if let Some(hit) = self.hit(ray, 0.001, f64::MAX) {
            let mut color = hit.material.color * hit.material.ambient * self.ambient;

            for light in &self.lights {
                if self.is_shadowed(hit.point, light.position) {
                    continue;
                }

                let light_dir = (light.position - hit.point).normalize();
                let view_dir = -ray.direction;

                // Diffuse
                let diff = hit.normal.dot(light_dir).max(0.0);
                color = color + hit.material.color * diff * hit.material.diffuse * light.intensity;

                // Specular
                let reflect_dir = (-light_dir).reflect(hit.normal);
                let spec = view_dir.dot(reflect_dir).max(0.0).powf(hit.material.shininess);
                color = color + light.color * spec * hit.material.specular * light.intensity;
            }

            // Reflection
            if hit.material.reflectivity > 0.0 {
                let reflect_dir = ray.direction.reflect(hit.normal);
                let reflect_ray = Ray::new(hit.point, reflect_dir);
                let reflect_color = self.trace(&reflect_ray, depth - 1);
                color = color * (1.0 - hit.material.reflectivity) + reflect_color * hit.material.reflectivity;
            }

            color
        } else {
            self.background
        }
    }
}

// ============ Renderer ============
pub fn render(scene: &Scene, camera: &Camera, width: u32, height: u32) -> Vec<Color> {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = (i as f64 + 0.5) / width as f64;
            let v = (j as f64 + 0.5) / height as f64;
            let ray = camera.get_ray(u, v);
            let color = scene.trace(&ray, 5);
            pixels.push(color);
        }
    }
    pixels
}

pub fn write_ppm<W: Write>(writer: &mut W, pixels: &[Color], width: u32, height: u32) -> io::Result<()> {
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?;

    for color in pixels {
        let r = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (color.z.clamp(0.0, 1.0) * 255.0) as u8;
        writeln!(writer, "{} {} {}", r, g, b)?;
    }
    Ok(())
}

pub fn save_ppm(filename: &str, pixels: &[Color], width: u32, height: u32) -> io::Result<()> {
    let mut file = File::create(filename)?;
    write_ppm(&mut file, pixels, width, height)
}

// ============ Scene Builders ============

fn scene_sphere() -> (Scene, Camera) {
    let mut scene = Scene::new();

    // Sphere
    let sphere_mat = Material::new(Color::new(0.8, 0.2, 0.2));
    scene.add(Sphere::new(Point3::new(0.0, 0.5, 0.0), 1.0, sphere_mat));

    // Ground plane
    let ground_mat = Material::new(Color::new(0.5, 0.5, 0.5));
    scene.add(Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), ground_mat));

    // Light
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

    // Cube
    let cube_mat = Material::new(Color::new(0.2, 0.5, 0.8));
    scene.add(Cube::new(Point3::new(0.0, 0.5, 0.0), 1.5, cube_mat));

    // Ground plane
    let ground_mat = Material::new(Color::new(0.4, 0.6, 0.3));
    scene.add(Plane::new(Point3::new(0.0, -0.25, 0.0), Vec3::new(0.0, 1.0, 0.0), ground_mat));

    // Light with lower brightness
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

    // Sphere
    let sphere_mat = Material::new(Color::new(0.8, 0.2, 0.2)).with_reflectivity(0.3);
    scene.add(Sphere::new(Point3::new(-2.0, 0.5, 0.0), 1.0, sphere_mat));

    // Cube
    let cube_mat = Material::new(Color::new(0.2, 0.5, 0.8));
    scene.add(Cube::new(Point3::new(2.0, 0.5, 0.0), 1.5, cube_mat));

    // Cylinder
    let cyl_mat = Material::new(Color::new(0.8, 0.7, 0.2));
    scene.add(Cylinder::new(Point3::new(0.0, 0.75, -2.0), 0.5, 1.5, cyl_mat));

    // Ground plane
    let ground_mat = Material::new(Color::new(0.5, 0.5, 0.5));
    scene.add(Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), ground_mat));

    // Light
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

    // Same objects as scene_all_objects
    let sphere_mat = Material::new(Color::new(0.8, 0.2, 0.2)).with_reflectivity(0.3);
    scene.add(Sphere::new(Point3::new(-2.0, 0.5, 0.0), 1.0, sphere_mat));

    let cube_mat = Material::new(Color::new(0.2, 0.5, 0.8));
    scene.add(Cube::new(Point3::new(2.0, 0.5, 0.0), 1.5, cube_mat));

    let cyl_mat = Material::new(Color::new(0.8, 0.7, 0.2));
    scene.add(Cylinder::new(Point3::new(0.0, 0.75, -2.0), 0.5, 1.5, cyl_mat));

    let ground_mat = Material::new(Color::new(0.5, 0.5, 0.5));
    scene.add(Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), ground_mat));

    scene.add_light(PointLight::new(Point3::new(5.0, 10.0, 5.0), 1.0));

    // Different camera position
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

    // Scene 1: Sphere
    eprintln!("Rendering scene 1: sphere...");
    let (scene, camera) = scene_sphere();
    let pixels = render(&scene, &camera, width, height);
    save_ppm("scene1_sphere.ppm", &pixels, width, height).expect("Failed to save scene 1");
    eprintln!("Saved scene1_sphere.ppm");

    // Scene 2: Plane and Cube (lower brightness)
    eprintln!("Rendering scene 2: plane and cube...");
    let (scene, camera) = scene_plane_cube();
    let pixels = render(&scene, &camera, width, height);
    save_ppm("scene2_plane_cube.ppm", &pixels, width, height).expect("Failed to save scene 2");
    eprintln!("Saved scene2_plane_cube.ppm");

    // Scene 3: All objects
    eprintln!("Rendering scene 3: all objects...");
    let (scene, camera) = scene_all_objects();
    let pixels = render(&scene, &camera, width, height);
    save_ppm("scene3_all_objects.ppm", &pixels, width, height).expect("Failed to save scene 3");
    eprintln!("Saved scene3_all_objects.ppm");

    // Scene 4: All objects, different camera
    eprintln!("Rendering scene 4: all objects (different camera)...");
    let (scene, camera) = scene_all_objects_alt_camera();
    let pixels = render(&scene, &camera, width, height);
    save_ppm("scene4_all_objects_alt.ppm", &pixels, width, height).expect("Failed to save scene 4");
    eprintln!("Saved scene4_all_objects_alt.ppm");

    eprintln!("Done! All 4 images rendered.");
}

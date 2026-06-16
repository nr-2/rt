//! Scene management, camera, and ray tracing

use std::f64::consts::PI;
use crate::math::{Point3, Vec3, Ray, Color};
use crate::objects::{Hittable, HitRecord};

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

// ============ SCENE ============

/// The complete 3D scene: objects and lights
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,  // All objects in scene
    pub lights: Vec<PointLight>,          // All light sources
    pub ambient: f64,                     // Global ambient light level
    pub background: Color,                // Sky color when ray hits nothing
}

impl Scene {
    /// Create a new empty scene
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            ambient: 0.1,
            background: Color::new(0.5, 0.7, 1.0),  // Sky blue
        }
    }

    /// Add an object to the scene
    pub fn add<T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }

    /// Add a light to the scene
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    /// Set overall scene brightness
    /// Scales both ambient light and light intensity

    #[allow(dead_code)]
    pub fn set_brightness(&mut self, brightness: f64) {
        self.ambient = 0.1 * brightness;
        for light in &mut self.lights {
            light.intensity = brightness;
        }
    }

    /// Check if a ray hits any object in the scene
    /// Returns the closest hit or None if no hit
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

    /// Check if a point is in shadow from a light source
    /// Shoots ray from point toward light, checks for blocking objects
    fn is_shadowed(&self, point: Point3, light_pos: Point3) -> bool {
        let to_light = light_pos - point;
        let distance = to_light.length();
        let shadow_ray = Ray::new(point, to_light.normalize());
        self.hit(&shadow_ray, 0.001, distance - 0.001).is_some()
    }

    /// Trace a ray through the scene and return its color
    ///
    /// Implements Phong lighting model:
    /// - Ambient: global light
    /// - Diffuse: matte surface lighting
    /// - Specular: shiny highlight lighting
    /// - Reflection: mirror reflections (recursive)
    ///
    /// Depth parameter prevents infinite reflection recursion
    pub fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::zero();
        }

        if let Some(hit) = self.hit(ray, 0.001, f64::MAX) {
            // Start with ambient lighting (light with no source)
            let mut color = hit.material.color * hit.material.ambient * self.ambient;

            // Add contribution from each light source
            for light in &self.lights {
                // Skip if point is in shadow of this light
                if self.is_shadowed(hit.point, light.position) {
                    continue;
                }

                let light_dir = (light.position - hit.point).normalize();
                let view_dir = -ray.direction;

                // ===== DIFFUSE LIGHTING =====
                // Matte surface: brightness based on angle to light
                let diff = hit.normal.dot(light_dir).max(0.0);
                color = color
                    + hit.material.color * diff * hit.material.diffuse * light.intensity;

                // ===== SPECULAR LIGHTING =====
                // Shiny highlight: bright reflection toward viewer
                let reflect_dir = (-light_dir).reflect(hit.normal);
                let spec = view_dir.dot(reflect_dir).max(0.0).powf(hit.material.shininess);
                color = color + light.color * spec * hit.material.specular * light.intensity;
            }

            // ===== REFLECTION =====
            // Mirror reflections: recursively trace reflected ray
            if hit.material.reflectivity > 0.0 {
                let reflect_dir = ray.direction.reflect(hit.normal);
                let reflect_ray = Ray::new(hit.point, reflect_dir);
                let reflect_color = self.trace(&reflect_ray, depth - 1);
                color = color * (1.0 - hit.material.reflectivity)
                    + reflect_color * hit.material.reflectivity;
            }

            color
        } else {
            // Ray hit nothing: return background color
            self.background
        }
    }
}

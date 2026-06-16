//! Objects: Sphere, Cube, Plane, Cylinder

use crate::math::{Point3, Vec3, Ray, Color};

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

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

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

// ============ PLANE ============

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

// ============ CUBE ============

pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub material: Material,
}

impl Cube {
    /// Create cube centered at position with edge length = size
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
    /// Ray-AABB intersection using slab method
    /// Fast algorithm that intersects ray with 3 pairs of parallel planes
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let inv_d = Vec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        // Intersect with X slabs
        let t0x = (self.min.x - ray.origin.x) * inv_d.x;
        let t1x = (self.max.x - ray.origin.x) * inv_d.x;

        // Intersect with Y slabs
        let t0y = (self.min.y - ray.origin.y) * inv_d.y;
        let t1y = (self.max.y - ray.origin.y) * inv_d.y;

        // Intersect with Z slabs
        let t0z = (self.min.z - ray.origin.z) * inv_d.z;
        let t1z = (self.max.z - ray.origin.z) * inv_d.z;

        // Order the t values
        let (t0x, t1x) = if inv_d.x < 0.0 { (t1x, t0x) } else { (t0x, t1x) };
        let (t0y, t1y) = if inv_d.y < 0.0 { (t1y, t0y) } else { (t0y, t1y) };
        let (t0z, t1z) = if inv_d.z < 0.0 { (t1z, t0z) } else { (t0z, t1z) };

        // Find nearest entry point and which axis
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

        // Find farthest exit point
        let t_far = t1x.min(t1y).min(t1z);

        if t_near > t_far || t_far < t_min {
            return None;  // No intersection
        }

        let t = if t_near < t_min { t_far } else { t_near };
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let mut normal = Vec3::zero();

        // Determine which face was hit
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

// ============ CYLINDER ============

/// A cylinder with circular top/bottom caps and curved side
/// Axis is along Y direction
pub struct Cylinder {
    pub center: Point3,       // Center position
    pub radius: f64,          // Radius of circular cross-section
    pub height: f64,          // Height along Y axis
    pub material: Material,
}

impl Cylinder {
    /// Create cylinder at center with given radius and height
    pub fn new(center: Point3, radius: f64, height: f64, material: Material) -> Self {
        Cylinder { center, radius, height, material }
    }
}

impl Hittable for Cylinder {
    /// Ray-cylinder intersection
    /// Tests: cylinder body (infinite cylinder) + top and bottom caps
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let half_h = self.height / 2.0;
        let y_min = self.center.y - half_h;
        let y_max = self.center.y + half_h;

        // ===== Check cylinder body (infinite cylinder along Y) =====
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
                        // Check if hit point is within height bounds
                        if p.y >= y_min && p.y <= y_max {
                            closest_t = t;
                            closest_normal =
                                Vec3::new(p.x - self.center.x, 0.0, p.z - self.center.z).normalize();
                        }
                    }
                }
            }
        }

        // ===== Check top and bottom caps =====
        if ray.direction.y.abs() > 1e-6 {
            for &cap_y in &[y_min, y_max] {
                let t = (cap_y - ray.origin.y) / ray.direction.y;
                if t >= t_min && t <= t_max && t < closest_t {
                    let p = ray.at(t);
                    let dx = p.x - self.center.x;
                    let dz = p.z - self.center.z;
                    // Check if hit point is within radius
                    if dx * dx + dz * dz <= self.radius * self.radius {
                        closest_t = t;
                        closest_normal = if cap_y == y_max {
                            Vec3::new(0.0, 1.0, 0.0)  // Top cap
                        } else {
                            Vec3::new(0.0, -1.0, 0.0)  // Bottom cap
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

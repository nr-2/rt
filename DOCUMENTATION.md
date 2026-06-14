# rt - Ray Tracer Documentation

A ray tracer written in Rust that renders 3D scenes to PPM images.

## Features

- **4 Object Types**: Sphere, Cube, Flat Plane, and Cylinder
- **Phong Lighting Model**: Ambient, diffuse, and specular lighting
- **Shadows**: Accurate shadow casting from point lights
- **Reflection**: Objects can have reflective surfaces
- **Configurable Camera**: Position, look-at target, field of view
- **Adjustable Brightness**: Control scene lighting intensity
- **PPM Output**: Standard P3 format images

## Usage

### Basic Usage

```bash
# Render at default 800x600 resolution
cargo run --release

# Render at custom resolution (faster for testing)
cargo run --release -- 200 150
```

### Output

The program generates 4 PPM images:
- `scene1_sphere.ppm` - Scene with a sphere
- `scene2_plane_cube.ppm` - Plane and cube with lower brightness
- `scene3_all_objects.ppm` - All 4 object types
- `scene4_all_objects_alt.ppm` - Same scene, different camera angle

## Code Examples

### Creating Objects

#### Sphere

```rust
// Create a red sphere at position (0, 1, 0) with radius 1.0
let material = Material::new(Color::new(0.8, 0.2, 0.2));  // RGB color
let sphere = Sphere::new(
    Point3::new(0.0, 1.0, 0.0),  // center position
    1.0,                          // radius
    material
);
scene.add(sphere);
```

#### Cube

```rust
// Create a blue cube centered at (2, 0.5, 0) with size 1.5
let material = Material::new(Color::new(0.2, 0.5, 0.8));
let cube = Cube::new(
    Point3::new(2.0, 0.5, 0.0),  // center position
    1.5,                          // size (edge length)
    material
);
scene.add(cube);
```

#### Flat Plane

```rust
// Create a gray ground plane at y = -0.5, facing upward
let material = Material::new(Color::new(0.5, 0.5, 0.5));
let plane = Plane::new(
    Point3::new(0.0, -0.5, 0.0),  // a point on the plane
    Vec3::new(0.0, 1.0, 0.0),     // normal direction (up)
    material
);
scene.add(plane);
```

#### Cylinder

```rust
// Create a yellow cylinder at (0, 0.75, -2) with radius 0.5 and height 1.5
let material = Material::new(Color::new(0.8, 0.7, 0.2));
let cylinder = Cylinder::new(
    Point3::new(0.0, 0.75, -2.0),  // center position
    0.5,                            // radius
    1.5,                            // height
    material
);
scene.add(cylinder);
```

### Changing Brightness

```rust
// Method 1: Adjust light intensity directly
let light = PointLight::new(
    Point3::new(5.0, 10.0, 5.0),  // position
    0.6                            // intensity (lower = dimmer)
);
scene.add_light(light);

// Method 2: Adjust ambient light level
scene.ambient = 0.05;  // default is 0.1

// Method 3: Use the convenience method to set both
scene.set_brightness(0.5);  // scales both ambient and light intensity
```

### Changing Camera Position and Angle

```rust
// Create a camera
let camera = Camera::new(
    Point3::new(0.0, 3.0, 8.0),   // camera position
    Point3::new(0.0, 0.0, 0.0),   // look-at target
    Vec3::new(0.0, 1.0, 0.0),     // up direction
    60.0,                          // field of view in degrees
    800.0 / 600.0                  // aspect ratio (width/height)
);

// Example: Move camera to the right and higher
let camera_alt = Camera::new(
    Point3::new(6.0, 5.0, 4.0),   // new position (right, up, forward)
    Point3::new(0.0, 0.0, -1.0),  // same look-at target
    Vec3::new(0.0, 1.0, 0.0),
    60.0,
    800.0 / 600.0
);
```

### Complete Scene Example

```rust
fn create_custom_scene() -> (Scene, Camera) {
    let mut scene = Scene::new();

    // Add objects
    scene.add(Sphere::new(
        Point3::new(-1.0, 0.5, 0.0),
        1.0,
        Material::new(Color::new(1.0, 0.0, 0.0))  // red
    ));

    scene.add(Cube::new(
        Point3::new(1.5, 0.5, 0.0),
        1.0,
        Material::new(Color::new(0.0, 1.0, 0.0))  // green
    ));

    scene.add(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::new(Color::new(0.8, 0.8, 0.8))  // light gray
    ));

    // Add light
    scene.add_light(PointLight::new(
        Point3::new(5.0, 10.0, 5.0),
        1.0
    ));

    // Create camera
    let camera = Camera::new(
        Point3::new(0.0, 2.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        800.0 / 600.0
    );

    (scene, camera)
}

// Render the scene
let (scene, camera) = create_custom_scene();
let pixels = render(&scene, &camera, 800, 600);
save_ppm("output.ppm", &pixels, 800, 600).unwrap();
```

### Adding Reflection (Bonus Feature)

```rust
// Create a reflective material
let material = Material::new(Color::new(0.8, 0.8, 0.8))
    .with_reflectivity(0.5);  // 0.0 = no reflection, 1.0 = mirror

scene.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material));
```

## Resolution Tips

- **Testing**: Use 200x150 for quick iteration (~1 second)
- **Preview**: Use 400x300 for checking composition (~5 seconds)
- **Final**: Use 800x600 for deliverables (~20 seconds)
- Higher resolutions like 1200x1000 can take several minutes

## Coordinate System

- **X-axis**: Left (-) to Right (+)
- **Y-axis**: Down (-) to Up (+)
- **Z-axis**: Into screen (-) to Towards viewer (+)

## Color Values

Colors use RGB values from 0.0 to 1.0:
- `Color::new(1.0, 0.0, 0.0)` = Red
- `Color::new(0.0, 1.0, 0.0)` = Green
- `Color::new(0.0, 0.0, 1.0)` = Blue
- `Color::new(1.0, 1.0, 1.0)` = White
- `Color::new(0.0, 0.0, 0.0)` = Black

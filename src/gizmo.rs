use three_d::*;

/// Transform gizmo colors - Fusion 360 style
#[allow(dead_code)]
pub struct GizmoColors;

#[allow(dead_code)]
impl GizmoColors {
    pub const X_AXIS: Srgba = Srgba::new(220, 70, 60, 255);      // Red
    pub const Y_AXIS: Srgba = Srgba::new(70, 175, 70, 255);      // Green
    pub const Z_AXIS: Srgba = Srgba::new(60, 130, 220, 255);     // Blue
    pub const X_AXIS_HOVER: Srgba = Srgba::new(255, 100, 90, 255);
    pub const Y_AXIS_HOVER: Srgba = Srgba::new(100, 220, 100, 255);
    pub const Z_AXIS_HOVER: Srgba = Srgba::new(90, 160, 255, 255);
    pub const CENTER: Srgba = Srgba::new(255, 255, 255, 200);    // White center
}

/// Gizmo mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GizmoMode {
    Translate,
    Rotate,
    Scale,
}

/// Which axis is being manipulated
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GizmoAxis {
    None,
    X,
    Y,
    Z,
    All,
}

/// Transform gizmo for manipulating objects - Fusion 360 style
#[allow(dead_code)]
pub struct TransformGizmo {
    // Translation arrows
    pub x_arrow: Gm<Mesh, ColorMaterial>,
    pub y_arrow: Gm<Mesh, ColorMaterial>,
    pub z_arrow: Gm<Mesh, ColorMaterial>,
    pub x_cone: Gm<Mesh, ColorMaterial>,
    pub y_cone: Gm<Mesh, ColorMaterial>,
    pub z_cone: Gm<Mesh, ColorMaterial>,
    // Center sphere
    pub center: Gm<Mesh, ColorMaterial>,
    // Rotation rings (simplified as circles)
    pub x_ring: Gm<Mesh, ColorMaterial>,
    pub y_ring: Gm<Mesh, ColorMaterial>,
    pub z_ring: Gm<Mesh, ColorMaterial>,
    // Scale cubes
    pub x_cube: Gm<Mesh, ColorMaterial>,
    pub y_cube: Gm<Mesh, ColorMaterial>,
    pub z_cube: Gm<Mesh, ColorMaterial>,
    // State
    pub position: Vec3,
    pub mode: GizmoMode,
    pub active_axis: GizmoAxis,
    pub visible: bool,
    pub scale: f32,
}

impl TransformGizmo {
    pub fn new(context: &Context) -> Self {
        let arrow_length = 1.0;
        let arrow_thickness = 0.02;
        let cone_size = 0.12;
        let ring_radius = 0.8;
        let cube_size = 0.08;

        // Create arrow shafts (thin cylinders as lines)
        let x_arrow = Self::create_arrow_shaft(context, vec3(arrow_length, 0.0, 0.0), arrow_thickness, GizmoColors::X_AXIS);
        let y_arrow = Self::create_arrow_shaft(context, vec3(0.0, arrow_length, 0.0), arrow_thickness, GizmoColors::Y_AXIS);
        let z_arrow = Self::create_arrow_shaft(context, vec3(0.0, 0.0, arrow_length), arrow_thickness, GizmoColors::Z_AXIS);

        // Create arrow cones (tips)
        let x_cone = Self::create_cone(context, vec3(arrow_length, 0.0, 0.0), cone_size, GizmoColors::X_AXIS, 0);
        let y_cone = Self::create_cone(context, vec3(0.0, arrow_length, 0.0), cone_size, GizmoColors::Y_AXIS, 1);
        let z_cone = Self::create_cone(context, vec3(0.0, 0.0, arrow_length), cone_size, GizmoColors::Z_AXIS, 2);

        // Create center sphere
        let center = Self::create_center_sphere(context, 0.1);

        // Create rotation rings
        let x_ring = Self::create_ring(context, ring_radius, GizmoColors::X_AXIS, 0);
        let y_ring = Self::create_ring(context, ring_radius, GizmoColors::Y_AXIS, 1);
        let z_ring = Self::create_ring(context, ring_radius, GizmoColors::Z_AXIS, 2);

        // Create scale cubes
        let x_cube = Self::create_scale_cube(context, vec3(arrow_length * 0.9, 0.0, 0.0), cube_size, GizmoColors::X_AXIS);
        let y_cube = Self::create_scale_cube(context, vec3(0.0, arrow_length * 0.9, 0.0), cube_size, GizmoColors::Y_AXIS);
        let z_cube = Self::create_scale_cube(context, vec3(0.0, 0.0, arrow_length * 0.9), cube_size, GizmoColors::Z_AXIS);

        Self {
            x_arrow,
            y_arrow,
            z_arrow,
            x_cone,
            y_cone,
            z_cone,
            center,
            x_ring,
            y_ring,
            z_ring,
            x_cube,
            y_cube,
            z_cube,
            position: vec3(0.0, 0.0, 0.0),
            mode: GizmoMode::Translate,
            active_axis: GizmoAxis::None,
            visible: false,
            scale: 1.0,
        }
    }

    fn create_arrow_shaft(context: &Context, direction: Vec3, _thickness: f32, color: Srgba) -> Gm<Mesh, ColorMaterial> {
        let positions = vec![vec3(0.0, 0.0, 0.0), direction];
        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            ..Default::default()
        };

        Gm::new(
            Mesh::new(context, &cpu_mesh),
            ColorMaterial {
                color,
                ..Default::default()
            },
        )
    }

    fn create_cone(context: &Context, tip_position: Vec3, size: f32, color: Srgba, axis: i32) -> Gm<Mesh, ColorMaterial> {
        // Create a simple triangle cone pointing in the direction
        let segments = 8;
        let mut positions = Vec::new();

        let base_center = tip_position * 0.85;

        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

            let (p1, p2) = match axis {
                0 => ( // X axis
                    base_center + vec3(0.0, angle1.cos() * size, angle1.sin() * size),
                    base_center + vec3(0.0, angle2.cos() * size, angle2.sin() * size),
                ),
                1 => ( // Y axis
                    base_center + vec3(angle1.cos() * size, 0.0, angle1.sin() * size),
                    base_center + vec3(angle2.cos() * size, 0.0, angle2.sin() * size),
                ),
                _ => ( // Z axis
                    base_center + vec3(angle1.cos() * size, angle1.sin() * size, 0.0),
                    base_center + vec3(angle2.cos() * size, angle2.sin() * size, 0.0),
                ),
            };

            // Triangle from tip to base edge
            positions.push(tip_position);
            positions.push(p1);
            positions.push(p2);
        }

        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            ..Default::default()
        };

        Gm::new(
            Mesh::new(context, &cpu_mesh),
            ColorMaterial {
                color,
                ..Default::default()
            },
        )
    }

    fn create_center_sphere(context: &Context, radius: f32) -> Gm<Mesh, ColorMaterial> {
        // Simple octahedron as center indicator
        let r = radius;
        let positions = vec![
            // Top pyramid
            vec3(r, 0.0, 0.0), vec3(0.0, r, 0.0), vec3(0.0, 0.0, r),
            vec3(0.0, r, 0.0), vec3(-r, 0.0, 0.0), vec3(0.0, 0.0, r),
            vec3(-r, 0.0, 0.0), vec3(0.0, -r, 0.0), vec3(0.0, 0.0, r),
            vec3(0.0, -r, 0.0), vec3(r, 0.0, 0.0), vec3(0.0, 0.0, r),
            // Bottom pyramid
            vec3(r, 0.0, 0.0), vec3(0.0, 0.0, -r), vec3(0.0, r, 0.0),
            vec3(0.0, r, 0.0), vec3(0.0, 0.0, -r), vec3(-r, 0.0, 0.0),
            vec3(-r, 0.0, 0.0), vec3(0.0, 0.0, -r), vec3(0.0, -r, 0.0),
            vec3(0.0, -r, 0.0), vec3(0.0, 0.0, -r), vec3(r, 0.0, 0.0),
        ];

        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            ..Default::default()
        };

        Gm::new(
            Mesh::new(context, &cpu_mesh),
            ColorMaterial {
                color: GizmoColors::CENTER,
                ..Default::default()
            },
        )
    }

    fn create_ring(context: &Context, radius: f32, color: Srgba, axis: i32) -> Gm<Mesh, ColorMaterial> {
        let segments = 32;
        let mut positions = Vec::new();

        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

            let (p1, p2) = match axis {
                0 => ( // X axis - YZ plane
                    vec3(0.0, angle1.cos() * radius, angle1.sin() * radius),
                    vec3(0.0, angle2.cos() * radius, angle2.sin() * radius),
                ),
                1 => ( // Y axis - XZ plane
                    vec3(angle1.cos() * radius, 0.0, angle1.sin() * radius),
                    vec3(angle2.cos() * radius, 0.0, angle2.sin() * radius),
                ),
                _ => ( // Z axis - XY plane
                    vec3(angle1.cos() * radius, angle1.sin() * radius, 0.0),
                    vec3(angle2.cos() * radius, angle2.sin() * radius, 0.0),
                ),
            };

            positions.push(p1);
            positions.push(p2);
        }

        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            ..Default::default()
        };

        Gm::new(
            Mesh::new(context, &cpu_mesh),
            ColorMaterial {
                color,
                ..Default::default()
            },
        )
    }

    fn create_scale_cube(context: &Context, position: Vec3, size: f32, color: Srgba) -> Gm<Mesh, ColorMaterial> {
        let s = size / 2.0;
        let p = position;

        // Simple cube faces
        let positions = vec![
            // Front face
            p + vec3(-s, -s, s), p + vec3(s, -s, s), p + vec3(s, s, s),
            p + vec3(-s, -s, s), p + vec3(s, s, s), p + vec3(-s, s, s),
            // Back face
            p + vec3(s, -s, -s), p + vec3(-s, -s, -s), p + vec3(-s, s, -s),
            p + vec3(s, -s, -s), p + vec3(-s, s, -s), p + vec3(s, s, -s),
            // Top face
            p + vec3(-s, s, -s), p + vec3(-s, s, s), p + vec3(s, s, s),
            p + vec3(-s, s, -s), p + vec3(s, s, s), p + vec3(s, s, -s),
            // Bottom face
            p + vec3(-s, -s, -s), p + vec3(s, -s, -s), p + vec3(s, -s, s),
            p + vec3(-s, -s, -s), p + vec3(s, -s, s), p + vec3(-s, -s, s),
            // Right face
            p + vec3(s, -s, -s), p + vec3(s, s, -s), p + vec3(s, s, s),
            p + vec3(s, -s, -s), p + vec3(s, s, s), p + vec3(s, -s, s),
            // Left face
            p + vec3(-s, -s, s), p + vec3(-s, s, s), p + vec3(-s, s, -s),
            p + vec3(-s, -s, s), p + vec3(-s, s, -s), p + vec3(-s, -s, -s),
        ];

        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            ..Default::default()
        };

        Gm::new(
            Mesh::new(context, &cpu_mesh),
            ColorMaterial {
                color,
                ..Default::default()
            },
        )
    }

    /// Update gizmo position and scale
    pub fn update(&mut self, position: Vec3, camera_distance: f32) {
        self.position = position;
        // Scale gizmo based on camera distance for consistent screen size
        self.scale = camera_distance * 0.15;
    }

    /// Set gizmo mode
    pub fn set_mode(&mut self, mode: GizmoMode) {
        self.mode = mode;
    }

    /// Get objects for rendering based on current mode
    pub fn get_render_objects(&self) -> Vec<&dyn Object> {
        if !self.visible {
            return Vec::new();
        }

        let mut objects: Vec<&dyn Object> = vec![&self.center];

        match self.mode {
            GizmoMode::Translate => {
                objects.push(&self.x_arrow);
                objects.push(&self.y_arrow);
                objects.push(&self.z_arrow);
                objects.push(&self.x_cone);
                objects.push(&self.y_cone);
                objects.push(&self.z_cone);
            }
            GizmoMode::Rotate => {
                objects.push(&self.x_ring);
                objects.push(&self.y_ring);
                objects.push(&self.z_ring);
            }
            GizmoMode::Scale => {
                objects.push(&self.x_arrow);
                objects.push(&self.y_arrow);
                objects.push(&self.z_arrow);
                objects.push(&self.x_cube);
                objects.push(&self.y_cube);
                objects.push(&self.z_cube);
            }
        }

        objects
    }

    /// Update transformations for all gizmo parts
    pub fn apply_transform(&mut self) {
        let transform = Mat4::from_translation(self.position) * Mat4::from_scale(self.scale);

        self.x_arrow.set_transformation(transform);
        self.y_arrow.set_transformation(transform);
        self.z_arrow.set_transformation(transform);
        self.x_cone.set_transformation(transform);
        self.y_cone.set_transformation(transform);
        self.z_cone.set_transformation(transform);
        self.center.set_transformation(transform);
        self.x_ring.set_transformation(transform);
        self.y_ring.set_transformation(transform);
        self.z_ring.set_transformation(transform);
        self.x_cube.set_transformation(transform);
        self.y_cube.set_transformation(transform);
        self.z_cube.set_transformation(transform);
    }
}

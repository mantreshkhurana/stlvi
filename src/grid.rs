use three_d::*;

#[allow(dead_code)]

/// Fusion 360-style grid colors
#[allow(dead_code)]
pub struct GridColors;

#[allow(dead_code)]
impl GridColors {
    pub const MAJOR: Srgba = Srgba::new(70, 75, 85, 140);
    pub const MINOR: Srgba = Srgba::new(55, 60, 68, 90);
    pub const CENTER_X: Srgba = Srgba::new(180, 60, 50, 180); // Red for X center line
    pub const CENTER_Z: Srgba = Srgba::new(50, 100, 180, 180); // Blue for Z center line
}

/// Create a Fusion 360-style grid for the XZ plane with major and minor lines
pub fn create_grid(context: &Context, size: f32, divisions: u32, _color: Srgba) -> Gm<Mesh, ColorMaterial> {
    let half_size = size / 2.0;
    let step = size / divisions as f32;

    let mut positions = Vec::new();

    // Lines along X axis (minor grid lines)
    for i in 0..=divisions {
        let z = -half_size + i as f32 * step;
        positions.push(vec3(-half_size, 0.0, z));
        positions.push(vec3(half_size, 0.0, z));
    }

    // Lines along Z axis (minor grid lines)
    for i in 0..=divisions {
        let x = -half_size + i as f32 * step;
        positions.push(vec3(x, 0.0, -half_size));
        positions.push(vec3(x, 0.0, half_size));
    }

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        ..Default::default()
    };

    let mesh = Mesh::new(context, &cpu_mesh);

    // Fusion 360 uses subtle gray grid lines
    Gm::new(
        mesh,
        ColorMaterial {
            color: GridColors::MINOR,
            ..Default::default()
        },
    )
}

/// Create major grid lines (every 5th line) - Fusion 360 style
#[allow(dead_code)]
pub fn create_major_grid(context: &Context, size: f32, major_divisions: u32) -> Gm<Mesh, ColorMaterial> {
    let half_size = size / 2.0;
    let step = size / major_divisions as f32;

    let mut positions = Vec::new();

    // Major lines along X axis
    for i in 0..=major_divisions {
        let z = -half_size + i as f32 * step;
        positions.push(vec3(-half_size, 0.0, z));
        positions.push(vec3(half_size, 0.0, z));
    }

    // Major lines along Z axis
    for i in 0..=major_divisions {
        let x = -half_size + i as f32 * step;
        positions.push(vec3(x, 0.0, -half_size));
        positions.push(vec3(x, 0.0, half_size));
    }

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        ..Default::default()
    };

    Gm::new(
        Mesh::new(context, &cpu_mesh),
        ColorMaterial {
            color: GridColors::MAJOR,
            ..Default::default()
        },
    )
}

/// Create axis lines at origin - Fusion 360 style colors
pub fn create_axis_lines(context: &Context, length: f32) -> Vec<Gm<Mesh, ColorMaterial>> {
    let mut axes = Vec::new();

    // X axis - Fusion 360 Red
    let x_positions = vec![vec3(0.0, 0.0, 0.0), vec3(length, 0.0, 0.0)];
    let x_mesh = CpuMesh {
        positions: Positions::F32(x_positions),
        ..Default::default()
    };
    axes.push(Gm::new(
        Mesh::new(context, &x_mesh),
        ColorMaterial {
            color: Srgba::new(220, 70, 60, 255), // Fusion 360 red
            ..Default::default()
        },
    ));

    // Y axis - Fusion 360 Green
    let y_positions = vec![vec3(0.0, 0.0, 0.0), vec3(0.0, length, 0.0)];
    let y_mesh = CpuMesh {
        positions: Positions::F32(y_positions),
        ..Default::default()
    };
    axes.push(Gm::new(
        Mesh::new(context, &y_mesh),
        ColorMaterial {
            color: Srgba::new(70, 175, 70, 255), // Fusion 360 green
            ..Default::default()
        },
    ));

    // Z axis - Fusion 360 Blue
    let z_positions = vec![vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, length)];
    let z_mesh = CpuMesh {
        positions: Positions::F32(z_positions),
        ..Default::default()
    };
    axes.push(Gm::new(
        Mesh::new(context, &z_mesh),
        ColorMaterial {
            color: Srgba::new(60, 130, 220, 255), // Fusion 360 blue
            ..Default::default()
        },
    ));

    axes
}

/// Create ground plane shadow receiver
#[allow(dead_code)]
pub fn create_ground_plane(context: &Context, size: f32) -> Gm<Mesh, PhysicalMaterial> {
    let cpu_mesh = CpuMesh::square();
    let mut mesh = Gm::new(
        Mesh::new(context, &cpu_mesh),
        PhysicalMaterial::new_transparent(
            context,
            &CpuMaterial {
                albedo: Srgba::new(128, 128, 128, 30),
                ..Default::default()
            },
        ),
    );

    mesh.set_transformation(
        Mat4::from_translation(vec3(0.0, -0.01, 0.0))
            * Mat4::from_angle_x(Deg(-90.0))
            * Mat4::from_scale(size),
    );

    mesh
}

use three_d::*;

#[allow(dead_code)]

/// Create a grid for the XZ plane
pub fn create_grid(context: &Context, size: f32, divisions: u32, color: Srgba) -> Gm<Mesh, ColorMaterial> {
    let half_size = size / 2.0;
    let step = size / divisions as f32;

    let mut positions = Vec::new();

    // Lines along X axis
    for i in 0..=divisions {
        let z = -half_size + i as f32 * step;
        positions.push(vec3(-half_size, 0.0, z));
        positions.push(vec3(half_size, 0.0, z));
    }

    // Lines along Z axis
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

    Gm::new(
        mesh,
        ColorMaterial {
            color,
            ..Default::default()
        },
    )
}

/// Create axis lines at origin
pub fn create_axis_lines(context: &Context, length: f32) -> Vec<Gm<Mesh, ColorMaterial>> {
    let mut axes = Vec::new();

    // X axis - Red
    let x_positions = vec![vec3(0.0, 0.0, 0.0), vec3(length, 0.0, 0.0)];
    let x_mesh = CpuMesh {
        positions: Positions::F32(x_positions),
        ..Default::default()
    };
    axes.push(Gm::new(
        Mesh::new(context, &x_mesh),
        ColorMaterial {
            color: Srgba::new(255, 80, 80, 255),
            ..Default::default()
        },
    ));

    // Y axis - Green
    let y_positions = vec![vec3(0.0, 0.0, 0.0), vec3(0.0, length, 0.0)];
    let y_mesh = CpuMesh {
        positions: Positions::F32(y_positions),
        ..Default::default()
    };
    axes.push(Gm::new(
        Mesh::new(context, &y_mesh),
        ColorMaterial {
            color: Srgba::new(80, 255, 80, 255),
            ..Default::default()
        },
    ));

    // Z axis - Blue
    let z_positions = vec![vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, length)];
    let z_mesh = CpuMesh {
        positions: Positions::F32(z_positions),
        ..Default::default()
    };
    axes.push(Gm::new(
        Mesh::new(context, &z_mesh),
        ColorMaterial {
            color: Srgba::new(80, 80, 255, 255),
            ..Default::default()
        },
    ));

    axes
}

/// Create ground plane shadow receiver
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

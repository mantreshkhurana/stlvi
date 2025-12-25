use three_d::*;

#[allow(dead_code)]
#[allow(unused)]

/// Axis cube gizmo for orientation reference
pub struct AxisCube {
    pub cube: Gm<Mesh, PhysicalMaterial>,
    pub x_arrow: Gm<Mesh, ColorMaterial>,
    pub y_arrow: Gm<Mesh, ColorMaterial>,
    pub z_arrow: Gm<Mesh, ColorMaterial>,
    pub labels: Vec<(String, Vec3, Srgba)>,
}

impl AxisCube {
    pub fn new(context: &Context) -> Self {
        // Create cube
        let cube_mesh = CpuMesh::cube();
        let cube = Gm::new(
            Mesh::new(context, &cube_mesh),
            PhysicalMaterial::new_opaque(
                context,
                &CpuMaterial {
                    albedo: Srgba::new(60, 60, 65, 255),
                    roughness: 0.8,
                    metallic: 0.1,
                    ..Default::default()
                },
            ),
        );

        // Create axis arrows
        let arrow_length = 1.8;
        let x_arrow = Self::create_arrow(context, vec3(arrow_length, 0.0, 0.0), Srgba::new(255, 80, 80, 255));
        let y_arrow = Self::create_arrow(context, vec3(0.0, arrow_length, 0.0), Srgba::new(80, 255, 80, 255));
        let z_arrow = Self::create_arrow(context, vec3(0.0, 0.0, arrow_length), Srgba::new(80, 120, 255, 255));

        // Labels
        let labels = vec![
            ("X".to_string(), vec3(2.2, 0.0, 0.0), Srgba::new(255, 80, 80, 255)),
            ("Y".to_string(), vec3(0.0, 2.2, 0.0), Srgba::new(80, 255, 80, 255)),
            ("Z".to_string(), vec3(0.0, 0.0, 2.2), Srgba::new(80, 120, 255, 255)),
        ];

        Self {
            cube,
            x_arrow,
            y_arrow,
            z_arrow,
            labels,
        }
    }

    fn create_arrow(context: &Context, direction: Vec3, color: Srgba) -> Gm<Mesh, ColorMaterial> {
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

    /// Get the axis cube objects for rendering
    pub fn get_objects(&self) -> Vec<&dyn Object> {
        vec![&self.cube, &self.x_arrow, &self.y_arrow, &self.z_arrow]
    }
}

/// Handle click on axis cube to change view
pub fn handle_axis_cube_click(
    click_pos: (f32, f32),
    viewport: Viewport,
    camera: &Camera,
) -> Option<super::state::CameraView> {
    let size = 100;
    let gizmo_x = (viewport.width - size - 20) as f32;
    let gizmo_y = (viewport.height - size - 20) as f32;

    // Check if click is in gizmo area
    if click_pos.0 >= gizmo_x
        && click_pos.0 <= gizmo_x + size as f32
        && click_pos.1 >= gizmo_y
        && click_pos.1 <= gizmo_y + size as f32
    {
        // Determine which face was clicked based on camera direction
        let dir = camera.view_direction();

        // Simple heuristic: which axis is most aligned with view
        let abs_x = dir.x.abs();
        let abs_y = dir.y.abs();
        let abs_z = dir.z.abs();

        if abs_x >= abs_y && abs_x >= abs_z {
            if dir.x > 0.0 {
                Some(super::state::CameraView::Left)
            } else {
                Some(super::state::CameraView::Right)
            }
        } else if abs_y >= abs_x && abs_y >= abs_z {
            if dir.y > 0.0 {
                Some(super::state::CameraView::Bottom)
            } else {
                Some(super::state::CameraView::Top)
            }
        } else {
            if dir.z > 0.0 {
                Some(super::state::CameraView::Back)
            } else {
                Some(super::state::CameraView::Front)
            }
        }
    } else {
        None
    }
}

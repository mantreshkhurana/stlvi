use serde::{Deserialize, Serialize};
use three_d::*;

/// Transform state for the loaded model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelTransform {
    pub translation: [f32; 3],
    pub rotation: [f32; 3],    // Euler angles in degrees
    pub scale: f32,
}

impl Default for ModelTransform {
    fn default() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: 1.0,
        }
    }
}

impl ModelTransform {
    pub fn to_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(vec3(
            self.translation[0],
            self.translation[1],
            self.translation[2],
        ));
        let rotation_x = Mat4::from_angle_x(Deg(self.rotation[0]));
        let rotation_y = Mat4::from_angle_y(Deg(self.rotation[1]));
        let rotation_z = Mat4::from_angle_z(Deg(self.rotation[2]));
        let scale = Mat4::from_scale(self.scale);

        translation * rotation_z * rotation_y * rotation_x * scale
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// View settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ViewSettings {
    pub show_grid: bool,
    pub show_axis_cube: bool,
    pub wireframe_mode: bool,
    pub show_normals: bool,
    pub background_color: [f32; 3],
    pub model_color: [f32; 3],
    pub grid_size: f32,
    pub grid_divisions: u32,
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            show_grid: true,
            show_axis_cube: true,
            wireframe_mode: false,
            show_normals: false,
            background_color: [0.1, 0.1, 0.12],
            model_color: [0.8, 0.8, 0.85],
            grid_size: 10.0,
            grid_divisions: 20,
        }
    }
}

/// Camera preset views
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraView {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Isometric,
    Custom,
}

impl CameraView {
    pub fn get_position(&self, distance: f32) -> Vec3 {
        match self {
            CameraView::Front => vec3(0.0, 0.0, distance),
            CameraView::Back => vec3(0.0, 0.0, -distance),
            CameraView::Left => vec3(-distance, 0.0, 0.0),
            CameraView::Right => vec3(distance, 0.0, 0.0),
            CameraView::Top => vec3(0.0, distance, 0.0),
            CameraView::Bottom => vec3(0.0, -distance, 0.0),
            CameraView::Isometric => vec3(distance * 0.7, distance * 0.7, distance * 0.7),
            CameraView::Custom => vec3(distance * 0.7, distance * 0.5, distance * 0.7),
        }
    }

    pub fn get_up(&self) -> Vec3 {
        match self {
            CameraView::Top => vec3(0.0, 0.0, -1.0),
            CameraView::Bottom => vec3(0.0, 0.0, 1.0),
            _ => vec3(0.0, 1.0, 0.0),
        }
    }
}

/// Edit mode for the model
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EditMode {
    View,
    Translate,
    Rotate,
    Scale,
}

/// Main application state
pub struct AppState {
    pub current_file: Option<String>,
    pub model_transform: ModelTransform,
    pub view_settings: ViewSettings,
    pub edit_mode: EditMode,
    pub current_view: CameraView,
    pub model_info: Option<ModelInfo>,
    pub unsaved_changes: bool,
    pub show_settings_panel: bool,
    pub show_transform_panel: bool,
    pub show_info_panel: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_file: None,
            model_transform: ModelTransform::default(),
            view_settings: ViewSettings::default(),
            edit_mode: EditMode::View,
            current_view: CameraView::Isometric,
            model_info: None,
            unsaved_changes: false,
            show_settings_panel: false,
            show_transform_panel: true,
            show_info_panel: true,
        }
    }
}

/// Information about the loaded model
#[derive(Clone, Debug)]
pub struct ModelInfo {
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub bounds_min: [f32; 3],
    pub bounds_max: [f32; 3],
    pub dimensions: [f32; 3],
}

impl ModelInfo {
    pub fn from_mesh(positions: &[[f32; 3]]) -> Self {
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];

        for pos in positions {
            for i in 0..3 {
                min[i] = min[i].min(pos[i]);
                max[i] = max[i].max(pos[i]);
            }
        }

        Self {
            vertex_count: positions.len(),
            triangle_count: positions.len() / 3,
            bounds_min: min,
            bounds_max: max,
            dimensions: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
        }
    }
}

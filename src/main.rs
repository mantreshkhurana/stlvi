mod axis_cube;
mod checker;
mod gizmo;
mod grid;
mod mesh_utils;
mod state;
mod theme;
mod ui;

use checker::FileRevisions;
use state::{AppState, CameraView, EditMode, ModelInfo};
use three_d::*;
use ui::MenuAction;

fn main() -> anyhow::Result<()> {
    // Get filename from args (optional now)
    let initial_file = std::env::args().nth(1);

    // Create window
    let window = Window::new(WindowSettings {
        title: "STLVI - STL Viewer".to_string(),
        max_size: Some((1920, 1080)),
        ..Default::default()
    })?;

    let context = window.gl();

    // Initialize state
    let mut state = AppState::default();
    let mut show_shortcuts = false;
    let mut show_about = false;

    // Camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(3.0, 2.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut orbit_control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 100.0);

    // Lighting
    let ambient = AmbientLight::new(&context, 0.4, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, &vec3(-1.0, -1.0, -1.0));
    let directional2 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(1.0, 0.5, 0.5));

    // Grid
    let mut grid_mesh = grid::create_grid(&context, 10.0, 20, Srgba::new(80, 80, 90, 255));
    let axis_lines = grid::create_axis_lines(&context, 1.5);

    // Axis cube gizmo (for future use with orientation display)
    let _axis_cube = axis_cube::AxisCube::new(&context);

    // Transform gizmo for model manipulation
    let mut transform_gizmo = gizmo::TransformGizmo::new(&context);

    // Model storage
    let mut model: Option<Gm<Mesh, PhysicalMaterial>> = None;
    let mut model_positions: Vec<[f32; 3]> = Vec::new();
    let mut model_normals: Vec<[f32; 3]> = Vec::new();

    // File watcher
    let mut file_watcher: Option<FileRevisions> = None;

    // Load initial file if provided
    if let Some(ref path) = initial_file {
        if let Ok((mut positions, normals)) =
            mesh_utils::load_stl(std::path::Path::new(path))
        {
            let original_positions = positions.clone();
            mesh_utils::normalize_mesh(&mut positions);

            let cpu_mesh = mesh_utils::create_mesh(&context, &positions, &normals);
            let material = PhysicalMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(204, 204, 217, 255),
                    roughness: 0.5,
                    metallic: 0.1,
                    ..Default::default()
                },
            );
            model = Some(Gm::new(Mesh::new(&context, &cpu_mesh), material));
            model_positions = positions;
            model_normals = normals;
            state.current_file = Some(path.clone());
            state.model_info = Some(ModelInfo::from_mesh(&original_positions));

            // Setup file watcher
            if let Ok(watcher) = FileRevisions::from_path(std::path::Path::new(path)) {
                file_watcher = Some(watcher);
            }
        }
    }

    // FPS tracking
    let mut frame_times: Vec<f32> = Vec::new();
    let mut last_time = std::time::Instant::now();

    // Create GUI once outside the render loop to prevent flickering
    let mut gui = three_d::GUI::new(&context);

    // Main loop
    window.render_loop(move |mut frame_input| {
        // Calculate FPS
        let now = std::time::Instant::now();
        let delta = now.duration_since(last_time).as_secs_f32();
        last_time = now;
        frame_times.push(delta);
        if frame_times.len() > 60 {
            frame_times.remove(0);
        }
        let avg_delta = frame_times.iter().sum::<f32>() / frame_times.len() as f32;
        let fps = if avg_delta > 0.0 { 1.0 / avg_delta } else { 0.0 };

        // Update camera
        camera.set_viewport(frame_input.viewport);

        // Handle keyboard shortcuts
        for event in frame_input.events.iter() {
            match event {
                Event::KeyPress { kind, modifiers, .. } => {
                    match kind {
                        Key::Escape => state.edit_mode = EditMode::View,
                        Key::G => state.edit_mode = EditMode::Translate,
                        Key::R => state.edit_mode = EditMode::Rotate,
                        Key::S if !modifiers.ctrl => state.edit_mode = EditMode::Scale,
                        Key::Num1 => {
                            state.current_view = CameraView::Front;
                            let pos = state.current_view.get_position(5.0);
                            camera.set_view(pos, vec3(0.0, 0.0, 0.0), state.current_view.get_up());
                        }
                        Key::Num2 => {
                            state.current_view = CameraView::Back;
                            let pos = state.current_view.get_position(5.0);
                            camera.set_view(pos, vec3(0.0, 0.0, 0.0), state.current_view.get_up());
                        }
                        Key::Num3 => {
                            state.current_view = CameraView::Left;
                            let pos = state.current_view.get_position(5.0);
                            camera.set_view(pos, vec3(0.0, 0.0, 0.0), state.current_view.get_up());
                        }
                        Key::Num4 => {
                            state.current_view = CameraView::Right;
                            let pos = state.current_view.get_position(5.0);
                            camera.set_view(pos, vec3(0.0, 0.0, 0.0), state.current_view.get_up());
                        }
                        Key::Num5 => {
                            state.current_view = CameraView::Top;
                            let pos = state.current_view.get_position(5.0);
                            camera.set_view(pos, vec3(0.0, 0.0, 0.0), state.current_view.get_up());
                        }
                        Key::Num6 => {
                            state.current_view = CameraView::Bottom;
                            let pos = state.current_view.get_position(5.0);
                            camera.set_view(pos, vec3(0.0, 0.0, 0.0), state.current_view.get_up());
                        }
                        Key::Num7 => {
                            state.current_view = CameraView::Isometric;
                            let pos = state.current_view.get_position(5.0);
                            camera.set_view(pos, vec3(0.0, 0.0, 0.0), state.current_view.get_up());
                        }
                        Key::Home => {
                            state.current_view = CameraView::Isometric;
                            let pos = state.current_view.get_position(5.0);
                            camera.set_view(pos, vec3(0.0, 0.0, 0.0), state.current_view.get_up());
                            state.model_transform.reset();
                        }
                        Key::O if modifiers.ctrl => {
                            // Open file dialog
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("STL files", &["stl", "STL"])
                                .pick_file()
                            {
                                load_model(
                                    &context,
                                    &path,
                                    &mut model,
                                    &mut model_positions,
                                    &mut model_normals,
                                    &mut state,
                                    &mut file_watcher,
                                );
                            }
                        }
                        Key::S if modifiers.ctrl && modifiers.shift => {
                            // Save As
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("STL files", &["stl"])
                                .save_file()
                            {
                                let transformed = mesh_utils::transform_positions(
                                    &model_positions,
                                    state.model_transform.translation,
                                    state.model_transform.rotation,
                                    state.model_transform.scale,
                                );
                                if mesh_utils::save_stl(&path, &transformed, &model_normals, true).is_ok() {
                                    state.current_file = Some(path.to_string_lossy().to_string());
                                    state.unsaved_changes = false;
                                }
                            }
                        }
                        Key::S if modifiers.ctrl => {
                            // Save
                            if let Some(ref path) = state.current_file {
                                let transformed = mesh_utils::transform_positions(
                                    &model_positions,
                                    state.model_transform.translation,
                                    state.model_transform.rotation,
                                    state.model_transform.scale,
                                );
                                if mesh_utils::save_stl(
                                    std::path::Path::new(path),
                                    &transformed,
                                    &model_normals,
                                    true,
                                ).is_ok() {
                                    state.unsaved_changes = false;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Handle camera control (only when not hovering UI)
        let orbit_event = orbit_control.handle_events(&mut camera, &mut frame_input.events);
        if orbit_event {
            state.current_view = CameraView::Custom;
            // Sync zoom level with camera distance
            let distance = camera.position().magnitude();
            state.zoom_level = distance.clamp(1.0, 20.0);
        }

        // Check for file changes (hot reload)
        if let Some(ref mut watcher) = file_watcher {
            if watcher.changed().unwrap_or(false) {
                if let Some(ref path) = state.current_file {
                    if let Ok((mut positions, normals)) =
                        mesh_utils::load_stl(std::path::Path::new(path))
                    {
                        let original_positions = positions.clone();
                        mesh_utils::normalize_mesh(&mut positions);

                        let cpu_mesh = mesh_utils::create_mesh(&context, &positions, &normals);
                        let material = PhysicalMaterial::new_opaque(
                            &context,
                            &CpuMaterial {
                                albedo: Srgba::new(
                                    (state.view_settings.model_color[0] * 255.0) as u8,
                                    (state.view_settings.model_color[1] * 255.0) as u8,
                                    (state.view_settings.model_color[2] * 255.0) as u8,
                                    255,
                                ),
                                roughness: 0.5,
                                metallic: 0.1,
                                ..Default::default()
                            },
                        );
                        model = Some(Gm::new(Mesh::new(&context, &cpu_mesh), material));
                        model_positions = positions;
                        model_normals = normals;
                        state.model_info = Some(ModelInfo::from_mesh(&original_positions));
                    }
                }
            }
        }

        // GUI update
        let mut menu_action = MenuAction::None;
        let mut zoom_changed = false;

        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                // Apply Fusion 360-style theme
                theme::apply_fusion_theme(ctx);

                menu_action = ui::draw_menu_bar(ctx, &mut state);
                ui::draw_transform_panel(ctx, &mut state);
                ui::draw_info_panel(ctx, &state);
                ui::draw_settings_panel(ctx, &mut state);
                zoom_changed = ui::draw_status_bar(ctx, &mut state, fps);

                if show_shortcuts {
                    ui::draw_shortcuts_window(ctx, &mut show_shortcuts);
                }
                if show_about {
                    ui::draw_about_window(ctx, &mut show_about);
                }
            },
        );

        // Handle zoom slider changes
        if zoom_changed {
            let target = vec3(0.0, 0.0, 0.0);
            let direction = (camera.position() - target).normalize();
            let new_pos = target + direction * state.zoom_level;
            let up = *camera.up();
            camera.set_view(new_pos, target, up);
        }

        // Handle menu actions
        match menu_action {
            MenuAction::OpenFile => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("STL files", &["stl", "STL"])
                    .pick_file()
                {
                    load_model(
                        &context,
                        &path,
                        &mut model,
                        &mut model_positions,
                        &mut model_normals,
                        &mut state,
                        &mut file_watcher,
                    );
                }
            }
            MenuAction::Save => {
                if let Some(ref path) = state.current_file {
                    let transformed = mesh_utils::transform_positions(
                        &model_positions,
                        state.model_transform.translation,
                        state.model_transform.rotation,
                        state.model_transform.scale,
                    );
                    if mesh_utils::save_stl(
                        std::path::Path::new(path),
                        &transformed,
                        &model_normals,
                        true,
                    ).is_ok() {
                        state.unsaved_changes = false;
                    }
                }
            }
            MenuAction::SaveAs | MenuAction::ExportBinary => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("STL files", &["stl"])
                    .save_file()
                {
                    let transformed = mesh_utils::transform_positions(
                        &model_positions,
                        state.model_transform.translation,
                        state.model_transform.rotation,
                        state.model_transform.scale,
                    );
                    if mesh_utils::save_stl(&path, &transformed, &model_normals, true).is_ok() {
                        if matches!(menu_action, MenuAction::SaveAs) {
                            state.current_file = Some(path.to_string_lossy().to_string());
                        }
                        state.unsaved_changes = false;
                    }
                }
            }
            MenuAction::ExportAscii => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("STL files", &["stl"])
                    .save_file()
                {
                    let transformed = mesh_utils::transform_positions(
                        &model_positions,
                        state.model_transform.translation,
                        state.model_transform.rotation,
                        state.model_transform.scale,
                    );
                    let _ = mesh_utils::save_stl(&path, &transformed, &model_normals, false);
                }
            }
            MenuAction::RecalculateNormals => {
                model_normals = mesh_utils::recalculate_normals(&model_positions);
                // Rebuild mesh with new normals
                let cpu_mesh = mesh_utils::create_mesh(&context, &model_positions, &model_normals);
                let material = PhysicalMaterial::new_opaque(
                    &context,
                    &CpuMaterial {
                        albedo: Srgba::new(
                            (state.view_settings.model_color[0] * 255.0) as u8,
                            (state.view_settings.model_color[1] * 255.0) as u8,
                            (state.view_settings.model_color[2] * 255.0) as u8,
                            255,
                        ),
                        roughness: 0.5,
                        metallic: 0.1,
                        ..Default::default()
                    },
                );
                model = Some(Gm::new(Mesh::new(&context, &cpu_mesh), material));
                state.unsaved_changes = true;
            }
            MenuAction::CenterModel => {
                state.model_transform.translation = [0.0, 0.0, 0.0];
                state.unsaved_changes = true;
            }
            MenuAction::SetCameraView(view) => {
                let pos = view.get_position(5.0);
                camera.set_view(pos, vec3(0.0, 0.0, 0.0), view.get_up());
            }
            MenuAction::ShowShortcuts => {
                show_shortcuts = true;
            }
            MenuAction::ShowAbout => {
                show_about = true;
            }
            MenuAction::Quit => {
                return FrameOutput {
                    exit: true,
                    ..Default::default()
                };
            }
            MenuAction::None => {}
        }

        // Update model transform
        if let Some(ref mut m) = model {
            m.set_transformation(state.model_transform.to_matrix());
        }

        // Update gizmo visibility and mode based on edit mode
        transform_gizmo.visible = model.is_some() && state.edit_mode != EditMode::View;
        match state.edit_mode {
            EditMode::Translate => transform_gizmo.set_mode(gizmo::GizmoMode::Translate),
            EditMode::Rotate => transform_gizmo.set_mode(gizmo::GizmoMode::Rotate),
            EditMode::Scale => transform_gizmo.set_mode(gizmo::GizmoMode::Scale),
            EditMode::View => {}
        }

        // Update gizmo position to follow model
        if model.is_some() {
            let camera_distance = camera.position().magnitude();
            let model_center = vec3(
                state.model_transform.translation[0],
                state.model_transform.translation[1] + 0.5, // Slightly above model center
                state.model_transform.translation[2],
            );
            transform_gizmo.update(model_center, camera_distance);
            transform_gizmo.apply_transform();
        }

        // Update grid based on settings
        if state.view_settings.show_grid {
            grid_mesh = grid::create_grid(
                &context,
                state.view_settings.grid_size,
                state.view_settings.grid_divisions,
                Srgba::new(80, 80, 90, 255),
            );
        }

        // Render
        let bg = &state.view_settings.background_color;
        let _ = frame_input
            .screen()
            .clear(ClearState::color_and_depth(bg[0], bg[1], bg[2], 1.0, 1.0))
            .render(
                &camera,
                {
                    let mut objects: Vec<&dyn Object> = Vec::new();

                    // Grid
                    if state.view_settings.show_grid {
                        objects.push(&grid_mesh);
                        for axis in &axis_lines {
                            objects.push(axis);
                        }
                    }

                    // Model
                    if let Some(ref m) = model {
                        objects.push(m);
                    }

                    // Transform gizmo
                    for gizmo_obj in transform_gizmo.get_render_objects() {
                        objects.push(gizmo_obj);
                    }

                    objects
                },
                &[&ambient, &directional, &directional2],
            )
            .write(|| {
                gui.render()
            });

        FrameOutput::default()
    });

    Ok(())
}

fn load_model(
    context: &Context,
    path: &std::path::Path,
    model: &mut Option<Gm<Mesh, PhysicalMaterial>>,
    model_positions: &mut Vec<[f32; 3]>,
    model_normals: &mut Vec<[f32; 3]>,
    state: &mut AppState,
    file_watcher: &mut Option<FileRevisions>,
) {
    if let Ok((mut positions, normals)) = mesh_utils::load_stl(path) {
        let original_positions = positions.clone();
        mesh_utils::normalize_mesh(&mut positions);

        let cpu_mesh = mesh_utils::create_mesh(context, &positions, &normals);
        let material = PhysicalMaterial::new_opaque(
            context,
            &CpuMaterial {
                albedo: Srgba::new(
                    (state.view_settings.model_color[0] * 255.0) as u8,
                    (state.view_settings.model_color[1] * 255.0) as u8,
                    (state.view_settings.model_color[2] * 255.0) as u8,
                    255,
                ),
                roughness: 0.5,
                metallic: 0.1,
                ..Default::default()
            },
        );
        *model = Some(Gm::new(Mesh::new(context, &cpu_mesh), material));
        *model_positions = positions;
        *model_normals = normals;
        state.current_file = Some(path.to_string_lossy().to_string());
        state.model_info = Some(ModelInfo::from_mesh(&original_positions));
        state.model_transform.reset();
        state.unsaved_changes = false;

        // Setup file watcher
        if let Ok(watcher) = FileRevisions::from_path(path) {
            *file_watcher = Some(watcher);
        }
    }
}

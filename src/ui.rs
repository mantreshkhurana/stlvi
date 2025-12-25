use crate::state::{AppState, CameraView, EditMode};
use three_d::egui::{self, *};

/// Draw the main menu bar
pub fn draw_menu_bar(ctx: &Context, state: &mut AppState) -> MenuAction {
    let mut action = MenuAction::None;

    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            // File menu
            ui.menu_button("File", |ui| {
                if ui.button("Open STL...").clicked() {
                    action = MenuAction::OpenFile;
                    ui.close_menu();
                }

                ui.separator();

                if ui.add_enabled(state.current_file.is_some(), Button::new("Save")).clicked() {
                    action = MenuAction::Save;
                    ui.close_menu();
                }

                if ui.add_enabled(state.current_file.is_some(), Button::new("Save As...")).clicked() {
                    action = MenuAction::SaveAs;
                    ui.close_menu();
                }

                ui.separator();

                ui.menu_button("Export", |ui| {
                    if ui.add_enabled(state.current_file.is_some(), Button::new("Binary STL")).clicked() {
                        action = MenuAction::ExportBinary;
                        ui.close_menu();
                    }
                    if ui.add_enabled(state.current_file.is_some(), Button::new("ASCII STL")).clicked() {
                        action = MenuAction::ExportAscii;
                        ui.close_menu();
                    }
                });

                ui.separator();

                if ui.button("Quit").clicked() {
                    action = MenuAction::Quit;
                    ui.close_menu();
                }
            });

            // Edit menu
            ui.menu_button("Edit", |ui| {
                if ui.add_enabled(state.current_file.is_some(), Button::new("Reset Transform")).clicked() {
                    state.model_transform.reset();
                    state.unsaved_changes = true;
                    ui.close_menu();
                }

                ui.separator();

                if ui.add_enabled(state.current_file.is_some(), Button::new("Recalculate Normals")).clicked() {
                    action = MenuAction::RecalculateNormals;
                    ui.close_menu();
                }

                if ui.add_enabled(state.current_file.is_some(), Button::new("Center Model")).clicked() {
                    action = MenuAction::CenterModel;
                    ui.close_menu();
                }
            });

            // View menu
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut state.view_settings.show_grid, "Show Grid");
                ui.checkbox(&mut state.view_settings.show_axis_cube, "Show Axis Cube");
                ui.checkbox(&mut state.view_settings.wireframe_mode, "Wireframe Mode");
                ui.checkbox(&mut state.view_settings.show_normals, "Show Normals");

                ui.separator();

                ui.menu_button("Camera View", |ui| {
                    if ui.selectable_label(state.current_view == CameraView::Front, "Front").clicked() {
                        state.current_view = CameraView::Front;
                        action = MenuAction::SetCameraView(CameraView::Front);
                        ui.close_menu();
                    }
                    if ui.selectable_label(state.current_view == CameraView::Back, "Back").clicked() {
                        state.current_view = CameraView::Back;
                        action = MenuAction::SetCameraView(CameraView::Back);
                        ui.close_menu();
                    }
                    if ui.selectable_label(state.current_view == CameraView::Left, "Left").clicked() {
                        state.current_view = CameraView::Left;
                        action = MenuAction::SetCameraView(CameraView::Left);
                        ui.close_menu();
                    }
                    if ui.selectable_label(state.current_view == CameraView::Right, "Right").clicked() {
                        state.current_view = CameraView::Right;
                        action = MenuAction::SetCameraView(CameraView::Right);
                        ui.close_menu();
                    }
                    if ui.selectable_label(state.current_view == CameraView::Top, "Top").clicked() {
                        state.current_view = CameraView::Top;
                        action = MenuAction::SetCameraView(CameraView::Top);
                        ui.close_menu();
                    }
                    if ui.selectable_label(state.current_view == CameraView::Bottom, "Bottom").clicked() {
                        state.current_view = CameraView::Bottom;
                        action = MenuAction::SetCameraView(CameraView::Bottom);
                        ui.close_menu();
                    }
                    if ui.selectable_label(state.current_view == CameraView::Isometric, "Isometric").clicked() {
                        state.current_view = CameraView::Isometric;
                        action = MenuAction::SetCameraView(CameraView::Isometric);
                        ui.close_menu();
                    }
                });

                ui.separator();

                ui.checkbox(&mut state.show_settings_panel, "Settings Panel");
                ui.checkbox(&mut state.show_transform_panel, "Transform Panel");
                ui.checkbox(&mut state.show_info_panel, "Info Panel");
            });

            // Help menu
            ui.menu_button("Help", |ui| {
                if ui.button("Keyboard Shortcuts").clicked() {
                    action = MenuAction::ShowShortcuts;
                    ui.close_menu();
                }
                if ui.button("About").clicked() {
                    action = MenuAction::ShowAbout;
                    ui.close_menu();
                }
            });

            // Show file name on the right
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(ref file) = state.current_file {
                    let display_name = std::path::Path::new(file)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(file);
                    let label = if state.unsaved_changes {
                        format!("{}*", display_name)
                    } else {
                        display_name.to_string()
                    };
                    ui.label(RichText::new(label).monospace());
                }
            });
        });
    });

    action
}

/// Draw the transform panel on the left side
pub fn draw_transform_panel(ctx: &Context, state: &mut AppState) {
    if !state.show_transform_panel || state.current_file.is_none() {
        return;
    }

    egui::SidePanel::left("transform_panel")
        .resizable(true)
        .default_width(220.0)
        .show(ctx, |ui| {
            ui.heading("Transform");
            ui.separator();

            // Edit mode selector
            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.selectable_value(&mut state.edit_mode, EditMode::View, "View");
                ui.selectable_value(&mut state.edit_mode, EditMode::Translate, "Move");
                ui.selectable_value(&mut state.edit_mode, EditMode::Rotate, "Rotate");
                ui.selectable_value(&mut state.edit_mode, EditMode::Scale, "Scale");
            });

            ui.add_space(10.0);

            // Translation
            ui.collapsing("Position", |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label("X:");
                    changed |= ui
                        .add(DragValue::new(&mut state.model_transform.translation[0]).speed(0.01))
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Y:");
                    changed |= ui
                        .add(DragValue::new(&mut state.model_transform.translation[1]).speed(0.01))
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Z:");
                    changed |= ui
                        .add(DragValue::new(&mut state.model_transform.translation[2]).speed(0.01))
                        .changed();
                });
                if changed {
                    state.unsaved_changes = true;
                }
            });

            // Rotation
            ui.collapsing("Rotation", |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label("X:");
                    changed |= ui
                        .add(
                            DragValue::new(&mut state.model_transform.rotation[0])
                                .speed(1.0)
                                .suffix("°"),
                        )
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Y:");
                    changed |= ui
                        .add(
                            DragValue::new(&mut state.model_transform.rotation[1])
                                .speed(1.0)
                                .suffix("°"),
                        )
                        .changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Z:");
                    changed |= ui
                        .add(
                            DragValue::new(&mut state.model_transform.rotation[2])
                                .speed(1.0)
                                .suffix("°"),
                        )
                        .changed();
                });
                if changed {
                    state.unsaved_changes = true;
                }
            });

            // Scale
            ui.collapsing("Scale", |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label("Uniform:");
                    changed |= ui
                        .add(
                            DragValue::new(&mut state.model_transform.scale)
                                .speed(0.01)
                                .clamp_range(0.01..=100.0),
                        )
                        .changed();
                });
                if changed {
                    state.unsaved_changes = true;
                }
            });

            ui.add_space(10.0);

            if ui.button("Reset Transform").clicked() {
                state.model_transform.reset();
                state.unsaved_changes = true;
            }
        });
}

/// Draw the info panel on the right side
pub fn draw_info_panel(ctx: &Context, state: &AppState) {
    if !state.show_info_panel || state.model_info.is_none() {
        return;
    }

    egui::SidePanel::right("info_panel")
        .resizable(true)
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Model Info");
            ui.separator();

            if let Some(ref info) = state.model_info {
                ui.label(format!("Vertices: {}", info.vertex_count));
                ui.label(format!("Triangles: {}", info.triangle_count));

                ui.add_space(10.0);
                ui.label("Dimensions:");
                ui.label(format!("  X: {:.3}", info.dimensions[0]));
                ui.label(format!("  Y: {:.3}", info.dimensions[1]));
                ui.label(format!("  Z: {:.3}", info.dimensions[2]));

                ui.add_space(10.0);
                ui.label("Bounds Min:");
                ui.label(format!(
                    "  ({:.3}, {:.3}, {:.3})",
                    info.bounds_min[0], info.bounds_min[1], info.bounds_min[2]
                ));
                ui.label("Bounds Max:");
                ui.label(format!(
                    "  ({:.3}, {:.3}, {:.3})",
                    info.bounds_max[0], info.bounds_max[1], info.bounds_max[2]
                ));
            }
        });
}

/// Draw the settings panel (floating window)
pub fn draw_settings_panel(ctx: &Context, state: &mut AppState) {
    if !state.show_settings_panel {
        return;
    }

    egui::Window::new("Settings")
        .open(&mut state.show_settings_panel)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("View Settings");
            ui.separator();

            // Grid settings
            ui.collapsing("Grid", |ui| {
                ui.checkbox(&mut state.view_settings.show_grid, "Show Grid");
                ui.horizontal(|ui| {
                    ui.label("Size:");
                    ui.add(
                        DragValue::new(&mut state.view_settings.grid_size)
                            .speed(0.5)
                            .clamp_range(1.0..=100.0),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Divisions:");
                    ui.add(
                        DragValue::new(&mut state.view_settings.grid_divisions)
                            .speed(1)
                            .clamp_range(2..=100),
                    );
                });
            });

            // Colors
            ui.collapsing("Colors", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Background:");
                    let mut bg = state.view_settings.background_color;
                    if ui.color_edit_button_rgb(&mut bg).changed() {
                        state.view_settings.background_color = bg;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Model:");
                    let mut mc = state.view_settings.model_color;
                    if ui.color_edit_button_rgb(&mut mc).changed() {
                        state.view_settings.model_color = mc;
                    }
                });
            });

            // Display options
            ui.collapsing("Display", |ui| {
                ui.checkbox(&mut state.view_settings.wireframe_mode, "Wireframe Mode");
                ui.checkbox(&mut state.view_settings.show_normals, "Show Normals");
                ui.checkbox(&mut state.view_settings.show_axis_cube, "Show Axis Cube");
            });
        });
}

/// Draw status bar at the bottom
pub fn draw_status_bar(ctx: &Context, state: &AppState, fps: f32) {
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Edit mode indicator
            let mode_text = match state.edit_mode {
                EditMode::View => "View Mode",
                EditMode::Translate => "Translate Mode (G)",
                EditMode::Rotate => "Rotate Mode (R)",
                EditMode::Scale => "Scale Mode (S)",
            };
            ui.label(mode_text);

            ui.separator();

            // View indicator
            let view_text = match state.current_view {
                CameraView::Front => "Front",
                CameraView::Back => "Back",
                CameraView::Left => "Left",
                CameraView::Right => "Right",
                CameraView::Top => "Top",
                CameraView::Bottom => "Bottom",
                CameraView::Isometric => "Isometric",
                CameraView::Custom => "Custom",
            };
            ui.label(format!("View: {}", view_text));

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(format!("{:.0} FPS", fps));
            });
        });
    });
}

/// Draw keyboard shortcuts help window
pub fn draw_shortcuts_window(ctx: &Context, open: &mut bool) {
    egui::Window::new("Keyboard Shortcuts")
        .open(open)
        .resizable(false)
        .show(ctx, |ui| {
            egui::Grid::new("shortcuts_grid").show(ui, |ui| {
                ui.label("Ctrl+O");
                ui.label("Open file");
                ui.end_row();

                ui.label("Ctrl+S");
                ui.label("Save file");
                ui.end_row();

                ui.label("Ctrl+Shift+S");
                ui.label("Save as");
                ui.end_row();

                ui.label("G");
                ui.label("Translate mode");
                ui.end_row();

                ui.label("R");
                ui.label("Rotate mode");
                ui.end_row();

                ui.label("S");
                ui.label("Scale mode");
                ui.end_row();

                ui.label("Escape");
                ui.label("View mode");
                ui.end_row();

                ui.label("1-7");
                ui.label("Camera views (Front/Back/Left/Right/Top/Bottom/Isometric)");
                ui.end_row();

                ui.label("Home");
                ui.label("Reset view");
                ui.end_row();

                ui.label("Mouse drag");
                ui.label("Orbit camera");
                ui.end_row();

                ui.label("Scroll");
                ui.label("Zoom");
                ui.end_row();

                ui.label("Middle drag");
                ui.label("Pan camera");
                ui.end_row();
            });
        });
}

/// Draw about window
pub fn draw_about_window(ctx: &Context, open: &mut bool) {
    egui::Window::new("About STLVI")
        .open(open)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("STLVI");
                ui.label("Version 2.0.0");
                ui.add_space(10.0);
                ui.label("A powerful STL 3D model viewer");
                ui.label("with editing capabilities");
                ui.add_space(10.0);
                ui.hyperlink_to("GitHub", "https://github.com/mantreshkhurana/stlvi");
                ui.add_space(10.0);
                ui.label("By Mantresh Khurana");
            });
        });
}

/// Menu actions that can be triggered
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MenuAction {
    None,
    OpenFile,
    Save,
    SaveAs,
    ExportBinary,
    ExportAscii,
    Quit,
    RecalculateNormals,
    CenterModel,
    SetCameraView(CameraView),
    ShowShortcuts,
    ShowAbout,
}

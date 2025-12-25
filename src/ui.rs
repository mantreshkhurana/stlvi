use crate::state::{AppState, CameraView, EditMode};
use crate::theme::FusionColors;
use three_d::egui::{self, *};

/// Draw the main menu bar - Fusion 360 style
pub fn draw_menu_bar(ctx: &Context, state: &mut AppState) -> MenuAction {
    let mut action = MenuAction::None;

    egui::TopBottomPanel::top("menu_bar")
        .frame(Frame::none()
            .fill(FusionColors::HEADER_BG)
            .inner_margin(Margin::symmetric(8.0, 2.0)))
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.style_mut().spacing.item_spacing = vec2(12.0, 4.0);

                // File menu
                ui.menu_button(RichText::new("File").color(FusionColors::TEXT_PRIMARY), |ui| {
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
                ui.menu_button(RichText::new("Edit").color(FusionColors::TEXT_PRIMARY), |ui| {
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
                ui.menu_button(RichText::new("View").color(FusionColors::TEXT_PRIMARY), |ui| {
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
                ui.menu_button(RichText::new("Help").color(FusionColors::TEXT_PRIMARY), |ui| {
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
                        ui.label(RichText::new(label).monospace().color(FusionColors::TEXT_SECONDARY));
                    }
                });
            });
        });

    action
}

/// Draw the transform panel on the left side - Fusion 360 style
pub fn draw_transform_panel(ctx: &Context, state: &mut AppState) {
    if !state.show_transform_panel || state.current_file.is_none() {
        return;
    }

    egui::SidePanel::left("transform_panel")
        .resizable(true)
        .default_width(240.0)
        .frame(Frame::none()
            .fill(FusionColors::PANEL_BG)
            .inner_margin(Margin::same(12.0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(55, 60, 68))))
        .show(ctx, |ui| {
            // Panel header
            ui.horizontal(|ui| {
                ui.label(RichText::new("TRANSFORM").size(11.0).color(FusionColors::TEXT_SECONDARY).strong());
            });
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Edit mode selector - Fusion 360 style button group
            ui.label(RichText::new("Mode").size(11.0).color(FusionColors::TEXT_SECONDARY));
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing = vec2(4.0, 0.0);
                let btn_size = vec2(50.0, 24.0);

                if ui.add_sized(btn_size, SelectableLabel::new(state.edit_mode == EditMode::View, "View")).clicked() {
                    state.edit_mode = EditMode::View;
                }
                if ui.add_sized(btn_size, SelectableLabel::new(state.edit_mode == EditMode::Translate, "Move")).clicked() {
                    state.edit_mode = EditMode::Translate;
                }
                if ui.add_sized(btn_size, SelectableLabel::new(state.edit_mode == EditMode::Rotate, "Rotate")).clicked() {
                    state.edit_mode = EditMode::Rotate;
                }
                if ui.add_sized(btn_size, SelectableLabel::new(state.edit_mode == EditMode::Scale, "Scale")).clicked() {
                    state.edit_mode = EditMode::Scale;
                }
            });

            ui.add_space(16.0);

            // Position section
            draw_collapsing_section(ui, "Position", true, |ui| {
                let mut changed = false;

                changed |= draw_xyz_control(ui, "X", &mut state.model_transform.translation[0], 0.01, FusionColors::AXIS_X);
                changed |= draw_xyz_control(ui, "Y", &mut state.model_transform.translation[1], 0.01, FusionColors::AXIS_Y);
                changed |= draw_xyz_control(ui, "Z", &mut state.model_transform.translation[2], 0.01, FusionColors::AXIS_Z);

                if changed {
                    state.unsaved_changes = true;
                }
            });

            ui.add_space(8.0);

            // Rotation section
            draw_collapsing_section(ui, "Rotation", true, |ui| {
                let mut changed = false;

                changed |= draw_xyz_control_angle(ui, "X", &mut state.model_transform.rotation[0], 1.0, FusionColors::AXIS_X);
                changed |= draw_xyz_control_angle(ui, "Y", &mut state.model_transform.rotation[1], 1.0, FusionColors::AXIS_Y);
                changed |= draw_xyz_control_angle(ui, "Z", &mut state.model_transform.rotation[2], 1.0, FusionColors::AXIS_Z);

                if changed {
                    state.unsaved_changes = true;
                }
            });

            ui.add_space(8.0);

            // Scale section
            draw_collapsing_section(ui, "Scale", true, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Uniform").size(11.0).color(FusionColors::TEXT_SECONDARY));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add(
                            DragValue::new(&mut state.model_transform.scale)
                                .speed(0.01)
                                .clamp_range(0.01..=100.0)
                                .min_decimals(2)
                                .max_decimals(3)
                        );
                    });
                });
                if state.model_transform.scale != 1.0 {
                    state.unsaved_changes = true;
                }
            });

            ui.add_space(16.0);

            // Reset button - Fusion 360 style
            if ui.add_sized(
                vec2(ui.available_width(), 28.0),
                Button::new(RichText::new("Reset Transform").color(FusionColors::TEXT_PRIMARY))
            ).clicked() {
                state.model_transform.reset();
                state.unsaved_changes = true;
            }
        });
}

/// Draw an XYZ control row
fn draw_xyz_control(ui: &mut Ui, label: &str, value: &mut f32, speed: f32, color: Color32) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        // Colored axis indicator
        let (rect, _) = ui.allocate_exact_size(vec2(3.0, 18.0), Sense::hover());
        ui.painter().rect_filled(rect, Rounding::same(1.0), color);

        ui.label(RichText::new(label).size(12.0).color(FusionColors::TEXT_SECONDARY));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            changed = ui.add(
                DragValue::new(value)
                    .speed(speed)
                    .min_decimals(2)
                    .max_decimals(3)
            ).changed();
        });
    });
    changed
}

/// Draw an XYZ control row for angles
fn draw_xyz_control_angle(ui: &mut Ui, label: &str, value: &mut f32, speed: f32, color: Color32) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        // Colored axis indicator
        let (rect, _) = ui.allocate_exact_size(vec2(3.0, 18.0), Sense::hover());
        ui.painter().rect_filled(rect, Rounding::same(1.0), color);

        ui.label(RichText::new(label).size(12.0).color(FusionColors::TEXT_SECONDARY));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            changed = ui.add(
                DragValue::new(value)
                    .speed(speed)
                    .suffix("°")
                    .min_decimals(1)
                    .max_decimals(2)
            ).changed();
        });
    });
    changed
}

/// Draw a collapsing section header
fn draw_collapsing_section<R>(ui: &mut Ui, title: &str, default_open: bool, add_contents: impl FnOnce(&mut Ui) -> R) {
    CollapsingHeader::new(RichText::new(title).size(12.0).color(FusionColors::TEXT_PRIMARY).strong())
        .default_open(default_open)
        .show(ui, |ui| {
            ui.add_space(4.0);
            add_contents(ui);
        });
}

/// Draw the info panel on the right side - Fusion 360 style
pub fn draw_info_panel(ctx: &Context, state: &AppState) {
    if !state.show_info_panel || state.model_info.is_none() {
        return;
    }

    egui::SidePanel::right("info_panel")
        .resizable(true)
        .default_width(220.0)
        .frame(Frame::none()
            .fill(FusionColors::PANEL_BG)
            .inner_margin(Margin::same(12.0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(55, 60, 68))))
        .show(ctx, |ui| {
            // Panel header
            ui.label(RichText::new("MODEL INFO").size(11.0).color(FusionColors::TEXT_SECONDARY).strong());
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            if let Some(ref info) = state.model_info {
                // Statistics section
                draw_info_row(ui, "Vertices", &format!("{}", info.vertex_count));
                draw_info_row(ui, "Triangles", &format!("{}", info.triangle_count));

                ui.add_space(12.0);
                ui.label(RichText::new("Dimensions").size(11.0).color(FusionColors::TEXT_SECONDARY).strong());
                ui.add_space(4.0);

                draw_dimension_row(ui, "X", info.dimensions[0], FusionColors::AXIS_X);
                draw_dimension_row(ui, "Y", info.dimensions[1], FusionColors::AXIS_Y);
                draw_dimension_row(ui, "Z", info.dimensions[2], FusionColors::AXIS_Z);

                ui.add_space(12.0);
                ui.label(RichText::new("Bounds").size(11.0).color(FusionColors::TEXT_SECONDARY).strong());
                ui.add_space(4.0);

                ui.label(RichText::new(format!(
                    "Min: ({:.2}, {:.2}, {:.2})",
                    info.bounds_min[0], info.bounds_min[1], info.bounds_min[2]
                )).size(11.0).color(FusionColors::TEXT_DIM).monospace());

                ui.label(RichText::new(format!(
                    "Max: ({:.2}, {:.2}, {:.2})",
                    info.bounds_max[0], info.bounds_max[1], info.bounds_max[2]
                )).size(11.0).color(FusionColors::TEXT_DIM).monospace());
            }
        });
}

/// Draw an info row
fn draw_info_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).size(11.0).color(FusionColors::TEXT_SECONDARY));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(value).size(11.0).color(FusionColors::TEXT_PRIMARY).monospace());
        });
    });
}

/// Draw a dimension row with colored indicator
fn draw_dimension_row(ui: &mut Ui, label: &str, value: f32, color: Color32) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(vec2(3.0, 14.0), Sense::hover());
        ui.painter().rect_filled(rect, Rounding::same(1.0), color);
        ui.label(RichText::new(label).size(11.0).color(FusionColors::TEXT_SECONDARY));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(format!("{:.3}", value)).size(11.0).color(FusionColors::TEXT_PRIMARY).monospace());
        });
    });
}

/// Draw the settings panel (floating window) - Fusion 360 style
pub fn draw_settings_panel(ctx: &Context, state: &mut AppState) {
    if !state.show_settings_panel {
        return;
    }

    egui::Window::new(RichText::new("Settings").color(FusionColors::TEXT_PRIMARY))
        .open(&mut state.show_settings_panel)
        .resizable(true)
        .default_width(280.0)
        .frame(Frame::window(&ctx.style())
            .fill(FusionColors::PANEL_BG)
            .rounding(Rounding::same(6.0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(65, 70, 78))))
        .show(ctx, |ui| {
            // Grid settings
            draw_collapsing_section(ui, "Grid", true, |ui| {
                ui.checkbox(&mut state.view_settings.show_grid, "Show Grid");
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Size").size(11.0).color(FusionColors::TEXT_SECONDARY));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add(
                            DragValue::new(&mut state.view_settings.grid_size)
                                .speed(0.5)
                                .clamp_range(1.0..=100.0)
                        );
                    });
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Divisions").size(11.0).color(FusionColors::TEXT_SECONDARY));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add(
                            DragValue::new(&mut state.view_settings.grid_divisions)
                                .speed(1)
                                .clamp_range(2..=100)
                        );
                    });
                });
            });

            ui.add_space(8.0);

            // Colors
            draw_collapsing_section(ui, "Colors", true, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Background").size(11.0).color(FusionColors::TEXT_SECONDARY));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let mut bg = state.view_settings.background_color;
                        if ui.color_edit_button_rgb(&mut bg).changed() {
                            state.view_settings.background_color = bg;
                        }
                    });
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Model").size(11.0).color(FusionColors::TEXT_SECONDARY));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let mut mc = state.view_settings.model_color;
                        if ui.color_edit_button_rgb(&mut mc).changed() {
                            state.view_settings.model_color = mc;
                        }
                    });
                });
            });

            ui.add_space(8.0);

            // Display options
            draw_collapsing_section(ui, "Display", true, |ui| {
                ui.checkbox(&mut state.view_settings.wireframe_mode, "Wireframe Mode");
                ui.checkbox(&mut state.view_settings.show_normals, "Show Normals");
                ui.checkbox(&mut state.view_settings.show_axis_cube, "Show Axis Cube");
            });
        });
}

/// Draw status bar with zoom slider at the bottom - Fusion 360 style
pub fn draw_status_bar(ctx: &Context, state: &mut AppState, fps: f32) -> bool {
    let mut zoom_changed = false;

    egui::TopBottomPanel::bottom("status_bar")
        .frame(Frame::none()
            .fill(FusionColors::HEADER_BG)
            .inner_margin(Margin::symmetric(12.0, 6.0)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Edit mode indicator with colored badge
                let (mode_text, mode_color) = match state.edit_mode {
                    EditMode::View => ("View", FusionColors::TEXT_SECONDARY),
                    EditMode::Translate => ("Move", FusionColors::ACCENT),
                    EditMode::Rotate => ("Rotate", FusionColors::ACCENT),
                    EditMode::Scale => ("Scale", FusionColors::ACCENT),
                };

                ui.label(RichText::new(mode_text).size(11.0).color(mode_color).strong());

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
                ui.label(RichText::new(view_text).size(11.0).color(FusionColors::TEXT_SECONDARY));

                // Center - Zoom slider (Fusion 360 style)
                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("-").size(14.0).color(FusionColors::TEXT_DIM));

                        let slider_response = ui.add(
                            Slider::new(&mut state.zoom_level, 1.0..=20.0)
                                .show_value(false)
                                .trailing_fill(true)
                        );

                        if slider_response.changed() {
                            zoom_changed = true;
                        }

                        ui.label(RichText::new("+").size(14.0).color(FusionColors::TEXT_DIM));

                        ui.add_space(8.0);
                        ui.label(RichText::new(format!("{:.0}%", (state.zoom_level / 5.0) * 100.0))
                            .size(11.0)
                            .color(FusionColors::TEXT_SECONDARY)
                            .monospace());
                    });
                });

                // FPS on the right
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(format!("{:.0} FPS", fps))
                        .size(11.0)
                        .color(FusionColors::TEXT_DIM)
                        .monospace());
                });
            });
        });

    zoom_changed
}

/// Draw keyboard shortcuts help window - Fusion 360 style
pub fn draw_shortcuts_window(ctx: &Context, open: &mut bool) {
    egui::Window::new(RichText::new("Keyboard Shortcuts").color(FusionColors::TEXT_PRIMARY))
        .open(open)
        .resizable(false)
        .default_width(320.0)
        .frame(Frame::window(&ctx.style())
            .fill(FusionColors::PANEL_BG)
            .rounding(Rounding::same(6.0)))
        .show(ctx, |ui| {
            ui.add_space(4.0);

            egui::Grid::new("shortcuts_grid")
                .num_columns(2)
                .spacing([40.0, 8.0])
                .show(ui, |ui| {
                    draw_shortcut_row(ui, "Ctrl+O", "Open file");
                    draw_shortcut_row(ui, "Ctrl+S", "Save file");
                    draw_shortcut_row(ui, "Ctrl+Shift+S", "Save as");

                    ui.end_row();
                    ui.separator();
                    ui.separator();
                    ui.end_row();

                    draw_shortcut_row(ui, "G", "Translate mode");
                    draw_shortcut_row(ui, "R", "Rotate mode");
                    draw_shortcut_row(ui, "S", "Scale mode");
                    draw_shortcut_row(ui, "Escape", "View mode");

                    ui.end_row();
                    ui.separator();
                    ui.separator();
                    ui.end_row();

                    draw_shortcut_row(ui, "1-7", "Camera views");
                    draw_shortcut_row(ui, "Home", "Reset view");

                    ui.end_row();
                    ui.separator();
                    ui.separator();
                    ui.end_row();

                    draw_shortcut_row(ui, "Mouse drag", "Orbit camera");
                    draw_shortcut_row(ui, "Scroll", "Zoom");
                    draw_shortcut_row(ui, "Middle drag", "Pan camera");
                });
        });
}

fn draw_shortcut_row(ui: &mut Ui, shortcut: &str, description: &str) {
    ui.label(RichText::new(shortcut).size(11.0).color(FusionColors::ACCENT).monospace().strong());
    ui.label(RichText::new(description).size(11.0).color(FusionColors::TEXT_SECONDARY));
    ui.end_row();
}

/// Draw about window - Fusion 360 style
pub fn draw_about_window(ctx: &Context, open: &mut bool) {
    egui::Window::new(RichText::new("About STLVI").color(FusionColors::TEXT_PRIMARY))
        .open(open)
        .resizable(false)
        .collapsible(false)
        .default_width(280.0)
        .frame(Frame::window(&ctx.style())
            .fill(FusionColors::PANEL_BG)
            .rounding(Rounding::same(6.0)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new("STLVI").size(24.0).color(FusionColors::ACCENT).strong());
                ui.label(RichText::new("Version 2.0.0").size(12.0).color(FusionColors::TEXT_SECONDARY));
                ui.add_space(12.0);
                ui.label(RichText::new("A powerful STL 3D model viewer").size(12.0).color(FusionColors::TEXT_PRIMARY));
                ui.label(RichText::new("with editing capabilities").size(12.0).color(FusionColors::TEXT_PRIMARY));
                ui.add_space(12.0);
                ui.hyperlink_to(
                    RichText::new("GitHub").color(FusionColors::ACCENT),
                    "https://github.com/mantreshkhurana/stlvi"
                );
                ui.add_space(8.0);
                ui.label(RichText::new("By Mantresh Khurana").size(11.0).color(FusionColors::TEXT_DIM));
                ui.add_space(8.0);
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

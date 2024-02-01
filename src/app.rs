// TODO clone the egui_demo_lib from https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs

use egui::Context;
use egui::{panel::Side, Ui};
use nalgebra::DMatrix;

use crate::canvas::Canvas;
use crate::canvas_actions::CanvasActions;
use crate::editors::Editor;
use crate::options::{Mode, Options};
use crate::panels::Layout;
use crate::tool::Tool;
use crate::EditorsContainer;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EframeApp {
    pub canvas: Canvas,

    pub canvas_actions: CanvasActions,

    pub editors: EditorsContainer,

    pub options: Options,

    pub layout: Layout,

    #[serde(skip)] // don't cache this tool for next startup
    pub selected_tool: Tool,
}

impl EframeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for EframeApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or
        // `Area`. For inspiration and more examples, go to https://emilk.github.io/egui

        // Custom font setup
        // let mut fonts = FontDefinitions::default();
        // fonts.font_data.insert(
        //     "arial".to_owned(),
        //     FontData::from_static(include_bytes!("../assets/fonts/arial.ttf")),
        // );
        // fonts
        //     .families
        //     .get_mut(&FontFamily::Monospace)
        //     .unwrap()
        //     .insert(0, "arial".to_owned());
        // ctx.set_fonts(fonts);

        self.show_top_panel(ctx);
        self.show_left_panel(ctx);
        self.show_right_panel(ctx);
        self.show_center_panel(ctx);
    }
}

impl EframeApp {
    fn show_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // TODO rework this when we can save in web
                {
                    ui.menu_button("File", |ui| self.show_file_menu(ui, ctx));
                    ui.add_space(16.0);
                }

                self.canvas_actions
                    .canvas_menu(ui, &mut self.canvas, &mut self.editors);
                ui.add_space(16.0);

                ui.menu_button("Layout", |ui| {
                    ui.checkbox(&mut self.layout.tools, "Tools");
                    ui.checkbox(&mut self.layout.mode, "Modes");
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn show_left_panel(&mut self, ctx: &egui::Context) {
        if self.layout.tools {
            egui::SidePanel::new(Side::Left, "left_panel").show(ctx, |ui| {
                ui.heading("Tools");
                let mut tool_buttons: Vec<Tool> =
                    vec![Tool::Move, Tool::Node, Tool::Line, Tool::Label];
                for tool in tool_buttons.iter_mut() {
                    tool.show(
                        ui,
                        &mut self.selected_tool,
                        &mut self.canvas_actions.add_label_text,
                    );
                }
                // BUG: this line is needed, allows left-panel resizing
                ui.separator();
            });
        }
    }

    fn show_right_panel(&mut self, ctx: &egui::Context) {
        if self.layout.mode {
            egui::SidePanel::new(Side::Right, "right_panel").show(ctx, |ui| {
                self.options.show_mode_buttons(ui);

                ui.separator();
                self.options.show_specific_options(ui);

                // if self.options.mode != Mode::Edit {
                //     ui.separator();
                //     self.options.show_generic_options(ui);
                // }

                if self.options.mode == Mode::Classical {
                    ui.separator();
                    self.editors.show_matrix_editor(ui, self.canvas.nodes.len());
                }
            });
        }
    }

    fn show_center_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_canvas_from_editors();

            self.canvas
                .show(ui, self.selected_tool, &self.options, &self.canvas_actions);
        });
    }

    fn show_file_menu(&mut self, ui: &mut Ui, ctx: &Context) {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            self.show_save_button(ui);
            self.show_load_button(ui);
            #[cfg(target_os = "macos")]
            {
                explorer_program_name = Some("open");
            }
            #[cfg(target_os = "linux")]
            {
                explorer_program_name = Some("xdg-open");
            }
        }
        self.show_quit_button(ui, ctx);
    }

    // TODO get this working for other OS's
    #[cfg(target_os = "windows")]
    fn show_save_button(&mut self, ui: &mut Ui) {
        use wfd::DialogParams;

        if ui.button("Save").clicked() {
            ui.close_menu();

            let dialog_result = wfd::save_dialog(DialogParams {
                default_extension: "json",
                file_types: vec![("JSON Files", "*.json")],
                file_name: "graph.json",
                ..Default::default()
            });
            if let Ok(dialog_result) = dialog_result {
                std::fs::write(
                    dialog_result.selected_file_path,
                    serde_json::to_string(self).unwrap(),
                )
                .ok();
            }
        }
    }

    // TODO get this working for other OS's
    #[cfg(target_os = "windows")]
    fn show_load_button(&mut self, ui: &mut Ui) {
        use wfd::DialogParams;

        if ui.button("Load").clicked() {
            ui.close_menu();

            let dialog_result = wfd::open_dialog(DialogParams {
                file_types: vec![("JSON Files", "*.json")],
                ..Default::default()
            });
            if let Ok(dialog_result) = dialog_result {
                if let Ok(file) = std::fs::read(dialog_result.selected_file_path) {
                    if let Ok(c) = serde_json::from_slice::<EframeApp>(file.as_slice()) {
                        *self = c;
                    }
                }
            }
        }
    }

    fn show_quit_button(&mut self, ui: &mut Ui, ctx: &Context) {
        if ui.button("Quit").clicked() {
            ui.close_menu();

            #[cfg(not(target_arch = "wasm32"))]
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn update_canvas_from_editors(&mut self) {
        if self.options.mode == Mode::Classical {
            if let Some(matrix_editor) = self.editors.get_matrix_editor_mut() {
                if matrix_editor.is_canvas_update_ready() {
                    let matrix = &matrix_editor.matrix;
                    let canvas = &mut self.canvas;
                    Self::update_edges_from_matrix(matrix, canvas);
                    matrix_editor.on_canvas_updated();
                }
            }
        }
    }

    fn update_edges_from_matrix(matrix: &DMatrix<f64>, canvas: &mut Canvas) {
        for (i, j) in (0..matrix.nrows()).flat_map(|i| (i + 1..matrix.ncols()).map(move |j| (i, j)))
        {
            if matrix[(i, j)] == 0.0 && matrix[(j, i)] == 0.0 {
                if canvas.is_line_between_nodes(i, j) {
                    canvas.remove_line_between_nodes(i, j);
                }
            } else if !canvas.is_line_between_nodes(i, j) {
                canvas.add_line_between_nodes(i, j);
            }
        }
    }
}

//// "Powered By" message
// fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//     ui.horizontal(|ui| {
//         ui.spacing_mut().item_spacing.x = 0.0;
//         ui.label("Powered by ");
//         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//         ui.label(" and ");
//         ui.hyperlink_to(
//             "eframe",
//             "https://github.com/emilk/egui/tree/master/crates/eframe",
//         );
//         ui.label(".");
//     });
// }

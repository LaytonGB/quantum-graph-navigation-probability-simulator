// TODO clone the egui_demo_lib from https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs

use eframe::Frame;
use egui::{panel::Side, Ui};
use serde::{Deserialize, Serialize};
use wfd::DialogParams;

use crate::{graph_settings::CanvasSettings, Canvas, ComputationOptions, Tool};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EframeApp {
    canvas: Canvas,

    graph_settings: CanvasSettings,

    computation_options: ComputationOptions,

    #[serde(skip)] // don't cache this tool for next startup
    selected_tool: Tool,
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

fn file_menu(app: &mut EframeApp, ui: &mut Ui, _frame: &mut Frame) {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        // TODO get this working for other OS's
        #[cfg(target_os = "windows")]
        {
            if ui.button("Save").clicked() {
                ui.close_menu();

                let dialog_result = wfd::save_dialog(DialogParams {
                    default_extension: "json",
                    file_types: vec![("JSON Files", "*.json")],
                    title: "Select a save location",
                    file_name: "graph.json",
                    ..Default::default()
                });
                if let Ok(dialog_result) = dialog_result {
                    std::fs::write(
                        dialog_result.selected_file_path,
                        serde_json::to_string(app).unwrap(),
                    )
                    .ok();
                }
            }
            if ui.button("Load").clicked() {
                ui.close_menu();

                let dialog_result = wfd::open_dialog(DialogParams {
                    file_types: vec![("JSON Files", "*.json")],
                    title: "Select a file to open",
                    ..Default::default()
                });
                if let Ok(dialog_result) = dialog_result {
                    if let Ok(file) = std::fs::read(dialog_result.selected_file_path) {
                        if let Ok(c) = serde_json::from_slice::<EframeApp>(file.as_slice()) {
                            *app = c;
                        }
                    }
                }
            }
        }
        #[cfg(target_os = "macos")]
        {
            explorer_program_name = Some("open");
        }
        #[cfg(target_os = "linux")]
        {
            explorer_program_name = Some("xdg-open");
        }
    }
    if ui.button("Quit").clicked() {
        ui.close_menu();

        _frame.close();
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| file_menu(self, ui, _frame));
                    ui.add_space(16.0);
                }

                self.graph_settings.canvas_menu(ui, &mut self.canvas);
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::new(Side::Left, "left_toolbar").show(ctx, |ui| {
            ui.heading("Tools");
            let mut tool_buttons: Vec<Tool> = vec![Tool::Move, Tool::Node, Tool::Line];
            for tool in tool_buttons.iter_mut() {
                tool.show(ui, &mut self.selected_tool);
            }
            // BUG: this line is needed, allows left-panel resizing
            ui.separator();
        });

        egui::SidePanel::new(Side::Right, "right_toolbar").show(ctx, |ui| {
            ui.heading("Computation Style");
            self.computation_options.show_style_buttons(ui);
            ui.separator();

            ui.heading(format!("{} Options", self.computation_options.style.name()));
            self.computation_options.show_specific_options(ui);
            ui.separator();

            ui.heading("Generic Options");
            self.computation_options.show_generic_options(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.canvas
                .show(ui, self.selected_tool, self.graph_settings.snap)
        });
    }
}

// Powered By message
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

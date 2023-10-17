// TODO clone the egui_demo_lib from https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs

use egui::panel::Side;

use crate::{Canvas, LineTool, NodeTool, Tool};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EframeApp {
    canvas: Canvas,

    #[serde(skip)]
    selected_tool: Option<&'static str>,
}

impl Default for EframeApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            canvas: Canvas::default(),
            selected_tool: None,
        }
    }
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

    pub fn update_tool(&mut self, new_tool: Option<&'static str>) {
        self.selected_tool = new_tool;
    }
}

impl eframe::App for EframeApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                ui.menu_button("Canvas", |ui| {
                    if ui.button("Clear").clicked() {
                        self.canvas.clear_nodes();
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::new(Side::Left, "left toolbar")
            .resizable(false)
            .exact_width(40.0)
            .show(ctx, |ui| {
                let mut tool_buttons: Vec<Box<dyn Tool>> =
                    vec![Box::new(NodeTool::default()), Box::new(LineTool::default())];
                for tool in tool_buttons.iter_mut() {
                    let active = self.selected_tool.clone().unwrap_or(&"") == tool.name();
                    if tool.show(ui, active) {
                        self.update_tool(Some(tool.name()));
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| self.canvas.show(ui));
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

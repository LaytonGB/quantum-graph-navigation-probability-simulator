use crate::{model::Model, EframeApp};

pub struct FileMenu;

impl FileMenu {
    pub fn show(ui: &mut egui::Ui, ctx: &egui::Context, model: &mut Model) {
        ui.menu_button("File", |ui| {
            #[cfg(any(target_os = "windows", target_os = "macos"))]
            {
                Self::show_save_button(ui, model);
                Self::show_load_button(ui, model);

                // TODO implement the rest of this process for macos and linux
                #[cfg(target_os = "macos")]
                {
                    explorer_program_name = Some("open");
                }
                #[cfg(target_os = "linux")]
                {
                    explorer_program_name = Some("xdg-open");
                }
            }
            Self::show_quit_button(ui, ctx);
        });
    }

    // TODO get this working for other OS's
    #[cfg(target_os = "windows")]
    fn show_save_button(ui: &mut egui::Ui, model: &mut Model) {
        use wfd::DialogParams;

        use crate::state::State;

        if ui.button("Save").clicked() {
            ui.close_menu();

            let dialog_result = wfd::save_dialog(DialogParams {
                default_extension: "json",
                file_types: vec![("JSON Files", "*.json")],
                file_name: "graph.json",
                ..Default::default()
            });

            if let Ok(dialog_result) = dialog_result {
                model.state = State::PendingSave {
                    path_buffer: dialog_result.selected_file_path,
                };
            }
        }
    }

    // TODO get this working for other OS's
    #[cfg(target_os = "windows")]
    fn show_load_button(ui: &mut egui::Ui, model: &mut Model) {
        use wfd::DialogParams;

        if ui.button("Load").clicked() {
            ui.close_menu();

            let dialog_result = wfd::open_dialog(DialogParams {
                file_types: vec![("JSON Files", "*.json")],
                ..Default::default()
            });

            if let Ok(dialog_result) = dialog_result {
                model.state = State::PendingLoad {
                    path_buffer: dialog_result.selected_file_path,
                }
            }
        }
    }

    fn show_quit_button(ui: &mut egui::Ui, ctx: &egui::Context) {
        if ui.button("Quit").clicked() {
            ui.close_menu();

            #[cfg(not(target_arch = "wasm32"))]
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

pub struct FileMenu;

impl FileMenu {
    pub fn show(ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| self.show_file_menu(ui, ctx));
        ui.add_space(16.0);
    }
}

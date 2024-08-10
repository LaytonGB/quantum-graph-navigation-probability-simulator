use egui::panel::Side;
use strum::IntoEnumIterator;

use crate::{
    canvas_menu::CanvasMenu, classical::Classical, file_menu::FileMenu, model::Model,
    quantum::Quantum, simulation_mode::SimulationMode, state::State, tool::Tool,
};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct View;

impl View {
    pub fn show(ctx: &egui::Context, model: &mut Model) {
        Self::show_top_panel(ctx, model);
        Self::show_left_panel(ctx, model);
        Self::show_right_panel(ctx, model);
        // self.show_center_panel(ctx);
    }

    fn show_top_panel(ctx: &egui::Context, model: &mut Model) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // TODO rework this when we can save in web
                #[cfg(not(target_arch = "wasm32"))]
                {
                    FileMenu::show(ui, ctx, model);
                    ui.add_space(16.0);
                }
                CanvasMenu::show(ui, model);
                ui.add_space(16.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn show_left_panel(ctx: &egui::Context, model: &mut Model) {
        if let (State::Editing { selected_tool }, true) = (&mut model.state, model.panels.tools) {
            egui::SidePanel::new(Side::Left, "left_panel").show(ctx, |ui| {
                ui.heading("Tools");
                for tool in Tool::iter() {
                    tool.show(ui, selected_tool);
                }
                // BUG: this line is needed, allows left-panel resizing
                // is likely fixed if egui is updated
                ui.separator();
            });
        }
    }

    fn show_right_panel(ctx: &egui::Context, model: &mut Model) {
        if model.panels.mode {
            egui::SidePanel::new(Side::Right, "right_panel").show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Computation Style");
                    Self::show_mode_buttons(ui, model);

                    // TODO: Show relevant options

                    Self::show_editors(ui, model);
                });
            });
        }
    }

    fn show_mode_buttons(ui: &mut egui::Ui, model: &mut Model) {
        ui.horizontal(|ui| {
            let edit_btn = ui.button("Edit");
            if let State::Editing { .. } = model.state {
                edit_btn.highlight();
            } else if edit_btn.clicked() {
                model.state = State::Editing {
                    selected_tool: Default::default(),
                };
            }

            let simulate_btn = ui.button("Simulate");
            if let State::Simulating { .. } = model.state {
                simulate_btn.highlight();
            } else if simulate_btn.clicked() {
                model.state = State::Simulating {
                    mode: Default::default(),
                    state: Default::default(),
                };
            }
        });

        Self::show_simulation_mode_buttons(ui, model);
    }

    fn show_simulation_mode_buttons(ui: &mut egui::Ui, model: &mut Model) {
        let State::Simulating { mode, .. } = &model.state else {
            return;
        };

        let mode = *mode;
        ui.horizontal(|ui| {
            for simulation_mode in SimulationMode::iter() {
                let btn = ui.button(simulation_mode.to_string());
                if simulation_mode == mode {
                    btn.highlight();
                } else if btn.clicked() {
                    model.state = State::Simulating {
                        mode: simulation_mode,
                        state: Default::default(),
                    };
                }
            }
        });
    }

    fn show_editors(ui: &mut egui::Ui, model: &mut Model) {
        let State::Simulating { ref mode, .. } = model.state else {
            return;
        };

        match mode {
            SimulationMode::SideBySide => {
                Classical::show_classical_editors(ui, model);
                Quantum::show_quantum_editors(ui, model);
            }
            SimulationMode::Classical => Classical::show_classical_editors(ui, model),
            SimulationMode::Quantum => Quantum::show_quantum_editors(ui, model),
        }
    }
}

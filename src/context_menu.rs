use egui::Ui;

use crate::canvas::Canvas;
use crate::graph_node::GraphNode;
use crate::options::Snap;

#[derive(Clone, Default, Debug)]
pub struct ContextMenuAddNode {
    pub x: String,
    pub y: String,
}

impl ContextMenuAddNode {
    pub fn clear(&mut self) {
        self.x = "".to_owned();
        self.y = "".to_owned();
    }
}

#[derive(Clone, Default, Debug)]
pub struct ContextMenuValues {
    pub add_node: ContextMenuAddNode,
}

pub struct ContextMenu;

impl ContextMenu {
    pub fn plot_context_menu(canvas: &mut Canvas, ctx_ui: &mut Ui) {
        ctx_ui.menu_button("Add Node", |ui| {
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.text_edit_singleline(&mut canvas.context_menu_values.add_node.x);
            });

            ui.horizontal(|ui| {
                ui.label("Y:");
                ui.text_edit_singleline(&mut canvas.context_menu_values.add_node.y);
            });

            if ui.button("Confirm").clicked() {
                if let (Ok(x), Ok(y)) = (
                    canvas.context_menu_values.add_node.x.parse(),
                    canvas.context_menu_values.add_node.y.parse(),
                ) {
                    canvas.add_node(GraphNode::new(x, y), Snap::None).ok();
                    canvas.context_menu_values.add_node.clear();
                    ui.close_menu();
                }
            }
        });

        if ctx_ui.button("Close this menu").clicked() {
            ctx_ui.close_menu();
        }
    }
}

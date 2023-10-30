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

pub trait Editor {
    fn is_canvas_update_ready(&self) -> bool;
    fn on_canvas_updated(&mut self);
}

#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod canvas;
pub mod canvas_actions;
pub mod constants;
pub mod context_menu;
pub mod editors;
pub mod graph_line;
pub mod graph_node;
pub mod options;
pub mod panels;
pub mod tool;
pub mod utils;
pub use app::EframeApp;

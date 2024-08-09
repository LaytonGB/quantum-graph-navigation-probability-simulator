#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod canvas;
pub mod canvas_actions;
pub mod canvas_menu;
pub mod canvas_old;
pub mod constants;
pub mod context_menu;
pub mod controller;
pub mod editors;
pub mod file_menu;
pub mod graph_line;
pub mod graph_node;
pub mod layout_menu;
pub mod menu;
pub mod model;
pub mod node;
pub mod options;
pub mod panels;
pub mod position;
pub mod state;
pub mod text_fields;
pub mod tool;
pub mod utils;
pub mod view;

pub use app::EframeApp;

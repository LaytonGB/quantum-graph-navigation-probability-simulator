#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod canvas;
pub mod canvas_actions;
pub mod canvas_menu;
pub mod constants;
pub mod context_menu;
pub mod controller;
pub mod editors;
pub mod file_menu;
pub mod graph_line;
pub mod graph_node;
pub mod model;
pub mod options;
pub mod panels;
pub mod state;
pub mod tool;
pub mod utils;
pub mod view;
pub use app::EframeApp;

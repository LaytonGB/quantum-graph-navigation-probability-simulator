use egui::Pos2;
use egui_plot::PlotPoint;

use crate::graph_node::GraphNode;

pub trait FloatCoordinates {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn x_32(&self) -> f32;
    fn y_32(&self) -> f32;
}

impl FloatCoordinates for GraphNode {
    #[inline]
    fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    fn y(&self) -> f64 {
        self.y
    }

    #[inline]
    fn x_32(&self) -> f32 {
        self.x as f32
    }

    #[inline]
    fn y_32(&self) -> f32 {
        self.y as f32
    }
}

impl FloatCoordinates for PlotPoint {
    #[inline]
    fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    fn y(&self) -> f64 {
        self.y
    }

    #[inline]
    fn x_32(&self) -> f32 {
        self.x as f32
    }

    #[inline]
    fn y_32(&self) -> f32 {
        self.y as f32
    }
}

impl FloatCoordinates for Pos2 {
    #[inline]
    fn x(&self) -> f64 {
        self.x as f64
    }

    #[inline]
    fn y(&self) -> f64 {
        self.y as f64
    }

    #[inline]
    fn x_32(&self) -> f32 {
        self.x
    }

    #[inline]
    fn y_32(&self) -> f32 {
        self.y
    }
}

#[inline]
pub fn euclidean_squared<C: FloatCoordinates>(a: &C, b: &C) -> f64 {
    (a.x() - b.x()).powi(2) + (a.y() - b.y()).powi(2)
}

#[inline]
pub fn euclidean_dist<C: FloatCoordinates>(a: &C, b: &C) -> f64 {
    euclidean_squared(a, b).sqrt()
}

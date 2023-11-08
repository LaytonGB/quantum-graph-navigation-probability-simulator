use egui::Pos2;
use egui_plot::PlotPoint;

use crate::options::Snap;

#[derive(Clone, PartialEq, PartialOrd, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct GraphNode {
    pub label: Option<String>,
    pub x: f64,
    pub y: f64,
}

impl GraphNode {
    pub fn new_unlabelled(x: f64, y: f64) -> Self {
        Self::new(x, y, None)
    }

    pub fn new_labelled<'a>(x: f64, y: f64, label: impl Into<&'a str>) -> Self {
        Self::new(x, y, Some(label.into().to_owned()))
    }

    pub fn new(x: f64, y: f64, label: Option<String>) -> Self {
        GraphNode { x, y, label }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn float_mul(&self, rhs: f64) -> Self {
        Self::new_unlabelled(self.x * rhs, self.y * rhs)
    }

    pub fn dist_squared(&self, other: &Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn dist(&self, other: &Self) -> f64 {
        self.dist_squared(other).sqrt()
    }

    pub fn round_to(self, snap: Snap) -> Option<Self> {
        match snap {
            Snap::None => Some(self),
            Snap::Half => {
                let (x, y) = ((self.x * 2.0).round() / 2.0, (self.y * 2.0).round() / 2.0);
                if x < self.x - 1.0 || y < self.y - 1.0 {
                    None
                } else {
                    Some(Self::new(x, y, self.label))
                }
            }
            Snap::One => Some(Self::new(self.x.round(), self.y.round(), self.label)),
            Snap::Five => Some(Self::new(
                (self.x / 5.0).round() * 5.0,
                (self.y / 5.0).round() * 5.0,
                self.label,
            )),
            Snap::Ten => Some(Self::new(
                (self.x / 10.0).round() * 10.0,
                (self.y / 10.0).round() * 10.0,
                self.label,
            )),
        }
    }
}

// TODO borrow, don't consume
impl std::ops::Add for GraphNode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_unlabelled(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for GraphNode {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_unlabelled(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Div for GraphNode {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new_unlabelled(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Into<GraphNode> for [f64; 2] {
    fn into(self) -> GraphNode {
        GraphNode {
            x: self[0],
            y: self[1],
            ..Default::default()
        }
    }
}

impl Into<GraphNode> for (f64, f64) {
    fn into(self) -> GraphNode {
        GraphNode {
            x: self.0,
            y: self.1,
            ..Default::default()
        }
    }
}

impl Into<GraphNode> for PlotPoint {
    fn into(self) -> GraphNode {
        let PlotPoint { x, y } = self;
        GraphNode {
            x,
            y,
            ..Default::default()
        }
    }
}

impl Into<GraphNode> for Pos2 {
    fn into(self) -> GraphNode {
        let Pos2 { x, y } = self;
        GraphNode {
            x: x as f64,
            y: y as f64,
            ..Default::default()
        }
    }
}

impl Into<[f64; 2]> for GraphNode {
    fn into(self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl Into<PlotPoint> for GraphNode {
    fn into(self) -> PlotPoint {
        PlotPoint {
            x: self.x,
            y: self.y,
        }
    }
}

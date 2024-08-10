use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl TryFrom<(String, String)> for Position {
    type Error = String;

    fn try_from((x, y): (String, String)) -> Result<Self, Self::Error> {
        let x = x.parse().map_err(|e| format!("Failed to parse x: {}", e))?;
        let y = y.parse().map_err(|e| format!("Failed to parse y: {}", e))?;
        Ok(Self { x, y })
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

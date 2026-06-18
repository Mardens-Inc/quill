#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StockUnit {
    Millimeters,
    Inches,
}

impl StockUnit {
    pub(crate) fn to_mm(self, value: f32) -> f32 {
        match self {
            StockUnit::Millimeters => value,
            StockUnit::Inches => value * 25.4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stock {
    pub width: f32,
    pub height: f32,
    pub exposed_liner_left: f32,
    pub exposed_liner_right: f32,
    pub gap: f32,
    pub unit: StockUnit,
}

impl Stock {
    pub fn millimeters(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            exposed_liner_left: 0.0,
            exposed_liner_right: 0.0,
            gap: 0.0,
            unit: StockUnit::Millimeters,
        }
    }

    pub fn inches(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            exposed_liner_left: 0.0,
            exposed_liner_right: 0.0,
            gap: 0.0,
            unit: StockUnit::Inches,
        }
    }

    pub fn with_exposed_liner(mut self, left: f32, right: f32) -> Self {
        self.exposed_liner_left = left;
        self.exposed_liner_right = right;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub(crate) fn width_mm(&self) -> f32 {
        self.unit.to_mm(self.width)
    }

    pub(crate) fn height_mm(&self) -> f32 {
        self.unit.to_mm(self.height)
    }

    pub(crate) fn exposed_liner_left_mm(&self) -> f32 {
        self.unit.to_mm(self.exposed_liner_left)
    }

    pub(crate) fn exposed_liner_right_mm(&self) -> f32 {
        self.unit.to_mm(self.exposed_liner_right)
    }

    pub(crate) fn gap_mm(&self) -> f32 {
        self.unit.to_mm(self.gap)
    }
}

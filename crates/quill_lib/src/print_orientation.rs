/// How an image should be rotated before it is rasterised onto the label.
///
/// Rotation is clockwise. The quarter-turn variants are lossless; `Degrees`
/// uses nearest-neighbour resampling onto an expanded white canvas so the whole
/// rotated picture stays on the label.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageOrientation {
    /// No rotation (0°).
    Normal,
    /// 90° clockwise.
    Rotate90,
    /// 180°.
    Rotate180,
    /// 270° clockwise (i.e. 90° counter-clockwise).
    Rotate270,
    /// An arbitrary clockwise angle in degrees (e.g. `Degrees(45.0)`).
    Degrees(f32),
}

impl PageOrientation {
    /// Clockwise rotation in degrees.
    pub(crate) fn degrees(self) -> f32 {
        match self {
            PageOrientation::Normal => 0.0,
            PageOrientation::Rotate90 => 90.0,
            PageOrientation::Rotate180 => 180.0,
            PageOrientation::Rotate270 => 270.0,
            PageOrientation::Degrees(d) => d,
        }
    }
}
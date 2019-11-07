use super::*;

/// A cardinal direction.
pub enum Cardinal {
    NW,
    NE,
    SE,
    SW,
}

/// A struct representing a rectangular plane in a Cartesian coordinate system.
/// X and y co-ordinates specify the *center* of the rectangle.
#[derive(Debug, PartialEq)]
pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, p: &Point) -> bool {
        let nw = self.corner(Cardinal::NW);
        let se = self.corner(Cardinal::SE);

        p.x > nw.x && p.x < se.x && p.y > nw.y && p.y < se.y
    }

    /// Return a [`Point`](../point/struct.Point.html) at the center of the rectangle.
    pub fn center(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Return a [`Point`](../point/struct.Point.html) representing the specified corner of the rectangle.
    pub fn corner(&self, corner: Cardinal) -> Point {
        let half_w = self.half_width();
        let half_h = self.half_height();
        match corner {
            Cardinal::NW => Point::new(self.x - half_w, self.y + half_h),
            Cardinal::NE => Point::new(self.x + half_w, self.y + half_h),
            Cardinal::SE => Point::new(self.x + half_w, self.y - half_h),
            Cardinal::SW => Point::new(self.x - half_w, self.y - half_h),
        }
    }

    /// Return a [`Rect`](../rect/struct.Rect.html) covering the area from the center to the specified corner.
    pub fn split_rect(&self, c: Cardinal) -> Self {
        let x = self.center().x;
        let y = self.center().y;
        let half_w = self.half_width();
        let half_h = self.half_height();

        match c {
            Cardinal::NW => Self::new(x - half_w / 2.0, y + half_h / 2.0, half_w, half_h),
            Cardinal::NE => Self::new(x + half_w / 2.0, y + half_h / 2.0, half_w, half_h),
            Cardinal::SE => Self::new(x + half_w / 2.0, y - half_h / 2.0, half_w, half_h),
            Cardinal::SW => Self::new(x - half_w / 2.0, y - half_h / 2.0, half_w, half_h),
        }
    }

    fn half_width(&self) -> f64 {
        self.width / 2.0
    }

    fn half_height(&self) -> f64 {
        self.height / 2.0
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{:>13}", "RECT")?;
        writeln!(f, "{:>12}: x: {}, y: {}", "Center", self.x, self.y)?;
        writeln!(f, "{:>12}: {}", "Width", self.width)?;
        writeln!(f, "{:>12}: {}", "Height", self.height)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let a = Rect::new(1.0, 1.0, 10.0, 10.0);
        let b = Rect::new(1.0, 1.0, 10.0, 10.0);
        assert_eq!(a, b);
    }
}

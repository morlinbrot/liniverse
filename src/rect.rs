use super::*;

pub enum Cardinal {
    NW,
    NE,
    SE,
    SW,
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// A struct representing a rectangular plane in a cartesian coordinate system.
/// X and y co-ordinates specify the *center* of the rectangle.
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

    /// Return a Point at the center of the rectangle.
    pub fn center(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Return a Point representing the specified corner of the rectangle.
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

    /// Return a Rect covering the area from the center to the specified corner.
    pub fn split_rect(&self, c: Cardinal) -> Self {
        let x = self.center().x;
        let y = self.center().y;
        let half_w = self.half_width();
        let half_h = self.half_height();

        match c {
            Cardinal::NW => Self::new(x - half_w, y + half_h, half_w, half_h),
            Cardinal::NE => Self::new(x + half_w, y + half_h, half_w, half_h),
            Cardinal::SE => Self::new(x + half_w, y - half_h, half_w, half_h),
            Cardinal::SW => Self::new(x - half_w, y - half_h, half_w, half_h),
        }
    }

    fn half_width(&self) -> f64 {
        self.width / 2.0
    }

    fn half_height(&self) -> f64 {
        self.height / 2.0
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

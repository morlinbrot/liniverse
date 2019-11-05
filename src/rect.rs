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
        let top_left = Point::new(self.x - self.width / 2.0, self.y - self.height / 2.0);
        let bot_right = Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0);

        p.x > top_left.x && p.x < bot_right.x && p.y > top_left.y && p.y < bot_right.y
    }

    pub fn center(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn corner(&self, corner: Cardinal) -> Point {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        match corner {
            Cardinal::NW => Point::new(self.x - half_width, self.y + half_height),
            Cardinal::NE => Point::new(self.x + half_width, self.y + half_height),
            Cardinal::SE => Point::new(self.x + half_width, self.y - half_height),
            Cardinal::SW => Point::new(self.x - half_width, self.y - half_height),
        }
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

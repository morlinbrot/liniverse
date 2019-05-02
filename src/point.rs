#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(&self) -> Self {
        Point {
            x: self.x / self.mag(),
            y: self.y / self.mag(),
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Mul<usize> for &Point {
    type Output = Point;

    fn mul(self, scalar: usize) -> Point {
        Point {
            x: self.x * scalar as f64,
            y: self.y * scalar as f64,
        }
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, scalar: f64) -> Point {
        Point {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl std::ops::Div<f64> for &Point {
    type Output = Point;

    fn div(self, scalar: f64) -> Point {
        Point {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl std::ops::Div<usize> for &Point {
    type Output = Point;

    fn div(self, scalar: usize) -> Point {
        Point {
            x: self.x / scalar as f64,
            y: self.y / scalar as f64,
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn point_basics() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 2.0);
        assert_eq!(&p1 + &p2, Point { x: 3.0, y: 3.0 });
        assert_eq!(&p1 - &p2, Point { x: -1.0, y: -1.0 });
        assert_eq!(&p1 * 3.0, Point { x: 3.0, y: 3.0 });
        assert_eq!(&p1 * 3, Point { x: 3.0, y: 3.0 });
        assert_eq!(&p1 / 2.0, Point { x: 0.5, y: 0.5 });
        assert_eq!(&p1 / 2, Point { x: 0.5, y: 0.5 });

        let p3 = Point::new(3.0, 4.0);
        let p3mag = p3.mag();
        let p3norm = p3.norm();
        assert_eq!(p3mag, 5.0);
        assert_eq!(p3norm, Point { x: 0.6, y: 0.8 });
        assert_eq!(p3norm.mag(), 1.0);
    }
}
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn create(x: i32, y: i32) -> Point {
        Point {x, y}
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub lower: Point,
    pub upper: Point
}

impl Rectangle {
    pub fn create(x: i32, y: i32, w: i32, h: i32) -> Rectangle {
        Rectangle {
            lower: Point::create(x, y),
            upper: Point::create(x + w, y + h)
        }
    }

    pub fn intersect(&self, other:&Rectangle) -> bool {
        self.lower <= other.upper && self.upper >= other.lower
    }

    pub fn width(&self) -> i32 {
        i32::abs(self.lower.x - self.upper.x)
    }

    pub fn height(&self) -> i32 {
        i32::abs(self.lower.y - self.upper.y)
    }

    pub fn center(&self) -> Point {
        Point::create((self.lower.x + self.upper.x)/2, (self.lower.y + self.upper.y)/2)
    }
}
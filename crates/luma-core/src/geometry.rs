/// A 2D point with integer coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

/// A 2D size with unsigned dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    pub fn zero() -> Self {
        Self { width: 0, height: 0 }
    }
}

/// A rectangle defined by position and size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn from_point_size(point: Point, size: Size) -> Self {
        Self {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
        }
    }
    
    pub fn point(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
    
    pub fn size(&self) -> Size {
        Size { width: self.width, height: self.height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point_creation() {
        let p = Point::new(10, 20);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
    }
    
    #[test]
    fn test_size_creation() {
        let s = Size::new(100, 200);
        assert_eq!(s.width, 100);
        assert_eq!(s.height, 200);
    }
    
    #[test]
    fn test_rect_from_point_size() {
        let p = Point::new(5, 10);
        let s = Size::new(50, 100);
        let r = Rect::from_point_size(p, s);
        
        assert_eq!(r.x, 5);
        assert_eq!(r.y, 10);
        assert_eq!(r.width, 50);
        assert_eq!(r.height, 100);
    }
}

use std::ops::Add;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Vec2D(i32, i32);

impl Vec2D {
    fn new() -> Self {
        Self::default()
    }
}

impl Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Vec2D {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Situation {
    head: Vec2D,
    tail: Vec2D
}

#[cfg(test)]
mod tests {
    use super::Vec2D;
    
    #[test]
    fn some_vec_tests() {
        let v = Vec2D::default();
    }
}
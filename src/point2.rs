use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Point2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Point2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Point2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

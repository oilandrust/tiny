use std::ops::{Add, AddAssign, Mul, Sub};

pub type Position = PositionT<i32>;
pub type Direction = DirectionT<i32>;

pub type Positionf32 = PositionT<f32>;
pub type Directionf32 = DirectionT<f32>;

#[derive(Debug, Clone, Copy)]
pub struct PositionT<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct DirectionT<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add<DirectionT<T>> for PositionT<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: DirectionT<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: AddAssign> AddAssign<DirectionT<T>> for PositionT<T> {
    fn add_assign(&mut self, other: DirectionT<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub<Output = T>> Sub<DirectionT<T>> for PositionT<T> {
    type Output = Self;

    fn sub(self, other: DirectionT<T>) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Add<Output = T>> Add for DirectionT<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul<T> for DirectionT<T>
where
    T: Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl From<Position> for Positionf32 {
    fn from(position: Position) -> Self {
        Self {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}

impl From<Direction> for Directionf32 {
    fn from(direction: Direction) -> Self {
        Self {
            x: direction.x as f32,
            y: direction.y as f32,
        }
    }
}

impl From<Positionf32> for Position {
    fn from(position: Positionf32) -> Self {
        Self {
            x: f32::round(position.x) as i32,
            y: f32::round(position.y) as i32,
        }
    }
}

impl From<Directionf32> for Direction {
    fn from(direction: Directionf32) -> Self {
        Self {
            x: f32::round(direction.x) as i32,
            y: f32::round(direction.y) as i32,
        }
    }
}

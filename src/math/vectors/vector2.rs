use super::{Vector, VectorMap};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::{real::Real, Num, NumCast, One, Zero};

#[derive(Copy, Clone, Debug, Default, Eq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    // Create a vector from the elements `x, y`.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn unit_x() -> Self
    where
        T: Zero + One,
    {
        Self {
            x: T::one(),
            y: T::zero(),
        }
    }

    pub fn unit_y() -> Self
    where
        T: Zero + One,
    {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }

    pub fn numcast<D>(self) -> Option<Vector2<D>>
    where
        T: NumCast,
        D: NumCast,
    {
        Some(Vector2::new(
            match D::from(self.x) {
                Some(x) => x,
                None => return None,
            },
            match D::from(self.y) {
                Some(y) => y,
                None => return None,
            },
        ))
    }
}

impl<T> Vector<T, 2> for Vector2<T>
where
    T: Num + Copy,
{
    fn broadcast(value: T) -> Self {
        Self { x: value, y: value }
    }

    fn zero() -> Self {
        Self::broadcast(T::zero())
    }

    fn one() -> Self {
        Self::broadcast(T::one())
    }

    fn iota() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }

    fn into_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn magnitude_squared(self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        self.dot(self)
    }

    fn magnitude(self) -> T
    where
        T: Add<Output = T> + Real,
    {
        self.magnitude_squared().sqrt()
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(T) -> T,
    {
        self.x = f(self.x);
        self.y = f(self.y);
    }

    fn min(&self, other: &Self) -> Self
    where
        T: Ord,
    {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(&self, other: &Self) -> Self
    where
        T: Ord,
    {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn ceil(&self) -> Self
    where
        T: Real,
    {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    fn floor(&self) -> Self
    where
        T: Real,
    {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    fn sum(self) -> T {
        self.x + self.y
    }

    fn sqrt(self) -> Self
    where
        T: Real,
    {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
        }
    }
}

impl<T> PartialEq for Vector2<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl<T> Add for Vector2<T>
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

impl<T> AddAssign for Vector2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vector2<T>
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

impl<T> SubAssign for Vector2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector2<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector2<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T, U> VectorMap<T, U> for Vector2<T>
where
    T: Copy,
{
    type Output = Vector2<U>;

    fn map<F>(&self, f: F) -> Self::Output
    where
        F: Fn(T) -> U,
    {
        Self::Output {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl<T> From<Vector2<T>> for (T, T) {
    fn from(vector: Vector2<T>) -> Self {
        (vector.x, vector.y)
    }
}

impl<T> From<Vector2<T>> for [T; 2] {
    fn from(vector: Vector2<T>) -> Self {
        [vector.x, vector.y]
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from(src: (T, T)) -> Self {
        Self { x: src.0, y: src.1 }
    }
}

impl<T> From<[T; 2]> for Vector2<T>
where
    T: Copy,
{
    fn from(array: [T; 2]) -> Self {
        Self::new(array[0], array[1])
    }
}

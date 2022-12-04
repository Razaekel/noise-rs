use super::{Vector, VectorMap};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::{real::Real, Num, NumCast, One, Zero};

#[derive(Copy, Clone, Debug, Default, Eq)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    // Create a vector from the elements `x, y, z, w`.
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn unit_x() -> Self
    where
        T: Zero + One,
    {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    pub fn unit_y() -> Self
    where
        T: Zero + One,
    {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    pub fn unit_z() -> Self
    where
        T: Zero + One,
    {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
            w: T::zero(),
        }
    }

    pub fn unit_w() -> Self
    where
        T: Zero + One,
    {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::one(),
        }
    }

    pub fn numcast<D>(self) -> Option<Vector4<D>>
    where
        T: NumCast,
        D: NumCast,
    {
        Some(Vector4::new(
            match D::from(self.x) {
                Some(x) => x,
                None => return None,
            },
            match D::from(self.y) {
                Some(y) => y,
                None => return None,
            },
            match D::from(self.z) {
                Some(z) => z,
                None => return None,
            },
            match D::from(self.w) {
                Some(w) => w,
                None => return None,
            },
        ))
    }
}

impl<T> Vector<T, 4> for Vector4<T>
where
    T: Num + Copy,
{
    fn broadcast(value: T) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
            w: value,
        }
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
            z: T::one() + T::one(),
            w: T::one() + T::one() + T::one(),
        }
    }

    fn into_array(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }

    fn dot(self, other: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
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
        self.z = f(self.z);
        self.w = f(self.w);
    }

    fn min(&self, other: &Self) -> Self
    where
        T: Ord,
    {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
            w: self.w.min(other.w),
        }
    }

    fn max(&self, other: &Self) -> Self
    where
        T: Ord,
    {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }

    fn ceil(&self) -> Self
    where
        T: Real,
    {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
            w: self.w.ceil(),
        }
    }

    fn floor(&self) -> Self
    where
        T: Real,
    {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
            w: self.w.floor(),
        }
    }

    fn sum(self) -> T {
        self.x + self.y + self.z + self.w
    }

    fn sqrt(self) -> Self
    where
        T: Real,
    {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
            w: self.w.sqrt(),
        }
    }
}

impl<T> PartialEq for Vector4<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z) && self.w.eq(&other.w)
    }
}

impl<T> Add for Vector4<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> AddAssign for Vector4<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T> Sub for Vector4<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T> SubAssign for Vector4<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl<T> Mul<T> for Vector4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector4<T>
where
    T: Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl<T> Div<T> for Vector4<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector4<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl<T, U> VectorMap<T, U> for Vector4<T>
where
    T: Copy,
{
    type Output = Vector4<U>;

    fn map<F>(&self, f: F) -> Self::Output
    where
        F: Fn(T) -> U,
    {
        Self::Output {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
            w: f(self.w),
        }
    }
}

impl<T> From<Vector4<T>> for (T, T, T, T) {
    fn from(vector: Vector4<T>) -> Self {
        (vector.x, vector.y, vector.z, vector.w)
    }
}

impl<T> From<Vector4<T>> for [T; 4] {
    fn from(vector: Vector4<T>) -> Self {
        [vector.x, vector.y, vector.z, vector.w]
    }
}

impl<T> From<(T, T, T, T)> for Vector4<T> {
    fn from(src: (T, T, T, T)) -> Self {
        Self {
            x: src.0,
            y: src.1,
            z: src.2,
            w: src.3,
        }
    }
}

impl<T> From<[T; 4]> for Vector4<T>
where
    T: Copy,
{
    fn from(array: [T; 4]) -> Self {
        Self::new(array[0], array[1], array[2], array[3])
    }
}

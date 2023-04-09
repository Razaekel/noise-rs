use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::{real::Real, Num, NumCast, One, Zero};

macro_rules! replace_expr {
    ($_t:tt $sub:ident) => {
        $sub
    };
}

macro_rules! vector_type {
    ($type_name:ident, $dim_count:literal, $($dim_index:literal:$dim:ident),+) => {
        #[derive(Copy, Clone, Debug, Default, Eq)]
        pub struct $type_name<T> {
            $(pub $dim: T),+
        }

        impl<T> $type_name<T> {
            // Create a vector from the elements `x, y`.
            #[inline]
            pub fn new($($dim: T),+) -> Self {
                Self { $($dim),+ }
            }

            #[inline]
            pub fn numcast<D>(self) -> Option<$type_name<D>>
            where
                T: NumCast,
                D: NumCast,
            {
                Some($type_name::new(
                    $(D::from(self.$dim)?,)+
                ))
            }
        }

        impl<T: Copy> $type_name<T> {
            #[inline]
            pub fn broadcast(value: T) -> Self {
                Self { $($dim: value),+ }
            }

            #[inline]
            pub fn zero() -> Self
            where
                T: Zero,
            {
                Self::broadcast(T::zero())
            }

            #[inline]
            pub fn one() -> Self
            where
                T: One,
            {
                Self::broadcast(T::one())
            }

            #[inline]
            pub fn into_array(self) -> [T; $dim_count] {
                [$(self.$dim),+]
            }

            #[inline]
            pub fn dot(self, other: Self) -> T
            where
                T: Zero + AddAssign + Mul<Output = T>
            {
                let mut result = T::zero();
                $(result += self.$dim * other.$dim;)+
                result
            }

            #[inline]
            pub fn magnitude_squared(self) -> T
            where
                T: Zero + AddAssign + Mul<Output = T>,
            {
                self.dot(self)
            }

            #[inline]
            pub fn magnitude(self) -> T
            where
                T: Zero + AddAssign + Real,
            {
                self.magnitude_squared().sqrt()
            }

            #[inline]
            pub fn normalize(self) -> Self
            where
                T: Zero + AddAssign + Real,
            {
                self / self.magnitude()
            }

            #[inline]
            pub fn range_squared(self, other: Self) -> T
            where
                T: Zero + AddAssign + Sub + Mul + Num,
            {
                (self - other).magnitude_squared()
            }

            #[inline]
            pub fn range(self, other: Self) -> T
            where
                T: Zero + AddAssign + Mul + Real,
            {
                (self - other).magnitude()
            }

            #[inline]
            pub fn apply<F>(&mut self, f: F)
            where
                F: Fn(T) -> T,
            {
                $(self.$dim = f(self.$dim);)+
            }

            #[inline]
            pub fn min(self, other: Self) -> Self
            where
                T: Ord,
            {
                Self {
                    $($dim: self.$dim.min(other.$dim),)+
                }
            }

            #[inline]
            pub fn max(self, other: Self) -> Self
            where
                T: Ord,
            {
                Self {
                    $($dim: self.$dim.max(other.$dim),)+
                }
            }

            #[inline]
            pub fn ceil(self) -> Self
            where
                T: Real,
            {
                Self {
                    $($dim: self.$dim.ceil(),)+
                }
            }

            #[inline]
            pub fn floor_to_isize(self) -> $type_name<isize>
            where
                T: Real,
            {
                $type_name {
                    $($dim: if self.$dim <= T::zero() {
                        <isize as NumCast>::from(self.$dim).unwrap() - 1
                    } else {
                        <isize as NumCast>::from(self.$dim).unwrap()
                    }),+
                }
            }

            #[inline]
            pub fn sum(self) -> T
            where
                T: Zero + AddAssign,
            {
                let mut result = T::zero();
                $(result += self.$dim;)+
                result
            }

            #[inline]
            pub fn sqrt(self) -> Self
            where
                T: Real,
            {
                Self {
                    $($dim: self.$dim.sqrt(),)+
                }
            }

            #[inline]
            pub fn map<F, U>(self, f: F) -> $type_name<U>
            where
                F: Fn(T) -> U,
            {
                $type_name::<U> {
                    $($dim: f(self.$dim),)+
                }
            }
        }

        impl<T> PartialEq for $type_name<T>
        where
            T: PartialEq,
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                $(self.$dim.eq(&other.$dim)) &&+
            }
        }

        impl<T> Add for $type_name<T>
        where
            T: Add<Output = T>,
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $($dim: self.$dim + rhs.$dim,)+
                }
            }
        }

        impl<T> Add<T> for $type_name<T>
        where
            T: Copy + Add<Output = T>,
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: T) -> Self::Output {
                Self {
                    $($dim: self.$dim + rhs,)+
                }
            }
        }

        impl<T> AddAssign for $type_name<T>
        where
            T: AddAssign,
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                $(self.$dim += rhs.$dim;)+
            }
        }

        impl<T> AddAssign<T> for $type_name<T>
        where
            T: Copy + AddAssign,
        {
            #[inline]
            fn add_assign(&mut self, rhs: T) {
                $(self.$dim += rhs;)+
            }
        }

        impl<T> Sub for $type_name<T>
        where
            T: Sub<Output = T>,
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $($dim: self.$dim - rhs.$dim,)+
                }
            }
        }

        impl<T> Sub<T> for $type_name<T>
        where
            T: Copy + Sub<Output = T>,
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: T) -> Self::Output {
                Self {
                    $($dim: self.$dim - rhs,)+
                }
            }
        }

        impl<T> SubAssign for $type_name<T>
        where
            T: SubAssign,
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$dim -= rhs.$dim;)+
            }
        }

        impl<T> SubAssign<T> for $type_name<T>
        where
            T: Copy + SubAssign,
        {
            #[inline]
            fn sub_assign(&mut self, rhs: T) {
                $(self.$dim -= rhs;)+
            }
        }

        impl<T> Mul for $type_name<T>
        where
            T: Mul<Output = T> + Copy,
        {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    $($dim: self.$dim * rhs.$dim,)+
                }
            }
        }

        impl<T> Mul<T> for $type_name<T>
        where
            T: Mul<Output = T> + Copy,
        {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                Self {
                    $($dim: self.$dim * rhs,)+
                }
            }
        }

        impl<T> MulAssign for $type_name<T>
        where
            T: MulAssign + Copy,
        {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$dim *= rhs.$dim;)+
            }
        }

        impl<T> MulAssign<T> for $type_name<T>
        where
            T: MulAssign + Copy,
        {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                $(self.$dim *= rhs;)+
            }
        }

        impl<T> Div<T> for $type_name<T>
        where
            T: Div<Output = T> + Copy,
        {
            type Output = Self;

            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                Self {
                    $($dim: self.$dim / rhs,)+
                }
            }
        }

        impl<T> DivAssign<T> for $type_name<T>
        where
            T: DivAssign + Copy,
        {
            #[inline]
            fn div_assign(&mut self, rhs: T) {
                $(self.$dim /= rhs;)+
            }
        }

        impl<T> From<$type_name<T>> for ($(replace_expr!($dim T)),+) {
            #[inline]
            fn from(vector: $type_name<T>) -> Self {
                ($(vector.$dim),+)
            }
        }

        impl<T> From<$type_name<T>> for [T; $dim_count] {
            #[inline]
            fn from(vector: $type_name<T>) -> Self {
                [$(vector.$dim),+]
            }
        }

        impl<T> From<($(replace_expr!($dim T)),+)> for $type_name<T> {
            #[inline]
            fn from(src: ($(replace_expr!($dim T)),+)) -> Self {
                let ($($dim),+) = src;
                Self {
                    $($dim,)+
                }
            }
        }

        impl<T> From<[T; $dim_count]> for $type_name<T>
        where
            T: Copy + Num,
        {
            #[inline]
            fn from(array: [T; $dim_count]) -> Self {
                Self {
                    $($dim: array[$dim_index],)+
                }
            }
        }
    }
}

vector_type!(Vector2, 2, 0: x, 1: y);
vector_type!(Vector3, 3, 0: x, 1: y, 2: z);
vector_type!(Vector4, 4, 0: x, 1: y, 2: z, 3: w);

impl<T: Copy> Vector3<T> {
    pub fn cross(&self, other: Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn rotate_axis_angle(&self, axis: Self, angle: T) -> Self
    where
        T: Copy + Real + AddAssign,
    {
        let cos = angle.cos();
        let sin = angle.sin();
        *self * cos + self.cross(axis) * sin + axis * self.dot(axis) * (T::one() - cos)
    }
}

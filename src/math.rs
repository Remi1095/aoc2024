use ndarray::{Ix2, NdIndex};
use num::{CheckedAdd, CheckedSub, NumCast, Signed, ToPrimitive};
use std::{
    fmt::Debug,
    ops::{Add, Neg, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn as_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn from_tuple(tup: (T, T)) -> Self {
        Self { x: tup.0, y: tup.1 }
    }
}

impl Vec2<usize> {
    pub fn from_index_tuple(idx: (usize, usize)) -> Self {
        Self { x: idx.1, y: idx.0 }
    }
}

impl<T> Vec2<T>
where
    T: PartialOrd,
{
    pub fn in_bounds(&self, (lower_bound, upper_bound): (Self, Self)) -> bool {
        self.x >= lower_bound.x
            && self.y >= lower_bound.y
            && self.x < upper_bound.x
            && self.y < upper_bound.x
    }
}

impl<T> Vec2<T>
where
    T: ToPrimitive,
{
    pub fn convert<U>(self) -> Option<Vec2<U>>
    where
        U: NumCast,
    {
        Some(Vec2 {
            x: NumCast::from(self.x)?,
            y: NumCast::from(self.y)?,
        })
    }
}
impl<T> Vec2<T>
where
    T: NumCast + Clone,
{
    pub fn signed_add<U>(&self, other: Vec2<U>) -> Option<Self>
    where
        U: NumCast + Signed + Add,
    {
        (self.clone().convert::<U>()? + other).convert::<T>()
    }
}

impl<T> Vec2<T>
where
    T: Signed,
{
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<T> Add for Vec2<T>
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

impl<T> Sub for Vec2<T>
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

impl<T> CheckedAdd for Vec2<T>
where
    T: CheckedAdd,
{
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(&v.x)?,
            y: self.y.checked_add(&v.y)?,
        })
    }
}

impl<T> CheckedSub for Vec2<T>
where
    T: CheckedSub,
{
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(&v.x)?,
            y: self.y.checked_sub(&v.y)?,
        })
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

unsafe impl NdIndex<Ix2> for Vec2<usize> {
    fn index_checked(&self, dim: &Ix2, strides: &Ix2) -> Option<isize> {
        if self.y < dim[0] && self.x < dim[1] {
            Some(self.index_unchecked(strides))
        } else {
            None
        }
    }

    fn index_unchecked(&self, strides: &Ix2) -> isize {
        (self.y * strides[0] + self.x * strides[1]) as isize
    }
}

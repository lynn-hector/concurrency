use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Mul, Deref};

pub struct Vector<T> {
    data: Vec<T>,
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign+Default,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }
    let mut result = T::default();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    Ok(result)
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}
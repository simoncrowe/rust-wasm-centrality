use serde::Serialize;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn magnitude(self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn unit(self) -> Option<Vector2> {
        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            return None;
        }

        Some(self / magnitude)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Sum<Self> for Vector2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self { x: 0.0, y: 0.0 }, |a, b| Self {
            x: a.x + b.x,
            y: a.y + b.y,
        })
    }
}

#[cfg(test)]
mod tests;
/// Translates a point in graph layout space to display space
///
/// (Display space is clip space in terms of the graphics library.)
pub fn layout_to_clipspace(
    layout_location: Vector2,
    display_offset: Vector2,
    display_scale: f32,
    aspect_ratio: f32,
) -> Vector2 {
    let x = ((layout_location.x - display_offset.x) * display_scale) / aspect_ratio;
    let y = (layout_location.y - display_offset.y) * display_scale;
    Vector2::new(x, y)
}

pub struct Rect {
    pub bottom_left: Vector2,
    pub top_right: Vector2,
}

impl Rect {
    pub fn new(bottom_left: Vector2, top_right: Vector2) -> Rect {
        Rect {
            bottom_left,
            top_right,
        }
    }

    pub fn contains(&self, point: Vector2) -> bool {
        point.x > self.bottom_left.x
            && point.x < self.top_right.x
            && point.y > self.bottom_left.y
            && point.y < self.top_right.y
    }
}

pub struct Points {
    data: Vec<f32>,
}

impl Points {
    pub fn new(data: Vec<f32>) -> Points {
        return Points { data };
    }

    pub fn len(&self) -> usize {
        self.data.len() / 2
    }

    pub fn get_point(&self, index: usize) -> Vector2 {
        let start_index = index * 2;
        Vector2 {
            x: self.data[start_index],
            y: self.data[start_index + 1],
        }
    }

    pub fn iter(&self) -> PointsIter {
        PointsIter {
            points: self,
            index: 0,
        }
    }

    pub fn get_data(&self) -> Vec<f32> {
        self.data.clone()
    }

    pub fn to_clipspace(
        &self,
        display_offset: Vector2,
        display_scale: &f32,
        aspect_ratio: &f32,
    ) -> Points {
        let mut clipspace_data: Vec<f32> = vec![0.0; self.data.len()];
        for index in (0..self.data.len()).step_by(2) {
            clipspace_data[index] =
                ((self.data[index] - display_offset.x) * display_scale) / aspect_ratio;
            clipspace_data[index + 1] = (self.data[index + 1] - display_offset.y) * display_scale;
        }
        Points {
            data: clipspace_data,
        }
    }
}

pub struct PointsIter<'a> {
    points: &'a Points,
    index: usize,
}

impl<'a> Iterator for PointsIter<'a> {
    type Item = Vector2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.points.len() {
            return None;
        }
        let point = self.points.get_point(self.index);
        self.index += 1;
        Some(point)
    }
}

const DIVISION_BY_ZERO_MESSAGE: &str = "Attempt to divide by zero";

use crate::logger::log_error;

/// Represents a two-dimensional vector
#[derive(Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Default for Vector2 {
    fn default() -> Vector2 {
        Vector2::zero()
    }
}

impl Vector2 {
    /// Creates a vector at x and y coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Creates a vector on the zero coordinates
    pub fn zero() -> Self {
        Self::new(0., 0.)
    }

    /// Adds one vector to another, mutating the first one in place
    pub fn add(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }

    /// Subtracts one vector from another, mutating the first one in place
    pub fn subtract(&mut self, other: &Vector2) {
        self.x -= other.x;
        self.y -= other.y;
    }

    /// Scales a vector, mutating it in place
    pub fn scale(&mut self, scaler: f64) {
        self.x *= scaler;
        self.y *= scaler;
    }

    /// Divides a vector, mutating it in place
    pub fn divide(&mut self, divider: f64) {
        if divider == 0. {
            log_error(DIVISION_BY_ZERO_MESSAGE);

            return;
        }

        self.x /= divider;
        self.y /= divider;
    }

    /// Translates a vector, mutating it in place
    pub fn translate(&mut self, increment: f64) {
        self.x += increment;
        self.y += increment;
    }

    /// Updates one vector with the values of another, mutating the first one in place
    pub fn set(&mut self, other: &Vector2) {
        self.x = other.x;
        self.y = other.y;
    }

    /// Sets a vector's values to zero
    pub fn reset(&mut self) {
        self.x = 0.;
        self.y = 0.;
    }

    /// Gets the squared magnitude of a vector
    /// Faster than [`Vector2::magnitude`]
    pub fn squared_magnitude(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    /// Gets the magnitude of a vector
    /// Slower than [`Vector2::squared_magnitude`]
    pub fn magnitude(&self) -> f64 {
        self.squared_magnitude().sqrt()
    }

    /// Gets the dot product of two vectors
    pub fn dot_product(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Gets the cross product of two vectors
    pub fn cross_product(&self, other: &Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Normalizes a vector (transform it to a unit vector), mutating it in place
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();

        self.divide(magnitude);
    }

    /// Creates a perpendicular vector to the given vector
    pub fn create_perpendicular(&self) -> Vector2 {
        let mut flipped_vector = Vector2::new(self.y, -self.x);

        flipped_vector.normalize();

        flipped_vector
    }

    /// Rotates a vector to the given angle
    /// ### Arguments
    /// * `angle` - angle in radians
    pub fn rotate(&mut self, angle: f64) {
        let cos = angle.cos();
        let sin = angle.sin();

        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;

        self.x = x;
        self.y = y;
    }

    /// Creates a copy of the given vector
    pub fn create_copy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    /// Rotates a vector to the given angle, using another vector as the pivot point, mutating the first vector in place
    /// ### Arguments
    /// * angle - angle in radians
    pub fn rotate_at(&mut self, pivot: &Vector2, angle: f64) {
        let x = self.x - pivot.x;
        let y = self.y - pivot.y;

        let mut temporary_vector = Vector2::new(x, y);

        temporary_vector.rotate(angle);
        temporary_vector.add(pivot);

        self.set(&temporary_vector);
    }

    /// Adds two vectors to create a new one
    pub fn to_added(&self, other: &Vector2) -> Vector2 {
        let mut copy = self.create_copy();

        copy.add(other);

        copy
    }

    /// Subtracts two vectors to create a new one
    pub fn to_subtracted(&self, other: &Vector2) -> Vector2 {
        let mut copy = self.create_copy();

        copy.subtract(other);

        copy
    }

    /// Scales a vector to create a new one
    pub fn to_scaled(&self, scaler: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.scale(scaler);

        copy
    }

    /// Divides a vector to create a new one
    pub fn to_divided(&self, divider: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.divide(divider);

        copy
    }

    /// Translates a vector to create a new one
    pub fn to_translated(&self, increment: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.translate(increment);

        copy
    }

    /// Creates a normalized version of the given vector
    pub fn to_normalized(&self) -> Vector2 {
        let mut copy = self.create_copy();

        copy.normalize();

        copy
    }

    /// Creates a rotated version of the given vector to the given angle
    /// ### Arguments
    /// * angle - angle in radians
    pub fn to_rotated(&self, angle: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.rotate(angle);

        copy
    }

    /// Creates a rotated version of the given vector to the given angle at the given pivot point
    /// ### Arguments
    /// * angle - angle in radians
    pub fn to_rotated_at(&self, pivot: &Vector2, angle: f64) -> Vector2 {
        let mut copy = self.create_copy();

        copy.rotate_at(pivot, angle);

        copy
    }
}

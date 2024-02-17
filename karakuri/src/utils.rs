/// Represents RGBA color
#[derive(Debug)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    /// Create a color array
    pub fn to_array(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

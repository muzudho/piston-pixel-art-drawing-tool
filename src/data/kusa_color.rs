use serde::{Deserialize, Serialize};

/// In the Piston library, the order is R, G, B, A.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")] // プロパティ名が JSON 側でスネークケースであることを指定
pub struct KusaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl KusaColor {
    pub fn to_rgba_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn to_rgba_rate_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255f32,
            self.g as f32 / 255f32,
            self.b as f32 / 255f32,
            self.a as f32 / 255f32,
        ]
    }
}
impl Default for KusaColor {
    fn default() -> Self {
        // とりあえず白
        KusaColor {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
        /*
        KusaColor {
            r: 0,
            g: 128,
            b: 128,
            a: 255,
        }
        */
    }
}

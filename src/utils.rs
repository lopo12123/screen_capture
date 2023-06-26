use std::ops::Mul;

struct I32(i32);

impl Mul<f32> for I32 {
    type Output = i32;

    fn mul(self, rhs: f32) -> Self::Output {
        (rhs * self.0 as f32) as i32
    }
}

impl Mul<f64> for I32 {
    type Output = i32;

    fn mul(self, rhs: f64) -> Self::Output {
        (rhs * self.0 as f64) as i32
    }
}


pub fn physical_to_logic(physical: i32, scale_factor: f32) -> i32 {
    if scale_factor == 1.0 { physical } else { I32(physical) * scale_factor }
}

/// physical to logic
pub fn physical_to_logic_xy(physical: (i32, i32), scale_factor: f32) -> (i32, i32) {
    if scale_factor == 1.0 { physical } else { (I32(physical.0) * scale_factor, I32(physical.1) * scale_factor) }
}

/// physical to logic
pub fn physical_to_logic_xywh(physical: (i32, i32, i32, i32), scale_factor: f32) -> (i32, i32, i32, i32) {
    if scale_factor == 1.0 { physical } else {
        let (x, y, w, h) = physical;

        (I32(x) * scale_factor, I32(y) * scale_factor, I32(w) * scale_factor, I32(h) * scale_factor)
    }
}
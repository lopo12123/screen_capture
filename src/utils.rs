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


/// 应用缩放后的坐标
#[allow(unused)]
pub fn logic_after_scale(logic: i32, scale_factor: f32) -> i32 {
    if scale_factor == 1.0 { logic } else { I32(logic) * scale_factor }
}

/// 应用缩放后的坐标
#[allow(unused)]
pub fn get_real_coord_xy(logic: (i32, i32), scale_factor: f32) -> (i32, i32) {
    // logic
    if scale_factor == 1.0 { logic } else { (I32(logic.0) * scale_factor, I32(logic.1) * scale_factor) }
}

/// 应用缩放后的坐标
///
/// scale_factors: `(primary_sf: f32, current_sf: f32)`
#[allow(unused)]
pub fn logic_after_scale_xywh(logic: (i32, i32, i32, i32), scale_factor: f32) -> (i32, i32, i32, i32) {
    let (x, y, w, h) = logic;

    (
        I32(x) * scale_factor,
        I32(y) * scale_factor,
        I32(w) * scale_factor,
        I32(h) * scale_factor,
    )
}
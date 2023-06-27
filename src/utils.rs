/// 根据聚焦的屏幕和目标屏幕的 scale_factor 换算出坐标
pub fn get_real_coord_of_event(sfp: f32, sft: f32, ev_coord: (i32, i32)) -> (i32, i32) {
    let rate = sft / sfp;
    let (x, y) = ev_coord;

    (
        (x as f32 * rate) as i32,
        (y as f32 * rate) as i32,
    )
}

/// 获取 scale_factor 为 1 时的宽高
pub fn get_real_wh_before_scale(sf: f32, w: i32, h: i32) -> (i32, i32) {
    (
        (w as f32 * sf) as i32,
        (h as f32 * sf) as i32,
    )
}
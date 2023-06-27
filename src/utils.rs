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
pub fn get_real_wh_before_scale(sf: f32, wh: (i32, i32)) -> (i32, i32) {
    (
        (wh.0 as f32 * sf) as i32,
        (wh.1 as f32 * sf) as i32,
    )
}

/// 获取 scale_factor 为 1 时的位置和宽高
pub fn get_real_xywh_before_scale(sf: f32, xywh: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    (
        (xywh.0 as f32 * sf) as i32,
        (xywh.1 as f32 * sf) as i32,
        (xywh.2 as f32 * sf) as i32,
        (xywh.3 as f32 * sf) as i32,
    )
}
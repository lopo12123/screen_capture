use std::cmp::max;

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

/// 获取按钮组的部署位置 (按钮组: 60 * 30)
pub fn get_position_of_buttons(start: (i32, i32), end: (i32, i32), screen_size: (i32, i32)) -> (i32, i32) {
    // 包围盒
    let dis_x = i32::abs(start.0 - end.0);
    let dis_y = i32::abs(start.1 - end.1);

    // 右下角点位
    let xr = max(start.0, end.0);
    let yr = max(start.1, end.1);

    // 锚点 y 坐标
    let anchor_y = if yr > 50 { yr - 30 } else { yr };

    if dis_x > 100 && dis_y > 50 {
        // 1. 默认情况 - 内部右下角
        (xr - 60, anchor_y)
    } else if screen_size.0 - xr > 100 {
        // 2. 选框过小 - 外部右下角
        (xr, anchor_y)
    } else {
        (xr - dis_x - 60, anchor_y)
    }
}
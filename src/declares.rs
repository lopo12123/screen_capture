/// 综合屏幕信息
#[derive(Debug, Clone)]
pub struct ScreenInfo {
    /// 是否是主要屏幕
    pub is_primary: bool,

    /// 屏幕id (screenshots 使用, 1 表示未找到)
    pub screen_id: u32,

    /// 屏幕序号 (从0开始递增, fltk 使用)
    pub screen_num: i32,

    /// 缩放比例
    pub scale_factor: f32,

    /// 屏幕坐标和宽高 (当前真实逻辑坐标系)
    ///
    /// `(x: i32, y: i32, w: i32, h: i32)`
    pub xywh_real: (i32, i32, i32, i32),

    /// 屏幕坐标和宽高 (scale_factor 为 1 时的数值)
    ///
    /// `(x: i32, y: i32, w: i32, h: i32)`
    pub xywh_origin: (i32, i32, i32, i32),
}

/// 屏幕捕获信息
#[derive(Debug, Clone)]
pub struct CaptureInfo {
    pub screen_id: u32,
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}
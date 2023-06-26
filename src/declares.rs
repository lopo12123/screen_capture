/// 综合屏幕信息
#[derive(Debug)]
pub struct ScreenInfo {
    /// 是否是主要屏幕
    pub is_primary: bool,

    /// 屏幕id (screenshots 使用)
    pub screen_id: u32,

    /// 屏幕序号 (从0开始递增, fltk 使用)
    pub screen_num: i32,

    /// 缩放比例
    pub scale_factor: f32,

    /// 屏幕坐标和宽高
    ///
    /// `(x: i32, y: i32, w: i32, h: i32)`
    pub xywh_physical: (i32, i32, i32, i32),

    /// 屏幕坐标和宽高
    ///
    /// `(x: i32, y: i32, w: i32, h: i32)`
    pub xywh_logic: (i32, i32, i32, i32),
}

/// 屏幕捕获信息
pub struct CaptureInfo {
    pub screen_id: u32,
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}
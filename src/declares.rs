/// 屏幕捕获信息
#[derive(Debug, Clone)]
#[napi(object)]
pub struct CaptureInfo {
    // 所处屏幕的 id
    pub screen_id: u32,
    // 所处屏幕的 scale_factor
    pub scale_factor: f64,
    // 截图左上角 x
    pub physical_x: i32,
    // 截图左上角 y
    pub physical_y: i32,
    // 图片宽度
    pub physical_width: u32,
    // 图片高度
    pub physical_height: u32,
    // 图片的 buffer (可直接使用)
    pub buffer: Vec<u8>,
    // 原始 rgba 阵列 (用于创建纹理)
    pub rgba: Vec<u8>,
}

/// 选中的区域
#[derive(Debug, Clone)]
#[napi(object)]
pub struct SelectedImage {
    /// \[xmin, ymin, xmax, ymax\]
    #[napi(ts_type = "[xmin: number, ymin: number, xmax: number, ymax: number]")]
    pub p1p2: Vec<f64>,
    // 图片的 buffer (可直接使用)
    pub buffer: Vec<u8>,
}
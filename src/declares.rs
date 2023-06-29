/// 屏幕捕获信息
#[derive(Debug, Clone)]
#[napi(object)]
pub struct CaptureInfo {
    pub screen_id: u32,
    pub scale_factor: f64,
    pub physical_x: i32,
    pub physical_y: i32,
    pub physical_width: u32,
    pub physical_height: u32,
    pub buffer: Vec<u8>,
}
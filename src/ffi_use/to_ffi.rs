use crate::declares::{CaptureInfo, ScreenInfo};

pub trait FfiConvertible<Output> {
    fn to_ffi(&self) -> Output;
}

// region 综合屏幕信息
/// 综合屏幕信息 (ffi convertible)
///
/// 原始结构 [ScreenInfo]
#[derive(Debug)]
#[napi(object)]
pub struct ScreenInfoFfi {
    /// 是否是主要屏幕
    pub is_primary: bool,

    /// 屏幕id (screenshots 使用)
    pub screen_id: u32,

    /// 屏幕序号 (从0开始递增, fltk 使用)
    pub screen_num: i32,

    /// 缩放比例
    pub scale_factor: f64,

    /// 屏幕坐标和宽高 (当前真实逻辑坐标系)
    ///
    /// from tuple: `(x: i32, y: i32, w: i32, h: i32)`
    #[napi(ts_type = "[x: number, y: number, w: number, h: number]")]
    pub xywh_real: Vec<i32>,
}

impl FfiConvertible<ScreenInfoFfi> for ScreenInfo {
    fn to_ffi(&self) -> ScreenInfoFfi {
        let (lx, ly, lw, lh) = self.xywh_real;

        ScreenInfoFfi {
            is_primary: self.is_primary,
            screen_id: self.screen_id,
            screen_num: self.screen_num,
            scale_factor: self.scale_factor as f64,
            xywh_real: vec![lx, ly, lw, lh],
        }
    }
}
// endregion

// region 综合屏幕信息
/// 屏幕捕获信息 (ffi convertible)
///
/// 原始结构 [CaptureInfo]
#[derive(Debug)]
#[napi(object)]
pub struct CaptureInfoFfi {
    pub screen_id: u32,
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl FfiConvertible<CaptureInfoFfi> for CaptureInfo {
    fn to_ffi(&self) -> CaptureInfoFfi {
        CaptureInfoFfi {
            screen_id: self.screen_id,
            width: self.width,
            height: self.height,
            buffer: self.buffer.clone(),
        }
    }
}
// endregion

// region 交互式选择区域
/// 交互式选择区域 (ffi convertible)
///
/// from tuple: `(screen_id: u32, x1: i32, y1: i32, x2: i32, y2: i32)`
#[derive(Debug)]
#[napi(object)]
pub struct BoundingBox {
    pub screen_id: u32,
    #[napi(ts_type = "[x1: number, y1: number, x2: number, y2: number]")]
    pub bounding_box: Vec<i32>,
}

impl BoundingBox {
    pub fn from_tuple(tuple: (u32, i32, i32, i32, i32)) -> BoundingBox {
        BoundingBox {
            screen_id: tuple.0,
            bounding_box: vec![tuple.1, tuple.2, tuple.3, tuple.4],
        }
    }
}
// endregion


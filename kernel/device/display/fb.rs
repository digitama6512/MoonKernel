use core::ops::Mul;

const FB_WIDTH: u32 = 2048; // 帧缓冲区宽度
const FB_HEIGHT: u32 = 1536; // 帧缓冲区高度
const FB_PITCH: u32 = FB_WIDTH * 4; // 帧缓冲区一行字节数

/// `RGBColor` RGB颜色格式
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RGBColor {
    Black = 0x000000,
    Red = 0xAA0000,
    Green = 0x00AA00,
    Yellow = 0xAAAA00,
    Brown = 0xAA5500,
    Blue = 0x0000AA,
    Magenta = 0xAA00AA,
    Cyan = 0x00AAAA,
    Grey = 0xAAAAAA,
}
/* 实现向u32转换 */
impl From<RGBColor> for u32 {
    fn from(c: RGBColor) -> u32 {
        c as u32
    }
}

const DEFAULT_FGCOLOR: u32 = RGBColor::Grey.into(); // 默认前显颜色
const DEFAULT_BGCOLOR: u32 = RGBColor::Black.into(); // 默认背景色

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct FbInfo {
    addr: *mut u8,
    bgbuffer: *mut u8,
    swapbuffer: *mut u8,
    width: u32,
    height: u32,
    pitch: u32,
    backbuffer: *mut u8,
    backbuffer_len: u32,
}
impl FbInfo {
    pub fn init(
        &mut self,
        framebuffer: Option<&limine::framebuffer::Framebuffer<'_>>,
    ) {
        match framebuffer {
            None => {
                if self.addr as *mut u64 == self.backbuffer as *mut u64 {
                    self.backbuffer = 
                }
            }
        }
    }
}

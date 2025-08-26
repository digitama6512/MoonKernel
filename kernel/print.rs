use limine;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RGBColor {
    Red = 0x00FF0000,
    Yellow = 0x00FFFF00,
    White = 0x00FFFFFF,
}
/* 实现向u32转换 */
impl From<RGBColor> for u32 {
    fn from(c: RGBColor) -> u32 {
        c as u32
    }
}

/* 每个字形是8x16位图 */
const PSF1FONT_WIDTH: u64 = 8;
const PSF1FONT_HEIGHT: u64 = 16;

pub struct FramebufferConsole {
    // 帧缓冲区
    framebuffer_saddr: *mut u8,
    // 字体
    psf1fonts_saddr: *mut u8,
    // 可容纳字形的宽度
    width: u64,
    // 可容纳字形的高度
    height: u64,
    // 帧缓冲区一行的字节数
    pitch: u64,
    // 当前x（像素）
    current_width: u64,
    // 当前y（像素）
    current_height: u64,
}

impl FramebufferConsole {
    /// 初始化控制台
    pub fn new(
        framebuffer: &limine::framebuffer::Framebuffer<'_>,
        psf1module: &limine::file::File,
    ) -> Self {
        Self {
            framebuffer_saddr: framebuffer.addr(),
            // 跳过PSF1 Header
            psf1fonts_saddr: unsafe { psf1module.addr().add(4) },
            width: framebuffer.width() / PSF1FONT_WIDTH,
            height: framebuffer.height() / PSF1FONT_HEIGHT,
            pitch: framebuffer.pitch(),
            current_width: 0,
            current_height: 0,
        }
    }

    fn clear_font_row(&mut self, font_row: u64) {
        // 帧缓存区待清除起始地址
        let framebuffer_caddr = unsafe {
            self.framebuffer_saddr.add(
                font_row as usize
                    * PSF1FONT_HEIGHT as usize
                    * self.pitch as usize,
            )
        };

        for row in 0..(PSF1FONT_HEIGHT as usize) {
            let clear_addr =
                unsafe { framebuffer_caddr.add(row * self.pitch as usize) };
            for col in 0..(self.width as usize) {
                unsafe {
                    core::ptr::write_volatile(
                        clear_addr.add(col * 4).cast::<u32>(),
                        0,
                    );
                }
            }
        }
    }

    fn new_line(&mut self) {
        if self.current_height == self.height - 1 {
            // 到底，滚动一行字形高度
            let row_bytes = (self.pitch * PSF1FONT_HEIGHT) as usize;
            let total_bytes =
                (self.height * PSF1FONT_HEIGHT * self.pitch) as usize;

            unsafe {
                // 整块内存往上移动一行字形高度
                core::ptr::copy(
                    self.framebuffer_saddr.add(row_bytes),
                    self.framebuffer_saddr,
                    total_bytes - row_bytes,
                );
            }

            self.clear_font_row(self.height - 1);
            self.current_height = self.height - 1;
            self.current_width = 0;
        } else {
            // 未到底，换行
            self.current_height += 1;
            self.current_width = 0;
        }
    }

    fn write_byte(&mut self, byte: u8, color: u32) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.current_width == self.width {
                    self.new_line();
                }

                // 字形起始地址
                let font_saddr =
                    unsafe { self.psf1fonts_saddr.add(16 * byte as usize) };

                // 帧缓存区待写入地址
                let framebuffer_waddr = unsafe {
                    self.framebuffer_saddr.add(
                        self.current_height as usize
                            * PSF1FONT_HEIGHT as usize
                            * self.pitch as usize
                            + self.current_width as usize
                                * PSF1FONT_WIDTH as usize
                                * 4,
                    )
                };
                // 打印字形
                for row in 0..PSF1FONT_HEIGHT as usize {
                    let row_of_font = unsafe { *(font_saddr.add(row)) };
                    for col in 0..PSF1FONT_WIDTH as usize {
                        // 寻找framebuffer对应像素位置
                        let write_addr = unsafe {
                            framebuffer_waddr
                                .add(row * self.pitch as usize + col * 4)
                                .cast::<u32>()
                        };
                        // 判断字形位图中第row行第col列的像素是否需要点亮
                        if (row_of_font >> (7 - col)) & 1 != 0 {
                            unsafe {
                                core::ptr::write_volatile(write_addr, color);
                            };
                        } else {
                            unsafe {
                                core::ptr::write_volatile(write_addr, 0);
                            };
                        }
                    }
                }

                self.current_width += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str, color: u32) {
        for byte in s.bytes() {
            match byte {
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                0x20..=0x7e | b'\n' => self.write_byte(byte, color),
                // 不包含在上述范围之内的字节
                _ => self.write_byte(b'*', color),
            }
        }
    }

    pub fn println_info(&mut self, s: &str) {
        let color = RGBColor::White.into();
        self.write_string("[info]: ", color);
        self.write_string(s, color);
        self.write_byte(b'\n', color);
    }

    pub fn println_warning(&mut self, s: &str) {
        let color = RGBColor::Yellow.into();
        self.write_string("[warning]: ", color);
        self.write_string(s, color);
        self.write_byte(b'\n', color);
    }

    pub fn println_error(&mut self, s: &str) {
        let color = RGBColor::Red.into();
        self.write_string("[error]: ", color);
        self.write_string(s, color);
        self.write_byte(b'\n', color);
    }
}

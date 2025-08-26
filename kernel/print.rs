use limine;

pub struct FramebufferConsole {
    // 帧缓冲区
    framebuffer: &'static mut [u8],
    // 字体
    psf1module: &'static limine::file::File,
    // 屏幕宽度（像素）
    width: usize,
    // 屏幕高度（像素）
    height: usize,
    // 每行字节数
    pitch: usize,
    // 当前光标x（像素）
    x: usize,
    // 当前光标y（像素）
    y: usize,
    color: u32,
}

impl FramebufferConsole {
    /// 初始化控制台
    pub fn new(
        framebuffer: &'static mut [u8],
        width: usize,
        height: usize,
        pitch: usize,
        color: u32,
    ) -> Self {
        Self {
            framebuffer,
            width,
            height,
            pitch,
            x: 0,
            y: 0,
            color,
        }
    }

    /// 设置像素
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.pitch + x * 4;
        if idx + 3 < self.framebuffer.len() {
            self.framebuffer[idx] = (color & 0xFF) as u8;
            self.framebuffer[idx + 1] = ((color >> 8) & 0xFF) as u8;
            self.framebuffer[idx + 2] = ((color >> 16) & 0xFF) as u8;
            self.framebuffer[idx + 3] = 0; // Alpha
        }
    }

    /// 绘制单字符（8x8字体）
    pub fn draw_char(&mut self, c: char) {
        let bitmap = FONT[c as usize];
        for row in 0..8 {
            for col in 0..8 {
                if (bitmap[row] >> col) & 1 != 0 {
                    self.put_pixel(self.x + col, self.y + row, self.color);
                }
            }
        }
        self.x += 8;
        if self.x + 8 > self.width {
            self.x = 0;
            self.y += 8;
        }
    }

    /// 打印字符串
    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            if c == '\n' {
                self.x = 0;
                self.y += 8;
            } else {
                self.draw_char(c);
            }
        }
    }

    /// 打印并换行
    pub fn writeln(&mut self, s: &str) {
        self.write_str(s);
        self.x = 0;
        self.y += 8;
    }
}

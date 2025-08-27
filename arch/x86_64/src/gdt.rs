use x86_64::structures::DescriptorTablePointer;

/// `GdtEntry` GDT条目（Global Descriptor Table Entry），每个条目 8 字节（64 位）
///
/// 描述段的基址、界限和访问权限
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct GdtEntry {
    pub limitl: u16,      // 段界限低16位
    pub offsetl: u16,     // 段基址低16位
    pub offsetm: u8,      // 段基址中间8位
    pub access: u8,       // 访问权限字节
    pub flags_limith: u8, // 高四位是Flags,第四位是段界限高4位
    pub offseth: u8,      // 段基址高8位
}

impl GdtEntry {
    /// `new` 创建一个GDT条目
    ///
    /// - `offset`: 段基址（32位）
    /// - `limit`: 段界限（20位）
    /// - `access`: 访问权限字节（8位）
    /// - `flags`: 标志字节（高4位）
    ///
    ///   `return`: GDT条目
    pub const fn new(offset: u32, limit: u32, access: u8, flags: u8) -> Self {
        GdtEntry {
            limitl: limit as u16,
            offsetl: offset as u16,
            offsetm: (offset >> 16) as u8,
            access: access,
            flags_limith: flags & 0xF0 | ((limit >> 16) as u8) & 0x0F,
            offseth: (offset >> 24) as u8,
        }
    }

    /// `set_offset` 设置段的基址
    ///
    /// - `offset`: 段基址
    pub fn set_offset(&mut self, offset: u32) {
        self.offsetl = offset as u16;
        self.offsetm = (offset >> 16) as u8;
        self.offseth = (offset >> 24) as u8;
    }

    /// `set_limit` 设置段界限
    ///
    /// - `limit`: 段界限
    pub fn set_limit(&mut self, limit: u32) {
        self.limitl = limit as u16;
        self.flags_limith =
            self.flags_limith & 0xF0 | ((limit >> 16) as u8) & 0x0F;
    }
}

// =====================================
// GDT 条目索引（Segment Selector Index）
// =====================================
pub const GDT_NULL: usize = 0; // 空描述符，必须存在
pub const GDT_KERNEL_CODE: usize = 1; // 内核代码段
pub const GDT_KERNEL_DATA: usize = 2; // 内核数据段
pub const GDT_KERNEL_TLS: usize = 3; // 内核 TLS（Thread Local Storage，线程本地存储）
pub const GDT_USER_CODE: usize = 4; // 用户态代码段
pub const GDT_USER_DATA: usize = 5; // 用户态数据段
pub const GDT_USER_TLS: usize = 6; // 用户态 TLS
pub const GDT_TSS: usize = 7; // 任务状态段（TSS）低 32 位
pub const GDT_TSS_HIGH: usize = 8; // TSS 高 32 位（64 位模式）

// ==============================
// GDT 访问字节（Access Byte）常量
// ==============================
pub const GDT_A_P_PRESENT: u8 = 1 << 7; // P 位：段存在
pub const GDT_A_DPL_RING_0: u8 = 0 << 5; // DPL = 0（最高特权级，内核态）
pub const GDT_A_DPL_RING_1: u8 = 1 << 5; // DPL = 1
pub const GDT_A_DPL_RING_2: u8 = 2 << 5; // DPL = 2
pub const GDT_A_DPL_RING_3: u8 = 3 << 5; // DPL = 3（最低特权级，用户态）
pub const GDT_A_S_CODEORDATA: u8 = 1 << 4; // S 位：1 = 代码/数据段，0 = 系统段（TSS/IDT）
pub const GDT_A_E_CODE: u8 = 1 << 3; // 可执行段（代码段）
pub const GDT_A_DC_ASSERT: u8 = 1 << 2; // 顺序/可执行（代码段）或向下扩展段（数据段）
pub const GDT_A_RW_ASSERT: u8 = 1 << 1; // 数据段可写，代码段可读
pub const GDT_A_A_WRITED: u8 = 1; // 已使用/修改标记（数据段或栈段被写入过）

// ==============================
// TSS 访问类型（系统段特有）
// ==============================
pub const GDT_A_TSS_AVAIL: u8 = 0x9; // TSS描述符可用
pub const GDT_A_TSS_BUSY: u8 = 0xB; // 正在使用TSS描述符

// ==============================
// GDT 标志（Flags，高 4 位）
// ==============================
pub const GDT_F_G_PAGE_SIZE: u8 = 1 << 7; // G 位：（1 = 段界限单位为4KB）
pub const GDT_F_DB_PROTECTEDMODE: u8 = 1 << 6; // D/B位（1 = 保护模式32位）
pub const GDT_F_L_LONG_MODE: u8 = 1 << 5; // L 位：（1 = 64 位代码段，x86_64模式）

/// `INIT_GDTR` GDTR初始化
static mut INIT_GDTR: DescriptorTablePointer =
    DescriptorTablePointer { limit: 0, base: 0 };

/// `INIT_GDT` GDT表初始化
static mut INIT_GDT: [GdtEntry; 4] = [
    // 空描述符
    GdtEntry::new(0, 0, 0, 0),
    // 内核代码段
    GdtEntry::new(
        0,
        0,
        GDT_A_P_PRESENT
            | GDT_A_DPL_RING_0
            | GDT_A_S_CODEORDATA
            | GDT_A_E_CODE
            | GDT_A_RW_ASSERT,
        GDT_F_L_LONG_MODE,
    ),
    // 内核数据段
    GdtEntry::new(
        0,
        0,
        GDT_A_P_PRESENT
            | GDT_A_DPL_RING_0
            | GDT_A_S_CODEORDATA
            | GDT_A_RW_ASSERT,
        GDT_F_L_LONG_MODE,
    ),
    // 内核TLS段
    GdtEntry::new(
        0,
        0,
        GDT_A_P_PRESENT
            | GDT_A_DPL_RING_3
            | GDT_A_S_CODEORDATA
            | GDT_A_RW_ASSERT,
        GDT_F_L_LONG_MODE,
    ),
];

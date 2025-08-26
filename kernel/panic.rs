use core::panic::PanicInfo;

/// `panic` - 自定义panic处理函数
///
/// 在 no_std 环境中，我们需要定义一个自己的 panic 处理函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    crate::hcf();
}

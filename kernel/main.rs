#![no_std] // 不使用标准库
#![no_main] // 不使用标准的主函数入口
#![feature(custom_test_frameworks)] // 由于不使用标准库，只能自定义test框架
#![test_runner(crate::test_runner)] // test_runner所在位置

mod print;
mod device;

use core::arch::asm;
use limine::BaseRevision;
use limine::file::File;
use limine::request::{
    FramebufferRequest, ModuleRequest, RequestsEndMarker, RequestsStartMarker,
};

#[cfg(target_arch = "x86_64")]
#[macro_use]
extern crate arch_x86_64 as arch;

use crate::print::FramebufferConsole;

#[used] // 强制编译器保留此变量（即使未被显式使用）
#[unsafe(link_section = ".requests")] // 强制将此变量放在 ELF 文件的 .requests 段
static BASE_REVISION: BaseRevision = BaseRevision::new(); // 声明内核支持的 Limine 引导协议版本
#[used]
#[unsafe(link_section = ".requests")]
static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(); // 向引导程序请求图形帧缓冲区，引导程序在初始化阶段会填充这个请求的响应数据（如内存地址、分辨率、像素格式等）

// ========
// 边界标志
// ========
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)] // 使用 no_mangle 标记这个函数，来对它禁用名称重整
/// `kmain` 程序的入口点，extern "C" 表示这个函数使用C语言的ABI，使其可以被引导加载程序调用
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    /* 获取帧缓冲区信息 */
    let framebuffer_response = FRAMEBUFFER_REQUEST.get_response().unwrap();
    let mut framebuffers = framebuffer_response.framebuffers();
    let framebuffer = framebuffers.next().unwrap();

    /* 获取模块信息 */
    let module_response = MODULE_REQUEST.get_response().unwrap();
    let modules = module_response.modules();
    let psf1module: &File = modules.get(0).unwrap(); // 获取psf1

    /* 创建内核打印器 */
    let mut frameconsole = FramebufferConsole::new(&framebuffer, psf1module);

    frameconsole.println_error("test errror");
    frameconsole.println_warning("test warning");
    frameconsole.println_info("test info");

    frameconsole.println_info("kernel hlt");

    hlt();
}

/// `panic` 自定义panic处理函数
///
/// 在 no_std 环境中，我们需要定义一个自己的 panic 处理函数
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    hlt();
}

/// `test_runner` 测试运行器函数
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

/// `hlt` cpu停止执行指令
fn hlt() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}

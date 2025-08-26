// 不使用标准库
#![no_std]
// 不使用标准的主函数入口
#![no_main]
// 由于不适用标准库，只能自定义test框架
#![feature(custom_test_frameworks)]
// test_runner所在位置
#![test_runner(crate::test::test_runner)]

use core::arch::asm;
use limine::BaseRevision;
use limine::file::File;
use limine::request::{
    FramebufferRequest, ModuleRequest, RequestsEndMarker, RequestsStartMarker,
};

use crate::print::{FramebufferConsole, RGBColor};

mod panic;
mod print;
mod test;

// 强制编译器保留此变量（即使未被显式使用）
#[used]
// 强制将此变量放在 ELF 文件的 .requests 段
#[unsafe(link_section = ".requests")]
// 声明内核支持的 Limine 引导协议版本
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
// 向引导程序请求图形帧缓冲区，引导程序在初始化阶段会填充这个请求的响应数据（如内存地址、分辨率、像素格式等）
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

// 边界标记
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

// 使用 no_mangle 标记这个函数，来对它禁用名称重整
#[unsafe(no_mangle)]
/// `kmain` 程序的入口点，extern "C" 表示这个函数使用C语言的ABI，，使其可以被引导加载程序调用
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    // 获取帧缓冲区信息
    let framebuffer_response = FRAMEBUFFER_REQUEST.get_response().unwrap();
    let mut framebuffers = framebuffer_response.framebuffers();
    let framebuffer = framebuffers.next().unwrap();

    // 获取模块信息
    let module_response = MODULE_REQUEST.get_response().unwrap();
    let modules = module_response.modules();
    // 获取psf1
    let psf1module: &File = modules.get(0).unwrap();

    let mut frameconsole = FramebufferConsole::new(&framebuffer, psf1module);

    frameconsole.println_error("test errror");
    frameconsole.println_warning("test warning");
    frameconsole.println_info("test info");

    frameconsole.println_info("kernel hlt");

    hcf();
}

/// `hcf` 停机函数
fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}

# MoonKernel

MoonKernel，主要参考自：

- [Writing an OS in Rust](https://os.phil-opp.com/zh-CN/)
- [limine-rust-template](https://github.com/jasondyoungberg/limine-rust-template)

## 环境

Linux 或者 Windows with MSYS2

- limine以及终端字体

  已经包含在项目limine文件夹中

- rust

  [官网下载](https://www.rust-lang.org/zh-CN/tools/install)

  Windows: x86_64-pc-windows-gnu

  Linux: x86_64-unknown-linux-gnu

- qemu

  linux:

  ```bash
  # debian系
  sudo apt install qemu-utils qemu-system
  ```

  windows: [qemu windows下载网址](https://www.qemu.org/download/#windows)

- xorriso

  linux:

  ```bash
  # debian系
  sudo apt install xorriso
  ```

  MSYS2:

  ```bash
  pacman -S xorriso
  ```

- 工具链

  linux:

  ```bash
  # debian系
  sudo apt install build-essential
  ```

  MSYS2:

  ```bash
  pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain
  ```

## 构建

Windows：需在MSYS2 UCRT64环境中构建

```bash
# 构建iso
make all
# 清理输出文件
make clean
# 清理输出文件和rust构建文件
make cleanall
```

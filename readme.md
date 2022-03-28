create a minimum no_std program for the ARM Cortex-M architecture. it content from [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/index.html)


# 1.env prepare：

for arm embed develop envirment, suggest to use the latest stable toolchain, I use stable rust 2021 edition。

```shell
$ rustup +stable target add thumbv7em-none-eabihf

$ cargo +stable install cargo-binutils
$ rustup +stable component add llvm-tools-preview
#optional
$ sudo apt-get install qemu-system-arm

```

it is different from the AVR,  according to [issure](https://github.com/avr-rust/blink/issues/38), rust avr only support "nightly-2021-01-07" toolchain。

# 2.Supplementary notes

about the app/rt source, full information can access from [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/index.html). here only records some myself notes.


```shell
#use cargo to compile
d:\rust-embed-scratch>cd app

d:\rust-embed-scratch\app>cargo build
   Compiling rt v0.1.0 (D:\rust-embed-scratch\rt)
   Compiling app v0.1.0 (D:\rust-embed-scratch\app)
    Finished dev [unoptimized + debuginfo] target(s) in 3.01s

# do some utils
d:\rust-embed-scratch\app> cargo objdump --bin app -- -d --no-show-raw-insn
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s

app:    file format elf32-littlearm

Disassembly of section .text:

00000008 <app::main::hc0d1d79c4cc270f6>:
       8:       sub     sp, #16
       a:       movw    r0, #100
       e:       movt    r0, #0
      12:       ldr     r1, [r0]
      14:       ldr     r0, [r0, #4]
      16:       str     r1, [sp]
      18:       str     r0, [sp, #4]
      1a:       movw    r0, #0
      1e:       movt    r0, #8192
      22:       str     r0, [sp, #8]
      24:       movw    r0, #2
      28:       movt    r0, #8192
      2c:       str     r0, [sp, #12]
      2e:       b       0x30 <app::main::hc0d1d79c4cc270f6+0x28> @ imm = #-2
      30:       b       0x30 <app::main::hc0d1d79c4cc270f6+0x28> @ imm = #-4

00000032 <main>:
      32:       push    {r7, lr}
      34:       mov     r7, sp
      36:       sub     sp, #8
      38:       movw    r0, #9
      3c:       movt    r0, #0
      40:       str     r0, [sp, #4]
      42:       bl      0x8 <app::main::hc0d1d79c4cc270f6> @ imm = #-62
      46:       trap

00000048 <Reset>:
      48:       push    {r7, lr}
      4a:       mov     r7, sp
      4c:       bl      0x32 <main>             @ imm = #-30
      50:       trap

d:rust-embed-scratch\app>
```


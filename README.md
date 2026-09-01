# LLLU — Low-Level Language Utilities for Rust

[![Crates.io](https://img.shields.io/crates/v/lllu.svg)](https://crates.io/crates/lllu)
[![Documentation](https://docs.rs/lllu/badge.svg)](https://docs.rs/lllu)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/vi-is-ramen/lllu-rs#license)

**lllu** (pronounced "loo") is a `#![no_std]` Rust library that provides safe
(and  (mostly) unsafe) wrappers around low‑level CPU instructions, registers,
and system tables. It is designed for operating system kernels, hypervisors,
firmware, and other system‑level code that needs direct hardware access. Not
only IA32/AMD64 support is planned, but only IA32/AMD64 provided for now.

---

## Features

- **CPU Instructions** (`ins` module) – wrappers for:
  - `CPUID`, `RDTSC`, `RDTSCP`, `RDPID`
  - `RDMSR` / `WRMSR`
  - `HLT`, `CLI` / `STI`, `CLC` / `STC`, `CLD` / `STD`
  - `MFENCE` / `LFENCE` / `SFENCE`
  - IN/OUT port I/O (generic over data width)
  - `LGDT` / `SGDT`, `LIDT` / `SIDT`, `LLDT` / `SLDT`
  - Software interrupt generation via `int!` macro
- **Register Abstractions** (`reg` module) – types for:
  - Control registers (`CR0`, `CR2`, `CR3`, `CR4`, `CR8` on x86-64)
  - Debug registers (`DR0`–`DR7`)
  - Model‑Specific Registers (MSRs) via the `MachineSpecificRegister` trait
  - Flags register (`EFLAGS`/`RFLAGS`) with bit‑field helpers
  - Extended Control Register (`XCR0`) for XSAVE
  - Segment selectors (`CS`, `DS`, `SS`, `ES`, `FS`, `GS`) with RPL, LDT/GDT,
    index helpers
- **System Table Pointers** (`tab` module) – `TablePtr<T>` for GDT, IDT, etc.
  with safe access methods
- **Portable Macros** – `ins_arch!` and `ins_mod!` to conditionally include
  architecture‑specific code

All operations are **zero‑cost** – inline assembly is used directly, with
minimal overhead.

---

## Safety

**lllu** is inherently unsafe – most functions and methods require `unsafe`
because they:

- Execute privileged instructions that may affect system stability,
- Access hardware registers directly,
- Can cause undefined behaviour if used incorrectly.

It is the caller's responsibility to ensure proper CPU mode, privilege level,
and correctness of parameters. The library provides no runtime checks; it trusts
the programmer.

---

## Examples

### Read CPUID information

```rust
use lllu::ins::cpuid;

let result = cpuid(0x1);      // leaf 1, sub‑leaf 0
println!("CPUID: EAX={:#x}, EBX={:#x}, ECX={:#x}, EDX={:#x}",
         result.eax, result.ebx, result.ecx, result.edx);
```

### Read and write an MSR

```rust
use lllu::reg::MSR;
use lllu::reg::Register;

let msr = MSR::IA32_APIC_BASE;
unsafe {
    let value = msr.read();                 // read current value
    msr.write(value | 0x800);               // set the APIC enable bit
}
```

### Port I/O

```rust
use lllu::ins::port;

unsafe {
    // Write a byte to port 0x80 (POST code)
    port::send(0x80u16, 0x55u8);

    // Read a 32‑bit value from port 0xCF8 (PCI config address)
    let config_addr: u32 = port::recv(0xCF8u16);
}
```

### Read segment selector

```rust
use lllu::reg::{Cs, Register};

let cs = Cs;
let selector: u16 = unsafe { cs.read() };
let cs_value = lllu::reg::Segment::from(selector);
println!("CS: RPL={}, LDT={}, index={}",
         cs_value.rpl(), cs_value.ldt(), cs_value.index());
```

### Load a GDT using `TablePtr`

```rust
use lllu::tab::TablePtr;
use lllu::ins::lgdt;

// Assume `my_gdt` is a properly constructed GDT
struct GdtEntry { /* ... */ }
struct Gdt { entries: [GdtEntry; 256] } // example

impl lllu::tab::Table for Gdt {
    type Entry = GdtEntry;
    fn len(&self) -> usize { self.entries.len() }
}

let gdt = Gdt { entries: [/* ... */] };
let ptr = TablePtr::from_table(&gdt);

unsafe {
    lgdt(&ptr);
}
```

---

## Modules Overview

| Module | Description |
|--------|-------------|
| `ins`  | Inline assembly wrappers for CPU instructions |
| `reg`  | Register types and traits for safe (un)checked access |
| `tab`  | System table descriptors (`TablePtr`) |

Each module is architecture‑conditional – only x86/x86‑64 are currently supported.

---

## License

This project is dual‑licensed under either:

- MIT License
- Apache License, Version 2.0

at your option.

---

## Contributing

Contributions are welcome! Please open an issue or pull request on
[GitHub](https://github.com/vi-is-ramen/lllu-rs).

---

Made with ❤️ for the Rust community

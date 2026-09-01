use core::fmt;

/// IA32 Exception description (see also Intel Vol. 3a Chapter 6).
#[derive(Debug)]
pub struct InterruptDescription
{
    pub vector:      u8,
    pub mnemonic:    &'static str,
    pub description: &'static str,
    pub irqtype:     &'static str,
    pub source:      &'static str,
}

impl fmt::Display for InterruptDescription
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(
            f,
            "{} ({}, vec={}) {}",
            self.mnemonic, self.irqtype, self.vector, self.description
        )
    }
}

pub mod vector
{
    pub const DIVIDE_ERROR: u8 = 0;
    pub const DEBUG: u8 = 1;
    pub const NONMASKABLE_INTERRUPT: u8 = 2;
    pub const BREAKPOINT: u8 = 3;
    pub const OVERFLOW: u8 = 4;
    pub const BOUND_RANGE_EXCEEDED: u8 = 5;
    pub const INVALID_OPCODE: u8 = 6;
    pub const DEVICE_NOT_AVAILABLE: u8 = 7;
    pub const DOUBLE_FAULT: u8 = 8;
    pub const COPROCESSOR_SEGMENT_OVERRUN: u8 = 9;
    pub const INVALID_TSS: u8 = 10;
    pub const SEGMENT_NOT_PRESENT: u8 = 11;
    pub const STACK_SEGMENT_FAULT: u8 = 12;
    pub const GENERAL_PROTECTION_FAULT: u8 = 13;
    pub const PAGE_FAULT: u8 = 14;
    pub const X87_FPU: u8 = 16;
    pub const ALIGNMENT_CHECK: u8 = 17;
    pub const MACHINE_CHECK: u8 = 18;
    pub const SIMD_FLOATING_POINT: u8 = 19;
    pub const VIRTUALIZATION: u8 = 20;
}

use vector::*;

pub static EXCEPTIONS: [InterruptDescription; 32] = [
    InterruptDescription {
        vector:      DIVIDE_ERROR,
        mnemonic:    "#DE",
        description: "Divide Error",
        irqtype:     "Fault",
        source:      "DIV and IDIV instructions.",
    },
    InterruptDescription {
        vector:      DEBUG,
        mnemonic:    "#DB",
        description: "Debug",
        irqtype:     "Fault / Trap",
        source:      "Debug condition",
    },
    InterruptDescription {
        vector:      NONMASKABLE_INTERRUPT,
        mnemonic:    "NMI",
        description: "Nonmaskable Interrupt",
        irqtype:     "Interrupt",
        source:      "Nonmaskable external interrupt.",
    },
    InterruptDescription {
        vector:      BREAKPOINT,
        mnemonic:    "#BP",
        description: "Breakpoint",
        irqtype:     "Trap",
        source:      "INT 3 instruction.",
    },
    InterruptDescription {
        vector:      OVERFLOW,
        mnemonic:    "#OF",
        description: "Overflow",
        irqtype:     "Trap",
        source:      "INTO instruction.",
    },
    InterruptDescription {
        vector:      BOUND_RANGE_EXCEEDED,
        mnemonic:    "#BR",
        description: "BOUND Range Exceeded",
        irqtype:     "Fault",
        source:      "BOUND instruction.",
    },
    InterruptDescription {
        vector:      INVALID_OPCODE,
        mnemonic:    "#UD",
        description: "Invalid Opcode (Undefined Opcode)",
        irqtype:     "Fault",
        source:      "UD2 instruction or reserved opcode.",
    },
    InterruptDescription {
        vector:      DEVICE_NOT_AVAILABLE,
        mnemonic:    "#NM",
        description: "Device Not Available (No Math Coprocessor)",
        irqtype:     "Fault",
        source:      "Floating-point or WAIT/FWAIT instruction.",
    },
    InterruptDescription {
        vector:      DOUBLE_FAULT,
        mnemonic:    "#DF",
        description: "Double Fault",
        irqtype:     "Abort",
        source:      "Any instruction that can generate an exception, an NMI, \
                      or an INTR.",
    },
    InterruptDescription {
        vector:      COPROCESSOR_SEGMENT_OVERRUN,
        mnemonic:    "",
        description: "Coprocessor Segment Overrun",
        irqtype:     "Fault",
        source:      "Floating-point instruction.",
    },
    InterruptDescription {
        vector:      INVALID_TSS,
        mnemonic:    "#TS",
        description: "Invalid TSS",
        irqtype:     "Fault",
        source:      "Task switch or TSS access.",
    },
    InterruptDescription {
        vector:      SEGMENT_NOT_PRESENT,
        mnemonic:    "#NP",
        description: "Segment Not Present",
        irqtype:     "Fault",
        source:      "Loading segment registers or accessing system segments.",
    },
    InterruptDescription {
        vector:      STACK_SEGMENT_FAULT,
        mnemonic:    "#SS",
        description: "Stack-Segment Fault",
        irqtype:     "Fault",
        source:      "Stack operations and SS register loads.",
    },
    InterruptDescription {
        vector:      GENERAL_PROTECTION_FAULT,
        mnemonic:    "#GP",
        description: "General Protection",
        irqtype:     "Fault",
        source:      "Any memory reference and other protection checks.",
    },
    InterruptDescription {
        vector:      PAGE_FAULT,
        mnemonic:    "#PF",
        description: "Page Fault",
        irqtype:     "Fault",
        source:      "Any memory reference.",
    },
    InterruptDescription {
        vector:      15,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      "None.",
    },
    InterruptDescription {
        vector:      X87_FPU,
        mnemonic:    "#MF",
        description: "x87 FPU Floating-Point",
        irqtype:     "Fault",
        source:      "x87 FPU instructions.",
    },
    InterruptDescription {
        vector:      ALIGNMENT_CHECK,
        mnemonic:    "#AC",
        description: "Alignment Check",
        irqtype:     "Fault",
        source:      "Unaligned memory access.",
    },
    InterruptDescription {
        vector:      MACHINE_CHECK,
        mnemonic:    "#MC",
        description: "Machine Check",
        irqtype:     "Abort",
        source:      "Internal machine error.",
    },
    InterruptDescription {
        vector:      SIMD_FLOATING_POINT,
        mnemonic:    "#XM",
        description: "SIMD Floating-Point",
        irqtype:     "Fault",
        source:      "SSE SIMD instructions.",
    },
    InterruptDescription {
        vector:      VIRTUALIZATION,
        mnemonic:    "#VE",
        description: "Virtualization",
        irqtype:     "Fault",
        source:      "EPT violation.",
    },
    InterruptDescription {
        vector:      21,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      ".",
    },
    InterruptDescription {
        vector:      22,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      ".",
    },
    InterruptDescription {
        vector:      23,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      ".",
    },
    InterruptDescription {
        vector:      24,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      ".",
    },
    InterruptDescription {
        vector:      25,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      ".",
    },
    InterruptDescription {
        vector:      26,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      ".",
    },
    InterruptDescription {
        vector:      27,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      "",
    },
    InterruptDescription {
        vector:      28,
        mnemonic:    "",
        description: "",
        irqtype:     "",
        source:      "",
    },
    InterruptDescription {
        vector:      29,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      ".",
    },
    InterruptDescription {
        vector:      30,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      "",
    },
    InterruptDescription {
        vector:      31,
        mnemonic:    "",
        description: "RESERVED",
        irqtype:     "",
        source:      "",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PageFaultError(u32);

impl From<u32> for PageFaultError
{
    #[inline(always)]
    fn from(value: u32) -> Self
    {
        PageFaultError(value)
    }
}

impl PageFaultError
{
    #[inline(always)]
    pub const fn present(self) -> bool
    {
        self.0 & (1 << 0) != 0
    }

    #[inline(always)]
    pub const fn read(self) -> bool
    {
        self.0 & (1 << 1) == 0
    }

    #[inline(always)]
    pub const fn write(self) -> bool
    {
        self.0 & (1 << 1) != 0
    }

    #[inline(always)]
    pub const fn user(self) -> bool
    {
        self.0 & (1 << 2) != 0
    }

    #[inline(always)]
    pub const fn supervisor(self) -> bool
    {
        self.0 & (1 << 2) == 0
    }

    #[inline(always)]
    pub const fn reserved(self) -> bool
    {
        self.0 & (1 << 3) != 0
    }

    #[inline(always)]
    pub const fn instruction(self) -> bool
    {
        self.0 & (1 << 4) != 0
    }

    #[inline(always)]
    pub const fn pkey(self) -> bool
    {
        self.0 & (1 << 5) != 0
    }
}

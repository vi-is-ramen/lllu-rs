use crate::reg::Cast as _;

pub trait MachineSpecificRegister
where
    Self: PartialEq + Eq,
{
    fn id(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct MSR(pub u32);

impl From<u32> for MSR {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl MachineSpecificRegister for MSR {
    fn id(&self) -> u32 {
        self.0
    }
}

impl<T> crate::reg::Register<8> for T
where
    T: MachineSpecificRegister,
{
    type Inner = u64;

    unsafe fn read(&self) -> Self::Inner {
        unsafe { crate::ins::rdmsr(self.id()) }
    }

    unsafe fn write(&mut self, value: Self::Inner) {
        unsafe {
            crate::ins::wrmsr(self.id(), value);
        }
    }

    unsafe fn read_raw(&self) -> [u8; 8] {
        let value = unsafe { self.read() };
        value.cast()
    }

    unsafe fn write_raw(&mut self, value: [u8; 8]) {
        unsafe {
            self.write(value.cast());
        }
    }

    unsafe fn try_read(&self) -> Option<Self::Inner> {
        if self.id() == u32::MAX {
            return None;
        }
        Some(unsafe { self.read() })
    }

    unsafe fn try_write(&mut self, value: Self::Inner) -> Option<()> {
        if self.id() == u32::MAX {
            return None;
        }
        Some(unsafe { self.write(value) })
    }

    unsafe fn try_read_raw(&self) -> Option<[u8; 8]> {
        if self.id() == u32::MAX {
            return None;
        }
        Some(unsafe { self.read_raw() })
    }

    unsafe fn try_write_raw(&mut self, value: [u8; 8]) -> Option<()> {
        if self.id() == u32::MAX {
            return None;
        }
        Some(unsafe { self.write_raw(value) })
    }
}

impl MSR {
    /// See Section 35.16, MSRs in Pentium Processors,  and see  Table 35-2.
    pub const P5_MC_ADDR: Self = Self(0x0);

    /// See Section 35.16, MSRs in Pentium Processors.
    pub const IA32_P5_MC_ADDR: Self = Self(0x0);

    /// See Section 35.16, MSRs in Pentium Processors,  and see  Table 35-2.
    pub const P5_MC_TYPE: Self = Self(0x1);

    /// See Section 35.16, MSRs in Pentium Processors.
    pub const IA32_P5_MC_TYPE: Self = Self(0x1);

    /// See Section 8.10.5, Monitor/Mwait Address Range Determination,   and see Table 35-2.
    pub const IA32_MONITOR_FILTER_SIZE: Self = Self(0x6);

    /// See Section 8.10.5, Monitor/Mwait Address  Range Determination.
    pub const IA32_MONITOR_FILTER_LINE_SIZE: Self = Self(0x6);

    /// See Section 17.13, Time-Stamp Counter,  and see Table 35-2.
    pub const IA32_TIME_STAMP_COUNTER: Self = Self(0x10);

    /// See Section 17.13, Time-Stamp Counter.
    pub const TSC: Self = Self(0x10);

    /// Model Specific Platform ID (R)
    pub const MSR_PLATFORM_ID: Self = Self(0x17);

    /// Platform ID (R)  See Table 35-2. The operating system can use this MSR to  determine slot  information for the processor and the proper microcode update to load.
    pub const IA32_PLATFORM_ID: Self = Self(0x17);

    /// Section 10.4.4, Local APIC Status and Location.
    pub const APIC_BASE: Self = Self(0x1b);

    /// APIC Location and Status (R/W) See Table 35-2. See Section 10.4.4, Local APIC  Status and Location.
    pub const IA32_APIC_BASE: Self = Self(0x1b);

    /// Processor Hard Power-On Configuration  (R/W) Enables and disables processor features);  (R) indicates current processor configuration.
    pub const EBL_CR_POWERON: Self = Self(0x2a);

    /// Processor Hard Power-On Configuration (R/W) Enables and  disables processor features);  (R) indicates current processor configuration.
    pub const MSR_EBL_CR_POWERON: Self = Self(0x2a);

    /// Processor Hard Power-On Configuration (R/W) Enables and disables processor features);  (R) indicates current processor configuration.
    pub const MSR_EBC_HARD_POWERON: Self = Self(0x2a);

    /// Processor Soft Power-On Configuration (R/W)  Enables and disables processor features.
    pub const MSR_EBC_SOFT_POWERON: Self = Self(0x2b);

    /// Processor Frequency Configuration The bit field layout of this MSR varies according to  the MODEL value in the CPUID version  information. The following bit field layout applies to Pentium 4 and Xeon Processors with MODEL  encoding equal or greater than 2.  (R) The field Indicates the current processor  frequency configuration.
    pub const MSR_EBC_FREQUENCY_ID: Self = Self(0x2c);

    /// Test Control Register
    pub const TEST_CTL: Self = Self(0x33);

    /// SMI Counter (R/O)
    pub const MSR_SMI_COUNT: Self = Self(0x34);

    /// Control Features in IA-32 Processor (R/W) See Table 35-2 (If CPUID.01H:ECX.\[bit 5\])
    pub const IA32_FEATURE_CONTROL: Self = Self(0x3a);

    /// Per-Logical-Processor TSC ADJUST (R/W) See Table 35-2.
    pub const IA32_TSC_ADJUST: Self = Self(0x3b);

    /// Last Branch Record 0 From IP (R/W) One of eight pairs of last branch record registers on the last branch  record stack. This part of the stack contains pointers to the source  instruction for one of the last eight branches, exceptions, or  interrupts taken by the processor. See also: Last Branch Record Stack TOS at 1C9H Section 17.11, Last Branch, Interrupt, and Exception Recording  (Pentium M Processors).
    pub const MSR_LASTBRANCH_0_FROM_IP: Self = Self(0x40);

    /// Last Branch Record 1 (R/W) See description of MSR_LASTBRANCH_0.
    pub const MSR_LASTBRANCH_1: Self = Self(0x41);

    /// Last Branch Record 1 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_1_FROM_IP: Self = Self(0x41);

    /// Last Branch Record 2 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_2_FROM_IP: Self = Self(0x42);

    /// Last Branch Record 3 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_3_FROM_IP: Self = Self(0x43);

    /// Last Branch Record 4 (R/W) See description of MSR_LASTBRANCH_0.
    pub const MSR_LASTBRANCH_4: Self = Self(0x44);

    /// Last Branch Record 4 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_4_FROM_IP: Self = Self(0x44);

    /// Last Branch Record 5 (R/W) See description of MSR_LASTBRANCH_0.
    pub const MSR_LASTBRANCH_5: Self = Self(0x45);

    /// Last Branch Record 5 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_5_FROM_IP: Self = Self(0x45);

    /// Last Branch Record 6 (R/W) See description of MSR_LASTBRANCH_0.
    pub const MSR_LASTBRANCH_6: Self = Self(0x46);

    /// Last Branch Record 6 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_6_FROM_IP: Self = Self(0x46);

    /// Last Branch Record 7 (R/W) See description of MSR_LASTBRANCH_0.
    pub const MSR_LASTBRANCH_7: Self = Self(0x47);

    /// Last Branch Record 7 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_7_FROM_IP: Self = Self(0x47);

    /// Last Branch Record 0 (R/W)  One of 16 pairs of last branch record registers on  the last branch record stack (6C0H-6CFH). This  part of the stack contains pointers to the  destination instruction for one of the last 16  branches, exceptions, or interrupts that the  processor took. See Section 17.9, Last Branch, Interrupt, and  Exception Recording (Processors based on Intel  NetBurst® Microarchitecture).
    pub const MSR_LASTBRANCH_0_TO_IP: Self = Self(0x6c0);

    /// Last Branch Record 1 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_1_TO_IP: Self = Self(0x61);

    /// Last Branch Record 2 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_2_TO_IP: Self = Self(0x62);

    /// Last Branch Record 3 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_3_TO_IP: Self = Self(0x63);

    /// Last Branch Record 4 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_4_TO_IP: Self = Self(0x64);

    /// Last Branch Record 5 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_5_TO_IP: Self = Self(0x65);

    /// Last Branch Record 6 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_6_TO_IP: Self = Self(0x66);

    /// Last Branch Record 7 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_7_TO_IP: Self = Self(0x67);

    /// BIOS Update Trigger Register (W)  See Table 35-2.
    pub const IA32_BIOS_UPDT_TRIG: Self = Self(0x79);

    /// BIOS Update Trigger Register.
    pub const BIOS_UPDT_TRIG: Self = Self(0x79);

    /// BIOS Update Signature ID (R/W) See Table 35-2.
    pub const IA32_BIOS_SIGN_ID: Self = Self(0x8b);

    /// SMM Monitor Configuration (R/W) See Table 35-2.
    pub const IA32_SMM_MONITOR_CTL: Self = Self(0x9b);

    /// If IA32_VMX_MISC\[bit 15\])
    pub const IA32_SMBASE: Self = Self(0x9e);

    /// System Management Mode Physical Address Mask register  (WO in SMM) Model-specific implementation of SMRR-like interface, read visible  and write only in SMM..
    pub const MSR_SMRR_PHYSMASK: Self = Self(0xa1);

    /// Performance Counter Register  See Table 35-2.
    pub const IA32_PMC0: Self = Self(0xc1);

    /// Performance Counter Register  See Table 35-2.
    pub const IA32_PMC1: Self = Self(0xc2);

    /// Performance Counter Register  See Table 35-2.
    pub const IA32_PMC2: Self = Self(0xc3);

    /// Performance Counter Register  See Table 35-2.
    pub const IA32_PMC3: Self = Self(0xc4);

    /// Performance Counter Register  See Table 35-2.
    pub const IA32_PMC4: Self = Self(0xc5);

    /// Performance Counter Register  See Table 35-2.
    pub const IA32_PMC5: Self = Self(0xc6);

    /// Performance Counter Register  See Table 35-2.
    pub const IA32_PMC6: Self = Self(0xc7);

    /// Performance Counter Register  See Table 35-2.
    pub const IA32_PMC7: Self = Self(0xc8);

    /// Scaleable Bus Speed(RO) This field indicates the intended scaleable bus clock speed for  processors based on Intel Atom microarchitecture:
    pub const MSR_FSB_FREQ: Self = Self(0xcd);

    /// see http://biosbits.org.
    pub const MSR_PLATFORM_INFO: Self = Self(0xce);

    /// C-State Configuration Control (R/W)  Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States. See http://biosbits.org.
    pub const MSR_PKG_CST_CONFIG_CONTROL: Self = Self(0xe2);

    /// Power Management IO Redirection in C-state (R/W)  See http://biosbits.org.
    pub const MSR_PMG_IO_CAPTURE_BASE: Self = Self(0xe4);

    /// Maximum Performance Frequency Clock Count (RW)  See Table 35-2.
    pub const IA32_MPERF: Self = Self(0xe7);

    /// Actual Performance Frequency Clock Count (RW)  See Table 35-2.
    pub const IA32_APERF: Self = Self(0xe8);

    /// MTRR Information See Section 11.11.1, MTRR Feature  Identification. .
    pub const IA32_MTRRCAP: Self = Self(0xfe);

    pub const MSR_BBL_CR_CTL: Self = Self(0x119);

    pub const MSR_BBL_CR_CTL3: Self = Self(0x11e);

    /// TSX Ctrl Register for TSX Async Abot (TAA) Migration. See Volume 3A, Section 2.1, Table 2-2.
    pub const MSR_IA32_TSX_CTRL: Self = Self(0x122);

    /// CS register target for CPL 0 code (R/W) See Table 35-2. See Section 5.8.7, Performing Fast Calls to  System Procedures with the SYSENTER and  SYSEXIT Instructions.
    pub const IA32_SYSENTER_CS: Self = Self(0x174);

    /// CS register target for CPL 0 code
    pub const SYSENTER_CS_MSR: Self = Self(0x174);

    /// Stack pointer for CPL 0 stack (R/W) See Table 35-2. See Section 5.8.7, Performing Fast Calls to  System Procedures with the SYSENTER and  SYSEXIT Instructions.
    pub const IA32_SYSENTER_ESP: Self = Self(0x175);

    /// Stack pointer for CPL 0 stack
    pub const SYSENTER_ESP_MSR: Self = Self(0x175);

    /// CPL 0 code entry point (R/W) See Table 35-2. See Section 5.8.7, Performing  Fast Calls to System Procedures with the SYSENTER and SYSEXIT Instructions.
    pub const IA32_SYSENTER_EIP: Self = Self(0x176);

    /// CPL 0 code entry point
    pub const SYSENTER_EIP_MSR: Self = Self(0x176);

    pub const MCG_CAP: Self = Self(0x179);

    /// Machine Check Capabilities (R) See Table 35-2. See Section 15.3.1.1,  IA32_MCG_CAP MSR.
    pub const IA32_MCG_CAP: Self = Self(0x179);

    /// Machine Check Status. (R) See Table 35-2. See Section 15.3.1.2,  IA32_MCG_STATUS MSR.
    pub const IA32_MCG_STATUS: Self = Self(0x17a);

    pub const MCG_STATUS: Self = Self(0x17a);

    pub const MCG_CTL: Self = Self(0x17b);

    /// Machine Check Feature Enable (R/W) See Table 35-2. See Section 15.3.1.3, IA32_MCG_CTL MSR.
    pub const IA32_MCG_CTL: Self = Self(0x17b);

    /// Enhanced SMM Capabilities (SMM-RO) Reports SMM capability Enhancement. Accessible only while in  SMM.
    pub const MSR_SMM_MCA_CAP: Self = Self(0x17d);

    /// MC Bank Error Configuration (R/W)
    pub const MSR_ERROR_CONTROL: Self = Self(0x17f);

    /// Machine Check EAX/RAX Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RAX: Self = Self(0x180);

    /// Machine Check EBX/RBX Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RBX: Self = Self(0x181);

    /// Machine Check ECX/RCX Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RCX: Self = Self(0x182);

    /// Machine Check EDX/RDX Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RDX: Self = Self(0x183);

    /// Machine Check ESI/RSI Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RSI: Self = Self(0x184);

    /// Machine Check EDI/RDI Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RDI: Self = Self(0x185);

    /// Machine Check EBP/RBP Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RBP: Self = Self(0x186);

    /// Performance Event Select for Counter 0 (R/W) Supports all fields described inTable 35-2 and the fields below.
    pub const IA32_PERFEVTSEL0: Self = Self(0x186);

    /// Performance Event Select for Counter 1 (R/W) Supports all fields described inTable 35-2 and the fields below.
    pub const IA32_PERFEVTSEL1: Self = Self(0x187);

    /// Performance Event Select for Counter 2 (R/W) Supports all fields described inTable 35-2 and the fields below.
    pub const IA32_PERFEVTSEL2: Self = Self(0x188);

    /// Machine Check EFLAGS/RFLAG Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RFLAGS: Self = Self(0x188);

    /// Performance Event Select for Counter 3 (R/W) Supports all fields described inTable 35-2 and the fields below.
    pub const IA32_PERFEVTSEL3: Self = Self(0x189);

    /// Machine Check EIP/RIP Save State See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_RIP: Self = Self(0x189);

    /// Machine Check Miscellaneous See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_MISC: Self = Self(0x18a);

    /// See Table 35-2); If CPUID.0AH:EAX\[15:8\] = 8
    pub const IA32_PERFEVTSEL4: Self = Self(0x18a);

    /// See Table 35-2); If CPUID.0AH:EAX\[15:8\] = 8
    pub const IA32_PERFEVTSEL5: Self = Self(0x18b);

    /// See Table 35-2); If CPUID.0AH:EAX\[15:8\] = 8
    pub const IA32_PERFEVTSEL6: Self = Self(0x18c);

    /// See Table 35-2); If CPUID.0AH:EAX\[15:8\] = 8
    pub const IA32_PERFEVTSEL7: Self = Self(0x18d);

    /// Machine Check R8 See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_R8: Self = Self(0x190);

    /// Machine Check R9D/R9 See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_R9: Self = Self(0x191);

    /// Machine Check R10 See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_R10: Self = Self(0x192);

    /// Machine Check R11 See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_R11: Self = Self(0x193);

    /// Machine Check R12 See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_R12: Self = Self(0x194);

    /// Machine Check R13 See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_R13: Self = Self(0x195);

    /// Machine Check R14 See Section 15.3.2.6, IA32_MCG Extended  Machine Check State MSRs.
    pub const MSR_MCG_R14: Self = Self(0x196);

    pub const MSR_PERF_STATUS: Self = Self(0x198);

    /// See Table 35-2. See Section 14.1, Enhanced Intel  Speedstep® Technology.
    pub const IA32_PERF_STATUS: Self = Self(0x198);

    /// See Table 35-2. See Section 14.1, Enhanced Intel  Speedstep® Technology.
    pub const IA32_PERF_CTL: Self = Self(0x199);

    /// Clock Modulation (R/W)  See Table 35-2. IA32_CLOCK_MODULATION MSR was originally named  IA32_THERM_CONTROL MSR.
    pub const IA32_CLOCK_MODULATION: Self = Self(0x19a);

    /// Thermal Interrupt Control (R/W) See Section 14.5.2, Thermal Monitor,  and see Table 35-2.
    pub const IA32_THERM_INTERRUPT: Self = Self(0x19b);

    /// Thermal Monitor Status (R/W) See Section 14.5.2, Thermal Monitor,  and see  Table 35-2.
    pub const IA32_THERM_STATUS: Self = Self(0x19c);

    /// Thermal Monitor 2 Control.
    pub const MSR_THERM2_CTL: Self = Self(0x19d);

    pub const IA32_MISC_ENABLE: Self = Self(0x1a0);

    /// Platform Feature Requirements (R)
    pub const MSR_PLATFORM_BRV: Self = Self(0x1a1);

    pub const MSR_TEMPERATURE_TARGET: Self = Self(0x1a2);

    /// Offcore Response Event Select Register (R/W)
    pub const MSR_OFFCORE_RSP_0: Self = Self(0x1a6);

    /// Offcore Response Event Select Register (R/W)
    pub const MSR_OFFCORE_RSP_1: Self = Self(0x1a7);

    /// See http://biosbits.org.
    pub const MSR_MISC_PWR_MGMT: Self = Self(0x1aa);

    /// See http://biosbits.org.
    pub const MSR_TURBO_POWER_CURRENT_LIMIT: Self = Self(0x1ac);

    /// Maximum Ratio Limit of Turbo Mode RO if MSR_PLATFORM_INFO.\[28\] = 0, RW if MSR_PLATFORM_INFO.\[28\] = 1
    pub const MSR_TURBO_RATIO_LIMIT: Self = Self(0x1ad);

    /// if CPUID.6H:ECX\[3\] = 1
    pub const IA32_ENERGY_PERF_BIAS: Self = Self(0x1b0);

    /// If CPUID.06H: EAX\[6\] = 1
    pub const IA32_PACKAGE_THERM_STATUS: Self = Self(0x1b1);

    /// If CPUID.06H: EAX\[6\] = 1
    pub const IA32_PACKAGE_THERM_INTERRUPT: Self = Self(0x1b2);

    /// Last Branch Record Filtering Select Register (R/W)  See Section 17.6.2, Filtering of Last Branch Records.
    pub const MSR_LBR_SELECT: Self = Self(0x1c8);

    /// Last Branch Record Stack TOS (R/W)  Contains an index (0-3 or 0-15) that points to the  top of the last branch record stack (that is, that points the index of the MSR containing the most  recent branch record). See Section 17.9.2, LBR Stack for Processors Based on Intel NetBurst® Microarchitecture ); and  addresses 1DBH-1DEH and 680H-68FH.
    pub const MSR_LASTBRANCH_TOS: Self = Self(0x1da);

    pub const DEBUGCTLMSR: Self = Self(0x1d9);

    /// Debug Control (R/W)  Controls how several debug features are used. Bit  definitions are discussed in the referenced section. See Section 17.9.1, MSR_DEBUGCTLA MSR.
    pub const MSR_DEBUGCTLA: Self = Self(0x1d9);

    /// Debug Control (R/W)  Controls how several debug features are used. Bit definitions are discussed in the referenced section. See Section 17.11, Last Branch, Interrupt, and Exception Recording  (Pentium M Processors).
    pub const MSR_DEBUGCTLB: Self = Self(0x1d9);

    /// Debug Control (R/W)  Controls how several debug features are used. Bit definitions are  discussed in the referenced section.
    pub const IA32_DEBUGCTL: Self = Self(0x1d9);

    pub const LASTBRANCHFROMIP: Self = Self(0x1db);

    /// Last Branch Record 0 (R/W)  One of four last branch record registers on the last  branch record stack. It contains pointers to the  source and destination instruction for one of the  last four branches, exceptions, or interrupts that  the processor took. MSR_LASTBRANCH_0 through  MSR_LASTBRANCH_3 at 1DBH-1DEH are  available only on family 0FH, models 0H-02H.  They have been replaced by the MSRs at 680H- 68FH and 6C0H-6CFH.
    pub const MSR_LASTBRANCH_0: Self = Self(0x1db);

    pub const LASTBRANCHTOIP: Self = Self(0x1dc);

    pub const LASTINTFROMIP: Self = Self(0x1dd);

    /// Last Branch Record 2 See description of the MSR_LASTBRANCH_0 MSR at 1DBH.
    pub const MSR_LASTBRANCH_2: Self = Self(0x1dd);

    /// Last Exception Record From Linear IP (R)  Contains a pointer to the last branch instruction that the processor  executed prior to the last exception that was generated or the last  interrupt that was handled. See Section 17.11, Last Branch, Interrupt, and Exception Recording  (Pentium M Processors)  and Section 17.12.2, Last Branch and Last  Exception MSRs.
    pub const MSR_LER_FROM_LIP: Self = Self(0x1de);

    pub const LASTINTTOIP: Self = Self(0x1de);

    /// Last Branch Record 3 See description of the MSR_LASTBRANCH_0 MSR  at 1DBH.
    pub const MSR_LASTBRANCH_3: Self = Self(0x1de);

    /// Last Exception Record To Linear IP (R)  This area contains a pointer to the target of the last branch instruction  that the processor executed prior to the last exception that was  generated or the last interrupt that was handled. See Section 17.11, Last Branch, Interrupt, and Exception Recording  (Pentium M Processors)  and Section 17.12.2, Last Branch and Last  Exception MSRs.
    pub const MSR_LER_TO_LIP: Self = Self(0x1dd);

    pub const ROB_CR_BKUPTMPDR6: Self = Self(0x1e0);

    /// See Table 35-2.
    pub const IA32_SMRR_PHYSBASE: Self = Self(0x1f2);

    /// If IA32_MTRR_CAP\[SMRR\]  = 1
    pub const IA32_SMRR_PHYSMASK: Self = Self(0x1f3);

    /// 06_0FH
    pub const IA32_PLATFORM_DCA_CAP: Self = Self(0x1f8);

    pub const IA32_CPU_DCA_CAP: Self = Self(0x1f9);

    /// 06_2EH
    pub const IA32_DCA_0_CAP: Self = Self(0x1fa);

    /// Power Control Register. See http://biosbits.org.
    pub const MSR_POWER_CTL: Self = Self(0x1fc);

    /// Variable Range Base MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSBASE0: Self = Self(0x200);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSMASK0: Self = Self(0x201);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSBASE1: Self = Self(0x202);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSMASK1: Self = Self(0x203);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSBASE2: Self = Self(0x204);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs .
    pub const IA32_MTRR_PHYSMASK2: Self = Self(0x205);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSBASE3: Self = Self(0x206);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSMASK3: Self = Self(0x207);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSBASE4: Self = Self(0x208);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSMASK4: Self = Self(0x209);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSBASE5: Self = Self(0x20a);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSMASK5: Self = Self(0x20b);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSBASE6: Self = Self(0x20c);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSMASK6: Self = Self(0x20d);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSBASE7: Self = Self(0x20e);

    /// Variable Range Mask MTRR See Section 11.11.2.3, Variable Range MTRRs.
    pub const IA32_MTRR_PHYSMASK7: Self = Self(0x20f);

    /// if IA32_MTRR_CAP\[7:0\] >  8
    pub const IA32_MTRR_PHYSBASE8: Self = Self(0x210);

    /// if IA32_MTRR_CAP\[7:0\] >  8
    pub const IA32_MTRR_PHYSMASK8: Self = Self(0x211);

    /// if IA32_MTRR_CAP\[7:0\] >  9
    pub const IA32_MTRR_PHYSBASE9: Self = Self(0x212);

    /// if IA32_MTRR_CAP\[7:0\] >  9
    pub const IA32_MTRR_PHYSMASK9: Self = Self(0x213);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX64K_00000: Self = Self(0x250);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX16K_80000: Self = Self(0x258);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX16K_A0000: Self = Self(0x259);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX4K_C0000: Self = Self(0x268);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs .
    pub const IA32_MTRR_FIX4K_C8000: Self = Self(0x269);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs .
    pub const IA32_MTRR_FIX4K_D0000: Self = Self(0x26a);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX4K_D8000: Self = Self(0x26b);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX4K_E0000: Self = Self(0x26c);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX4K_E8000: Self = Self(0x26d);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX4K_F0000: Self = Self(0x26e);

    /// Fixed Range MTRR See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_MTRR_FIX4K_F8000: Self = Self(0x26f);

    /// Page Attribute Table See Section 11.11.2.2, Fixed Range MTRRs.
    pub const IA32_PAT: Self = Self(0x277);

    /// See Table 35-2.
    pub const IA32_MC0_CTL2: Self = Self(0x280);

    /// See Table 35-2.
    pub const IA32_MC1_CTL2: Self = Self(0x281);

    /// See Table 35-2.
    pub const IA32_MC2_CTL2: Self = Self(0x282);

    /// See Table 35-2.
    pub const IA32_MC3_CTL2: Self = Self(0x283);

    /// See Table 35-2.
    pub const IA32_MC4_CTL2: Self = Self(0x284);

    /// Always 0 (CMCI not supported).
    pub const MSR_MC4_CTL2: Self = Self(0x284);

    /// See Table 35-2.
    pub const IA32_MC5_CTL2: Self = Self(0x285);

    /// See Table 35-2.
    pub const IA32_MC6_CTL2: Self = Self(0x286);

    /// See Table 35-2.
    pub const IA32_MC7_CTL2: Self = Self(0x287);

    /// See Table 35-2.
    pub const IA32_MC8_CTL2: Self = Self(0x288);

    /// See Table 35-2.
    pub const IA32_MC9_CTL2: Self = Self(0x289);

    /// See Table 35-2.
    pub const IA32_MC10_CTL2: Self = Self(0x28a);

    /// See Table 35-2.
    pub const IA32_MC11_CTL2: Self = Self(0x28b);

    /// See Table 35-2.
    pub const IA32_MC12_CTL2: Self = Self(0x28c);

    /// See Table 35-2.
    pub const IA32_MC13_CTL2: Self = Self(0x28d);

    /// See Table 35-2.
    pub const IA32_MC14_CTL2: Self = Self(0x28e);

    /// See Table 35-2.
    pub const IA32_MC15_CTL2: Self = Self(0x28f);

    /// See Table 35-2.
    pub const IA32_MC16_CTL2: Self = Self(0x290);

    /// See Table 35-2.
    pub const IA32_MC17_CTL2: Self = Self(0x291);

    /// See Table 35-2.
    pub const IA32_MC18_CTL2: Self = Self(0x292);

    /// See Table 35-2.
    pub const IA32_MC19_CTL2: Self = Self(0x293);

    /// See Table 35-2.
    pub const IA32_MC20_CTL2: Self = Self(0x294);

    /// See Table 35-2.
    pub const IA32_MC21_CTL2: Self = Self(0x295);

    /// Default Memory Types (R/W)  Sets the memory type for the regions of physical memory that are not  mapped by the MTRRs.  See Section 11.11.2.1, IA32_MTRR_DEF_TYPE MSR.
    pub const IA32_MTRR_DEF_TYPE: Self = Self(0x2ff);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_BPU_COUNTER0: Self = Self(0x300);

    pub const MSR_GQ_SNOOP_MESF: Self = Self(0x301);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_BPU_COUNTER1: Self = Self(0x301);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_BPU_COUNTER2: Self = Self(0x302);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_BPU_COUNTER3: Self = Self(0x303);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_MS_COUNTER0: Self = Self(0x304);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_MS_COUNTER1: Self = Self(0x305);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_MS_COUNTER2: Self = Self(0x306);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_MS_COUNTER3: Self = Self(0x307);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_FLAME_COUNTER0: Self = Self(0x308);

    /// Fixed-Function Performance Counter Register 0 (R/W)
    pub const MSR_PERF_FIXED_CTR0: Self = Self(0x309);

    /// Fixed-Function Performance Counter Register 0 (R/W)  See Table 35-2.
    pub const IA32_FIXED_CTR0: Self = Self(0x309);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_FLAME_COUNTER1: Self = Self(0x309);

    /// Fixed-Function Performance Counter Register 1 (R/W)
    pub const MSR_PERF_FIXED_CTR1: Self = Self(0x30a);

    /// Fixed-Function Performance Counter Register 1 (R/W)  See Table 35-2.
    pub const IA32_FIXED_CTR1: Self = Self(0x30a);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_FLAME_COUNTER2: Self = Self(0x30a);

    /// Fixed-Function Performance Counter Register 2 (R/W)
    pub const MSR_PERF_FIXED_CTR2: Self = Self(0x30b);

    /// Fixed-Function Performance Counter Register 2 (R/W)  See Table 35-2.
    pub const IA32_FIXED_CTR2: Self = Self(0x30b);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_FLAME_COUNTER3: Self = Self(0x30b);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_IQ_COUNTER4: Self = Self(0x310);

    /// See Section 18.12.2, Performance Counters.
    pub const MSR_IQ_COUNTER5: Self = Self(0x311);

    /// See Table 35-2. See Section 17.4.1, IA32_DEBUGCTL MSR.
    pub const IA32_PERF_CAPABILITIES: Self = Self(0x345);

    /// RO. This applies to processors that do not support architectural  perfmon version 2.
    pub const MSR_PERF_CAPABILITIES: Self = Self(0x345);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_BPU_CCCR0: Self = Self(0x360);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_BPU_CCCR1: Self = Self(0x361);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_BPU_CCCR2: Self = Self(0x362);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_BPU_CCCR3: Self = Self(0x363);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_MS_CCCR0: Self = Self(0x364);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_MS_CCCR1: Self = Self(0x365);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_MS_CCCR2: Self = Self(0x366);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_MS_CCCR3: Self = Self(0x367);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_FLAME_CCCR0: Self = Self(0x368);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_FLAME_CCCR1: Self = Self(0x369);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_FLAME_CCCR2: Self = Self(0x36a);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_FLAME_CCCR3: Self = Self(0x36b);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_IQ_CCCR0: Self = Self(0x36c);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_IQ_CCCR1: Self = Self(0x36d);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_IQ_CCCR2: Self = Self(0x36e);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_IQ_CCCR3: Self = Self(0x36f);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_IQ_CCCR4: Self = Self(0x370);

    /// See Section 18.12.3, CCCR MSRs.
    pub const MSR_IQ_CCCR5: Self = Self(0x371);

    /// Fixed-Function-Counter Control Register (R/W)
    pub const MSR_PERF_FIXED_CTR_CTRL: Self = Self(0x38d);

    /// Fixed-Function-Counter Control Register (R/W)  See Table 35-2.
    pub const IA32_FIXED_CTR_CTRL: Self = Self(0x38d);

    /// See Section 18.4.2, Global Counter Control Facilities.
    pub const MSR_PERF_GLOBAL_STAUS: Self = Self(0x38e);

    /// See Table 35-2. See Section 18.4.2, Global Counter Control  Facilities.
    pub const IA32_PERF_GLOBAL_STAUS: Self = Self(0x38e);

    /// See Section 18.4.2, Global Counter Control Facilities.
    pub const MSR_PERF_GLOBAL_CTRL: Self = Self(0x38f);

    /// See Table 35-2. See Section 18.4.2, Global Counter Control  Facilities.
    pub const IA32_PERF_GLOBAL_CTRL: Self = Self(0x38f);

    /// See Section 18.4.2, Global Counter Control Facilities.
    pub const MSR_PERF_GLOBAL_OVF_CTRL: Self = Self(0x390);

    /// See Table 35-2. See Section 18.4.2, Global Counter Control  Facilities.
    pub const IA32_PERF_GLOBAL_OVF_CTRL: Self = Self(0x390);

    /// See Section 18.7.2.1, Uncore Performance Monitoring  Management Facility.
    pub const MSR_UNCORE_PERF_GLOBAL_CTRL: Self = Self(0x391);

    /// Uncore PMU global control
    pub const MSR_UNC_PERF_GLOBAL_CTRL: Self = Self(0x391);

    /// See Section 18.7.2.1, Uncore Performance Monitoring  Management Facility.
    pub const MSR_UNCORE_PERF_GLOBAL_STATUS: Self = Self(0x392);

    /// Uncore PMU main status
    pub const MSR_UNC_PERF_GLOBAL_STATUS: Self = Self(0x392);

    /// See Section 18.7.2.1, Uncore Performance Monitoring  Management Facility.
    pub const MSR_UNCORE_PERF_GLOBAL_OVF_CTRL: Self = Self(0x393);

    /// See Section 18.7.2.1, Uncore Performance Monitoring  Management Facility.
    pub const MSR_UNCORE_FIXED_CTR0: Self = Self(0x394);

    /// Uncore W-box perfmon fixed counter
    pub const MSR_W_PMON_FIXED_CTR: Self = Self(0x394);

    /// Uncore fixed counter control (R/W)
    pub const MSR_UNC_PERF_FIXED_CTRL: Self = Self(0x394);

    /// See Section 18.7.2.1, Uncore Performance Monitoring  Management Facility.
    pub const MSR_UNCORE_FIXED_CTR_CTRL: Self = Self(0x395);

    /// Uncore U-box perfmon fixed counter control MSR
    pub const MSR_W_PMON_FIXED_CTR_CTL: Self = Self(0x395);

    /// Uncore fixed counter
    pub const MSR_UNC_PERF_FIXED_CTR: Self = Self(0x395);

    /// See Section 18.7.2.3, Uncore Address/Opcode Match MSR.
    pub const MSR_UNCORE_ADDR_OPCODE_MATCH: Self = Self(0x396);

    /// Uncore C-Box configuration information (R/O)
    pub const MSR_UNC_CBO_CONFIG: Self = Self(0x396);

    pub const MSR_PEBS_NUM_ALT: Self = Self(0x39c);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_BSU_ESCR0: Self = Self(0x3a0);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_BSU_ESCR1: Self = Self(0x3a1);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_FSB_ESCR0: Self = Self(0x3a2);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_FSB_ESCR1: Self = Self(0x3a3);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_FIRM_ESCR0: Self = Self(0x3a4);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_FIRM_ESCR1: Self = Self(0x3a5);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_FLAME_ESCR0: Self = Self(0x3a6);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_FLAME_ESCR1: Self = Self(0x3a7);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_DAC_ESCR0: Self = Self(0x3a8);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_DAC_ESCR1: Self = Self(0x3a9);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_MOB_ESCR0: Self = Self(0x3aa);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_MOB_ESCR1: Self = Self(0x3ab);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_PMH_ESCR0: Self = Self(0x3ac);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_PMH_ESCR1: Self = Self(0x3ad);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_SAAT_ESCR0: Self = Self(0x3ae);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_SAAT_ESCR1: Self = Self(0x3af);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_U2L_ESCR0: Self = Self(0x3b0);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration  Facility.
    pub const MSR_UNCORE_PMC0: Self = Self(0x3b0);

    /// Uncore Arb unit, performance counter 0
    pub const MSR_UNC_ARB_PER_CTR0: Self = Self(0x3b0);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_U2L_ESCR1: Self = Self(0x3b1);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration  Facility.
    pub const MSR_UNCORE_PMC1: Self = Self(0x3b1);

    /// Uncore Arb unit, performance counter 1
    pub const MSR_UNC_ARB_PER_CTR1: Self = Self(0x3b1);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_BPU_ESCR0: Self = Self(0x3b2);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration  Facility.
    pub const MSR_UNCORE_PMC2: Self = Self(0x3b2);

    /// Uncore Arb unit, counter 0 event select MSR
    pub const MSR_UNC_ARB_PERFEVTSEL0: Self = Self(0x3b2);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_BPU_ESCR1: Self = Self(0x3b3);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PMC3: Self = Self(0x3b3);

    /// Uncore Arb unit, counter 1 event select MSR
    pub const MSR_UNC_ARB_PERFEVTSEL1: Self = Self(0x3b3);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_IS_ESCR0: Self = Self(0x3b4);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PMC4: Self = Self(0x3b4);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_IS_ESCR1: Self = Self(0x3b5);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration  Facility.
    pub const MSR_UNCORE_PMC5: Self = Self(0x3b5);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_ITLB_ESCR0: Self = Self(0x3b6);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PMC6: Self = Self(0x3b6);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_ITLB_ESCR1: Self = Self(0x3b7);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PMC7: Self = Self(0x3b7);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_CRU_ESCR0: Self = Self(0x3b8);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_CRU_ESCR1: Self = Self(0x3b9);

    /// See Section 18.12.1, ESCR MSRs. This MSR is not available on later processors. It is  only available on processor family 0FH, models  01H-02H.
    pub const MSR_IQ_ESCR0: Self = Self(0x3ba);

    /// See Section 18.12.1, ESCR MSRs. This MSR is not available on later processors. It is  only available on processor family 0FH, models  01H-02H.
    pub const MSR_IQ_ESCR1: Self = Self(0x3bb);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_RAT_ESCR0: Self = Self(0x3bc);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_RAT_ESCR1: Self = Self(0x3bd);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_SSU_ESCR0: Self = Self(0x3be);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_MS_ESCR0: Self = Self(0x3c0);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PERFEVTSEL0: Self = Self(0x3c0);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_MS_ESCR1: Self = Self(0x3c1);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PERFEVTSEL1: Self = Self(0x3c1);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_TBPU_ESCR0: Self = Self(0x3c2);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PERFEVTSEL2: Self = Self(0x3c2);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_TBPU_ESCR1: Self = Self(0x3c3);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PERFEVTSEL3: Self = Self(0x3c3);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_TC_ESCR0: Self = Self(0x3c4);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PERFEVTSEL4: Self = Self(0x3c4);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_TC_ESCR1: Self = Self(0x3c5);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PERFEVTSEL5: Self = Self(0x3c5);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PERFEVTSEL6: Self = Self(0x3c6);

    /// See Section 18.7.2.2, Uncore Performance Event Configuration Facility.
    pub const MSR_UNCORE_PERFEVTSEL7: Self = Self(0x3c7);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_IX_ESCR0: Self = Self(0x3c8);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_ALF_ESCR0: Self = Self(0x3ca);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_ALF_ESCR1: Self = Self(0x3cb);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_CRU_ESCR2: Self = Self(0x3cc);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_CRU_ESCR3: Self = Self(0x3cd);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_CRU_ESCR4: Self = Self(0x3e0);

    /// See Section 18.12.1, ESCR MSRs.
    pub const MSR_CRU_ESCR5: Self = Self(0x3e1);

    pub const IA32_PEBS_ENABLE: Self = Self(0x3f1);

    /// Precise Event-Based Sampling (PEBS) (R/W)  Controls the enabling of precise event sampling  and replay tagging.
    pub const MSR_PEBS_ENABLE: Self = Self(0x3f1);

    /// See Table 19-26.
    pub const MSR_PEBS_MATRIX_VERT: Self = Self(0x3f2);

    /// see See Section 18.7.1.2, Load Latency Performance Monitoring  Facility.
    pub const MSR_PEBS_LD_LAT: Self = Self(0x3f6);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_PKG_C3_RESIDENCY: Self = Self(0x3f8);

    /// Package C2 Residency Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C-States
    pub const MSR_PKG_C2_RESIDENCY: Self = Self(0x3f8);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_PKG_C6C_RESIDENCY: Self = Self(0x3f9);

    /// Package C4 Residency Note: C-state values are processor specific C-state code names, unrelated to MWAIT extension C-state parameters or ACPI C-States
    pub const MSR_PKG_C4_RESIDENCY: Self = Self(0x3f9);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_PKG_C7_RESIDENCY: Self = Self(0x3fa);

    /// Package C6 Residency Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C-States
    pub const MSR_PKG_C6_RESIDENCY: Self = Self(0x3fa);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_CORE_C3_RESIDENCY: Self = Self(0x3fc);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_CORE_C4_RESIDENCY: Self = Self(0x3fc);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_CORE_C6_RESIDENCY: Self = Self(0x3fd);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_CORE_C7_RESIDENCY: Self = Self(0x3fe);

    pub const MC0_CTL: Self = Self(0x400);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const IA32_MC0_CTL: Self = Self(0x400);

    pub const MC0_STATUS: Self = Self(0x401);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const IA32_MC0_STATUS: Self = Self(0x401);

    pub const MC0_ADDR: Self = Self(0x402);

    /// P6 Family Processors
    pub const IA32_MC0_ADDR1: Self = Self(0x402);

    /// See Section 14.3.2.3., IA32_MCi_ADDR MSRs .  The IA32_MC0_ADDR register is either not implemented or contains no address if the ADDRV flag in the IA32_MC0_STATUS register is clear.  When not implemented in the processor, all reads and writes to this MSR  will cause a general-protection exception.
    pub const IA32_MC0_ADDR: Self = Self(0x402);

    /// Defined in MCA architecture but not implemented in the P6 family  processors.
    pub const MC0_MISC: Self = Self(0x403);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs. The IA32_MC0_MISC MSR is either not  implemented or does not contain additional  information if the MISCV flag in the  IA32_MC0_STATUS register is clear. When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception.
    pub const IA32_MC0_MISC: Self = Self(0x403);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC0_MISC: Self = Self(0x403);

    pub const MC1_CTL: Self = Self(0x404);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const IA32_MC1_CTL: Self = Self(0x404);

    /// Bit definitions same as MC0_STATUS.
    pub const MC1_STATUS: Self = Self(0x405);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const IA32_MC1_STATUS: Self = Self(0x405);

    pub const MC1_ADDR: Self = Self(0x406);

    /// P6 Family Processors
    pub const IA32_MC1_ADDR2: Self = Self(0x406);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs. The IA32_MC1_ADDR register is either not implemented or  contains no address if the ADDRV flag in the IA32_MC1_STATUS  register is clear.  When not implemented in the processor, all reads and writes to this  MSR will cause a general-protection exception.
    pub const IA32_MC1_ADDR: Self = Self(0x406);

    /// Defined in MCA architecture but not implemented in the P6 family  processors.
    pub const MC1_MISC: Self = Self(0x407);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs. The IA32_MC1_MISC MSR is either not  implemented or does not contain additional  information if the MISCV flag in the  IA32_MC1_STATUS register is clear. When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception.
    pub const IA32_MC1_MISC: Self = Self(0x407);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC1_MISC: Self = Self(0x407);

    pub const MC2_CTL: Self = Self(0x408);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const IA32_MC2_CTL: Self = Self(0x408);

    /// Bit definitions same as MC0_STATUS.
    pub const MC2_STATUS: Self = Self(0x409);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const IA32_MC2_STATUS: Self = Self(0x409);

    pub const MC2_ADDR: Self = Self(0x40a);

    /// P6 Family Processors
    pub const IA32_MC2_ADDR1: Self = Self(0x40a);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs. The IA32_MC2_ADDR register is either not  implemented or contains no address if the ADDRV  flag in the IA32_MC2_STATUS register is clear.  When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception.
    pub const IA32_MC2_ADDR: Self = Self(0x40a);

    /// Defined in MCA architecture but not implemented in the P6 family  processors.
    pub const MC2_MISC: Self = Self(0x40b);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs. The IA32_MC2_MISC MSR is either not  implemented or does not contain additional  information if the MISCV flag in the IA32_MC2_STATUS register is clear.  When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception.
    pub const IA32_MC2_MISC: Self = Self(0x40b);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC2_MISC: Self = Self(0x40b);

    pub const MC4_CTL: Self = Self(0x40c);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const IA32_MC3_CTL: Self = Self(0x40c);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC4_CTL: Self = Self(0x40c);

    /// Bit definitions same as MC0_STATUS, except bits 0, 4, 57, and 61 are  hardcoded to 1.
    pub const MC4_STATUS: Self = Self(0x40d);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const IA32_MC3_STATUS: Self = Self(0x40d);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS.
    pub const MSR_MC4_STATUS: Self = Self(0x40d);

    /// Defined in MCA architecture but not implemented in P6 Family processors.
    pub const MC4_ADDR: Self = Self(0x40e);

    /// P6 Family Processors
    pub const IA32_MC3_ADDR1: Self = Self(0x40e);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs. The IA32_MC3_ADDR register is either not  implemented or contains no address if the ADDRV  flag in the IA32_MC3_STATUS register is clear. When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception.
    pub const IA32_MC3_ADDR: Self = Self(0x40e);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs. The MSR_MC4_ADDR register is either not implemented or  contains no address if the ADDRV flag in the MSR_MC4_STATUS  register is clear. When not implemented in the processor, all reads and writes to this  MSR will cause a general-protection exception.
    pub const MSR_MC4_ADDR: Self = Self(0x412);

    /// Defined in MCA architecture but not implemented in the P6 family  processors.
    pub const MC4_MISC: Self = Self(0x40f);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs. The IA32_MC3_MISC MSR is either not  implemented or does not contain additional  information if the MISCV flag in the  IA32_MC3_STATUS register is clear. When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception.
    pub const IA32_MC3_MISC: Self = Self(0x40f);

    pub const MC3_CTL: Self = Self(0x410);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const IA32_MC4_CTL: Self = Self(0x410);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC3_CTL: Self = Self(0x410);

    /// Bit definitions same as MC0_STATUS.
    pub const MC3_STATUS: Self = Self(0x411);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const IA32_MC4_STATUS: Self = Self(0x411);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS.
    pub const MSR_MC3_STATUS: Self = Self(0x411);

    pub const MC3_ADDR: Self = Self(0x412);

    /// P6 Family Processors
    pub const IA32_MC4_ADDR1: Self = Self(0x412);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs. The IA32_MC2_ADDR register is either not  implemented or contains no address if the ADDRV  flag in the IA32_MC4_STATUS register is clear.  When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception.
    pub const IA32_MC4_ADDR: Self = Self(0x412);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs. The MSR_MC3_ADDR register is either not implemented or  contains no address if the ADDRV flag in the MSR_MC3_STATUS register is clear.  When not implemented in the processor, all reads and writes to this  MSR will cause a general-protection exception.
    pub const MSR_MC3_ADDR: Self = Self(0x412);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC3_MISC: Self = Self(0x40f);

    /// Defined in MCA architecture but not implemented in the P6 family  processors.
    pub const MC3_MISC: Self = Self(0x413);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.   The IA32_MC2_MISC MSR is either not  implemented or does not contain additional  information if the MISCV flag in the  IA32_MC4_STATUS register is clear.  When not implemented in the processor, all reads  and writes to this MSR will cause a general- protection exception.
    pub const IA32_MC4_MISC: Self = Self(0x413);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC4_MISC: Self = Self(0x413);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC5_CTL: Self = Self(0x414);

    /// 06_0FH
    pub const IA32_MC5_CTL: Self = Self(0x414);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC5_STATUS: Self = Self(0x415);

    /// 06_0FH
    pub const IA32_MC5_STATUS: Self = Self(0x415);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs. The MSR_MC4_ADDR register is either not implemented or  contains no address if the ADDRV flag in the MSR_MC4_STATUS  register is clear. When not implemented in the processor, all reads and writes to this  MSR will cause a general-protection exception.
    pub const MSR_MC5_ADDR: Self = Self(0x416);

    /// 06_0FH
    pub const IA32_MC5_ADDR1: Self = Self(0x416);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC5_MISC: Self = Self(0x417);

    /// 06_0FH
    pub const IA32_MC5_MISC: Self = Self(0x417);

    /// 06_1DH
    pub const IA32_MC6_CTL: Self = Self(0x418);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC6_CTL: Self = Self(0x418);

    /// 06_1DH
    pub const IA32_MC6_STATUS: Self = Self(0x419);

    /// Apply to Intel Xeon processor 7400 series (processor signature  06_1D) only. See Section 15.3.2.2, IA32_MCi_STATUS MSRS.  and  Chapter 23.
    pub const MSR_MC6_STATUS: Self = Self(0x419);

    /// 06_1DH
    pub const IA32_MC6_ADDR1: Self = Self(0x41a);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC6_ADDR: Self = Self(0x41a);

    /// Misc MAC information of Integrated I/O. (R/O) see Section 15.3.2.4
    pub const IA32_MC6_MISC: Self = Self(0x41b);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC6_MISC: Self = Self(0x41b);

    /// 06_1AH
    pub const IA32_MC7_CTL: Self = Self(0x41c);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC7_CTL: Self = Self(0x41c);

    /// 06_1AH
    pub const IA32_MC7_STATUS: Self = Self(0x41d);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC7_STATUS: Self = Self(0x41d);

    /// 06_1AH
    pub const IA32_MC7_ADDR1: Self = Self(0x41e);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC7_ADDR: Self = Self(0x41e);

    /// 06_1AH
    pub const IA32_MC7_MISC: Self = Self(0x41f);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC7_MISC: Self = Self(0x41f);

    /// 06_1AH
    pub const IA32_MC8_CTL: Self = Self(0x420);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC8_CTL: Self = Self(0x420);

    /// 06_1AH
    pub const IA32_MC8_STATUS: Self = Self(0x421);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC8_STATUS: Self = Self(0x421);

    /// 06_1AH
    pub const IA32_MC8_ADDR1: Self = Self(0x422);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC8_ADDR: Self = Self(0x422);

    /// 06_1AH
    pub const IA32_MC8_MISC: Self = Self(0x423);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC8_MISC: Self = Self(0x423);

    /// 06_2EH
    pub const IA32_MC9_CTL: Self = Self(0x424);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC9_CTL: Self = Self(0x424);

    /// 06_2EH
    pub const IA32_MC9_STATUS: Self = Self(0x425);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC9_STATUS: Self = Self(0x425);

    /// 06_2EH
    pub const IA32_MC9_ADDR1: Self = Self(0x426);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC9_ADDR: Self = Self(0x426);

    /// 06_2EH
    pub const IA32_MC9_MISC: Self = Self(0x427);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC9_MISC: Self = Self(0x427);

    /// 06_2EH
    pub const IA32_MC10_CTL: Self = Self(0x428);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC10_CTL: Self = Self(0x428);

    /// 06_2EH
    pub const IA32_MC10_STATUS: Self = Self(0x429);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC10_STATUS: Self = Self(0x429);

    /// 06_2EH
    pub const IA32_MC10_ADDR1: Self = Self(0x42a);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC10_ADDR: Self = Self(0x42a);

    /// 06_2EH
    pub const IA32_MC10_MISC: Self = Self(0x42b);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC10_MISC: Self = Self(0x42b);

    /// 06_2EH
    pub const IA32_MC11_CTL: Self = Self(0x42c);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC11_CTL: Self = Self(0x42c);

    /// 06_2EH
    pub const IA32_MC11_STATUS: Self = Self(0x42d);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC11_STATUS: Self = Self(0x42d);

    /// 06_2EH
    pub const IA32_MC11_ADDR1: Self = Self(0x42e);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC11_ADDR: Self = Self(0x42e);

    /// 06_2EH
    pub const IA32_MC11_MISC: Self = Self(0x42f);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC11_MISC: Self = Self(0x42f);

    /// 06_2EH
    pub const IA32_MC12_CTL: Self = Self(0x430);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC12_CTL: Self = Self(0x430);

    /// 06_2EH
    pub const IA32_MC12_STATUS: Self = Self(0x431);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC12_STATUS: Self = Self(0x431);

    /// 06_2EH
    pub const IA32_MC12_ADDR1: Self = Self(0x432);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC12_ADDR: Self = Self(0x432);

    /// 06_2EH
    pub const IA32_MC12_MISC: Self = Self(0x433);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC12_MISC: Self = Self(0x433);

    /// 06_2EH
    pub const IA32_MC13_CTL: Self = Self(0x434);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC13_CTL: Self = Self(0x434);

    /// 06_2EH
    pub const IA32_MC13_STATUS: Self = Self(0x435);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC13_STATUS: Self = Self(0x435);

    /// 06_2EH
    pub const IA32_MC13_ADDR1: Self = Self(0x436);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC13_ADDR: Self = Self(0x436);

    /// 06_2EH
    pub const IA32_MC13_MISC: Self = Self(0x437);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC13_MISC: Self = Self(0x437);

    /// 06_2EH
    pub const IA32_MC14_CTL: Self = Self(0x438);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC14_CTL: Self = Self(0x438);

    /// 06_2EH
    pub const IA32_MC14_STATUS: Self = Self(0x439);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC14_STATUS: Self = Self(0x439);

    /// 06_2EH
    pub const IA32_MC14_ADDR1: Self = Self(0x43a);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC14_ADDR: Self = Self(0x43a);

    /// 06_2EH
    pub const IA32_MC14_MISC: Self = Self(0x43b);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC14_MISC: Self = Self(0x43b);

    /// 06_2EH
    pub const IA32_MC15_CTL: Self = Self(0x43c);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC15_CTL: Self = Self(0x43c);

    /// 06_2EH
    pub const IA32_MC15_STATUS: Self = Self(0x43d);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC15_STATUS: Self = Self(0x43d);

    /// 06_2EH
    pub const IA32_MC15_ADDR1: Self = Self(0x43e);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC15_ADDR: Self = Self(0x43e);

    /// 06_2EH
    pub const IA32_MC15_MISC: Self = Self(0x43f);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC15_MISC: Self = Self(0x43f);

    /// 06_2EH
    pub const IA32_MC16_CTL: Self = Self(0x440);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC16_CTL: Self = Self(0x440);

    /// 06_2EH
    pub const IA32_MC16_STATUS: Self = Self(0x441);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC16_STATUS: Self = Self(0x441);

    /// 06_2EH
    pub const IA32_MC16_ADDR1: Self = Self(0x442);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC16_ADDR: Self = Self(0x442);

    /// 06_2EH
    pub const IA32_MC16_MISC: Self = Self(0x443);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC16_MISC: Self = Self(0x443);

    /// 06_2EH
    pub const IA32_MC17_CTL: Self = Self(0x444);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC17_CTL: Self = Self(0x444);

    /// 06_2EH
    pub const IA32_MC17_STATUS: Self = Self(0x445);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC17_STATUS: Self = Self(0x445);

    /// 06_2EH
    pub const IA32_MC17_ADDR1: Self = Self(0x446);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC17_ADDR: Self = Self(0x446);

    /// 06_2EH
    pub const IA32_MC17_MISC: Self = Self(0x447);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC17_MISC: Self = Self(0x447);

    /// 06_2EH
    pub const IA32_MC18_CTL: Self = Self(0x448);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC18_CTL: Self = Self(0x448);

    /// 06_2EH
    pub const IA32_MC18_STATUS: Self = Self(0x449);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC18_STATUS: Self = Self(0x449);

    /// 06_2EH
    pub const IA32_MC18_ADDR1: Self = Self(0x44a);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC18_ADDR: Self = Self(0x44a);

    /// 06_2EH
    pub const IA32_MC18_MISC: Self = Self(0x44b);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC18_MISC: Self = Self(0x44b);

    /// 06_2EH
    pub const IA32_MC19_CTL: Self = Self(0x44c);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC19_CTL: Self = Self(0x44c);

    /// 06_2EH
    pub const IA32_MC19_STATUS: Self = Self(0x44d);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC19_STATUS: Self = Self(0x44d);

    /// 06_2EH
    pub const IA32_MC19_ADDR1: Self = Self(0x44e);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC19_ADDR: Self = Self(0x44e);

    /// 06_2EH
    pub const IA32_MC19_MISC: Self = Self(0x44f);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC19_MISC: Self = Self(0x44f);

    /// 06_2EH
    pub const IA32_MC20_CTL: Self = Self(0x450);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC20_CTL: Self = Self(0x450);

    /// 06_2EH
    pub const IA32_MC20_STATUS: Self = Self(0x451);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC20_STATUS: Self = Self(0x451);

    /// 06_2EH
    pub const IA32_MC20_ADDR1: Self = Self(0x452);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC20_ADDR: Self = Self(0x452);

    /// 06_2EH
    pub const IA32_MC20_MISC: Self = Self(0x453);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC20_MISC: Self = Self(0x453);

    /// 06_2EH
    pub const IA32_MC21_CTL: Self = Self(0x454);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC21_CTL: Self = Self(0x454);

    /// 06_2EH
    pub const IA32_MC21_STATUS: Self = Self(0x455);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC21_STATUS: Self = Self(0x455);

    /// 06_2EH
    pub const IA32_MC21_ADDR1: Self = Self(0x456);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC21_ADDR: Self = Self(0x456);

    /// 06_2EH
    pub const IA32_MC21_MISC: Self = Self(0x457);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC21_MISC: Self = Self(0x457);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC22_CTL: Self = Self(0x458);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC22_STATUS: Self = Self(0x459);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC22_ADDR: Self = Self(0x45a);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC22_MISC: Self = Self(0x45b);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC23_CTL: Self = Self(0x45c);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC23_STATUS: Self = Self(0x45d);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC23_ADDR: Self = Self(0x45e);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC23_MISC: Self = Self(0x45f);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC24_CTL: Self = Self(0x460);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC24_STATUS: Self = Self(0x461);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC24_ADDR: Self = Self(0x462);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC24_MISC: Self = Self(0x463);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC25_CTL: Self = Self(0x464);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC25_STATUS: Self = Self(0x465);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC25_ADDR: Self = Self(0x466);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC25_MISC: Self = Self(0x467);

    /// See Section 15.3.2.1,  IA32_MCi_CTL MSRs.
    pub const MSR_MC26_CTL: Self = Self(0x468);

    /// See Section 15.3.2.2, IA32_MCi_STATUS MSRS,  and Chapter 16.
    pub const MSR_MC26_STATUS: Self = Self(0x469);

    /// See Section 15.3.2.3, IA32_MCi_ADDR MSRs.
    pub const MSR_MC26_ADDR: Self = Self(0x46a);

    /// See Section 15.3.2.4,  IA32_MCi_MISC MSRs.
    pub const MSR_MC26_MISC: Self = Self(0x46b);

    /// Reporting Register of Basic VMX Capabilities (R/O) See Table 35-2. See Appendix A.1, Basic VMX Information (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_BASIC: Self = Self(0x480);

    /// Capability Reporting Register of Pin-based VM-execution  Controls (R/O) See Appendix A.3, VM-Execution Controls (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_PINBASED_CTLS: Self = Self(0x481);

    /// Capability Reporting Register of Primary Processor-based  VM-execution Controls (R/O) See Appendix A.3, VM-Execution Controls (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_PROCBASED_CTLS: Self = Self(0x482);

    /// Capability Reporting Register of VM-exit Controls (R/O) See Appendix A.4, VM-Exit Controls (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_EXIT_CTLS: Self = Self(0x483);

    /// Capability Reporting Register of VM-entry Controls (R/O) See Appendix A.5, VM-Entry Controls (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_ENTRY_CTLS: Self = Self(0x484);

    /// Reporting Register of Miscellaneous VMX Capabilities (R/O) See Appendix A.6, Miscellaneous Data (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_MISC: Self = Self(0x485);

    /// Capability Reporting Register of CR0 Bits Fixed to 0 (R/O) See Appendix A.7, VMX-Fixed Bits in CR0 (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_CR0_FIXED0: Self = Self(0x486);

    /// If CPUID.01H:ECX.\[bit 5\] = 1
    pub const IA32_VMX_CRO_FIXED0: Self = Self(0x486);

    /// Capability Reporting Register of CR0 Bits Fixed to 1 (R/O) See Appendix A.7, VMX-Fixed Bits in CR0 (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_CR0_FIXED1: Self = Self(0x487);

    /// If CPUID.01H:ECX.\[bit 5\] = 1
    pub const IA32_VMX_CRO_FIXED1: Self = Self(0x487);

    /// Capability Reporting Register of CR4 Bits Fixed to 0 (R/O) See Appendix A.8, VMX-Fixed Bits in CR4 (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_CR4_FIXED0: Self = Self(0x488);

    /// Capability Reporting Register of CR4 Bits Fixed to 1 (R/O) See Appendix A.8, VMX-Fixed Bits in CR4 (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_CR4_FIXED1: Self = Self(0x489);

    /// Capability Reporting Register of VMCS Field Enumeration (R/O) See Appendix A.9, VMCS Enumeration (If CPUID.01H:ECX.\[bit 9\])
    pub const IA32_VMX_VMCS_ENUM: Self = Self(0x48a);

    /// Capability Reporting Register of Secondary Processor-based  VM-execution Controls (R/O) See Appendix A.3, VM-Execution Controls (If CPUID.01H:ECX.\[bit 9\] and  IA32_VMX_PROCBASED_CTLS\[bit 63\])
    pub const IA32_VMX_PROCBASED_CTLS2: Self = Self(0x48b);

    /// Capability Reporting Register of EPT and VPID (R/O)  See Table 35-2
    pub const IA32_VMX_EPT_VPID_ENUM: Self = Self(0x48c);

    /// If ( CPUID.01H:ECX.\[bit 5\],  IA32_VMX_PROCBASED_C TLS\[bit 63\], and either  IA32_VMX_PROCBASED_C TLS2\[bit 33\] or  IA32_VMX_PROCBASED_C TLS2\[bit 37\])
    pub const IA32_VMX_EPT_VPID_CAP: Self = Self(0x48c);

    /// Capability Reporting Register of Pin-based VM-execution Flex  Controls (R/O) See Table 35-2
    pub const IA32_VMX_TRUE_PINBASED_CTLS: Self = Self(0x48d);

    /// Capability Reporting Register of Primary Processor-based  VM-execution Flex Controls (R/O) See Table 35-2
    pub const IA32_VMX_TRUE_PROCBASED_CTLS: Self = Self(0x48e);

    /// Capability Reporting Register of VM-exit Flex Controls (R/O) See Table 35-2
    pub const IA32_VMX_TRUE_EXIT_CTLS: Self = Self(0x48f);

    /// Capability Reporting Register of VM-entry Flex Controls (R/O) See Table 35-2
    pub const IA32_VMX_TRUE_ENTRY_CTLS: Self = Self(0x490);

    /// Capability Reporting Register of VM-function Controls (R/O) See Table 35-2
    pub const IA32_VMX_FMFUNC: Self = Self(0x491);

    /// If( CPUID.01H:ECX.\[bit 5\] =  1 and IA32_VMX_BASIC\[bit 55\] )
    pub const IA32_VMX_VMFUNC: Self = Self(0x491);

    /// (If CPUID.0AH: EAX\[15:8\] >  0) & IA32_PERF_CAPABILITIES\[ 13\] = 1
    pub const IA32_A_PMC0: Self = Self(0x4c1);

    /// (If CPUID.0AH: EAX\[15:8\] >  1) & IA32_PERF_CAPABILITIES\[ 13\] = 1
    pub const IA32_A_PMC1: Self = Self(0x4c2);

    /// (If CPUID.0AH: EAX\[15:8\] >  2) & IA32_PERF_CAPABILITIES\[ 13\] = 1
    pub const IA32_A_PMC2: Self = Self(0x4c3);

    /// (If CPUID.0AH: EAX\[15:8\] >  3) & IA32_PERF_CAPABILITIES\[ 13\] = 1
    pub const IA32_A_PMC3: Self = Self(0x4c4);

    /// (If CPUID.0AH: EAX\[15:8\] >  4) & IA32_PERF_CAPABILITIES\[ 13\] = 1
    pub const IA32_A_PMC4: Self = Self(0x4c5);

    /// (If CPUID.0AH: EAX\[15:8\] >  5) & IA32_PERF_CAPABILITIES\[ 13\] = 1
    pub const IA32_A_PMC5: Self = Self(0x4c6);

    /// (If CPUID.0AH: EAX\[15:8\] >  6) & IA32_PERF_CAPABILITIES\[ 13\] = 1
    pub const IA32_A_PMC6: Self = Self(0x4c7);

    /// (If CPUID.0AH: EAX\[15:8\] >  7) & IA32_PERF_CAPABILITIES\[ 13\] = 1
    pub const IA32_A_PMC7: Self = Self(0x4c8);

    /// Enhanced SMM Feature Control (SMM-RW) Reports SMM capability Enhancement. Accessible only while in  SMM.
    pub const MSR_SMM_FEATURE_CONTROL: Self = Self(0x4e0);

    /// SMM Delayed (SMM-RO) Reports the interruptible state of all logical processors in the  package . Available only while in SMM and  MSR_SMM_MCA_CAP\[LONG_FLOW_INDICATION\] == 1.
    pub const MSR_SMM_DELAYED: Self = Self(0x4e2);

    /// SMM Blocked (SMM-RO) Reports the blocked state of all logical processors in the package .  Available only while in SMM.
    pub const MSR_SMM_BLOCKED: Self = Self(0x4e3);

    /// Trace Output Base Register (R/W)
    pub const MSR_IA32_RTIT_OUTPUT_BASE: Self = Self(0x560);

    /// Trace Output Mask Pointers Register (R/W)
    pub const MSR_IA32_RTIT_OUTPUT_MASK_PTRS: Self = Self(0x561);

    /// Trace Control Register (R/W)
    pub const MSR_IA32_RTIT_CTL: Self = Self(0x570);

    /// Tracing Status Register (R/W)
    pub const MSR_IA32_RTIT_STATUS: Self = Self(0x571);

    /// Trace Filter CR3 Match Register (R/W)
    pub const MSR_IA32_CR3_MATCH: Self = Self(0x572);

    /// Trace Start Address 0
    pub const MSR_IA32_ADDR0_START: Self = Self(0x580);

    /// Trace End Address 0
    pub const MSR_IA32_ADDR0_END: Self = Self(0x581);

    /// Trace Start Address 1
    pub const MSR_IA32_ADDR1_START: Self = Self(0x582);

    /// Trace End Address 1
    pub const MSR_IA32_ADDR1_END: Self = Self(0x583);

    /// Trace Start Address 3
    pub const MSR_IA32_ADDR2_START: Self = Self(0x584);

    /// Trace End Address 3
    pub const MSR_IA32_ADDR2_END: Self = Self(0x585);

    /// Trace Start Address 4
    pub const MSR_IA32_ADDR3_START: Self = Self(0x586);

    /// Trace End Address 4
    pub const MSR_IA32_ADDR3_END: Self = Self(0x587);

    /// DS Save Area (R/W) See Table 35-2. Points to the DS buffer management area, which is used to manage the  BTS and PEBS buffers. See Section 18.12.4, Debug Store (DS)  Mechanism.
    pub const IA32_DS_AREA: Self = Self(0x600);

    /// Unit Multipliers used in RAPL Interfaces (R/O)  See Section 14.7.1, RAPL Interfaces.
    pub const MSR_RAPL_POWER_UNIT: Self = Self(0x606);

    /// Package C3 Interrupt Response Limit (R/W)  Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_PKGC3_IRTL: Self = Self(0x60a);

    /// Package C6 Interrupt Response Limit (R/W)  This MSR defines the budget allocated for the package to exit from  C6 to a C0 state, where interrupt request can be delivered to the  core and serviced. Additional core-exit latency amy be applicable  depending on the actual C-state the core is in.  Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_PKGC6_IRTL: Self = Self(0x60b);

    /// Package C7 Interrupt Response Limit (R/W)  This MSR defines the budget allocated for the package to exit from  C7 to a C0 state, where interrupt request can be delivered to the  core and serviced. Additional core-exit latency amy be applicable  depending on the actual C-state the core is in.  Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C-States.
    pub const MSR_PKGC7_IRTL: Self = Self(0x60c);

    /// PKG RAPL Power Limit Control (R/W)  See Section 14.7.3, Package RAPL Domain.
    pub const MSR_PKG_POWER_LIMIT: Self = Self(0x610);

    /// PKG Energy Status (R/O)  See Section 14.7.3, Package RAPL Domain.
    pub const MSR_PKG_ENERGY_STATUS: Self = Self(0x611);

    /// Package RAPL Perf Status (R/O)
    pub const MSR_PKG_PERF_STATUS: Self = Self(0x613);

    /// PKG RAPL Parameters (R/W) See Section 14.7.3,  Package RAPL  Domain.
    pub const MSR_PKG_POWER_INFO: Self = Self(0x614);

    /// DRAM RAPL Power Limit Control (R/W)  See Section 14.7.5, DRAM RAPL Domain.
    pub const MSR_DRAM_POWER_LIMIT: Self = Self(0x618);

    /// DRAM Energy Status (R/O)  See Section 14.7.5, DRAM RAPL Domain.
    pub const MSR_DRAM_ENERGY_STATUS: Self = Self(0x619);

    /// DRAM Performance Throttling Status (R/O) See Section 14.7.5,  DRAM RAPL Domain.
    pub const MSR_DRAM_PERF_STATUS: Self = Self(0x61b);

    /// DRAM RAPL Parameters (R/W) See Section 14.7.5, DRAM RAPL Domain.
    pub const MSR_DRAM_POWER_INFO: Self = Self(0x61c);

    /// Note: C-state values are processor specific C-state code names, unrelated to MWAIT extension C-state parameters or ACPI C-States.
    pub const MSR_PKG_C9_RESIDENCY: Self = Self(0x631);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C-States.
    pub const MSR_PKG_C10_RESIDENCY: Self = Self(0x632);

    /// PP0 RAPL Power Limit Control (R/W)  See Section 14.7.4, PP0/PP1 RAPL Domains.
    pub const MSR_PP0_POWER_LIMIT: Self = Self(0x638);

    /// PP0 Energy Status (R/O)  See Section 14.7.4, PP0/PP1 RAPL Domains.
    pub const MSR_PP0_ENERGY_STATUS: Self = Self(0x639);

    /// PP0 Balance Policy (R/W)  See Section 14.7.4, PP0/PP1 RAPL Domains.
    pub const MSR_PP0_POLICY: Self = Self(0x63a);

    /// PP0 Performance Throttling Status (R/O) See Section 14.7.4,  PP0/PP1 RAPL Domains.
    pub const MSR_PP0_PERF_STATUS: Self = Self(0x63b);

    /// PP1 RAPL Power Limit Control (R/W)  See Section 14.7.4, PP0/PP1 RAPL Domains.
    pub const MSR_PP1_POWER_LIMIT: Self = Self(0x640);

    /// PP1 Energy Status (R/O)  See Section 14.7.4, PP0/PP1 RAPL Domains.
    pub const MSR_PP1_ENERGY_STATUS: Self = Self(0x641);

    /// PP1 Balance Policy (R/W)  See Section 14.7.4, PP0/PP1 RAPL Domains.
    pub const MSR_PP1_POLICY: Self = Self(0x642);

    /// Nominal TDP Ratio (R/O)
    pub const MSR_CONFIG_TDP_NOMINAL: Self = Self(0x648);

    /// ConfigTDP Level 1 ratio and power level (R/O)
    pub const MSR_CONFIG_TDP_LEVEL1: Self = Self(0x649);

    /// ConfigTDP Level 2 ratio and power level (R/O)
    pub const MSR_CONFIG_TDP_LEVEL2: Self = Self(0x64a);

    /// ConfigTDP Control (R/W)
    pub const MSR_CONFIG_TDP_CONTROL: Self = Self(0x64b);

    /// ConfigTDP Control (R/W)
    pub const MSR_TURBO_ACTIVATION_RATIO: Self = Self(0x64c);

    /// Note: C-state values are processor specific C-state code names,  unrelated to MWAIT extension C-state parameters or ACPI C- States.
    pub const MSR_CORE_C1_RESIDENCY: Self = Self(0x660);

    /// Last Branch Record 8 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_8_FROM_IP: Self = Self(0x688);

    /// Last Branch Record 9 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_9_FROM_IP: Self = Self(0x689);

    /// Last Branch Record 10 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_10_FROM_IP: Self = Self(0x68a);

    /// Last Branch Record 11 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_11_FROM_IP: Self = Self(0x68b);

    /// Last Branch Record 12 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_12_FROM_IP: Self = Self(0x68c);

    /// Last Branch Record 13 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_13_FROM_IP: Self = Self(0x68d);

    /// Last Branch Record 14 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_14_FROM_IP: Self = Self(0x68e);

    /// Last Branch Record 15 From IP (R/W) See description of MSR_LASTBRANCH_0_FROM_IP.
    pub const MSR_LASTBRANCH_15_FROM_IP: Self = Self(0x68f);

    /// Last Branch Record 8 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_8_TO_IP: Self = Self(0x6c8);

    /// Last Branch Record 9 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_9_TO_IP: Self = Self(0x6c9);

    /// Last Branch Record 10 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_10_TO_IP: Self = Self(0x6ca);

    /// Last Branch Record 11 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_11_TO_IP: Self = Self(0x6cb);

    /// Last Branch Record 12 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_12_TO_IP: Self = Self(0x6cc);

    /// Last Branch Record 13 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_13_TO_IP: Self = Self(0x6cd);

    /// Last Branch Record 14 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_14_TO_IP: Self = Self(0x6ce);

    /// Last Branch Record 15 To IP (R/W) See description of MSR_LASTBRANCH_0_TO_IP.
    pub const MSR_LASTBRANCH_15_TO_IP: Self = Self(0x6cf);

    /// TSC Target of Local APIC s TSC Deadline Mode (R/W)  See Table 35-2
    pub const IA32_TSC_DEADLINE: Self = Self(0x6e0);

    /// Uncore C-Box 0, counter 0 event select MSR
    pub const MSR_UNC_CBO_0_PERFEVTSEL0: Self = Self(0x700);

    /// Uncore C-Box 0, counter 1 event select MSR
    pub const MSR_UNC_CBO_0_PERFEVTSEL1: Self = Self(0x701);

    /// Uncore C-Box 0, performance counter 0
    pub const MSR_UNC_CBO_0_PER_CTR0: Self = Self(0x706);

    /// Uncore C-Box 0, performance counter 1
    pub const MSR_UNC_CBO_0_PER_CTR1: Self = Self(0x707);

    /// Uncore C-Box 1, counter 0 event select MSR
    pub const MSR_UNC_CBO_1_PERFEVTSEL0: Self = Self(0x710);

    /// Uncore C-Box 1, counter 1 event select MSR
    pub const MSR_UNC_CBO_1_PERFEVTSEL1: Self = Self(0x711);

    /// Uncore C-Box 1, performance counter 0
    pub const MSR_UNC_CBO_1_PER_CTR0: Self = Self(0x716);

    /// Uncore C-Box 1, performance counter 1
    pub const MSR_UNC_CBO_1_PER_CTR1: Self = Self(0x717);

    /// Uncore C-Box 2, counter 0 event select MSR
    pub const MSR_UNC_CBO_2_PERFEVTSEL0: Self = Self(0x720);

    /// Uncore C-Box 2, counter 1 event select MSR
    pub const MSR_UNC_CBO_2_PERFEVTSEL1: Self = Self(0x721);

    /// Uncore C-Box 2, performance counter 0
    pub const MSR_UNC_CBO_2_PER_CTR0: Self = Self(0x726);

    /// Uncore C-Box 2, performance counter 1
    pub const MSR_UNC_CBO_2_PER_CTR1: Self = Self(0x727);

    /// Uncore C-Box 3, counter 0 event select MSR
    pub const MSR_UNC_CBO_3_PERFEVTSEL0: Self = Self(0x730);

    /// Uncore C-Box 3, counter 1 event select MSR.
    pub const MSR_UNC_CBO_3_PERFEVTSEL1: Self = Self(0x731);

    /// Uncore C-Box 3, performance counter 0.
    pub const MSR_UNC_CBO_3_PER_CTR0: Self = Self(0x736);

    /// Uncore C-Box 3, performance counter 1.
    pub const MSR_UNC_CBO_3_PER_CTR1: Self = Self(0x737);

    /// x2APIC ID register (R/O) See x2APIC Specification.
    pub const IA32_X2APIC_APICID: Self = Self(0x802);

    /// x2APIC Version. If ( CPUID.01H:ECX.\[bit 21\]  = 1 )
    pub const IA32_X2APIC_VERSION: Self = Self(0x803);

    /// x2APIC Task Priority register (R/W)
    pub const IA32_X2APIC_TPR: Self = Self(0x808);

    /// x2APIC Processor Priority register (R/O)
    pub const IA32_X2APIC_PPR: Self = Self(0x80a);

    /// x2APIC End of Interrupt. If ( CPUID.01H:ECX.\[bit 21\]  = 1 )
    pub const IA32_X2APIC_EOI: Self = Self(0x80b);

    /// x2APIC Logical Destination register (R/O)
    pub const IA32_X2APIC_LDR: Self = Self(0x80d);

    /// x2APIC Spurious Interrupt Vector register (R/W)
    pub const IA32_X2APIC_SIVR: Self = Self(0x80f);

    /// x2APIC In-Service register bits \[31:0\] (R/O)
    pub const IA32_X2APIC_ISR0: Self = Self(0x810);

    /// x2APIC In-Service register bits \[63:32\] (R/O)
    pub const IA32_X2APIC_ISR1: Self = Self(0x811);

    /// x2APIC In-Service register bits \[95:64\] (R/O)
    pub const IA32_X2APIC_ISR2: Self = Self(0x812);

    /// x2APIC In-Service register bits \[127:96\] (R/O)
    pub const IA32_X2APIC_ISR3: Self = Self(0x813);

    /// x2APIC In-Service register bits \[159:128\] (R/O)
    pub const IA32_X2APIC_ISR4: Self = Self(0x814);

    /// x2APIC In-Service register bits \[191:160\] (R/O)
    pub const IA32_X2APIC_ISR5: Self = Self(0x815);

    /// x2APIC In-Service register bits \[223:192\] (R/O)
    pub const IA32_X2APIC_ISR6: Self = Self(0x816);

    /// x2APIC In-Service register bits \[255:224\] (R/O)
    pub const IA32_X2APIC_ISR7: Self = Self(0x817);

    /// x2APIC Trigger Mode register bits \[31:0\] (R/O)
    pub const IA32_X2APIC_TMR0: Self = Self(0x818);

    /// x2APIC Trigger Mode register bits \[63:32\] (R/O)
    pub const IA32_X2APIC_TMR1: Self = Self(0x819);

    /// x2APIC Trigger Mode register bits \[95:64\] (R/O)
    pub const IA32_X2APIC_TMR2: Self = Self(0x81a);

    /// x2APIC Trigger Mode register bits \[127:96\] (R/O)
    pub const IA32_X2APIC_TMR3: Self = Self(0x81b);

    /// x2APIC Trigger Mode register bits \[159:128\] (R/O)
    pub const IA32_X2APIC_TMR4: Self = Self(0x81c);

    /// x2APIC Trigger Mode register bits \[191:160\] (R/O)
    pub const IA32_X2APIC_TMR5: Self = Self(0x81d);

    /// x2APIC Trigger Mode register bits \[223:192\] (R/O)
    pub const IA32_X2APIC_TMR6: Self = Self(0x81e);

    /// x2APIC Trigger Mode register bits \[255:224\] (R/O)
    pub const IA32_X2APIC_TMR7: Self = Self(0x81f);

    /// x2APIC Interrupt Request register bits \[31:0\] (R/O)
    pub const IA32_X2APIC_IRR0: Self = Self(0x820);

    /// x2APIC Interrupt Request register bits \[63:32\] (R/O)
    pub const IA32_X2APIC_IRR1: Self = Self(0x821);

    /// x2APIC Interrupt Request register bits \[95:64\] (R/O)
    pub const IA32_X2APIC_IRR2: Self = Self(0x822);

    /// x2APIC Interrupt Request register bits \[127:96\] (R/O)
    pub const IA32_X2APIC_IRR3: Self = Self(0x823);

    /// x2APIC Interrupt Request register bits \[159:128\] (R/O)
    pub const IA32_X2APIC_IRR4: Self = Self(0x824);

    /// x2APIC Interrupt Request register bits \[191:160\] (R/O)
    pub const IA32_X2APIC_IRR5: Self = Self(0x825);

    /// x2APIC Interrupt Request register bits \[223:192\] (R/O)
    pub const IA32_X2APIC_IRR6: Self = Self(0x826);

    /// x2APIC Interrupt Request register bits \[255:224\] (R/O)
    pub const IA32_X2APIC_IRR7: Self = Self(0x827);

    /// Error Status Register. If ( CPUID.01H:ECX.\[bit 21\]  = 1 )
    pub const IA32_X2APIC_ESR: Self = Self(0x828);

    /// x2APIC LVT Corrected Machine Check Interrupt register (R/W)
    pub const IA32_X2APIC_LVT_CMCI: Self = Self(0x82f);

    /// x2APIC Interrupt Command register (R/W)
    pub const IA32_X2APIC_ICR: Self = Self(0x830);

    /// x2APIC LVT Timer Interrupt register (R/W)
    pub const IA32_X2APIC_LVT_TIMER: Self = Self(0x832);

    /// x2APIC LVT Thermal Sensor Interrupt register (R/W)
    pub const IA32_X2APIC_LVT_THERMAL: Self = Self(0x833);

    /// x2APIC LVT Performance Monitor register (R/W)
    pub const IA32_X2APIC_LVT_PMI: Self = Self(0x834);

    /// If ( CPUID.01H:ECX.\[bit 21\]  = 1 )
    pub const IA32_X2APIC_LVT_LINT0: Self = Self(0x835);

    /// If ( CPUID.01H:ECX.\[bit 21\]  = 1 )
    pub const IA32_X2APIC_LVT_LINT1: Self = Self(0x836);

    /// If ( CPUID.01H:ECX.\[bit 21\]  = 1 )
    pub const IA32_X2APIC_LVT_ERROR: Self = Self(0x837);

    /// x2APIC Initial Count register (R/W)
    pub const IA32_X2APIC_INIT_COUNT: Self = Self(0x838);

    /// x2APIC Current Count register (R/O)
    pub const IA32_X2APIC_CUR_COUNT: Self = Self(0x839);

    /// x2APIC Divide Configuration register (R/W)
    pub const IA32_X2APIC_DIV_CONF: Self = Self(0x83e);

    /// If ( CPUID.01H:ECX.\[bit 21\]  = 1 )
    pub const IA32_X2APIC_SELF_IPI: Self = Self(0x83f);

    /// Uncore U-box perfmon global control MSR.
    pub const MSR_U_PMON_GLOBAL_CTRL: Self = Self(0xc00);

    /// Uncore U-box perfmon global status MSR.
    pub const MSR_U_PMON_GLOBAL_STATUS: Self = Self(0xc01);

    /// Uncore U-box perfmon global overflow control MSR.
    pub const MSR_U_PMON_GLOBAL_OVF_CTRL: Self = Self(0xc02);

    /// Uncore U-box perfmon event select MSR.
    pub const MSR_U_PMON_EVNT_SEL: Self = Self(0xc10);

    /// Uncore U-box perfmon counter MSR.
    pub const MSR_U_PMON_CTR: Self = Self(0xc11);

    /// Uncore B-box 0 perfmon local box control MSR.
    pub const MSR_B0_PMON_BOX_CTRL: Self = Self(0xc20);

    /// Uncore B-box 0 perfmon local box status MSR.
    pub const MSR_B0_PMON_BOX_STATUS: Self = Self(0xc21);

    /// Uncore B-box 0 perfmon local box overflow control MSR.
    pub const MSR_B0_PMON_BOX_OVF_CTRL: Self = Self(0xc22);

    /// Uncore B-box 0 perfmon event select MSR.
    pub const MSR_B0_PMON_EVNT_SEL0: Self = Self(0xc30);

    /// Uncore B-box 0 perfmon counter MSR.
    pub const MSR_B0_PMON_CTR0: Self = Self(0xc31);

    /// Uncore B-box 0 perfmon event select MSR.
    pub const MSR_B0_PMON_EVNT_SEL1: Self = Self(0xc32);

    /// Uncore B-box 0 perfmon counter MSR.
    pub const MSR_B0_PMON_CTR1: Self = Self(0xc33);

    /// Uncore B-box 0 perfmon event select MSR.
    pub const MSR_B0_PMON_EVNT_SEL2: Self = Self(0xc34);

    /// Uncore B-box 0 perfmon counter MSR.
    pub const MSR_B0_PMON_CTR2: Self = Self(0xc35);

    /// Uncore B-box 0 perfmon event select MSR.
    pub const MSR_B0_PMON_EVNT_SEL3: Self = Self(0xc36);

    /// Uncore B-box 0 perfmon counter MSR.
    pub const MSR_B0_PMON_CTR3: Self = Self(0xc37);

    /// Uncore S-box 0 perfmon local box control MSR.
    pub const MSR_S0_PMON_BOX_CTRL: Self = Self(0xc40);

    /// Uncore S-box 0 perfmon local box status MSR.
    pub const MSR_S0_PMON_BOX_STATUS: Self = Self(0xc41);

    /// Uncore S-box 0 perfmon local box overflow control MSR.
    pub const MSR_S0_PMON_BOX_OVF_CTRL: Self = Self(0xc42);

    /// Uncore S-box 0 perfmon event select MSR.
    pub const MSR_S0_PMON_EVNT_SEL0: Self = Self(0xc50);

    /// Uncore S-box 0 perfmon counter MSR.
    pub const MSR_S0_PMON_CTR0: Self = Self(0xc51);

    /// Uncore S-box 0 perfmon event select MSR.
    pub const MSR_S0_PMON_EVNT_SEL1: Self = Self(0xc52);

    /// Uncore S-box 0 perfmon counter MSR.
    pub const MSR_S0_PMON_CTR1: Self = Self(0xc53);

    /// Uncore S-box 0 perfmon event select MSR.
    pub const MSR_S0_PMON_EVNT_SEL2: Self = Self(0xc54);

    /// Uncore S-box 0 perfmon counter MSR.
    pub const MSR_S0_PMON_CTR2: Self = Self(0xc55);

    /// Uncore S-box 0 perfmon event select MSR.
    pub const MSR_S0_PMON_EVNT_SEL3: Self = Self(0xc56);

    /// Uncore S-box 0 perfmon counter MSR.
    pub const MSR_S0_PMON_CTR3: Self = Self(0xc57);

    /// Uncore B-box 1 perfmon local box control MSR.
    pub const MSR_B1_PMON_BOX_CTRL: Self = Self(0xc60);

    /// Uncore B-box 1 perfmon local box status MSR.
    pub const MSR_B1_PMON_BOX_STATUS: Self = Self(0xc61);

    /// Uncore B-box 1 perfmon local box overflow control MSR.
    pub const MSR_B1_PMON_BOX_OVF_CTRL: Self = Self(0xc62);

    /// Uncore B-box 1 perfmon event select MSR.
    pub const MSR_B1_PMON_EVNT_SEL0: Self = Self(0xc70);

    /// Uncore B-box 1 perfmon counter MSR.
    pub const MSR_B1_PMON_CTR0: Self = Self(0xc71);

    /// Uncore B-box 1 perfmon event select MSR.
    pub const MSR_B1_PMON_EVNT_SEL1: Self = Self(0xc72);

    /// Uncore B-box 1 perfmon counter MSR.
    pub const MSR_B1_PMON_CTR1: Self = Self(0xc73);

    /// Uncore B-box 1 perfmon event select MSR.
    pub const MSR_B1_PMON_EVNT_SEL2: Self = Self(0xc74);

    /// Uncore B-box 1 perfmon counter MSR.
    pub const MSR_B1_PMON_CTR2: Self = Self(0xc75);

    /// Uncore B-box 1vperfmon event select MSR.
    pub const MSR_B1_PMON_EVNT_SEL3: Self = Self(0xc76);

    /// Uncore B-box 1 perfmon counter MSR.
    pub const MSR_B1_PMON_CTR3: Self = Self(0xc77);

    /// Uncore W-box perfmon local box control MSR.
    pub const MSR_W_PMON_BOX_CTRL: Self = Self(0xc80);

    /// Uncore W-box perfmon local box status MSR.
    pub const MSR_W_PMON_BOX_STATUS: Self = Self(0xc81);

    /// Uncore W-box perfmon local box overflow control MSR.
    pub const MSR_W_PMON_BOX_OVF_CTRL: Self = Self(0xc82);

    /// If ( CPUID.(EAX=07H,  ECX=0):EBX.\[bit 12\] = 1 )
    pub const IA32_QM_EVTSEL: Self = Self(0xc8d);

    /// If ( CPUID.(EAX=07H,  ECX=0):EBX.\[bit 12\] = 1 )
    pub const IA32_QM_CTR: Self = Self(0xc8e);

    /// If ( CPUID.(EAX=07H,  ECX=0):EBX.\[bit 12\] = 1 )
    pub const IA32_PQR_ASSOC: Self = Self(0xc8f);

    /// Uncore W-box perfmon event select MSR.
    pub const MSR_W_PMON_EVNT_SEL0: Self = Self(0xc90);

    /// Uncore W-box perfmon counter MSR.
    pub const MSR_W_PMON_CTR0: Self = Self(0xc91);

    /// Uncore W-box perfmon event select MSR.
    pub const MSR_W_PMON_EVNT_SEL1: Self = Self(0xc92);

    /// Uncore W-box perfmon counter MSR.
    pub const MSR_W_PMON_CTR1: Self = Self(0xc93);

    /// Uncore W-box perfmon event select MSR.
    pub const MSR_W_PMON_EVNT_SEL2: Self = Self(0xc94);

    /// Uncore W-box perfmon counter MSR.
    pub const MSR_W_PMON_CTR2: Self = Self(0xc95);

    /// Uncore W-box perfmon event select MSR.
    pub const MSR_W_PMON_EVNT_SEL3: Self = Self(0xc96);

    /// Uncore W-box perfmon counter MSR.
    pub const MSR_W_PMON_CTR3: Self = Self(0xc97);

    /// Uncore M-box 0 perfmon local box control MSR.
    pub const MSR_M0_PMON_BOX_CTRL: Self = Self(0xca0);

    /// Uncore M-box 0 perfmon local box status MSR.
    pub const MSR_M0_PMON_BOX_STATUS: Self = Self(0xca1);

    /// Uncore M-box 0 perfmon local box overflow control MSR.
    pub const MSR_M0_PMON_BOX_OVF_CTRL: Self = Self(0xca2);

    /// Uncore M-box 0 perfmon time stamp unit select MSR.
    pub const MSR_M0_PMON_TIMESTAMP: Self = Self(0xca4);

    /// Uncore M-box 0 perfmon DSP unit select MSR.
    pub const MSR_M0_PMON_DSP: Self = Self(0xca5);

    /// Uncore M-box 0 perfmon ISS unit select MSR.
    pub const MSR_M0_PMON_ISS: Self = Self(0xca6);

    /// Uncore M-box 0 perfmon MAP unit select MSR.
    pub const MSR_M0_PMON_MAP: Self = Self(0xca7);

    /// Uncore M-box 0 perfmon MIC THR select MSR.
    pub const MSR_M0_PMON_MSC_THR: Self = Self(0xca8);

    /// Uncore M-box 0 perfmon PGT unit select MSR.
    pub const MSR_M0_PMON_PGT: Self = Self(0xca9);

    /// Uncore M-box 0 perfmon PLD unit select MSR.
    pub const MSR_M0_PMON_PLD: Self = Self(0xcaa);

    /// Uncore M-box 0 perfmon ZDP unit select MSR.
    pub const MSR_M0_PMON_ZDP: Self = Self(0xcab);

    /// Uncore M-box 0 perfmon event select MSR.
    pub const MSR_M0_PMON_EVNT_SEL0: Self = Self(0xcb0);

    /// Uncore M-box 0 perfmon counter MSR.
    pub const MSR_M0_PMON_CTR0: Self = Self(0xcb1);

    /// Uncore M-box 0 perfmon event select MSR.
    pub const MSR_M0_PMON_EVNT_SEL1: Self = Self(0xcb2);

    /// Uncore M-box 0 perfmon counter MSR.
    pub const MSR_M0_PMON_CTR1: Self = Self(0xcb3);

    /// Uncore M-box 0 perfmon event select MSR.
    pub const MSR_M0_PMON_EVNT_SEL2: Self = Self(0xcb4);

    /// Uncore M-box 0 perfmon counter MSR.
    pub const MSR_M0_PMON_CTR2: Self = Self(0xcb5);

    /// Uncore M-box 0 perfmon event select MSR.
    pub const MSR_M0_PMON_EVNT_SEL3: Self = Self(0xcb6);

    /// Uncore M-box 0 perfmon counter MSR.
    pub const MSR_M0_PMON_CTR3: Self = Self(0xcb7);

    /// Uncore M-box 0 perfmon event select MSR.
    pub const MSR_M0_PMON_EVNT_SEL4: Self = Self(0xcb8);

    /// Uncore M-box 0 perfmon counter MSR.
    pub const MSR_M0_PMON_CTR4: Self = Self(0xcb9);

    /// Uncore M-box 0 perfmon event select MSR.
    pub const MSR_M0_PMON_EVNT_SEL5: Self = Self(0xcba);

    /// Uncore M-box 0 perfmon counter MSR.
    pub const MSR_M0_PMON_CTR5: Self = Self(0xcbb);

    /// Uncore S-box 1 perfmon local box control MSR.
    pub const MSR_S1_PMON_BOX_CTRL: Self = Self(0xcc0);

    /// Uncore S-box 1 perfmon local box status MSR.
    pub const MSR_S1_PMON_BOX_STATUS: Self = Self(0xcc1);

    /// Uncore S-box 1 perfmon local box overflow control MSR.
    pub const MSR_S1_PMON_BOX_OVF_CTRL: Self = Self(0xcc2);

    /// Uncore S-box 1 perfmon event select MSR.
    pub const MSR_S1_PMON_EVNT_SEL0: Self = Self(0xcd0);

    /// Uncore S-box 1 perfmon counter MSR.
    pub const MSR_S1_PMON_CTR0: Self = Self(0xcd1);

    /// Uncore S-box 1 perfmon event select MSR.
    pub const MSR_S1_PMON_EVNT_SEL1: Self = Self(0xcd2);

    /// Uncore S-box 1 perfmon counter MSR.
    pub const MSR_S1_PMON_CTR1: Self = Self(0xcd3);

    /// Uncore S-box 1 perfmon event select MSR.
    pub const MSR_S1_PMON_EVNT_SEL2: Self = Self(0xcd4);

    /// Uncore S-box 1 perfmon counter MSR.
    pub const MSR_S1_PMON_CTR2: Self = Self(0xcd5);

    /// Uncore S-box 1 perfmon event select MSR.
    pub const MSR_S1_PMON_EVNT_SEL3: Self = Self(0xcd6);

    /// Uncore S-box 1 perfmon counter MSR.
    pub const MSR_S1_PMON_CTR3: Self = Self(0xcd7);

    /// Uncore M-box 1 perfmon local box control MSR.
    pub const MSR_M1_PMON_BOX_CTRL: Self = Self(0xce0);

    /// Uncore M-box 1 perfmon local box status MSR.
    pub const MSR_M1_PMON_BOX_STATUS: Self = Self(0xce1);

    /// Uncore M-box 1 perfmon local box overflow control MSR.
    pub const MSR_M1_PMON_BOX_OVF_CTRL: Self = Self(0xce2);

    /// Uncore M-box 1 perfmon time stamp unit select MSR.
    pub const MSR_M1_PMON_TIMESTAMP: Self = Self(0xce4);

    /// Uncore M-box 1 perfmon DSP unit select MSR.
    pub const MSR_M1_PMON_DSP: Self = Self(0xce5);

    /// Uncore M-box 1 perfmon ISS unit select MSR.
    pub const MSR_M1_PMON_ISS: Self = Self(0xce6);

    /// Uncore M-box 1 perfmon MAP unit select MSR.
    pub const MSR_M1_PMON_MAP: Self = Self(0xce7);

    /// Uncore M-box 1 perfmon MIC THR select MSR.
    pub const MSR_M1_PMON_MSC_THR: Self = Self(0xce8);

    /// Uncore M-box 1 perfmon PGT unit select MSR.
    pub const MSR_M1_PMON_PGT: Self = Self(0xce9);

    /// Uncore M-box 1 perfmon PLD unit select MSR.
    pub const MSR_M1_PMON_PLD: Self = Self(0xcea);

    /// Uncore M-box 1 perfmon ZDP unit select MSR.
    pub const MSR_M1_PMON_ZDP: Self = Self(0xceb);

    /// Uncore M-box 1 perfmon event select MSR.
    pub const MSR_M1_PMON_EVNT_SEL0: Self = Self(0xcf0);

    /// Uncore M-box 1 perfmon counter MSR.
    pub const MSR_M1_PMON_CTR0: Self = Self(0xcf1);

    /// Uncore M-box 1 perfmon event select MSR.
    pub const MSR_M1_PMON_EVNT_SEL1: Self = Self(0xcf2);

    /// Uncore M-box 1 perfmon counter MSR.
    pub const MSR_M1_PMON_CTR1: Self = Self(0xcf3);

    /// Uncore M-box 1 perfmon event select MSR.
    pub const MSR_M1_PMON_EVNT_SEL2: Self = Self(0xcf4);

    /// Uncore M-box 1 perfmon counter MSR.
    pub const MSR_M1_PMON_CTR2: Self = Self(0xcf5);

    /// Uncore M-box 1 perfmon event select MSR.
    pub const MSR_M1_PMON_EVNT_SEL3: Self = Self(0xcf6);

    /// Uncore M-box 1 perfmon counter MSR.
    pub const MSR_M1_PMON_CTR3: Self = Self(0xcf7);

    /// Uncore M-box 1 perfmon event select MSR.
    pub const MSR_M1_PMON_EVNT_SEL4: Self = Self(0xcf8);

    /// Uncore M-box 1 perfmon counter MSR.
    pub const MSR_M1_PMON_CTR4: Self = Self(0xcf9);

    /// Uncore M-box 1 perfmon event select MSR.
    pub const MSR_M1_PMON_EVNT_SEL5: Self = Self(0xcfa);

    /// Uncore M-box 1 perfmon counter MSR.
    pub const MSR_M1_PMON_CTR5: Self = Self(0xcfb);

    /// Uncore C-box 0 perfmon local box control MSR.
    pub const MSR_C0_PMON_BOX_CTRL: Self = Self(0xd00);

    /// Uncore C-box 0 perfmon local box status MSR.
    pub const MSR_C0_PMON_BOX_STATUS: Self = Self(0xd01);

    /// Uncore C-box 0 perfmon local box overflow control MSR.
    pub const MSR_C0_PMON_BOX_OVF_CTRL: Self = Self(0xd02);

    /// Uncore C-box 0 perfmon event select MSR.
    pub const MSR_C0_PMON_EVNT_SEL0: Self = Self(0xd10);

    /// Uncore C-box 0 perfmon counter MSR.
    pub const MSR_C0_PMON_CTR0: Self = Self(0xd11);

    /// Uncore C-box 0 perfmon event select MSR.
    pub const MSR_C0_PMON_EVNT_SEL1: Self = Self(0xd12);

    /// Uncore C-box 0 perfmon counter MSR.
    pub const MSR_C0_PMON_CTR1: Self = Self(0xd13);

    /// Uncore C-box 0 perfmon event select MSR.
    pub const MSR_C0_PMON_EVNT_SEL2: Self = Self(0xd14);

    /// Uncore C-box 0 perfmon counter MSR.
    pub const MSR_C0_PMON_CTR2: Self = Self(0xd15);

    /// Uncore C-box 0 perfmon event select MSR.
    pub const MSR_C0_PMON_EVNT_SEL3: Self = Self(0xd16);

    /// Uncore C-box 0 perfmon counter MSR.
    pub const MSR_C0_PMON_CTR3: Self = Self(0xd17);

    /// Uncore C-box 0 perfmon event select MSR.
    pub const MSR_C0_PMON_EVNT_SEL4: Self = Self(0xd18);

    /// Uncore C-box 0 perfmon counter MSR.
    pub const MSR_C0_PMON_CTR4: Self = Self(0xd19);

    /// Uncore C-box 0 perfmon event select MSR.
    pub const MSR_C0_PMON_EVNT_SEL5: Self = Self(0xd1a);

    /// Uncore C-box 0 perfmon counter MSR.
    pub const MSR_C0_PMON_CTR5: Self = Self(0xd1b);

    /// Uncore C-box 4 perfmon local box control MSR.
    pub const MSR_C4_PMON_BOX_CTRL: Self = Self(0xd20);

    /// Uncore C-box 4 perfmon local box status MSR.
    pub const MSR_C4_PMON_BOX_STATUS: Self = Self(0xd21);

    /// Uncore C-box 4 perfmon local box overflow control MSR.
    pub const MSR_C4_PMON_BOX_OVF_CTRL: Self = Self(0xd22);

    /// Uncore C-box 4 perfmon event select MSR.
    pub const MSR_C4_PMON_EVNT_SEL0: Self = Self(0xd30);

    /// Uncore C-box 4 perfmon counter MSR.
    pub const MSR_C4_PMON_CTR0: Self = Self(0xd31);

    /// Uncore C-box 4 perfmon event select MSR.
    pub const MSR_C4_PMON_EVNT_SEL1: Self = Self(0xd32);

    /// Uncore C-box 4 perfmon counter MSR.
    pub const MSR_C4_PMON_CTR1: Self = Self(0xd33);

    /// Uncore C-box 4 perfmon event select MSR.
    pub const MSR_C4_PMON_EVNT_SEL2: Self = Self(0xd34);

    /// Uncore C-box 4 perfmon counter MSR.
    pub const MSR_C4_PMON_CTR2: Self = Self(0xd35);

    /// Uncore C-box 4 perfmon event select MSR.
    pub const MSR_C4_PMON_EVNT_SEL3: Self = Self(0xd36);

    /// Uncore C-box 4 perfmon counter MSR.
    pub const MSR_C4_PMON_CTR3: Self = Self(0xd37);

    /// Uncore C-box 4 perfmon event select MSR.
    pub const MSR_C4_PMON_EVNT_SEL4: Self = Self(0xd38);

    /// Uncore C-box 4 perfmon counter MSR.
    pub const MSR_C4_PMON_CTR4: Self = Self(0xd39);

    /// Uncore C-box 4 perfmon event select MSR.
    pub const MSR_C4_PMON_EVNT_SEL5: Self = Self(0xd3a);

    /// Uncore C-box 4 perfmon counter MSR.
    pub const MSR_C4_PMON_CTR5: Self = Self(0xd3b);

    /// Uncore C-box 2 perfmon local box control MSR.
    pub const MSR_C2_PMON_BOX_CTRL: Self = Self(0xd40);

    /// Uncore C-box 2 perfmon local box status MSR.
    pub const MSR_C2_PMON_BOX_STATUS: Self = Self(0xd41);

    /// Uncore C-box 2 perfmon local box overflow control MSR.
    pub const MSR_C2_PMON_BOX_OVF_CTRL: Self = Self(0xd42);

    /// Uncore C-box 2 perfmon event select MSR.
    pub const MSR_C2_PMON_EVNT_SEL0: Self = Self(0xd50);

    /// Uncore C-box 2 perfmon counter MSR.
    pub const MSR_C2_PMON_CTR0: Self = Self(0xd51);

    /// Uncore C-box 2 perfmon event select MSR.
    pub const MSR_C2_PMON_EVNT_SEL1: Self = Self(0xd52);

    /// Uncore C-box 2 perfmon counter MSR.
    pub const MSR_C2_PMON_CTR1: Self = Self(0xd53);

    /// Uncore C-box 2 perfmon event select MSR.
    pub const MSR_C2_PMON_EVNT_SEL2: Self = Self(0xd54);

    /// Uncore C-box 2 perfmon counter MSR.
    pub const MSR_C2_PMON_CTR2: Self = Self(0xd55);

    /// Uncore C-box 2 perfmon event select MSR.
    pub const MSR_C2_PMON_EVNT_SEL3: Self = Self(0xd56);

    /// Uncore C-box 2 perfmon counter MSR.
    pub const MSR_C2_PMON_CTR3: Self = Self(0xd57);

    /// Uncore C-box 2 perfmon event select MSR.
    pub const MSR_C2_PMON_EVNT_SEL4: Self = Self(0xd58);

    /// Uncore C-box 2 perfmon counter MSR.
    pub const MSR_C2_PMON_CTR4: Self = Self(0xd59);

    /// Uncore C-box 2 perfmon event select MSR.
    pub const MSR_C2_PMON_EVNT_SEL5: Self = Self(0xd5a);

    /// Uncore C-box 2 perfmon counter MSR.
    pub const MSR_C2_PMON_CTR5: Self = Self(0xd5b);

    /// Uncore C-box 6 perfmon local box control MSR.
    pub const MSR_C6_PMON_BOX_CTRL: Self = Self(0xd60);

    /// Uncore C-box 6 perfmon local box status MSR.
    pub const MSR_C6_PMON_BOX_STATUS: Self = Self(0xd61);

    /// Uncore C-box 6 perfmon local box overflow control MSR.
    pub const MSR_C6_PMON_BOX_OVF_CTRL: Self = Self(0xd62);

    /// Uncore C-box 6 perfmon event select MSR.
    pub const MSR_C6_PMON_EVNT_SEL0: Self = Self(0xd70);

    /// Uncore C-box 6 perfmon counter MSR.
    pub const MSR_C6_PMON_CTR0: Self = Self(0xd71);

    /// Uncore C-box 6 perfmon event select MSR.
    pub const MSR_C6_PMON_EVNT_SEL1: Self = Self(0xd72);

    /// Uncore C-box 6 perfmon counter MSR.
    pub const MSR_C6_PMON_CTR1: Self = Self(0xd73);

    /// Uncore C-box 6 perfmon event select MSR.
    pub const MSR_C6_PMON_EVNT_SEL2: Self = Self(0xd74);

    /// Uncore C-box 6 perfmon counter MSR.
    pub const MSR_C6_PMON_CTR2: Self = Self(0xd75);

    /// Uncore C-box 6 perfmon event select MSR.
    pub const MSR_C6_PMON_EVNT_SEL3: Self = Self(0xd76);

    /// Uncore C-box 6 perfmon counter MSR.
    pub const MSR_C6_PMON_CTR3: Self = Self(0xd77);

    /// Uncore C-box 6 perfmon event select MSR.
    pub const MSR_C6_PMON_EVNT_SEL4: Self = Self(0xd78);

    /// Uncore C-box 6 perfmon counter MSR.
    pub const MSR_C6_PMON_CTR4: Self = Self(0xd79);

    /// Uncore C-box 6 perfmon event select MSR.
    pub const MSR_C6_PMON_EVNT_SEL5: Self = Self(0xd7a);

    /// Uncore C-box 6 perfmon counter MSR.
    pub const MSR_C6_PMON_CTR5: Self = Self(0xd7b);

    /// Uncore C-box 1 perfmon local box control MSR.
    pub const MSR_C1_PMON_BOX_CTRL: Self = Self(0xd80);

    /// Uncore C-box 1 perfmon local box status MSR.
    pub const MSR_C1_PMON_BOX_STATUS: Self = Self(0xd81);

    /// Uncore C-box 1 perfmon local box overflow control MSR.
    pub const MSR_C1_PMON_BOX_OVF_CTRL: Self = Self(0xd82);

    /// Uncore C-box 1 perfmon event select MSR.
    pub const MSR_C1_PMON_EVNT_SEL0: Self = Self(0xd90);

    /// Uncore C-box 1 perfmon counter MSR.
    pub const MSR_C1_PMON_CTR0: Self = Self(0xd91);

    /// Uncore C-box 1 perfmon event select MSR.
    pub const MSR_C1_PMON_EVNT_SEL1: Self = Self(0xd92);

    /// Uncore C-box 1 perfmon counter MSR.
    pub const MSR_C1_PMON_CTR1: Self = Self(0xd93);

    /// Uncore C-box 1 perfmon event select MSR.
    pub const MSR_C1_PMON_EVNT_SEL2: Self = Self(0xd94);

    /// Uncore C-box 1 perfmon counter MSR.
    pub const MSR_C1_PMON_CTR2: Self = Self(0xd95);

    /// Uncore C-box 1 perfmon event select MSR.
    pub const MSR_C1_PMON_EVNT_SEL3: Self = Self(0xd96);

    /// Uncore C-box 1 perfmon counter MSR.
    pub const MSR_C1_PMON_CTR3: Self = Self(0xd97);

    /// Uncore C-box 1 perfmon event select MSR.
    pub const MSR_C1_PMON_EVNT_SEL4: Self = Self(0xd98);

    /// Uncore C-box 1 perfmon counter MSR.
    pub const MSR_C1_PMON_CTR4: Self = Self(0xd99);

    /// Uncore C-box 1 perfmon event select MSR.
    pub const MSR_C1_PMON_EVNT_SEL5: Self = Self(0xd9a);

    /// Uncore C-box 1 perfmon counter MSR.
    pub const MSR_C1_PMON_CTR5: Self = Self(0xd9b);

    /// Uncore C-box 5 perfmon local box control MSR.
    pub const MSR_C5_PMON_BOX_CTRL: Self = Self(0xda0);

    /// Uncore C-box 5 perfmon local box status MSR.
    pub const MSR_C5_PMON_BOX_STATUS: Self = Self(0xda1);

    /// Uncore C-box 5 perfmon local box overflow control MSR.
    pub const MSR_C5_PMON_BOX_OVF_CTRL: Self = Self(0xda2);

    /// Uncore C-box 5 perfmon event select MSR.
    pub const MSR_C5_PMON_EVNT_SEL0: Self = Self(0xdb0);

    /// Uncore C-box 5 perfmon counter MSR.
    pub const MSR_C5_PMON_CTR0: Self = Self(0xdb1);

    /// Uncore C-box 5 perfmon event select MSR.
    pub const MSR_C5_PMON_EVNT_SEL1: Self = Self(0xdb2);

    /// Uncore C-box 5 perfmon counter MSR.
    pub const MSR_C5_PMON_CTR1: Self = Self(0xdb3);

    /// Uncore C-box 5 perfmon event select MSR.
    pub const MSR_C5_PMON_EVNT_SEL2: Self = Self(0xdb4);

    /// Uncore C-box 5 perfmon counter MSR.
    pub const MSR_C5_PMON_CTR2: Self = Self(0xdb5);

    /// Uncore C-box 5 perfmon event select MSR.
    pub const MSR_C5_PMON_EVNT_SEL3: Self = Self(0xdb6);

    /// Uncore C-box 5 perfmon counter MSR.
    pub const MSR_C5_PMON_CTR3: Self = Self(0xdb7);

    /// Uncore C-box 5 perfmon event select MSR.
    pub const MSR_C5_PMON_EVNT_SEL4: Self = Self(0xdb8);

    /// Uncore C-box 5 perfmon counter MSR.
    pub const MSR_C5_PMON_CTR4: Self = Self(0xdb9);

    /// Uncore C-box 5 perfmon event select MSR.
    pub const MSR_C5_PMON_EVNT_SEL5: Self = Self(0xdba);

    /// Uncore C-box 5 perfmon counter MSR.
    pub const MSR_C5_PMON_CTR5: Self = Self(0xdbb);

    /// Uncore C-box 3 perfmon local box control MSR.
    pub const MSR_C3_PMON_BOX_CTRL: Self = Self(0xdc0);

    /// Uncore C-box 3 perfmon local box status MSR.
    pub const MSR_C3_PMON_BOX_STATUS: Self = Self(0xdc1);

    /// Uncore C-box 3 perfmon local box overflow control MSR.
    pub const MSR_C3_PMON_BOX_OVF_CTRL: Self = Self(0xdc2);

    /// Uncore C-box 3 perfmon event select MSR.
    pub const MSR_C3_PMON_EVNT_SEL0: Self = Self(0xdd0);

    /// Uncore C-box 3 perfmon counter MSR.
    pub const MSR_C3_PMON_CTR0: Self = Self(0xdd1);

    /// Uncore C-box 3 perfmon event select MSR.
    pub const MSR_C3_PMON_EVNT_SEL1: Self = Self(0xdd2);

    /// Uncore C-box 3 perfmon counter MSR.
    pub const MSR_C3_PMON_CTR1: Self = Self(0xdd3);

    /// Uncore C-box 3 perfmon event select MSR.
    pub const MSR_C3_PMON_EVNT_SEL2: Self = Self(0xdd4);

    /// Uncore C-box 3 perfmon counter MSR.
    pub const MSR_C3_PMON_CTR2: Self = Self(0xdd5);

    /// Uncore C-box 3 perfmon event select MSR.
    pub const MSR_C3_PMON_EVNT_SEL3: Self = Self(0xdd6);

    /// Uncore C-box 3 perfmon counter MSR.
    pub const MSR_C3_PMON_CTR3: Self = Self(0xdd7);

    /// Uncore C-box 3 perfmon event select MSR.
    pub const MSR_C3_PMON_EVNT_SEL4: Self = Self(0xdd8);

    /// Uncore C-box 3 perfmon counter MSR.
    pub const MSR_C3_PMON_CTR4: Self = Self(0xdd9);

    /// Uncore C-box 3 perfmon event select MSR.
    pub const MSR_C3_PMON_EVNT_SEL5: Self = Self(0xdda);

    /// Uncore C-box 3 perfmon counter MSR.
    pub const MSR_C3_PMON_CTR5: Self = Self(0xddb);

    /// Uncore C-box 7 perfmon local box control MSR.
    pub const MSR_C7_PMON_BOX_CTRL: Self = Self(0xde0);

    /// Uncore C-box 7 perfmon local box status MSR.
    pub const MSR_C7_PMON_BOX_STATUS: Self = Self(0xde1);

    /// Uncore C-box 7 perfmon local box overflow control MSR.
    pub const MSR_C7_PMON_BOX_OVF_CTRL: Self = Self(0xde2);

    /// Uncore C-box 7 perfmon event select MSR.
    pub const MSR_C7_PMON_EVNT_SEL0: Self = Self(0xdf0);

    /// Uncore C-box 7 perfmon counter MSR.
    pub const MSR_C7_PMON_CTR0: Self = Self(0xdf1);

    /// Uncore C-box 7 perfmon event select MSR.
    pub const MSR_C7_PMON_EVNT_SEL1: Self = Self(0xdf2);

    /// Uncore C-box 7 perfmon counter MSR.
    pub const MSR_C7_PMON_CTR1: Self = Self(0xdf3);

    /// Uncore C-box 7 perfmon event select MSR.
    pub const MSR_C7_PMON_EVNT_SEL2: Self = Self(0xdf4);

    /// Uncore C-box 7 perfmon counter MSR.
    pub const MSR_C7_PMON_CTR2: Self = Self(0xdf5);

    /// Uncore C-box 7 perfmon event select MSR.
    pub const MSR_C7_PMON_EVNT_SEL3: Self = Self(0xdf6);

    /// Uncore C-box 7 perfmon counter MSR.
    pub const MSR_C7_PMON_CTR3: Self = Self(0xdf7);

    /// Uncore C-box 7 perfmon event select MSR.
    pub const MSR_C7_PMON_EVNT_SEL4: Self = Self(0xdf8);

    /// Uncore C-box 7 perfmon counter MSR.
    pub const MSR_C7_PMON_CTR4: Self = Self(0xdf9);

    /// Uncore C-box 7 perfmon event select MSR.
    pub const MSR_C7_PMON_EVNT_SEL5: Self = Self(0xdfa);

    /// Uncore C-box 7 perfmon counter MSR.
    pub const MSR_C7_PMON_CTR5: Self = Self(0xdfb);

    /// Uncore R-box 0 perfmon local box control MSR.
    pub const MSR_R0_PMON_BOX_CTRL: Self = Self(0xe00);

    /// Uncore R-box 0 perfmon local box status MSR.
    pub const MSR_R0_PMON_BOX_STATUS: Self = Self(0xe01);

    /// Uncore R-box 0 perfmon local box overflow control MSR.
    pub const MSR_R0_PMON_BOX_OVF_CTRL: Self = Self(0xe02);

    /// Uncore R-box 0 perfmon IPERF0 unit Port 0 select MSR.
    pub const MSR_R0_PMON_IPERF0_P0: Self = Self(0xe04);

    /// Uncore R-box 0 perfmon IPERF0 unit Port 1 select MSR.
    pub const MSR_R0_PMON_IPERF0_P1: Self = Self(0xe05);

    /// Uncore R-box 0 perfmon IPERF0 unit Port 2 select MSR.
    pub const MSR_R0_PMON_IPERF0_P2: Self = Self(0xe06);

    /// Uncore R-box 0 perfmon IPERF0 unit Port 3 select MSR.
    pub const MSR_R0_PMON_IPERF0_P3: Self = Self(0xe07);

    /// Uncore R-box 0 perfmon IPERF0 unit Port 4 select MSR.
    pub const MSR_R0_PMON_IPERF0_P4: Self = Self(0xe08);

    /// Uncore R-box 0 perfmon IPERF0 unit Port 5 select MSR.
    pub const MSR_R0_PMON_IPERF0_P5: Self = Self(0xe09);

    /// Uncore R-box 0 perfmon IPERF0 unit Port 6 select MSR.
    pub const MSR_R0_PMON_IPERF0_P6: Self = Self(0xe0a);

    /// Uncore R-box 0 perfmon IPERF0 unit Port 7 select MSR.
    pub const MSR_R0_PMON_IPERF0_P7: Self = Self(0xe0b);

    /// Uncore R-box 0 perfmon QLX unit Port 0 select MSR.
    pub const MSR_R0_PMON_QLX_P0: Self = Self(0xe0c);

    /// Uncore R-box 0 perfmon QLX unit Port 1 select MSR.
    pub const MSR_R0_PMON_QLX_P1: Self = Self(0xe0d);

    /// Uncore R-box 0 perfmon QLX unit Port 2 select MSR.
    pub const MSR_R0_PMON_QLX_P2: Self = Self(0xe0e);

    /// Uncore R-box 0 perfmon QLX unit Port 3 select MSR.
    pub const MSR_R0_PMON_QLX_P3: Self = Self(0xe0f);

    /// Uncore R-box 0 perfmon event select MSR.
    pub const MSR_R0_PMON_EVNT_SEL0: Self = Self(0xe10);

    /// Uncore R-box 0 perfmon counter MSR.
    pub const MSR_R0_PMON_CTR0: Self = Self(0xe11);

    /// Uncore R-box 0 perfmon event select MSR.
    pub const MSR_R0_PMON_EVNT_SEL1: Self = Self(0xe12);

    /// Uncore R-box 0 perfmon counter MSR.
    pub const MSR_R0_PMON_CTR1: Self = Self(0xe13);

    /// Uncore R-box 0 perfmon event select MSR.
    pub const MSR_R0_PMON_EVNT_SEL2: Self = Self(0xe14);

    /// Uncore R-box 0 perfmon counter MSR.
    pub const MSR_R0_PMON_CTR2: Self = Self(0xe15);

    /// Uncore R-box 0 perfmon event select MSR.
    pub const MSR_R0_PMON_EVNT_SEL3: Self = Self(0xe16);

    /// Uncore R-box 0 perfmon counter MSR.
    pub const MSR_R0_PMON_CTR3: Self = Self(0xe17);

    /// Uncore R-box 0 perfmon event select MSR.
    pub const MSR_R0_PMON_EVNT_SEL4: Self = Self(0xe18);

    /// Uncore R-box 0 perfmon counter MSR.
    pub const MSR_R0_PMON_CTR4: Self = Self(0xe19);

    /// Uncore R-box 0 perfmon event select MSR.
    pub const MSR_R0_PMON_EVNT_SEL5: Self = Self(0xe1a);

    /// Uncore R-box 0 perfmon counter MSR.
    pub const MSR_R0_PMON_CTR5: Self = Self(0xe1b);

    /// Uncore R-box 0 perfmon event select MSR.
    pub const MSR_R0_PMON_EVNT_SEL6: Self = Self(0xe1c);

    /// Uncore R-box 0 perfmon counter MSR.
    pub const MSR_R0_PMON_CTR6: Self = Self(0xe1d);

    /// Uncore R-box 0 perfmon event select MSR.
    pub const MSR_R0_PMON_EVNT_SEL7: Self = Self(0xe1e);

    /// Uncore R-box 0 perfmon counter MSR.
    pub const MSR_R0_PMON_CTR7: Self = Self(0xe1f);

    /// Uncore R-box 1 perfmon local box control MSR.
    pub const MSR_R1_PMON_BOX_CTRL: Self = Self(0xe20);

    /// Uncore R-box 1 perfmon local box status MSR.
    pub const MSR_R1_PMON_BOX_STATUS: Self = Self(0xe21);

    /// Uncore R-box 1 perfmon local box overflow control MSR.
    pub const MSR_R1_PMON_BOX_OVF_CTRL: Self = Self(0xe22);

    /// Uncore R-box 1 perfmon IPERF1 unit Port 8 select MSR.
    pub const MSR_R1_PMON_IPERF1_P8: Self = Self(0xe24);

    /// Uncore R-box 1 perfmon IPERF1 unit Port 9 select MSR.
    pub const MSR_R1_PMON_IPERF1_P9: Self = Self(0xe25);

    /// Uncore R-box 1 perfmon IPERF1 unit Port 10 select MSR.
    pub const MSR_R1_PMON_IPERF1_P10: Self = Self(0xe26);

    /// Uncore R-box 1 perfmon IPERF1 unit Port 11 select MSR.
    pub const MSR_R1_PMON_IPERF1_P11: Self = Self(0xe27);

    /// Uncore R-box 1 perfmon IPERF1 unit Port 12 select MSR.
    pub const MSR_R1_PMON_IPERF1_P12: Self = Self(0xe28);

    /// Uncore R-box 1 perfmon IPERF1 unit Port 13 select MSR.
    pub const MSR_R1_PMON_IPERF1_P13: Self = Self(0xe29);

    /// Uncore R-box 1 perfmon IPERF1 unit Port 14 select MSR.
    pub const MSR_R1_PMON_IPERF1_P14: Self = Self(0xe2a);

    /// Uncore R-box 1 perfmon IPERF1 unit Port 15 select MSR.
    pub const MSR_R1_PMON_IPERF1_P15: Self = Self(0xe2b);

    /// Uncore R-box 1 perfmon QLX unit Port 4 select MSR.
    pub const MSR_R1_PMON_QLX_P4: Self = Self(0xe2c);

    /// Uncore R-box 1 perfmon QLX unit Port 5 select MSR.
    pub const MSR_R1_PMON_QLX_P5: Self = Self(0xe2d);

    /// Uncore R-box 1 perfmon QLX unit Port 6 select MSR.
    pub const MSR_R1_PMON_QLX_P6: Self = Self(0xe2e);

    /// Uncore R-box 1 perfmon QLX unit Port 7 select MSR.
    pub const MSR_R1_PMON_QLX_P7: Self = Self(0xe2f);

    /// Uncore R-box 1 perfmon event select MSR.
    pub const MSR_R1_PMON_EVNT_SEL8: Self = Self(0xe30);

    /// Uncore R-box 1 perfmon counter MSR.
    pub const MSR_R1_PMON_CTR8: Self = Self(0xe31);

    /// Uncore R-box 1 perfmon event select MSR.
    pub const MSR_R1_PMON_EVNT_SEL9: Self = Self(0xe32);

    /// Uncore R-box 1 perfmon counter MSR.
    pub const MSR_R1_PMON_CTR9: Self = Self(0xe33);

    /// Uncore R-box 1 perfmon event select MSR.
    pub const MSR_R1_PMON_EVNT_SEL10: Self = Self(0xe34);

    /// Uncore R-box 1 perfmon counter MSR.
    pub const MSR_R1_PMON_CTR10: Self = Self(0xe35);

    /// Uncore R-box 1 perfmon event select MSR.
    pub const MSR_R1_PMON_EVNT_SEL11: Self = Self(0xe36);

    /// Uncore R-box 1 perfmon counter MSR.
    pub const MSR_R1_PMON_CTR11: Self = Self(0xe37);

    /// Uncore R-box 1 perfmon event select MSR.
    pub const MSR_R1_PMON_EVNT_SEL12: Self = Self(0xe38);

    /// Uncore R-box 1 perfmon counter MSR.
    pub const MSR_R1_PMON_CTR12: Self = Self(0xe39);

    /// Uncore R-box 1 perfmon event select MSR.
    pub const MSR_R1_PMON_EVNT_SEL13: Self = Self(0xe3a);

    /// Uncore R-box 1perfmon counter MSR.
    pub const MSR_R1_PMON_CTR13: Self = Self(0xe3b);

    /// Uncore R-box 1 perfmon event select MSR.
    pub const MSR_R1_PMON_EVNT_SEL14: Self = Self(0xe3c);

    /// Uncore R-box 1 perfmon counter MSR.
    pub const MSR_R1_PMON_CTR14: Self = Self(0xe3d);

    /// Uncore R-box 1 perfmon event select MSR.
    pub const MSR_R1_PMON_EVNT_SEL15: Self = Self(0xe3e);

    /// Uncore R-box 1 perfmon counter MSR.
    pub const MSR_R1_PMON_CTR15: Self = Self(0xe3f);

    /// Uncore B-box 0 perfmon local box match MSR.
    pub const MSR_B0_PMON_MATCH: Self = Self(0xe45);

    /// Uncore B-box 0 perfmon local box mask MSR.
    pub const MSR_B0_PMON_MASK: Self = Self(0xe46);

    /// Uncore S-box 0 perfmon local box match MSR.
    pub const MSR_S0_PMON_MATCH: Self = Self(0xe49);

    /// Uncore S-box 0 perfmon local box mask MSR.
    pub const MSR_S0_PMON_MASK: Self = Self(0xe4a);

    /// Uncore B-box 1 perfmon local box match MSR.
    pub const MSR_B1_PMON_MATCH: Self = Self(0xe4d);

    /// Uncore B-box 1 perfmon local box mask MSR.
    pub const MSR_B1_PMON_MASK: Self = Self(0xe4e);

    /// Uncore M-box 0 perfmon local box address match/mask config MSR.
    pub const MSR_M0_PMON_MM_CONFIG: Self = Self(0xe54);

    /// Uncore M-box 0 perfmon local box address match MSR.
    pub const MSR_M0_PMON_ADDR_MATCH: Self = Self(0xe55);

    /// Uncore M-box 0 perfmon local box address mask MSR.
    pub const MSR_M0_PMON_ADDR_MASK: Self = Self(0xe56);

    /// Uncore S-box 1 perfmon local box match MSR.
    pub const MSR_S1_PMON_MATCH: Self = Self(0xe59);

    /// Uncore S-box 1 perfmon local box mask MSR.
    pub const MSR_S1_PMON_MASK: Self = Self(0xe5a);

    /// Uncore M-box 1 perfmon local box address match/mask config MSR.
    pub const MSR_M1_PMON_MM_CONFIG: Self = Self(0xe5c);

    /// Uncore M-box 1 perfmon local box address match MSR.
    pub const MSR_M1_PMON_ADDR_MATCH: Self = Self(0xe5d);

    /// Uncore M-box 1 perfmon local box address mask MSR.
    pub const MSR_M1_PMON_ADDR_MASK: Self = Self(0xe5e);

    /// Uncore C-box 8 perfmon local box control MSR.
    pub const MSR_C8_PMON_BOX_CTRL: Self = Self(0xf40);

    /// Uncore C-box 8 perfmon local box status MSR.
    pub const MSR_C8_PMON_BOX_STATUS: Self = Self(0xf41);

    /// Uncore C-box 8 perfmon local box overflow control MSR.
    pub const MSR_C8_PMON_BOX_OVF_CTRL: Self = Self(0xf42);

    /// Uncore C-box 8 perfmon event select MSR.
    pub const MSR_C8_PMON_EVNT_SEL0: Self = Self(0xf50);

    /// Uncore C-box 8 perfmon counter MSR.
    pub const MSR_C8_PMON_CTR0: Self = Self(0xf51);

    /// Uncore C-box 8 perfmon event select MSR.
    pub const MSR_C8_PMON_EVNT_SEL1: Self = Self(0xf52);

    /// Uncore C-box 8 perfmon counter MSR.
    pub const MSR_C8_PMON_CTR1: Self = Self(0xf53);

    /// Uncore C-box 8 perfmon event select MSR.
    pub const MSR_C8_PMON_EVNT_SEL2: Self = Self(0xf54);

    /// Uncore C-box 8 perfmon counter MSR.
    pub const MSR_C8_PMON_CTR2: Self = Self(0xf55);

    /// Uncore C-box 8 perfmon event select MSR.
    pub const MSR_C8_PMON_EVNT_SEL3: Self = Self(0xf56);

    /// Uncore C-box 8 perfmon counter MSR.
    pub const MSR_C8_PMON_CTR3: Self = Self(0xf57);

    /// Uncore C-box 8 perfmon event select MSR.
    pub const MSR_C8_PMON_EVNT_SEL4: Self = Self(0xf58);

    /// Uncore C-box 8 perfmon counter MSR.
    pub const MSR_C8_PMON_CTR4: Self = Self(0xf59);

    /// Uncore C-box 8 perfmon event select MSR.
    pub const MSR_C8_PMON_EVNT_SEL5: Self = Self(0xf5a);

    /// Uncore C-box 8 perfmon counter MSR.
    pub const MSR_C8_PMON_CTR5: Self = Self(0xf5b);

    /// Uncore C-box 9 perfmon local box control MSR.
    pub const MSR_C9_PMON_BOX_CTRL: Self = Self(0xfc0);

    /// Uncore C-box 9 perfmon local box status MSR.
    pub const MSR_C9_PMON_BOX_STATUS: Self = Self(0xfc1);

    /// Uncore C-box 9 perfmon local box overflow control MSR.
    pub const MSR_C9_PMON_BOX_OVF_CTRL: Self = Self(0xfc2);

    /// Uncore C-box 9 perfmon event select MSR.
    pub const MSR_C9_PMON_EVNT_SEL0: Self = Self(0xfd0);

    /// Uncore C-box 9 perfmon counter MSR.
    pub const MSR_C9_PMON_CTR0: Self = Self(0xfd1);

    /// Uncore C-box 9 perfmon event select MSR.
    pub const MSR_C9_PMON_EVNT_SEL1: Self = Self(0xfd2);

    /// Uncore C-box 9 perfmon counter MSR.
    pub const MSR_C9_PMON_CTR1: Self = Self(0xfd3);

    /// Uncore C-box 9 perfmon event select MSR.
    pub const MSR_C9_PMON_EVNT_SEL2: Self = Self(0xfd4);

    /// Uncore C-box 9 perfmon counter MSR.
    pub const MSR_C9_PMON_CTR2: Self = Self(0xfd5);

    /// Uncore C-box 9 perfmon event select MSR.
    pub const MSR_C9_PMON_EVNT_SEL3: Self = Self(0xfd6);

    /// Uncore C-box 9 perfmon counter MSR.
    pub const MSR_C9_PMON_CTR3: Self = Self(0xfd7);

    /// Uncore C-box 9 perfmon event select MSR.
    pub const MSR_C9_PMON_EVNT_SEL4: Self = Self(0xfd8);

    /// Uncore C-box 9 perfmon counter MSR.
    pub const MSR_C9_PMON_CTR4: Self = Self(0xfd9);

    /// Uncore C-box 9 perfmon event select MSR.
    pub const MSR_C9_PMON_EVNT_SEL5: Self = Self(0xfda);

    /// Uncore C-box 9 perfmon counter MSR.
    pub const MSR_C9_PMON_CTR5: Self = Self(0xfdb);

    /// GBUSQ Event Control and Counter  Register (R/W) See Section 18.17, Performance  Monitoring on 64-bit Intel Xeon Processor MP with Up to 8-MByte L3 Cache.
    pub const MSR_EMON_L3_CTR_CTL0: Self = Self(0x107cc);

    /// IFSB BUSQ Event Control and Counter  Register (R/W) See Section 18.17, Performance  Monitoring on 64-bit Intel Xeon Processor  MP with Up to 8-MByte L3 Cache.
    pub const MSR_IFSB_BUSQ0: Self = Self(0x107cc);

    /// GBUSQ Event Control/Counter Register (R/W) Apply to Intel Xeon processor 7400 series (processor signature  06_1D) only. See Section 17.2.2
    pub const MSR_EMON_L3_CTR_CTL1: Self = Self(0x107cd);

    /// IFSB BUSQ Event Control and Counter Register (R/W)
    pub const MSR_IFSB_BUSQ1: Self = Self(0x107cd);

    /// GSNPQ Event Control and Counter  Register (R/W)  See Section 18.17, Performance Monitoring on 64-bit Intel Xeon Processor MP with Up to 8-MByte L3 Cache.
    pub const MSR_EMON_L3_CTR_CTL2: Self = Self(0x107ce);

    /// IFSB SNPQ Event Control and Counter  Register (R/W)  See Section 18.17, Performance  Monitoring on 64-bit Intel Xeon Processor  MP with Up to 8-MByte L3 Cache.
    pub const MSR_IFSB_SNPQ0: Self = Self(0x107ce);

    /// GSNPQ Event Control/Counter Register (R/W) Apply to Intel Xeon processor 7400 series (processor signature  06_1D) only. See Section 17.2.2
    pub const MSR_EMON_L3_CTR_CTL3: Self = Self(0x107cf);

    /// IFSB SNPQ Event Control and Counter  Register (R/W)
    pub const MSR_IFSB_SNPQ1: Self = Self(0x107cf);

    /// EFSB DRDY Event Control and Counter Register (R/W)  See Section 18.17, Performance  Monitoring on 64-bit Intel Xeon Processor MP with Up to 8-MByte L3 Cache  for  details.
    pub const MSR_EFSB_DRDY0: Self = Self(0x107d0);

    /// FSB Event Control and Counter Register (R/W)  See Section 18.17, Performance  Monitoring on 64-bit Intel Xeon Processor MP with Up to 8-MByte L3 Cache  for  details.
    pub const MSR_EMON_L3_CTR_CTL4: Self = Self(0x107d0);

    /// EFSB DRDY Event Control and Counter  Register (R/W)
    pub const MSR_EFSB_DRDY1: Self = Self(0x107d1);

    /// FSB Event Control/Counter Register (R/W) Apply to Intel Xeon processor 7400 series (processor signature  06_1D) only. See Section 17.2.2
    pub const MSR_EMON_L3_CTR_CTL5: Self = Self(0x107d1);

    /// FSB Event Control/Counter Register (R/W) Apply to Intel Xeon processor 7400 series (processor signature  06_1D) only. See Section 17.2.2
    pub const MSR_EMON_L3_CTR_CTL6: Self = Self(0x107d2);

    /// IFSB Latency Event Control Register  (R/W) See Section 18.17, Performance  Monitoring on 64-bit Intel Xeon Processor MP with Up to 8-MByte L3 Cache  for  details.
    pub const MSR_IFSB_CTL6: Self = Self(0x107d2);

    /// FSB Event Control/Counter Register (R/W) Apply to Intel Xeon processor 7400 series (processor signature  06_1D) only. See Section 17.2.2
    pub const MSR_EMON_L3_CTR_CTL7: Self = Self(0x107d3);

    /// IFSB Latency Event Counter Register  (R/W)  See Section 18.17, Performance  Monitoring on 64-bit Intel Xeon Processor  MP with Up to 8-MByte L3 Cache.
    pub const MSR_IFSB_CNTR7: Self = Self(0x107d3);

    /// L3/FSB Common Control Register (R/W) Apply to Intel Xeon processor 7400 series (processor signature  06_1D) only. See Section 17.2.2
    pub const MSR_EMON_L3_GL_CTL: Self = Self(0x107d8);

    /// If (  CPUID.80000001.EDX.\[bit  20\] or  CPUID.80000001.EDX.\[bit 29\])
    pub const IA32_EFER: Self = Self(0xc0000080);

    /// System Call Target Address (R/W)  See Table 35-2.
    pub const IA32_STAR: Self = Self(0xc0000081);

    /// IA-32e Mode System Call Target Address (R/W)  See Table 35-2.
    pub const IA32_LSTAR: Self = Self(0xc0000082);

    /// System Call Target Address the compatibility mode.
    pub const IA32_CSTAR: Self = Self(0xc0000083);

    /// System Call Flag Mask (R/W)  See Table 35-2.
    pub const IA32_FMASK: Self = Self(0xc0000084);

    /// Map of BASE Address of FS (R/W)  See Table 35-2.
    pub const IA32_FS_BASE: Self = Self(0xc0000100);

    /// Map of BASE Address of GS (R/W)  See Table 35-2.
    pub const IA32_GS_BASE: Self = Self(0xc0000101);

    /// Swap Target of BASE Address of GS (R/W) See Table 35-2.
    pub const IA32_KERNEL_GSBASE: Self = Self(0xc0000102);

    /// AUXILIARY TSC Signature. (R/W) See Table 35-2 and Section  17.13.2, IA32_TSC_AUX Register and RDTSCP Support.
    pub const IA32_TSC_AUX: Self = Self(0xc0000103);
}

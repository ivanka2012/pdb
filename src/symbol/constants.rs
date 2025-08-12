// Copyright 2017 pdb Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// A list of known symbol kinds.
// from:
//  https://github.com/Microsoft/microsoft-pdb/blob/082c5290e5aff028ae84e43affa8be717aa7af73/include/cvinfo.h#L2735

#![allow(unused, non_upper_case_globals, non_camel_case_types, missing_docs)]

use std::fmt;

use scroll::{ctx::TryFromCtx, Endian};

/// Compile flags symbol
pub const S_COMPILE: u16 = 0x0001;

/// Register variable
pub const S_REGISTER_16t: u16 = 0x0002;

/// constant symbol
pub const S_CONSTANT_16t: u16 = 0x0003;

/// User defined type
pub const S_UDT_16t: u16 = 0x0004;

/// Start Search
pub const S_SSEARCH: u16 = 0x0005;

/// Block procedure "with" or thunk end
pub const S_END: u16 = 0x0006;

/// Reserve symbol space in $$Symbols table
pub const S_SKIP: u16 = 0x0007;

/// Reserved symbol for CV internal use
pub const S_CVRESERVE: u16 = 0x0008;

/// path to object file name
pub const S_OBJNAME_ST: u16 = 0x0009;

/// end of argument/return list
pub const S_ENDARG: u16 = 0x000a;

/// special UDT for cobol that does not symbol pack
pub const S_COBOLUDT_16t: u16 = 0x000b;

/// multiple register variable
pub const S_MANYREG_16t: u16 = 0x000c;

/// return description symbol
pub const S_RETURN: u16 = 0x000d;

/// description of this pointer on entry
pub const S_ENTRYTHIS: u16 = 0x000e;


/// BP-relative
pub const S_BPREL16: u16 = 0x0100;

/// Module-local symbol
pub const S_LDATA16: u16 = 0x0101;

/// Global data symbol
pub const S_GDATA16: u16 = 0x0102;

/// a public symbol
pub const S_PUB16: u16 = 0x0103;

/// Local procedure start
pub const S_LPROC16: u16 = 0x0104;

/// Global procedure start
pub const S_GPROC16: u16 = 0x0105;

/// Thunk Start
pub const S_THUNK16: u16 = 0x0106;

/// block start
pub const S_BLOCK16: u16 = 0x0107;

/// with start
pub const S_WITH16: u16 = 0x0108;

/// code label
pub const S_LABEL16: u16 = 0x0109;

/// change execution model
pub const S_CEXMODEL16: u16 = 0x010a;

/// address of virtual function table
pub const S_VFTABLE16: u16 = 0x010b;

/// register relative address
pub const S_REGREL16: u16 = 0x010c;


/// BP-relative
pub const S_BPREL32_16t: u16 = 0x0200;

/// Module-local symbol
pub const S_LDATA32_16t: u16 = 0x0201;

/// Global data symbol
pub const S_GDATA32_16t: u16 = 0x0202;

/// a public symbol (CV internal reserved)
pub const S_PUB32_16t: u16 = 0x0203;

/// Local procedure start
pub const S_LPROC32_16t: u16 = 0x0204;

/// Global procedure start
pub const S_GPROC32_16t: u16 = 0x0205;

/// Thunk Start
pub const S_THUNK32_ST: u16 = 0x0206;

/// block start
pub const S_BLOCK32_ST: u16 = 0x0207;

/// with start
pub const S_WITH32_ST: u16 = 0x0208;

/// code label
pub const S_LABEL32_ST: u16 = 0x0209;

/// change execution model
pub const S_CEXMODEL32: u16 = 0x020a;

/// address of virtual function table
pub const S_VFTABLE32_16t: u16 = 0x020b;

/// register relative address
pub const S_REGREL32_16t: u16 = 0x020c;

/// local thread storage
pub const S_LTHREAD32_16t: u16 = 0x020d;

/// global thread storage
pub const S_GTHREAD32_16t: u16 = 0x020e;

/// static link for MIPS EH implementation
pub const S_SLINK32: u16 = 0x020f;


/// Local procedure start
pub const S_LPROCMIPS_16t: u16 = 0x0300;

/// Global procedure start
pub const S_GPROCMIPS_16t: u16 = 0x0301;


// if these ref symbols have names following then the names are in ST format
/// Reference to a procedure
pub const S_PROCREF_ST: u16 = 0x0400;

/// Reference to data
pub const S_DATAREF_ST: u16 = 0x0401;

/// Used for page alignment of symbols
pub const S_ALIGN: u16 = 0x0402;


/// Local Reference to a procedure
pub const S_LPROCREF_ST: u16 = 0x0403;

/// OEM defined symbol
pub const S_OEM: u16 = 0x0404;


/// sym records with 32-bit types embedded instead of 16-bit
/// all have 0x1000 bit set for easy identification
/// only do the 32-bit target versions since we don't really
/// care about 16-bit ones anymore.
pub const S_TI16_MAX: u16 = 0x1000;

/// Register variable
pub const S_REGISTER_ST: u16 = 0x1001;

/// constant symbol
pub const S_CONSTANT_ST: u16 = 0x1002;

/// User defined type
pub const S_UDT_ST: u16 = 0x1003;

/// special UDT for cobol that does not symbol pack
pub const S_COBOLUDT_ST: u16 = 0x1004;

/// multiple register variable
pub const S_MANYREG_ST: u16 = 0x1005;

/// BP-relative
pub const S_BPREL32_ST: u16 = 0x1006;

/// Module-local symbol
pub const S_LDATA32_ST: u16 = 0x1007;

/// Global data symbol
pub const S_GDATA32_ST: u16 = 0x1008;

/// a public symbol (CV internal reserved)
pub const S_PUB32_ST: u16 = 0x1009;

/// Local procedure start
pub const S_LPROC32_ST: u16 = 0x100a;

/// Global procedure start
pub const S_GPROC32_ST: u16 = 0x100b;

/// address of virtual function table
pub const S_VFTABLE32: u16 = 0x100c;

/// register relative address
pub const S_REGREL32_ST: u16 = 0x100d;

/// local thread storage
pub const S_LTHREAD32_ST: u16 = 0x100e;

/// global thread storage
pub const S_GTHREAD32_ST: u16 = 0x100f;


/// Local procedure start
pub const S_LPROCMIPS_ST: u16 = 0x1010;

/// Global procedure start
pub const S_GPROCMIPS_ST: u16 = 0x1011;


/// extra frame and proc information
pub const S_FRAMEPROC: u16 = 0x1012;

/// extended compile flags and info
pub const S_COMPILE2_ST: u16 = 0x1013;


// new symbols necessary for 16-bit enumerates of IA64 registers
// and IA64 specific symbols

/// multiple register variable
pub const S_MANYREG2_ST: u16 = 0x1014;

/// Local procedure start (IA64)
pub const S_LPROCIA64_ST: u16 = 0x1015;

/// Global procedure start (IA64)
pub const S_GPROCIA64_ST: u16 = 0x1016;


// Local symbols for IL
/// local IL sym with field for local slot index
pub const S_LOCALSLOT_ST: u16 = 0x1017;

/// local IL sym with field for parameter slot index
pub const S_PARAMSLOT_ST: u16 = 0x1018;


/// Annotation string literals
pub const S_ANNOTATION: u16 = 0x1019;


// symbols to support managed code debugging
/// Global proc
pub const S_GMANPROC_ST: u16 = 0x101a;

/// Local proc
pub const S_LMANPROC_ST: u16 = 0x101b;

/// reserved
pub const S_RESERVED1: u16 = 0x101c;

/// reserved
pub const S_RESERVED2: u16 = 0x101d;

/// reserved
pub const S_RESERVED3: u16 = 0x101e;

/// reserved
pub const S_RESERVED4: u16 = 0x101f;

pub const S_LMANDATA_ST: u16 = 0x1020;
pub const S_GMANDATA_ST: u16 = 0x1021;
pub const S_MANFRAMEREL_ST: u16 = 0x1022;
pub const S_MANREGISTER_ST: u16 = 0x1023;
pub const S_MANSLOT_ST: u16 = 0x1024;
pub const S_MANMANYREG_ST: u16 = 0x1025;
pub const S_MANREGREL_ST: u16 = 0x1026;
pub const S_MANMANYREG2_ST: u16 = 0x1027;
/// Index for type referenced by name from metadata
pub const S_MANTYPREF: u16 = 0x1028;

/// Using namespace
pub const S_UNAMESPACE_ST: u16 = 0x1029;


// Symbols w/ SZ name fields. All name fields contain utf8 encoded strings.
/// starting point for SZ name symbols
pub const S_ST_MAX: u16 = 0x1100;


/// path to object file name
pub const S_OBJNAME: u16 = 0x1101;

/// Thunk Start
pub const S_THUNK32: u16 = 0x1102;

/// block start
pub const S_BLOCK32: u16 = 0x1103;

/// with start
pub const S_WITH32: u16 = 0x1104;

/// code label
pub const S_LABEL32: u16 = 0x1105;

/// Register variable
pub const S_REGISTER: u16 = 0x1106;

/// constant symbol
pub const S_CONSTANT: u16 = 0x1107;

/// User defined type
pub const S_UDT: u16 = 0x1108;

/// special UDT for cobol that does not symbol pack
pub const S_COBOLUDT: u16 = 0x1109;

/// multiple register variable
pub const S_MANYREG: u16 = 0x110a;

/// BP-relative
pub const S_BPREL32: u16 = 0x110b;

/// Module-local symbol
pub const S_LDATA32: u16 = 0x110c;

/// Global data symbol
pub const S_GDATA32: u16 = 0x110d;

/// a public symbol (CV internal reserved)
pub const S_PUB32: u16 = 0x110e;

/// Local procedure start
pub const S_LPROC32: u16 = 0x110f;

/// Global procedure start
pub const S_GPROC32: u16 = 0x1110;

/// register relative address
pub const S_REGREL32: u16 = 0x1111;

/// local thread storage
pub const S_LTHREAD32: u16 = 0x1112;

/// global thread storage
pub const S_GTHREAD32: u16 = 0x1113;


/// Local procedure start
pub const S_LPROCMIPS: u16 = 0x1114;

/// Global procedure start
pub const S_GPROCMIPS: u16 = 0x1115;

/// extended compile flags and info
pub const S_COMPILE2: u16 = 0x1116;

/// multiple register variable
pub const S_MANYREG2: u16 = 0x1117;

/// Local procedure start (IA64)
pub const S_LPROCIA64: u16 = 0x1118;

/// Global procedure start (IA64)
pub const S_GPROCIA64: u16 = 0x1119;

/// local IL sym with field for local slot index
pub const S_LOCALSLOT: u16 = 0x111a;

/// local IL sym with field for parameter slot index
pub const S_PARAMSLOT: u16 = 0x111b;


// symbols to support managed code debugging
pub const S_LMANDATA: u16 = 0x111c;
pub const S_GMANDATA: u16 = 0x111d;
pub const S_MANFRAMEREL: u16 = 0x111e;
pub const S_MANREGISTER: u16 = 0x111f;
pub const S_MANSLOT: u16 = 0x1120;
pub const S_MANMANYREG: u16 = 0x1121;
pub const S_MANREGREL: u16 = 0x1122;
pub const S_MANMANYREG2: u16 = 0x1123;
/// Using namespace
pub const S_UNAMESPACE: u16 = 0x1124;


// ref symbols with name fields
/// Reference to a procedure
pub const S_PROCREF: u16 = 0x1125;

/// Reference to data
pub const S_DATAREF: u16 = 0x1126;

/// Local Reference to a procedure
pub const S_LPROCREF: u16 = 0x1127;

/// Reference to an S_ANNOTATION symbol
pub const S_ANNOTATIONREF: u16 = 0x1128;

/// Reference to one of the many MANPROCSYM's
pub const S_TOKENREF: u16 = 0x1129;


// continuation of managed symbols
/// Global proc
pub const S_GMANPROC: u16 = 0x112a;

/// Local proc
pub const S_LMANPROC: u16 = 0x112b;


// short light-weight thunks
/// trampoline thunks
pub const S_TRAMPOLINE: u16 = 0x112c;

/// constants with metadata type info
pub const S_MANCONSTANT: u16 = 0x112d;


// native attributed local/parms
/// relative to virtual frame ptr
pub const S_ATTR_FRAMEREL: u16 = 0x112e;

/// stored in a register
pub const S_ATTR_REGISTER: u16 = 0x112f;

/// relative to register (alternate frame ptr)
pub const S_ATTR_REGREL: u16 = 0x1130;

/// stored in >1 register
pub const S_ATTR_MANYREG: u16 = 0x1131;


// Separated code (from the compiler) support
pub const S_SEPCODE: u16 = 0x1132;

/// defines a local symbol in optimized code
pub const S_LOCAL_2005: u16 = 0x1133;

/// defines a single range of addresses in which symbol can be evaluated
pub const S_DEFRANGE_2005: u16 = 0x1134;

/// defines ranges of addresses in which symbol can be evaluated
pub const S_DEFRANGE2_2005: u16 = 0x1135;


/// A COFF section in a PE executable
pub const S_SECTION: u16 = 0x1136;

/// A COFF group
pub const S_COFFGROUP: u16 = 0x1137;

/// A export
pub const S_EXPORT: u16 = 0x1138;


/// Indirect call site information
pub const S_CALLSITEINFO: u16 = 0x1139;

/// Security cookie information
pub const S_FRAMECOOKIE: u16 = 0x113a;


/// Discarded by LINK /OPT:REF (experimental see richards)
pub const S_DISCARDED: u16 = 0x113b;


/// Replacement for S_COMPILE2
pub const S_COMPILE3: u16 = 0x113c;

/// Environment block split off from S_COMPILE2
pub const S_ENVBLOCK: u16 = 0x113d;


/// defines a local symbol in optimized code
pub const S_LOCAL: u16 = 0x113e;

/// defines a single range of addresses in which symbol can be evaluated
pub const S_DEFRANGE: u16 = 0x113f;

/// ranges for a subfield
pub const S_DEFRANGE_SUBFIELD: u16 = 0x1140;


/// ranges for en-registered symbol
pub const S_DEFRANGE_REGISTER: u16 = 0x1141;

/// range for stack symbol.
pub const S_DEFRANGE_FRAMEPOINTER_REL: u16 = 0x1142;

/// ranges for en-registered field of symbol
pub const S_DEFRANGE_SUBFIELD_REGISTER: u16 = 0x1143;

/// range for stack symbol span valid full scope of function body gap might apply.
pub const S_DEFRANGE_FRAMEPOINTER_REL_FULL_SCOPE: u16 = 0x1144;

/// range for symbol address as register + offset.
pub const S_DEFRANGE_REGISTER_REL: u16 = 0x1145;


// S_PROC symbols that reference ID instead of type
pub const S_LPROC32_ID: u16 = 0x1146;
pub const S_GPROC32_ID: u16 = 0x1147;
pub const S_LPROCMIPS_ID: u16 = 0x1148;
pub const S_GPROCMIPS_ID: u16 = 0x1149;
pub const S_LPROCIA64_ID: u16 = 0x114a;
pub const S_GPROCIA64_ID: u16 = 0x114b;

/// build information.
pub const S_BUILDINFO: u16 = 0x114c;

/// inlined function callsite.
pub const S_INLINESITE: u16 = 0x114d;

pub const S_INLINESITE_END: u16 = 0x114e;
pub const S_PROC_ID_END: u16 = 0x114f;

pub const S_DEFRANGE_HLSL: u16 = 0x1150;
pub const S_GDATA_HLSL: u16 = 0x1151;
pub const S_LDATA_HLSL: u16 = 0x1152;

pub const S_FILESTATIC: u16 = 0x1153;

/// DPC groupshared variable
pub const S_LOCAL_DPC_GROUPSHARED: u16 = 0x1154;

/// DPC local procedure start
pub const S_LPROC32_DPC: u16 = 0x1155;

pub const S_LPROC32_DPC_ID: u16 = 0x1156;
/// DPC pointer tag definition range
pub const S_DEFRANGE_DPC_PTR_TAG: u16 = 0x1157;

/// DPC pointer tag value to symbol record map
pub const S_DPC_SYM_TAG_MAP: u16 = 0x1158;


pub const S_ARMSWITCHTABLE: u16 = 0x1159;
pub const S_CALLEES: u16 = 0x115a;
pub const S_CALLERS: u16 = 0x115b;
pub const S_POGODATA: u16 = 0x115c;
/// extended inline site information
pub const S_INLINESITE2: u16 = 0x115d;


/// heap allocation site
pub const S_HEAPALLOCSITE: u16 = 0x115e;


/// only generated at link time
pub const S_MOD_TYPEREF: u16 = 0x115f;


/// only generated at link time for mini PDB
pub const S_REF_MINIPDB: u16 = 0x1160;

/// only generated at link time for mini PDB
pub const S_PDBMAP: u16 = 0x1161;


pub const S_GDATA_HLSL32: u16 = 0x1162;
pub const S_LDATA_HLSL32: u16 = 0x1163;

pub const S_GDATA_HLSL32_EX: u16 = 0x1164;
pub const S_LDATA_HLSL32_EX: u16 = 0x1165;

/// generated at link time for /DEBUG:FASTLINK
pub const S_FASTLINK: u16 = 0x1167;

pub const S_INLINEES: u16 = 0x1168;

/// These values correspond to the CV_CPU_TYPE_e enumeration, and are documented
/// [on MSDN](https://msdn.microsoft.com/en-us/library/b2fc64ek.aspx).
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CPUType {
    Intel8080 = 0x0,
    Intel8086 = 0x1,
    Intel80286 = 0x2,
    Intel80386 = 0x3,
    Intel80486 = 0x4,
    Pentium = 0x5,
    PentiumPro = 0x6,
    Pentium3 = 0x7,
    MIPS = 0x10,
    MIPS16 = 0x11,
    MIPS32 = 0x12,
    MIPS64 = 0x13,
    MIPSI = 0x14,
    MIPSII = 0x15,
    MIPSIII = 0x16,
    MIPSIV = 0x17,
    MIPSV = 0x18,
    M68000 = 0x20,
    M68010 = 0x21,
    M68020 = 0x22,
    M68030 = 0x23,
    M68040 = 0x24,
    Alpha = 0x30,
    Alpha21164 = 0x31,
    Alpha21164A = 0x32,
    Alpha21264 = 0x33,
    Alpha21364 = 0x34,
    PPC601 = 0x40,
    PPC603 = 0x41,
    PPC604 = 0x42,
    PPC620 = 0x43,
    PPCFP = 0x44,
    PPCBE = 0x45,
    SH3 = 0x50,
    SH3E = 0x51,
    SH3DSP = 0x52,
    SH4 = 0x53,
    SHMedia = 0x54,
    ARM3 = 0x60,
    ARM4 = 0x61,
    ARM4T = 0x62,
    ARM5 = 0x63,
    ARM5T = 0x64,
    ARM6 = 0x65,
    ARM_XMAC = 0x66,
    ARM_WMMX = 0x67,
    ARM7 = 0x68,
    ARM64 = 0x69,
    Omni = 0x70,
    Ia64 = 0x80,
    Ia64_2 = 0x81,
    CEE = 0x90,
    AM33 = 0xa0,
    M32R = 0xb0,
    TriCore = 0xc0,
    X64 = 0xd0,
    EBC = 0xe0,
    Thumb = 0xf0,
    ARMNT = 0xf4,
    D3D11_Shader = 0x100,
}

impl fmt::Display for CPUType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Intel8080 => write!(f, "Intel8080"),
            Self::Intel8086 => write!(f, "Intel8086"),
            Self::Intel80286 => write!(f, "Intel80286"),
            Self::Intel80386 => write!(f, "Intel80386"),
            Self::Intel80486 => write!(f, "Intel80486"),
            Self::Pentium => write!(f, "Pentium"),
            Self::PentiumPro => write!(f, "PentiumPro"),
            Self::Pentium3 => write!(f, "Pentium3"),
            Self::MIPS => write!(f, "MIPS"),
            Self::MIPS16 => write!(f, "MIPS16"),
            Self::MIPS32 => write!(f, "MIPS32"),
            Self::MIPS64 => write!(f, "MIPS64"),
            Self::MIPSI => write!(f, "MIPSI"),
            Self::MIPSII => write!(f, "MIPSII"),
            Self::MIPSIII => write!(f, "MIPSIII"),
            Self::MIPSIV => write!(f, "MIPSIV"),
            Self::MIPSV => write!(f, "MIPSV"),
            Self::M68000 => write!(f, "M68000"),
            Self::M68010 => write!(f, "M68010"),
            Self::M68020 => write!(f, "M68020"),
            Self::M68030 => write!(f, "M68030"),
            Self::M68040 => write!(f, "M68040"),
            Self::Alpha => write!(f, "Alpha"),
            Self::Alpha21164 => write!(f, "Alpha21164"),
            Self::Alpha21164A => write!(f, "Alpha21164A"),
            Self::Alpha21264 => write!(f, "Alpha21264"),
            Self::Alpha21364 => write!(f, "Alpha21364"),
            Self::PPC601 => write!(f, "PPC601"),
            Self::PPC603 => write!(f, "PPC603"),
            Self::PPC604 => write!(f, "PPC604"),
            Self::PPC620 => write!(f, "PPC620"),
            Self::PPCFP => write!(f, "PPCFP"),
            Self::PPCBE => write!(f, "PPCBE"),
            Self::SH3 => write!(f, "SH3"),
            Self::SH3E => write!(f, "SH3E"),
            Self::SH3DSP => write!(f, "SH3DSP"),
            Self::SH4 => write!(f, "SH4"),
            Self::SHMedia => write!(f, "SHMedia"),
            Self::ARM3 => write!(f, "ARM3"),
            Self::ARM4 => write!(f, "ARM4"),
            Self::ARM4T => write!(f, "ARM4T"),
            Self::ARM5 => write!(f, "ARM5"),
            Self::ARM5T => write!(f, "ARM5T"),
            Self::ARM6 => write!(f, "ARM6"),
            Self::ARM_XMAC => write!(f, "ARM_XMAC"),
            Self::ARM_WMMX => write!(f, "ARM_WMMX"),
            Self::ARM7 => write!(f, "ARM7"),
            Self::ARM64 => write!(f, "ARM64"),
            Self::Omni => write!(f, "Omni"),
            Self::Ia64 => write!(f, "Ia64"),
            Self::Ia64_2 => write!(f, "Ia64_2"),
            Self::CEE => write!(f, "CEE"),
            Self::AM33 => write!(f, "AM33"),
            Self::M32R => write!(f, "M32R"),
            Self::TriCore => write!(f, "TriCore"),
            Self::X64 => write!(f, "X64"),
            Self::EBC => write!(f, "EBC"),
            Self::Thumb => write!(f, "Thumb"),
            Self::ARMNT => write!(f, "ARMNT"),
            Self::D3D11_Shader => write!(f, "D3D11_Shader"),
        }
    }
}

impl From<u16> for CPUType {
    fn from(value: u16) -> Self {
        match value {
            0x0 => Self::Intel8080,
            0x1 => Self::Intel8086,
            0x2 => Self::Intel80286,
            0x3 => Self::Intel80386,
            0x4 => Self::Intel80486,
            0x5 => Self::Pentium,
            0x6 => Self::PentiumPro,
            0x7 => Self::Pentium3,
            0x10 => Self::MIPS,
            0x11 => Self::MIPS16,
            0x12 => Self::MIPS32,
            0x13 => Self::MIPS64,
            0x14 => Self::MIPSI,
            0x15 => Self::MIPSII,
            0x16 => Self::MIPSIII,
            0x17 => Self::MIPSIV,
            0x18 => Self::MIPSV,
            0x20 => Self::M68000,
            0x21 => Self::M68010,
            0x22 => Self::M68020,
            0x23 => Self::M68030,
            0x24 => Self::M68040,
            0x30 => Self::Alpha,
            0x31 => Self::Alpha21164,
            0x32 => Self::Alpha21164A,
            0x33 => Self::Alpha21264,
            0x34 => Self::Alpha21364,
            0x40 => Self::PPC601,
            0x41 => Self::PPC603,
            0x42 => Self::PPC604,
            0x43 => Self::PPC620,
            0x44 => Self::PPCFP,
            0x45 => Self::PPCBE,
            0x50 => Self::SH3,
            0x51 => Self::SH3E,
            0x52 => Self::SH3DSP,
            0x53 => Self::SH4,
            0x54 => Self::SHMedia,
            0x60 => Self::ARM3,
            0x61 => Self::ARM4,
            0x62 => Self::ARM4T,
            0x63 => Self::ARM5,
            0x64 => Self::ARM5T,
            0x65 => Self::ARM6,
            0x66 => Self::ARM_XMAC,
            0x67 => Self::ARM_WMMX,
            0x68 => Self::ARM7,
            0x69 => Self::ARM64,
            0x70 => Self::Omni,
            0x80 => Self::Ia64,
            0x81 => Self::Ia64_2,
            0x90 => Self::CEE,
            0xa0 => Self::AM33,
            0xb0 => Self::M32R,
            0xc0 => Self::TriCore,
            0xd0 => Self::X64,
            0xe0 => Self::EBC,
            0xf0 => Self::Thumb,
            0xf4 => Self::ARMNT,
            0x100 => Self::D3D11_Shader,
            _ => Self::Intel8080, // This enum doesn't have an unknown value, so we just force it to Intel8080 since it's 0x0.
        }
    }
}

impl<'a> TryFromCtx<'a, Endian> for CPUType {
    type Error = scroll::Error;

    fn try_from_ctx(this: &'a [u8], le: Endian) -> scroll::Result<(Self, usize)> {
        u16::try_from_ctx(this, le).map(|(v, l)| (v.into(), l))
    }
}

/// These values correspond to the CV_CFL_LANG enumeration, and are documented
/// [on MSDN](https://msdn.microsoft.com/en-us/library/bw3aekw6.aspx).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceLanguage {
    /// Application language is C.
    C = 0x00,
    /// Application language is C++.
    Cpp = 0x01,
    /// Application language is FORTRAN.
    Fortran = 0x02,
    /// Application language is Microsoft Macro Assembler.
    Masm = 0x03,
    /// Application language is Pascal.
    Pascal = 0x04,
    /// Application language is BASIC.
    Basic = 0x05,
    /// Application language is COBOL.
    Cobol = 0x06,
    /// Application is a linker-generated module.
    Link = 0x07,
    /// Application is a resource module converted with CVTRES tool.
    Cvtres = 0x08,
    /// Application is a POGO optimized module generated with CVTPGD tool.
    Cvtpgd = 0x09,
    /// Application language is C#.
    CSharp = 0x0a,
    /// Application language is Visual Basic.
    VB = 0x0b,
    /// Application language is intermediate language assembly (that is, Common Language Runtime
    /// (CLR) assembly).
    ILAsm = 0x0c,
    /// Application language is Java.
    Java = 0x0d,
    /// Application language is Jscript.
    JScript = 0x0e,
    /// Application language is an unknown Microsoft Intermediate Language (MSIL), possibly a result
    /// of using the [/LTCG (Link-time Code
    /// Generation)](https://docs.microsoft.com/en-us/cpp/build/reference/ltcg-link-time-code-generation)
    /// switch.
    MSIL = 0x0f,
    /// Application language is High Level Shader Language.
    HLSL = 0x10,

    /// The DMD compiler emits 'D' for the CV source language. Microsoft doesn't
    /// have an enumerator for it yet.
    D = 0x44,
}

impl fmt::Display for SourceLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::C => write!(f, "C"),
            Self::Cpp => write!(f, "Cpp"),
            Self::Fortran => write!(f, "Fortran"),
            Self::Masm => write!(f, "Masm"),
            Self::Pascal => write!(f, "Pascal"),
            Self::Basic => write!(f, "Basic"),
            Self::Cobol => write!(f, "Cobol"),
            Self::Link => write!(f, "Link"),
            Self::Cvtres => write!(f, "Cvtres"),
            Self::Cvtpgd => write!(f, "Cvtpgd"),
            Self::CSharp => write!(f, "CSharp"),
            Self::VB => write!(f, "VB"),
            Self::ILAsm => write!(f, "ILAsm"),
            Self::Java => write!(f, "Java"),
            Self::JScript => write!(f, "JScript"),
            Self::MSIL => write!(f, "MSIL"),
            Self::HLSL => write!(f, "HLSL"),
            Self::D => write!(f, "D"),
        }
    }
}

impl From<u8> for SourceLanguage {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::C,
            0x01 => Self::Cpp,
            0x02 => Self::Fortran,
            0x03 => Self::Masm,
            0x04 => Self::Pascal,
            0x05 => Self::Basic,
            0x06 => Self::Cobol,
            0x07 => Self::Link,
            0x08 => Self::Cvtres,
            0x09 => Self::Cvtpgd,
            0x0a => Self::CSharp,
            0x0b => Self::VB,
            0x0c => Self::ILAsm,
            0x0d => Self::Java,
            0x0e => Self::JScript,
            0x0f => Self::MSIL,
            0x10 => Self::HLSL,
            0x44 => Self::D,
            _ => Self::Masm, // There is no unknown, so we just force to Masm as the default.
        }
    }
}

impl<'a> TryFromCtx<'a, Endian> for SourceLanguage {
    type Error = scroll::Error;

    fn try_from_ctx(this: &'a [u8], le: Endian) -> scroll::Result<(Self, usize)> {
        u8::try_from_ctx(this, le).map(|(v, l)| (v.into(), l))
    }
}

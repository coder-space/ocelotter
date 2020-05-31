// AALOAD 0x32
// AASTORE 0x53
pub const ACONST_NULL: u8 = 0x01;
pub const ALOAD: u8 = 0x19;
pub const ALOAD_0: u8 = 0x2a;
pub const ALOAD_1: u8 = 0x2b;
pub const ALOAD_2: u8 = 0x2c;
pub const ALOAD_3: u8 = 0x2d;
// ANEWARRAY 0xbd
pub const ARETURN: u8 = 0xb0;
// ARRAYLENGTH 0xbe
pub const ASTORE: u8 = 0x53;
pub const ASTORE_0: u8 = 0x4b;
pub const ASTORE_1: u8 = 0x4c;
pub const ASTORE_2: u8 = 0x4d;
pub const ASTORE_3: u8 = 0x4e;
// ATHROW 0xbf
// BALOAD 0x33
// BASTORE 0x54
pub const BIPUSH: u8 = 0x10;
pub const BREAKPOINT: u8 = 0xca;
// CALOAD 0x34
// CASTORE 0x55
// CHECKCAST 0xc0
// D2F 0x90
// D2I 0x8e
// D2L 0x8f
pub const DADD: u8 = 0x63;
// DALOAD 0x31
// DASTORE 0x52
pub const DCMPG: u8 = 0x98;
pub const DCMPL: u8 = 0x97;
pub const DCONST_0: u8 = 0x0e;
pub const DCONST_1: u8 = 0x0f;
pub const DDIV: u8 = 0x6f;
pub const DLOAD: u8 = 0x18;
pub const DLOAD_0: u8 = 0x26;
pub const DLOAD_1: u8 = 0x27;
pub const DLOAD_2: u8 = 0x28;
pub const DLOAD_3: u8 = 0x29;
pub const DMUL: u8 = 0x6b;
pub const DNEG: u8 = 0x77;
// DREM 0x73
pub const DRETURN: u8 = 0xaf;
pub const DSTORE: u8 = 0x39;
pub const DSTORE_0: u8 = 0x47;
pub const DSTORE_1: u8 = 0x48;
pub const DSTORE_2: u8 = 0x49;
pub const DSTORE_3: u8 = 0x4a;
pub const DSUB: u8 = 0x67;
pub const DUP: u8 = 0x59;
pub const DUP_X1: u8 = 0x5a;
// DUP_X2 0x5b
//pub const DUP2: u8 = 0x5c;
// DUP2_X1 0x5d
// DUP2_X2 0x5e
//
// All the F Opcodes...
//
pub const GETFIELD: u8 = 0xb4;
pub const GETSTATIC: u8 = 0xb2;
pub const GOTO: u8 = 0xa7;
pub const GOTO_W: u8 = 0xc8;
// I2B 0x91
// I2C 0x92
pub const I2D: u8 = 0x87;
// I2F 0x86
// I2L 0x85
// I2S 0x93
pub const IADD: u8 = 0x60;
pub const IALOAD: u8 = 0x2e;
pub const IAND: u8 = 0x7e;
pub const IASTORE: u8 = 0x4f;
pub const ICONST_M1: u8 = 0x02;
pub const ICONST_0: u8 = 0x03;
pub const ICONST_1: u8 = 0x04;
pub const ICONST_2: u8 = 0x05;
pub const ICONST_3: u8 = 0x06;
pub const ICONST_4: u8 = 0x07;
pub const ICONST_5: u8 = 0x08;
pub const IDIV: u8 = 0x6c;
//
// Many IF variants...
//
pub const IF_ICMPEQ: u8 = 0x9f;
pub const IF_ICMPGT: u8 = 0xa3;
pub const IF_ICMPLT: u8 = 0xa1;
pub const IF_ICMPNE: u8 = 0xa0;
pub const IFEQ: u8 = 0x99;
pub const IFGE: u8 = 0x9c;
pub const IFGT: u8 = 0x9d;
pub const IFLE: u8 = 0x9e;
pub const IFLT: u8 = 0x9b;
pub const IFNE: u8 = 0x9a;
pub const IFNONNULL: u8 = 0xc7;
pub const IFNULL: u8 = 0xc6;
pub const IINC: u8 = 0x84;
pub const ILOAD: u8 = 0x15;
pub const ILOAD_0: u8 = 0x1a;
pub const ILOAD_1: u8 = 0x1b;
pub const ILOAD_2: u8 = 0x1c;
pub const ILOAD_3: u8 = 0x1d;
pub const IMPDEP1: u8 = 0xfe;
pub const IMPDEP2: u8 = 0xff;
pub const IMUL: u8 = 0x68;
pub const INEG: u8 = 0x74;
// INSTANCEOF 0xc1
// INVOKEDYNAMIC 0xba
// INVOKEINTERFACE 0xb9
pub const INVOKESPECIAL: u8 = 0xb7;
pub const INVOKESTATIC: u8 = 0xb8;
pub const INVOKEVIRTUAL: u8 = 0xb6;
pub const IOR: u8 = 0x80;
pub const IREM: u8 = 0x70;
pub const IRETURN: u8 = 0xac;
pub const ISHL: u8 = 0x78;
pub const ISHR: u8 = 0x7a;
pub const ISTORE: u8 = 0x36;
pub const ISTORE_0: u8 = 0x3b;
pub const ISTORE_1: u8 = 0x3c;
pub const ISTORE_2: u8 = 0x3d;
pub const ISTORE_3: u8 = 0x3e;
pub const ISUB: u8 = 0x64;
// IUSHR 0x7c
pub const IXOR: u8 = 0x82;
pub const JSR: u8 = 0xa8;
pub const JSR_W: u8 = 0xc9;
// L2D 0x8a
// L2F 0x89
pub const L2I: u8 = 0x88;
//
// L opcodes
//
pub const LDC: u8 = 0x12;
pub const LDC2_W: u8 = 0x14;
//
// More L opcodes
//
// LOOKUPSWITCH 0xab
pub const MONITORENTER: u8 = 0xc2;
pub const MONITOREXIT: u8 = 0xc3;
// MULTINEWARRAY 0xc5
pub const NEW: u8 = 0xbb;
pub const NEWARRAY: u8 = 0xbc;
pub const NOP: u8 = 0x00;
pub const POP: u8 = 0x57;
pub const POP2: u8 = 0x58;
pub const PUTFIELD: u8 = 0xb5;
pub const PUTSTATIC: u8 = 0xb3;
pub const RET: u8 = 0xa9;
pub const RETURN: u8 = 0xb1;
// SALOAD 0x35
// SASTORE 0x56
pub const SIPUSH: u8 = 0x11;
pub const SWAP: u8 = 0x5f;
// TABLESWITCH 0xaa
// WIDE 0xc4

// [UNUSED] 0cb - 0xfd

fn num_params(c: u8) -> u8 {
    match c {
        ALOAD => 1,
        ASTORE => 1,
        BIPUSH => 1,
        DLOAD => 1,
        DSTORE => 1,
        GETFIELD => 2,
        GETSTATIC => 2,
        GOTO => 2,
        IF_ICMPEQ => 2,
        IFEQ => 2,
        IFGE => 2,
        IFGT => 2,
        IFLE => 2,
        IFLT => 2,
        IFNE => 2,
        IFNONNULL => 2,
        IFNULL => 2,
        IINC => 2,
        ILOAD => 1,
        INVOKESPECIAL => 2,
        INVOKESTATIC => 2,
        INVOKEVIRTUAL => 2,
        ISTORE => 1,
        NEW => 2,
        NEWARRAY => 1,
        JSR => 2,
        JSR_W => 2,
        LDC => 1,
        PUTFIELD => 2,
        PUTSTATIC => 2,
        RET => 1,
        SIPUSH => 2,
        _ => 0,
    }
}

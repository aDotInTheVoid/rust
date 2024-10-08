//@ min-lldb-version: 310

//@ compile-flags:-g

// === GDB TESTS ===================================================================================

// gdbg-command:print/d 'simple_tuple::NO_PADDING_8'
// gdbr-command:print simple_tuple::NO_PADDING_8
// gdbg-check:$1 = {__0 = -50, __1 = 50}
// gdbr-check:$1 = (-50, 50)
// gdbg-command:print 'simple_tuple::NO_PADDING_16'
// gdbr-command:print simple_tuple::NO_PADDING_16
// gdbg-check:$2 = {__0 = -1, __1 = 2, __2 = 3}
// gdbr-check:$2 = (-1, 2, 3)
// gdbg-command:print 'simple_tuple::NO_PADDING_32'
// gdbr-command:print simple_tuple::NO_PADDING_32
// gdbg-check:$3 = {__0 = 4, __1 = 5, __2 = 6}
// gdbr-check:$3 = (4, 5, 6)
// gdbg-command:print 'simple_tuple::NO_PADDING_64'
// gdbr-command:print simple_tuple::NO_PADDING_64
// gdbg-check:$4 = {__0 = 7, __1 = 8, __2 = 9}
// gdbr-check:$4 = (7, 8, 9)

// gdbg-command:print 'simple_tuple::INTERNAL_PADDING_1'
// gdbr-command:print simple_tuple::INTERNAL_PADDING_1
// gdbg-check:$5 = {__0 = 10, __1 = 11}
// gdbr-check:$5 = (10, 11)
// gdbg-command:print 'simple_tuple::INTERNAL_PADDING_2'
// gdbr-command:print simple_tuple::INTERNAL_PADDING_2
// gdbg-check:$6 = {__0 = 12, __1 = 13, __2 = 14, __3 = 15}
// gdbr-check:$6 = (12, 13, 14, 15)

// gdbg-command:print 'simple_tuple::PADDING_AT_END'
// gdbr-command:print simple_tuple::PADDING_AT_END
// gdbg-check:$7 = {__0 = 16, __1 = 17}
// gdbr-check:$7 = (16, 17)

// gdb-command:run

// gdbg-command:print/d noPadding8
// gdbr-command:print noPadding8
// gdbg-check:$8 = {__0 = -100, __1 = 100}
// gdbr-check:$8 = (-100, 100)
// gdb-command:print noPadding16
// gdbg-check:$9 = {__0 = 0, __1 = 1, __2 = 2}
// gdbr-check:$9 = (0, 1, 2)
// gdb-command:print noPadding32
// gdbg-check:$10 = {__0 = 3, __1 = 4.5, __2 = 5}
// gdbr-check:$10 = (3, 4.5, 5)
// gdb-command:print noPadding64
// gdbg-check:$11 = {__0 = 6, __1 = 7.5, __2 = 8}
// gdbr-check:$11 = (6, 7.5, 8)

// gdb-command:print internalPadding1
// gdbg-check:$12 = {__0 = 9, __1 = 10}
// gdbr-check:$12 = (9, 10)
// gdb-command:print internalPadding2
// gdbg-check:$13 = {__0 = 11, __1 = 12, __2 = 13, __3 = 14}
// gdbr-check:$13 = (11, 12, 13, 14)

// gdb-command:print paddingAtEnd
// gdbg-check:$14 = {__0 = 15, __1 = 16}
// gdbr-check:$14 = (15, 16)

// gdbg-command:print/d 'simple_tuple::NO_PADDING_8'
// gdbr-command:print simple_tuple::NO_PADDING_8
// gdbg-check:$15 = {__0 = -127, __1 = 127}
// gdbr-check:$15 = (-127, 127)
// gdbg-command:print 'simple_tuple::NO_PADDING_16'
// gdbr-command:print simple_tuple::NO_PADDING_16
// gdbg-check:$16 = {__0 = -10, __1 = 10, __2 = 9}
// gdbr-check:$16 = (-10, 10, 9)
// gdbg-command:print 'simple_tuple::NO_PADDING_32'
// gdbr-command:print simple_tuple::NO_PADDING_32
// gdbg-check:$17 = {__0 = 14, __1 = 15, __2 = 16}
// gdbr-check:$17 = (14, 15, 16)
// gdbg-command:print 'simple_tuple::NO_PADDING_64'
// gdbr-command:print simple_tuple::NO_PADDING_64
// gdbg-check:$18 = {__0 = 17, __1 = 18, __2 = 19}
// gdbr-check:$18 = (17, 18, 19)

// gdbg-command:print 'simple_tuple::INTERNAL_PADDING_1'
// gdbr-command:print simple_tuple::INTERNAL_PADDING_1
// gdbg-check:$19 = {__0 = 110, __1 = 111}
// gdbr-check:$19 = (110, 111)
// gdbg-command:print 'simple_tuple::INTERNAL_PADDING_2'
// gdbr-command:print simple_tuple::INTERNAL_PADDING_2
// gdbg-check:$20 = {__0 = 112, __1 = 113, __2 = 114, __3 = 115}
// gdbr-check:$20 = (112, 113, 114, 115)

// gdbg-command:print 'simple_tuple::PADDING_AT_END'
// gdbr-command:print simple_tuple::PADDING_AT_END
// gdbg-check:$21 = {__0 = 116, __1 = 117}
// gdbr-check:$21 = (116, 117)


// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:v/d noPadding8
// lldbg-check:[...] { 0 = -100 1 = 100 }
// lldbr-check:((i8, u8)) noPadding8 = { 0 = -100 1 = 100 }
// lldb-command:v noPadding16
// lldbg-check:[...] { 0 = 0 1 = 1 2 = 2 }
// lldbr-check:((i16, i16, u16)) noPadding16 = { 0 = 0 1 = 1 2 = 2 }
// lldb-command:v noPadding32
// lldbg-check:[...] { 0 = 3 1 = 4.5 2 = 5 }
// lldbr-check:((i32, f32, u32)) noPadding32 = { 0 = 3 1 = 4.5 2 = 5 }
// lldb-command:v noPadding64
// lldbg-check:[...] { 0 = 6 1 = 7.5 2 = 8 }
// lldbr-check:((i64, f64, u64)) noPadding64 = { 0 = 6 1 = 7.5 2 = 8 }

// lldb-command:v internalPadding1
// lldbg-check:[...] { 0 = 9 1 = 10 }
// lldbr-check:((i16, i32)) internalPadding1 = { 0 = 9 1 = 10 }
// lldb-command:v internalPadding2
// lldbg-check:[...] { 0 = 11 1 = 12 2 = 13 3 = 14 }
// lldbr-check:((i16, i32, u32, u64)) internalPadding2 = { 0 = 11 1 = 12 2 = 13 3 = 14 }

// lldb-command:v paddingAtEnd
// lldbg-check:[...] { 0 = 15 1 = 16 }
// lldbr-check:((i32, i16)) paddingAtEnd = { 0 = 15 1 = 16 }


// === CDB TESTS ==================================================================================

// cdb-command: g

// cdb-command:dx noPadding8,d
// cdb-check:noPadding8,d [...]: (-100, 100) [Type: tuple$<i8,u8>]
// cdb-check:[...][0]              : -100 [Type: [...]]
// cdb-check:[...][1]              : 100 [Type: [...]]
// cdb-command:dx noPadding16,d
// cdb-check:noPadding16,d [...]: (0, 1, 2) [Type: tuple$<i16,i16,u16>]
// cdb-check:[...][0]              : 0 [Type: [...]]
// cdb-check:[...][1]              : 1 [Type: [...]]
// cdb-check:[...][2]              : 2 [Type: [...]]
// cdb-command:dx noPadding32,d
// cdb-check:noPadding32,d [...]: (3, 4.5[...], 5) [Type: tuple$<i32,f32,u32>]
// cdb-check:[...][0]              : 3 [Type: [...]]
// cdb-check:[...][1]              : 4.5[...] [Type: [...]]
// cdb-check:[...][2]              : 5 [Type: [...]]
// cdb-command:dx noPadding64,d
// cdb-check:noPadding64,d [...]: (6, 7.5[...], 8) [Type: tuple$<i64,f64,u64>]
// cdb-check:[...][0]              : 6 [Type: [...]]
// cdb-check:[...][1]              : 7.500000 [Type: [...]]
// cdb-check:[...][2]              : 8 [Type: [...]]

// cdb-command:dx internalPadding1,d
// cdb-check:internalPadding1,d [...]: (9, 10) [Type: tuple$<i16,i32>]
// cdb-check:[...][0]              : 9 [Type: short]
// cdb-check:[...][1]              : 10 [Type: int]
// cdb-command:dx internalPadding2,d
// cdb-check:internalPadding2,d [...]: (11, 12, 13, 14) [Type: tuple$<i16,i32,u32,u64>]
// cdb-check:[...][0]              : 11 [Type: [...]]
// cdb-check:[...][1]              : 12 [Type: [...]]
// cdb-check:[...][2]              : 13 [Type: [...]]
// cdb-check:[...][3]              : 14 [Type: [...]]

// cdb-command:dx paddingAtEnd,d
// cdb-check:paddingAtEnd,d [...]: (15, 16) [Type: tuple$<i32,i16>]
// cdb-check:[...][0]              : 15 [Type: [...]]
// cdb-check:[...][1]              : 16 [Type: [...]]


#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

static mut NO_PADDING_8: (i8, u8) = (-50, 50);
static mut NO_PADDING_16: (i16, i16, u16) = (-1, 2, 3);

static mut NO_PADDING_32: (i32, f32, u32) = (4, 5.0, 6);
static mut NO_PADDING_64: (i64, f64, u64) = (7, 8.0, 9);

static mut INTERNAL_PADDING_1: (i16, i32) = (10, 11);
static mut INTERNAL_PADDING_2: (i16, i32, u32, u64) = (12, 13, 14, 15);

static mut PADDING_AT_END: (i32, i16) = (16, 17);

fn main() {
    let noPadding8: (i8, u8) = (-100, 100);
    let noPadding16: (i16, i16, u16) = (0, 1, 2);
    let noPadding32: (i32, f32, u32) = (3, 4.5, 5);
    let noPadding64: (i64, f64, u64) = (6, 7.5, 8);

    let internalPadding1: (i16, i32) = (9, 10);
    let internalPadding2: (i16, i32, u32, u64) = (11, 12, 13, 14);

    let paddingAtEnd: (i32, i16) = (15, 16);

    unsafe {
        NO_PADDING_8 = (-127, 127);
        NO_PADDING_16 = (-10, 10, 9);

        NO_PADDING_32 = (14, 15.0, 16);
        NO_PADDING_64 = (17, 18.0, 19);

        INTERNAL_PADDING_1 = (110, 111);
        INTERNAL_PADDING_2 = (112, 113, 114, 115);

        PADDING_AT_END = (116, 117);
    }

    zzz(); // #break
}

fn zzz() {()}

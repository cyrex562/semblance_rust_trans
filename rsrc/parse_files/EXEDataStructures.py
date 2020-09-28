from _ctypes import Structure, sizeof
from ctypes import c_char, c_ushort, c_uint32, c_uint8, c_uint16, c_short, \
    c_int8
from enum import Enum

DOS_STUB = [0xba, 0x10, 0x00, 0x0e, 0x1f, 0xb4, 0x09, 0xcd, 0x21, 0xb8, 0x01,
            0x4c, 0xcd, 0x21, 0x90, 0x90,
            0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61,
            0x6d, 0x20, 0x6d, 0x75, 0x73,
            0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x20, 0x75, 0x6e,
            0x64, 0x65, 0x72, 0x20, 0x4d,
            0x69, 0x63, 0x72, 0x6f, 0x73, 0x6f, 0x66, 0x74, 0x20, 0x57, 0x69,
            0x6e, 0x64, 0x6f, 0x77, 0x73,
            0x2e, 0x0d, 0x0a, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00]


class SectionType(Enum):
    UNDEFINED = 0,
    DOS_HEADER = 1,
    NE_HEADER = 2,
    SEG_TABLE = 3,
    RESC_TBL = 4,
    RES_NAME_TBL = 5,
    MOD_REF_TBL = 6,
    IMPORT_TBL = 7,
    ENTRY_TBL = 8,
    NRES_NAME_TBL = 9,
    CODE = 10,
    DATA = 11


class ImageDosHeaderStruct(Structure):
    _fields_ = [
        # 0x00: u8[2]: ID="MZ"
        ("e_magic", c_char * 2),
        # 0x02: u16: bytes on last 512-byte page of file
        ("e_cblp", c_ushort),
        # 0x04: u16: pages in file
        ("e_cp", c_ushort),
        # 0x06: u16: relocations
        ("e_crlc", c_ushort),
        # 0x08: u16: size of header in paragraphs
        ("e_cparhdr", c_ushort),
        # 0x0a: u16: min extra paragraphs needed
        ("e_minalloc", c_ushort),
        # 0x0c: u16: max extra paragraphs needed
        ("e_maxalloc", c_ushort),
        # 0x0e: u16: initial relative SS value
        ("e_ss", c_ushort),
        # 0x10: u16: initial SP value
        ("e_sp", c_ushort),
        # 0x12: u16: checksum.
        ("e_csum", c_ushort),
        # 0x14: u16: initial IP value
        ("e_ip", c_uint32),
        # 0x16: u16: Initial relative CS
        ("e_cs", c_ushort),
        # 0x18: u16: file address of relocation table
        ("e_lfarlc", c_ushort),
        # 0x1a: u16: overlay number
        ("e_ovno", c_ushort),
        # 0x1c: s16[4]: reserved words
        ("e_res", c_short * 4),
        # u16: OEM Identifier
        ("e_oemid", c_ushort),
        # u16: OEM Information
        ("e_oeminfo", c_ushort),
        # short[10]: reserved words
        ("e_res", c_short * 10),
        # file address of new exe header
        ("e_lfanew", c_uint32)
    ]


# e_lfanew field (0x3c) points to the NE header

# A paragraph is 16 bytes

# each pointer in the relocation table has the following format:
# Offset: u16
# Segment: u16

# 0xc: u8: program flags
# bits 0-1: DGroupType: 0: node, 1: single shared, 2: multiple, 3: null
PF_DGROUP_TYPE_NODE = 0
PF_DGROUP_TYPE_SINGLE = 1
PF_DGROUP_TYPE_MULTI = 2
PF_DGROUP_TYPE_NULL = 3
# bit 2: global init
PF_GLOBAL_INIT = 0x4
# bit 3: protected mode only
PF_PROT_MODE = 0x8
# bit4: 8086 instructions
PF_INSTR_8086 = 0x10
# bit5: 80286 instructions
PF_INSTR_80286 = 0x20
# bit 6: 80386 instructions
PF_INSTR_80386 = 0x40
# bit 7: 80x87 instructions
PF_INSTR_80X87 = 0x80

# 0x0D: u8: application flags:
#   1: full screen app
AF_FULL_SCREEN = 0x1
#   2: compatible with windows/PM API
AF_WIN_PM_API_COMPAT = 0x4
#   3: uses windows/PM API
AF_WIN_PM_API = 0x8
#   4: reserved
#   5: errrors in image/executable
AF_EXEC_ERR = 0x20
#   6: non-conforming program
AF_NON_CONFORM = 0x40
#   7: DLL or driver
AF_DLL_DRIVER = 0x80

# 0x36: u8: target operating system: 0: unknown, 1: 0S/2, 2: windows,
#               3: european ms-dos 4.x, 4: windows 386, 5: BOSS
TOS_UNK = 0x1
TOS_OS2 = 0x2
TOS_WIN = 0x4
TOS_EURO_DOS = 0x8
TOS_WIN386 = 0x10
TOS_BOSS = 0x20

#   0x37:   u8:   other OS/2 EXE flags: 0: long filename support,
#                   1: 2.x protected mode, 2: 2.x proportional fonts,
#                   3: executable has gangload area
OF_LONG_FNAME = 0x1
OF_PROT_MODE = 0x2
OF_PROPORTIONAL_FONTS = 0x4
OF_GANGLOAD_AREA = 0x8


class NEHeader(object):
    def __init__(self, magic=b'',
                 version=0,
                 revision=0,
                 et_off=0,
                 et_len=0,
                 crc=0,
                 flags=0,
                 auto_data_seg_num=0,
                 heap_sz=0,
                 stack_sz=0,
                 cs_ip=0,
                 ss_sp=0,
                 seg_cnt=0,
                 mod_ref_cnt=0,
                 nrnt_sz=0,
                 st_off=0,
                 rt_off=0,
                 rnt_off=0,
                 mrt_off=0,
                 int_off=0,
                 nrnt_off=0,
                 mov_ent_cnt=0,
                 align=0,
                 res_cnt=0,
                 tgt_os=0,
                 other_flags=0,
                 thunk_off=0,
                 thunk_len=0,
                 swap_area=0,
                 win_vers=0):
        self.magic = magic
        self.version = version
        self.revision = revision
        self.entry_tbl_off = et_off
        self.entry_tbl_len = et_len
        self.crc = crc
        self.flags = flags
        self.auto_data_seg_num = auto_data_seg_num
        self.heap_sz = heap_sz
        self.stack_sz = stack_sz
        self.cs_ip = cs_ip
        self.ss_sp = ss_sp
        self.seg_cnt = seg_cnt
        self.mod_ref_cnt = mod_ref_cnt
        self.nonres_sz = nrnt_sz
        self.st_off = st_off
        self.rt_off = rt_off
        self.rnt_off = rnt_off
        self.mrt_off = mrt_off
        self.int_off = int_off
        self.nonres_off = nrnt_off
        self.mov_ent_cnt = mov_ent_cnt
        self.align = align
        self.res_cnt = res_cnt
        self.tgt_os = tgt_os
        self.other_flags = other_flags
        self.thunk_off = thunk_off
        self.thunk_len = thunk_len
        self.swap_area = swap_area
        self.win_vers = win_vers

NE_HDR_SZ = 0x3F


class NonResName(object):
    def __init__(self, index: int = 0, offset: int = 0, length: int = 0,
                 value: str = ""):
        self.offset = offset
        self.index = index
        self.length = length
        self.value = value


class NEHeaderStruct(Structure):
    """
    http://wiki.osdev.org/NE
    offsets are given from the beginning of this header in the file
    """
    _pack_ = 1
    _fields_ = [
        # NE
        # 0x00: char[2]: signature N (0x4E), E (0x45)
        ("ne_magic", c_char * 2),
        # 0x02: u8: version number
        ("ne_ver", c_int8),
        # 0x3: u8: revision number
        ("ne_rev", c_int8),
        # 0x4: u16: offset to the entry table
        ("ne_enttab", c_uint16),
        # 0x6: u16: length of the entry table
        ("ne_cbenttab", c_uint16),
        # 0x8: file load crc
        ("ne_crc", c_uint32),
        # 0xc: program flags (see above)
        ("ne_prog_flags", c_uint8),
        # 0xd: app flags (see above)
        ("ne_app_flags", c_uint8),
        # 0xe: u16: Specifies the automatic data segment number. (0Eh is
        # zero if the SINGLEDATA and MULTIPLEDATA bits are
        # cleared.)
        ("ne_autodata", c_uint16),
        # 0x10: u16: Specifies the initial size, in bytes, of the local
        # heap. This value is zero if there is no local
        # allocation.
        ("ne_heap", c_uint16),
        # 0x12: u16: Specifies the initial size, in bytes, of the stack.
        # This value is zero if the SS register value does not
        # equal the DS register value.
        ("ne_stack", c_uint16),
        # 0x14: u32: Specifies the segment:offset value of CS:IP.
        ("ne_csip", c_uint32),
        # 0x18: u32: Specifies the segment:offset value of SS:SP.
        #  The value specified in SS is an index to the module's
        # segment table. The first entry in the segment table
        # corresponds to segment number 1.
        # If SS addresses the automatic data segment and SP is
        # zero, SP is set to the address obtained by adding the
        # size of the automatic data segment to the size of the
        # stack.
        ("ne_sssp", c_uint32),
        # 0x1c: u16: Specifies the number of entries in the segment table.
        ("ne_cseg", c_uint16),
        # 0x1E: u16: Specifies the number of entries in the module-reference
        # table.
        ("ne_cmod", c_uint16),
        # 0x20: u16: Specifies the number of bytes in the nonresident-name
        # table.
        ("ne_cbnrestab", c_uint16),
        # 0x22: u16: Specifies a relative offset from the beginning of the
        # Windows header to the beginning of the segment table.
        ("ne_segtab", c_uint16),
        # 0x24: u16: Specifies a relative offset from the beginning of the
        # Windows header to the beginning of the resource table.
        ("ne_rsrctab", c_uint16),
        # 0x26: u16: Specifies a relative offset from the beginning of the
        # Windows header to the beginning of the resident-name table.
        ("ne_restab", c_uint16),
        # 0x28: Specifies a relative offset from the beginning of the Windows
        # header to the beginning of the module-reference table.
        ("ne_modtab", c_uint16),
        # 0x2a: Specifies a relative offset from the beginning of the Windows
        # header to the beginning of the imported-name table.
        ("ne_imptab", c_uint16),
        # 0x2c: Specifies a relative offset from the beginning of the file to
        # the beginning of the nonresident-name table.
        ("ne_nrestab", c_uint16),
        # 0x30: Specifies the number of movable entry points.
        ("ne_cmovent", c_uint16),
        # 0x32: Specifies a shift count that is used to align the logical
        # sector.This count is log2 of the segment sector size.It is typically
        # 4, although the default count is 9.(This value corresponds to the /
        # alignment [ / a] linker switch. When the linker command line contains
        # / a: 16, the shift count is 4. When the linker command line contains /
        #  a: 512, the shift count is 9.)
        ("ne_align", c_uint16),
        # 0x34: specifies the number of resource segments.
        ("ne_cres", c_uint16),
        # 0x36: Specifies the target operating system, depending on which bits
        # are set:
        ("ne_exetyp", c_uint8),
        # 0x37: Specifies additional information about the executable file. It
        # can be one or more of the following values:
        ("ne_flagsothers", c_uint8),
        # 0x38: Specifies the offset, in sectors, to the beginning of the
        # fast-load area. (Only Windows uses this value.)
        ("ne_pretthunks", c_uint16),
        # 0x3A: Specifies the length, in sectors, of the fast-load area. (Only
        # Windows uses this value.)
        ("ne_psegrefbytes", c_uint16),
        # 0x3c: Reserved.
        ("ne_swaparea", c_uint16),
        # 0x3e: Specifies the expected version number for Windows. (Only Windows
        #  uses this value.)
        ("ne_expver", c_uint8 * 2)
    ]


class SegmentTableEntryFlags(Enum):
    # segment table entry flags
    # 0: If this bit is set, the segment is a data segment. Otherwise, the segment
    # is a code segment.
    STEF_DATA_SEG = 0x1
    # 1: If this bit is set, the loader has allocated memory for the segment.
    STEF_LOADER_ALLOC_MEM = 0x2
    # 2: If this bit is set, the segment is loaded.
    STEF_SEG_LOADED = 0x4
    # 3: reserved
    # 4: If this bit is set, the segment type is MOVABLE. Otherwise, the segment
    # type is FIXED.
    STEF_MOVABLE = 0x10
    # 5: If this bit is set, the segment type is PURE or SHAREABLE. Otherwise, the
    # segment type is IMPURE or NONSHAREABLE.
    STEF_PURE = 0x20
    # 6: If this bit is set, the segment type is PRELOAD. Otherwise, the segment
    # type is LOADONCALL.
    STEF_RELOAD = 0x40
    # 7: If this bit is set and the segment is a code segment, the segment type is
    # EXECUTEONLY. If this bit is set and the segment is a data segment, the
    # segment type is READONLY.
    STEF_RXONLY = 0x80
    # 8: If this bit is set, the segment contains relocation data.
    STEF_HAS_RELOC = 0x100
    # 9:-11: reserved
    # 12: If this bit is set, the segment is discardable.
    STEF_DISCARD = 0x1000
    # 13-16:reserved


# http://www.program-transformation.org/Transform/NeFormat
# 0x0: u16: The offset, in sectors, to the segment data (relative to the
#           beginning of the file). A value of zero means no data exists.
# 0x2: u16: The length, in bytes, of the segment, in the file. A value of zero
#           indicates that the segment length is 64K, unless the selector
#           offset is also zero.
# 0x4: u16: flags (see above)
# 0x6: minimum allocation size of the segment in bytes
class SegmentTableEntryStruct(Structure):
    _pack_ = 1
    _fields_ = [
        ("offset", c_ushort),
        ("length", c_ushort),
        ("flags", c_ushort),
        ("min_alloc_size", c_ushort)
    ]


STE_SZ = sizeof(SegmentTableEntryStruct)


def is_data_seg(ste_flags: int) -> bool:
    return ste_flags & SegmentTableEntryFlags.STEF_DATA_SEG.value == 1


def is_code_seg(ste_flags: int) -> bool:
    return ste_flags & SegmentTableEntryFlags.STEF_DATA_SEG.value == 0


def seg_has_reloc(ste_flags: int) -> bool:
    return ste_flags & SegmentTableEntryFlags.STEF_HAS_RELOC.value == 1


class SegmentTableEntry(object):
    def __init__(self, offset: int = 0, length: int = 0, flags: int = 0,
                 alloc_sz: int = 0, align: int = 0):
        self.offset = offset
        self.length = length
        self.flags = flags
        self.alloc_sz = alloc_sz
        self.start = offset << align
        self.end = self.start + self.length
        self.next = self.end
        self.prev = 0


class ImportedName(object):
    def __init__(self, length=0, value="", ):


class EntryPointBundle(object):
    def __init__(self,
                 entry_point_count=0,
                 movable=False):
        self.entry_pt_cnt = entry_point_count
        self.movable = movable
        self.entry_points = []


# Movable:
# 0x0: u8: flags
# 0x1: u16: int 3f inst
# 0x3: u8: seg count
# 0x4: u16: segment offset
# size: 0x6
# Fixed:
# 0x0: u8: flags
# 0x1: u16: segment offset
# size: 3
# Flags:
# 0x1: is entry exported?
# 0x2: entry uses a global data segment?
# 0x4: reserved
# 0xf8: number of stack words
class EntryPoint(object):
    def __init__(self,
                 exported: bool = False,
                 use_global_data: bool = False,
                 movable: bool = False,
                 stack_word_cnt: int = 0,
                 int3f_inst: int = 0,
                 seg_num: int = 0,
                 seg_off: int = 0):
        self.exported = exported
        self.use_global_data = use_global_data
        self.movable = movable
        self.stack_word_cnt = stack_word_cnt
        self.int3f_inst = int3f_inst
        self.seg_num = seg_num
        self.seg_off = seg_off


# noinspection PyDefaultArgument
class ProgramSection(object):
    def __init__(self,
                 start_off: int = 0,
                 end_off: int = 0,
                 length: int = 0,
                 next_sec_off: int = 0,
                 prev_sec_off: int = 0,
                 name: str = "",
                 sec_type: SectionType = SectionType.UNDEFINED,
                 sec_buf: bytes = b"",
                 sec_data: list = []):
        self.start = start_off
        self.end = end_off
        self.next = next_sec_off
        self.prev = prev_sec_off
        self.length = length
        self.name = name
        self.sec_type = sec_type
        self.sec_buf = sec_buf
        self.sec_data = sec_data


class ResidentNameTableEntry(object):
    def __init__(self, entry_len, entry_str, entry_ord):
        self.entry_len = entry_len
        self.entry_str = entry_str
        self.entry_ord = entry_ord


class NameInfo(object):
    def __init__(self, offset, length, flags, name_id, handle, usage):
        self.offset = offset
        self.length = length
        self.flags = flags
        self.name_id = name_id
        self.handle = handle
        self.usage = usage





class TypeInfo(object):
    def __init__(self, type_id, count, reserved, name_info: list,
                 res_res_name_offsets: list):
        self.type_id = type_id
        self.count = count
        self.reerved = reserved
        self.name_info = name_info
        self.res_res_name_offsets = res_res_name_offsets


# END OF FILE #

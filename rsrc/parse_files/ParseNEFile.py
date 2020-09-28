# noinspection PyProtectedMember
import ctypes
import logging
import os
import socket
import struct
import sys
import distorm3
from ctypes import Structure, c_char, c_ushort, c_uint32, sizeof, c_ubyte, \
    Array, _SimpleCData, c_byte, c_char_p, c_double, c_longdouble, c_float, \
    c_int, c_int8, c_int16, c_int32, c_int64, c_long, c_longlong, c_short, \
    c_size_t, c_ssize_t, c_uint, c_uint8, c_uint16, c_uint64, c_ulong, \
    c_ulonglong, c_void_p, c_wchar, c_wchar_p, c_bool

from ParseFiles.EXEDataStructures import ImageDosHeaderStruct, NEHeader, \
    ProgramSection, STE_SZ, \
    ResidentNameTableEntry, SegmentTableEntryStruct, NameInfo, TypeInfo, \
    SegmentTableEntryFlags, seg_has_reloc, SegmentTableEntry, is_code_seg, \
    NonResName, SectionType, NE_HDR_SZ, EntryPointBundle, EntryPoint

CTYPES_TYPES = [
    c_byte, c_char, c_char_p, c_double, c_longdouble, c_float, c_int, c_int8,
    c_int16, c_int32, c_int64, c_long, c_longlong, c_short, c_size_t, c_ssize_t,
    c_ubyte, c_uint, c_uint8, c_uint16, c_uint32, c_uint64, c_ulong,
    c_ulonglong, c_ushort, c_void_p, c_wchar, c_wchar_p, c_bool
]

LOGGING_FACILITY = 16 * 8

g_hostname = socket.gethostname()


class SyslogFilter(logging.Filter):
    def filter(self, record):
        record.priority = LOGGING_FACILITY + record.levelno
        record.version = 1
        record.hostname = g_hostname
        record.messageid = "{}:{}:{}".format(record.module, record.funcName,
                                             record.lineno)
        return True


FORMAT = "<%(priority)d>%(version)d %(asctime)s %(hostname)s %(module)s " \
         "%(process)d %(messageid)s - %(message)s"
logger = logging.getLogger(__name__)
logger.setLevel(logging.DEBUG)
logger.addFilter(SyslogFilter())
# adapter = SyslogAdapter(logger, {})
sh = logging.StreamHandler(sys.stdout)
sh.setLevel(logging.DEBUG)
formatter = logging.Formatter(FORMAT)
sh.setFormatter(formatter)
logger.addHandler(sh)


# noinspection PyProtectedMember
def struct_fields_to_string(s: ctypes.Structure, pretty: bool = True) -> str:
    fields = []
    for field in s._fields_:
        field_name = field[0]
        field_val = getattr(s, field[0])
        if isinstance(field_val, Array):
            if field_val._type_ in CTYPES_TYPES:
                field_val = "({})" \
                    .format(", ".join("{:06x}".format(x) for x in field_val))
        elif isinstance(field_val, Structure):
            # noinspection PyTypeChecker
            field_val = struct_fields_to_string(field_val)
        elif isinstance(field_val, _SimpleCData):
            field_val = "{:#06x}".format(field_val)
        elif isinstance(field_val, int):
            field_val = "{:#06x}".format(field_val)
        elif isinstance(field_val, bytes):
            field_val = "".join("{:02x}".format(x) for x in field_val)
        else:
            logger.error("unhandled type")
        fields.append("{}: {}".format(field_name, field_val))
    if pretty is True:
        return "\n".join(fields)
    return ", ".join(fields)


# def ste_struct_to_string(ste):
#     return "off: {:#06x}, sz: {:#06x}, flags: {:#06x}, alloc sz: {:#06x}" \
#         .format(ste.seg_off, ste.seg_length, ste.flags, ste.min_alloc_size)


# def ste_to_string(ste):
#     return "prev: {:#06x}, off: {:#06x}, end: {:#06x}, next: {:#06x}, " \
#            "sz: {:#06x}, flags: {:#06x}, alloc sz: {:#06x}" \
#         .format(ste.prev, ste.offset, ste.end, ste.next, ste.size, ste.flags,
#                 ste.alloc_sz)


# def seg_table_to_pretty_string(seg_tbl):
#     output = "prev____|offset__|_end____|_size___|offset2_|_end2___|_size2___" \
#              "|_next___|_flags__|_alloc_sz\n"
#     for ste in seg_tbl:
#         output += "{:#08x} {:#08x} {:#08x} {:#08x} {:#08x} {:#08x} {:#08x} " \
#                   "{:#08x} {:#08x} {:#08x}\n" \
#             .format(ste.prev, ste.offset, ste.end, ste.size, ste.offset2,
#                     ste.end2, ste.size2, ste.next, ste.flags, ste.alloc_sz)
#     return output


def run():
    file_to_parse = \
        "D:\\Projects\\OutpostPort\\OriginalFiles\\OUTPOST\\OUTPOST.EXE"

    section_map = {}

    if not os.path.exists(file_to_parse):
        logger.error("file %s does not exist", file_to_parse)
        return -1

    ftp = open(file_to_parse, 'rb')
    buf = ftp.read()
    buf_len = len(buf)
    ftp.close()

    # buf_start = 0
    buf_end = len(buf)
    logger.info("file buffer end: {:#06x}".format(buf_end))
    # ste_start = 0

    # parse MZ Header
    logger.info("Parsing the MZ Header")
    mz_header_start = 0
    mz_header_end = mz_header_start + sizeof(ImageDosHeaderStruct)
    mz_header = ImageDosHeaderStruct.from_buffer_copy(
        buf[mz_header_start:mz_header_end])
    logger.info("MZ Header:\n%s\n", struct_fields_to_string(mz_header))
    section_map[mz_header_start] = ProgramSection(
        start_off=mz_header_start,
        end_off=mz_header_end,
        length=sizeof(ImageDosHeaderStruct),
        name="MZ Header")

    ne_hdr_off = struct.unpack_from("H", buf, 0x3c)[0]
    # st_off = struct.unpack_from("H", buf, ne_hdr_off + 0x22)[0]
    align = struct.unpack_from("H", buf, ne_hdr_off + 0x32)[0]
    # st_ptr = st_off + ne_hdr_off

    # Parse the NE Header
    ne_hdr, ne_hdr_start = parse_ne_hdr(buf, ne_hdr_off, section_map)

    # sector_size = 1 << ne_hdr.ne_align

    # Parse the segment table
    segment_table_entries = parse_segment_table(buf,
                                                ne_hdr,
                                                ne_hdr_start,
                                                section_map,
                                                align)

    # parse resource table
    parse_resc_tbl(buf, ne_hdr, ne_hdr_start, section_map)


    # Parse the resident names table
    parse_resn_tbl(buf, ne_hdr, ne_hdr_start, section_map)

    # parse module references
    parse_mod_refs(buf, ne_hdr, ne_hdr_start, section_map)

    # parse imported names
    parse_imported_names(buf, ne_hdr, ne_hdr_start, section_map)

    # bundles of entry-point definitions
    parse_entry_points(buf, ne_hdr, ne_hdr_start, section_map)

    # non-resident names table
    parse_nonres_names(buf, ne_hdr, section_map)

    for ste in segment_table_entries:
        section_map_name = "CODE"
        if ste.flags & SegmentTableEntryFlags.STEF_DATA_SEG.value:
            section_map_name = "DATA"
        start = ste.offset << align
        section_map[start] = \
            ProgramSection(
                start_off=start,
                length=ste.length,
                end_off=start + ste.length,
                name=section_map_name
            )

    # TODO: parse segments
    # identify all code segments
    #   parse relocations
    # identify other segments
    #   parse relocations

    # FIXME: either the segment table entry parsing is broken or the
    # interpretation of the data is broken
    # sections_sorted = sorted(section_map.items(), key=section_map.get)
    sorted_sections = []
    for k, v in section_map.items():
        sorted_sections.append(v)
    sorted_sections = sorted(
        sorted_sections,
        key=lambda binary_section: binary_section.start)
    sorted_sec_str = "prev_____|_start____|__end_____|__next____|__length__|__name____\n"

    for i in range(0, len(sorted_sections)):
        if i == 0:
            sorted_sections[i].prev = -1
            sorted_sections[i].next = sorted_sections[i + 1].start
        elif 0 < i < len(sorted_sections) - 2:
            sorted_sections[i].prev = sorted_sections[i-1].start
            sorted_sections[i].next = sorted_sections[i+1].start
        else:
            sorted_sections[i].prev = sorted_sections[i-1].start
            sorted_sections[i].next = buf_end

    for sec in sorted_sections:
        sorted_sec_str += "{:#08x} | {:#08x} | {:#08x} | {:#08x} | {:#08x} | {}\n".format(
            sec.prev, sec.start, sec.end, sec.next, sec.length, sec.name)

    logger.info("file end: {:#08x}".format(buf_len))
    logger.info("sections:\n{}\n".format(sorted_sec_str))

    # we should test three different modes of section sizing:
    # 1: based on the starting offsets of each, where a section is
    #   offset:next offset
    # 2: based on the starting offset and size, where a section is
    #   offset:offset+size
    # 3: based on pushing subsesquent offsets

    for ste in segment_table_entries:
        if is_code_seg(ste.flags):
        #
        # if ste.name == "CODE":
            bin_sec = section_map[ste.start]
            start = ste.start
            end = ste.end
            # test three different disassembly
            section_buf = buf[start:end]
            if seg_has_reloc(ste.flags):
                logger.warning("section has relocations")

            # disassemble with distorm:
            listing = distorm3.Decode(0, section_buf, distorm3.Decode16Bits)
            for offset, size, mnemonic, hex_strings in listing:
                # offset = line[0],
                # size = line[1][0],
                # mnemonic = line[2][0],
                # hex_strings = line[3]
                print("{:#08x} ({:#02x}) {} {}".format(offset,
                                                       size,
                                                       hex_strings,
                                                       mnemonic))

    logger.info("finished")


def get_u8(buf, offset):
    return struct.unpack_from("B", buf, offset)[0]


def get_char(buf, offset, count=1):
    return struct.unpack_from("{}c".format(count), buf, offset)


def get_u16(buf, offset):
    return struct.unpack_from("H", buf, offset)[0]


def get_u32(buf, offset):
    return struct.unpack_from("I", buf, offset)[0]


def parse_ne_hdr(buf: bytes, ne_hdr_off: int, sections: dict)->(NEHeader, int):
    """
    Parse the NE Header
    :param buf:
    :param ne_hdr_off:
    :param sections:
    :return:
    """
    logger.info("Parsing the NE Header")
    ne_hdr_start = ne_hdr_off
    ne_hdr_end = ne_hdr_start + NE_HDR_SZ
    neh = NEHeader()
    neh_buf = buf[ne_hdr_start:ne_hdr_end]
    neh.magic = get_char(neh_buf, 0, 2)
    neh.version = get_u8(neh_buf, 2)
    neh.revision = get_u8(neh_buf, 3)
    neh.entry_tbl_off = get_u16(neh_buf, 4)
    neh.entry_tbl_len = get_u16(neh_buf, 6)
    neh.crc = get_u32(neh_buf, 8)
    neh.flags = get_u16(neh_buf, 0xc)
    neh.auto_data_seg_num = get_u16(buf, 0xe)
    neh.heap_sz = get_u16(buf, 0x10)
    neh.stack_sz = get_u16(buf, 0x12)
    neh.cs_ip = get_u32(buf, 0x14)
    neh.ss_sp = get_u32(buf, 0x18)
    neh.seg_cnt = get_u16(buf, 0x1c)
    neh.mod_ref_cnt = get_u16(buf, 0x1e)
    neh.nonres_sz = get_u16(buf, 0x20)
    neh.st_off = get_u16(buf, 0x22)
    neh.rt_off = get_u16(buf, 0x24)
    neh.rnt_off = get_u16(buf, 0x26)
    neh.mrt_off = get_u16(buf, 0x28)
    neh.int_off = get_u16(buf, 0x2a)
    neh.nonres_off = get_u32(buf, 0x2c)
    neh.mov_ent_cnt = get_u16(buf, 0x30)
    neh.align = get_u16(buf, 0x32)
    neh.res_cnt = get_u16(buf, 0x34)
    neh.tgt_os = get_u8(buf, 0x36)
    neh.other_flags = get_u8(buf, 0x37)
    neh.thunk_off = get_u16(buf, 0x38)
    neh.thunk_len = get_u16(buf, 0x3a)
    neh.swap_area = get_u16(buf, 0x3c)
    neh.win_vers = get_u16(buf, 0x3e)

    logger.info("NE Header: \n%s\n", neh)
    sections[ne_hdr_start] = ProgramSection(
                                start_off=ne_hdr_start,
                                end_off=ne_hdr_end,
                                length=sizeof(NEHeader),
                                name="NE Header",
                                sec_buf=buf[ne_hdr_start:ne_hdr_end],
                                sec_type=SectionType.NE_HEADER,
                                sec_data=[neh])
    return neh, ne_hdr_start


def parse_nonres_names(buf: bytes, ne_hdr: NEHeader, section_map: dict) -> list:
    """
    Parse the Non-Resident Names Table
    :param buf:
    :param ne_hdr:
    :param section_map:
    :return:
    """
    logger.info("Parsing the Non-Resident Names Table")
    nrnt_start = ne_hdr.nonres_off
    nrnt_size = ne_hdr.nonres_sz
    nrnt_end = nrnt_start + nrnt_size
    nrnt_ptr = nrnt_start
    non_res_names = []

    index = 0
    while True:
        # 0x0: u8 length of the string
        nrn_len = buf[nrnt_ptr]
        if nrn_len == 0:
            logger.info("End of NRNT reached")
            break
        # 0x1--len: the string
        nrn = buf[nrnt_ptr + 1:nrnt_ptr + nrn_len + 1].decode('utf-8')
        non_res_names.append(NonResName(offset=nrnt_ptr,
                                        index=index,
                                        length=nrn_len,
                                        value=nrn))
        nrnt_ptr += 1 + nrn_len
        index += 1
    section_map[nrnt_start] = ProgramSection(
        start_off=nrnt_start,
        length=nrnt_size,
        end_off=nrnt_end,
        name="Non-Resident Names Table",
        sec_type=SectionType.NRES_NAME_TBL,
        sec_buf=buf[nrnt_start:nrnt_end],
        sec_data=non_res_names)
    logger.info("Non-Resident Names Count: %d", len(non_res_names))
    return non_res_names


def parse_entry_points(buf: bytes,
                       ne_hdr: NEHeader,
                       ne_hdr_start: int,
                       sections: dict) -> list:
    """
    Parse the Entry Point Table
    :param buf:
    :param ne_hdr:
    :param ne_hdr_start:
    :param sections:
    :return:
    """
    logger.debug("parsing entry points")
    ept_start = ne_hdr.entry_tbl_off + ne_hdr_start
    ept_size = ne_hdr.entry_tbl_len
    ept_end = ept_start + ept_size

    ept_ptr = ept_start
    bundles = []
    while True:
        bundle = EntryPointBundle(get_u8(buf, ept_ptr),
                                  get_u8(buf, ept_ptr + 1))
        ept_ptr += 2
        if bundle.entry_pt_cnt == 0:
            logger.debug("end of entry table reached")
            break
        elif bundle.movable == 0:  # no data
            pass
        elif bundle.movable == 0xff:  # movable
            for i in range(0, bundle.entry_pt_cnt):
                bundle.entry_points.append(
                    EntryPoint(exported=get_u8(ept_ptr, 0) & 0x1 == 1,
                               use_global_data=get_u8(ept_ptr, 0) & 0x2 == 1,
                               stack_word_cnt=get_u8(ept_ptr, 0) & 0xf8,
                               int3f_inst=get_u16(ept_ptr, 1),
                               seg_num=get_u8(ept_ptr, 3),
                               seg_off=get_u16(ept_ptr, 4),
                               movable=True))
                # entry = MovableSegmentEntryStruct.from_buffer_copy(
                #     buf[ept_ptr:ept_ptr + sizeof(MovableSegmentEntryStruct)])
                # entry_points.append(entry)
                ept_ptr += 6
        elif bundle.movable == 0xFE:  # module const
            for i in range(0, bundle.entry_pt_cnt):
                bundle.entry_points.append(
                    EntryPoint(exported=get_u8(ept_ptr, 0) & 0x1 == 1,
                               use_global_data=get_u8(ept_ptr, 0) & 0x2 == 1,
                               seg_off=get_u16(ept_ptr, 1),
                               movable=False))
                # entry = FixedSegmentEntryStruct.from_buffer_copy(
                #     buf[ept_ptr:ept_ptr + sizeof(MovableSegmentEntryStruct)])
                # entry_points.append(entry)
                ept_ptr += 3
        else:  # segment index
            for i in range(0, bundle.entry_pt_cnt):
                bundle.entry_points.append(
                    EntryPoint(exported=get_u8(ept_ptr, 0) & 0x1 == 1,
                               use_global_data=get_u8(ept_ptr, 0) & 0x2 == 1,
                               seg_off=get_u16(ept_ptr, 1),
                               movable=False))
                # entry = FixedSegmentEntryStruct.from_buffer_copy(
                #     buf[ept_ptr:ept_ptr + sizeof(MovableSegmentEntryStruct)])
                # entry_points.append(entry)
                ept_ptr += 3
    #entry_pt_cnt = len(bundles)
    sections[ept_start] = ProgramSection(start_off=ept_start,
                                         end_off=ept_end,
                                         length=ept_size,
                                         name="Entry Table",
                                         sec_type=SectionType.ENTRY_TBL,
                                         sec_buf=buf[ept_start:ept_end],
                                         sec_data=bundles)
    #logger.info("segment entry count: %d", entry_pt_cnt)


def parse_imported_names(buf: bytes,
                         ne_hdr: NEHeader,
                         ne_hdr_start: int,
                         sections: dict) -> list:
    """

    :param buf:
    :param ne_hdr:
    :param ne_hdr_start:
    :param sections:
    :return:
    """
    logger.info("parsing imported names table")
    # for each entry:
    #   0x0: u8: length of string
    #   0x1-len: string
    int_start = ne_hdr.int_off + ne_hdr_start
    int_sz = 0
    int_ptr = int_start
    imported_names = []
    while True:
        entry_len = buf[int_ptr]
        int_ptr += 1
        if entry_len == 0:
            logger.info("end of imported names table")
            break
        fmt_str = "{}s".format(entry_len)
        entry = struct.unpack_from(fmt_str, buf, int_ptr)[0]
        imported_names.append(entry)
        int_ptr += entry_len
    int_end = int_ptr
    imported_name_cnt = len(imported_names)
    logger.info("Imported Name Count: %d", imported_name_cnt)
    sections[int_start] = ProgramSection(
        start_off=int_start,
        end_off=int_end,
        length=int_sz,
        name="Import Table",
        sec_type=SectionType.IMPORT_TBL,
        sec_buf=buf[int_start:int_end],
        sec_data=imported_names)


def parse_mod_refs(buf, ne_hdr, ne_hdr_start, section_map):
    logger.info("parsing module reference table")
    # for each entry:
    #   0x0: u16: offset within imported names table to referenced module
    # name string
    mrt_start = ne_hdr.ne_modtab + ne_hdr_start
    mod_refs = struct.unpack_from("{}H".format(ne_hdr.ne_cmod),
                                  buf,
                                  offset=mrt_start)
    logger.info("Module Reference Count: %d", len(mod_refs))
    mrt_sz = len(mod_refs) * 2
    mrt_end = mrt_start + mrt_sz
    section_map[mrt_start] = \
        ProgramSection(start_off=mrt_start,
                       end_off=mrt_end,
                       length=mrt_sz,
                       name="Module Reference Table")


def parse_resn_tbl(buf, ne_hdr, ne_hdr_start, section_map):
    logger.info("Parsing the Resident Names Table")
    # strings that identify exported functions
    # for each entry:
    #   0x0: u8: length of string
    #   0x1--len : ascii string
    #   len + 1: u16: ordinal that identifies string
    rnt_start = ne_hdr.ne_restab + ne_hdr_start
    rnt_sz = 0
    rnt_ptr = rnt_start
    resident_names = []
    while True:
        # entry_len = struct.unpack("B", buf[rnt_ptr])[0]
        entry_len = buf[rnt_ptr]
        entry_buf = buf[rnt_ptr + 1: rnt_ptr + entry_len + 3]
        if entry_len == 0:
            rnt_sz = 1
            logger.info("end of resident name table reached")
            break
        rnt_ptr += 1
        entry = entry_buf[:-2].decode("utf-8")
        ordinal = c_ushort.from_buffer_copy(entry_buf[-2:])
        resident_names.append(
            ResidentNameTableEntry(entry_len, entry, ordinal))
        rnt_sz += 1 + entry_len + 2
        rnt_ptr += entry_len + 2
    rnt_end = rnt_start + rnt_sz
    section_map[rnt_start] = ProgramSection(
        start_off=rnt_start,
        end_off=rnt_end,
        length=rnt_sz,
        name="Resident Name Table")
    if len(resident_names) == 0:
        logger.info("no exported functions")


def parse_resc_tbl(buf, ne_hdr, ne_hdr_start, section_map):
    rt_start = ne_hdr.ne_rsrctab + ne_hdr_start
    rt_sz = -1
    rt_end = -1
    rt_ptr = rt_start
    rte_cnt = ne_hdr.ne_cres
    if ne_hdr.ne_cres > 0:
        logger.info("parsing resource table")
        # table structure:
        # res_align_shift: u16: alignment shift count for resource data.
        #     specifies factor in bytes for computing the location of a
        #     resource in the exec file.
        res_shift_cnt = struct.unpack_from("H", buf, rt_start)[0]
        rt_ptr += 2
        res_types = []
        #   // an array of typeinfo structures
        #   typeinfo res_types[]
        for i in range(0, rte_cnt):
            # TYPEINFO:
            # u16: rt_type_id
            # u16: rt_res_count // num of structs in array
            # u32: rt_reserved
            rti_type_id, rti_res_cnt, rti_res = \
                struct.unpack_from("HHL", buf, rt_ptr)
            rt_ptr += 8
            # nameinfo rt_name_info[]
            rti_name_info = []
            res_res_names_offsets = []
            for j in range(0, rti_res_cnt):
                # NAMEINFO
                # rn_off: u16: offset to contents of the resource data from
                #     beginning of file
                # rn_len: u16: res length in bytes
                # rn_flags: u16: 0x10: movable, 0x20: shared, 0x40: preloaded
                # rn_id: u16: if high bit set (0x8000) it's an rid int,
                #     otherwise it's an offset to a res string relative to the
                #     beginning of the res table
                # rn_handle: u16: reserved
                # rn_usage: u16: reserved
                rni_off, rni_len, rni_flags, rni_id, rni_handle, rni_usage = \
                    struct.unpack_from("HHHHHH", buf, rt_ptr)
                if rni_off & 0x8000 == 0:
                    res_res_names_offsets.append(rni_off)
                rti_name_info.append(NameInfo(rni_off, rni_len, rni_flags,
                                              rni_id, rni_handle, rni_usage))
                rt_ptr += 12
            res_types.append(
                TypeInfo(rti_type_id, rti_res_cnt, rti_res, rti_name_info,
                         res_res_names_offsets))

        # res_end_types: u16  // end of the res type defs, must be 0
        rt_ptr += 2
        # res_resource_names[]: u8  // names associated with resources in this
        # table
        rt_end = rt_ptr
        rt_sz = rt_end - rt_start + 1
        all_res_res_name_offsets = []
        for res_type in res_types:
            all_res_res_name_offsets.extend(res_type.res_res_name_offsets)

        res_res_names = []
        if len(all_res_res_name_offsets) > 0:
            while True:
                re_end_names = struct.unpack_from("H", buf, rt_ptr)[0]
                # u8 res_end_names // end of resource names, must be 0
                if re_end_names == 0:
                    break
                length = struct.unpack_from("B", buf, rt_ptr)[0]
                str_buf = buf[rt_ptr + 1:rt_ptr + 1 + length]
                res_res_names.append(
                    buf[rt_ptr + 1:rt_ptr + 1 + length].decode('ascii'))
    else:
        logger.info("resource count 0, skipping")
    section_map[rt_start] = ProgramSection(start_off=rt_start,
                                           end_off=rt_end,
                                           length=rt_sz,
                                           name="Resource Table")


def parse_segment_table(buf, ne_hdr, ne_hdr_start, section_map, align):
    logger.info("Parsing the Segment Table")
    st_start = ne_hdr.ne_segtab + ne_hdr_start
    ste_count = ne_hdr.ne_cseg
    st_sz = ste_count * STE_SZ
    st_end = st_start + st_sz
    # st_buf = buf[st_start:st_end]
    ste_start = st_start
    segment_table_entries = []
    for i in range(0, ste_count):
        ste_end = ste_start + STE_SZ
        ste_buf = buf[ste_start:ste_end]
        #ste = SegmentTableEntryStruct.from_buffer_copy(ste_buf)
        seg_off = struct.unpack_from("H", ste_buf, 0)[0]
        seg_len = struct.unpack_from("H", ste_buf, 0x2)[0]
        flags = struct.unpack_from("H", ste_buf, 0x4)[0]
        alloc_sz = struct.unpack_from("H", ste_buf, 0x6)[0]
        segment_table_entries.append(SegmentTableEntry(seg_off,
                                                       seg_len,
                                                       flags,
                                                       alloc_sz,
                                                       align))
        #segment_table_entries.append(ste)
        ste_start += STE_SZ
    section_map[st_start] = ProgramSection(start_off=st_start,
                                           end_off=st_end,
                                           length=st_sz,
                                           name="Segment Table")
    return segment_table_entries


if __name__ == "__main__":
    sys.exit(run())

# END OF FILE #

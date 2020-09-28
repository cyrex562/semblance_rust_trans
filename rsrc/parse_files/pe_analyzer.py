# IMPORTS #
import pefile
import struct
import ctypes

# DEFINES #
file_path = 'OUTPOST.EXE'

# paragraph: 16 bytes
#
# double word: 4 bytes
# word: 2 bytes
# half-word: 1 byte

# FUNCTIONS #
def run():
    #pe = pefile.PE(file_path, fast_load=True)
    bin = open(file_path, 'rb').read()
    mz_id = struct.unpack('2s', bin[0:2])
    # number of bytes in last 512-byte page
    last_page_sz = struct.unpack('H', bin[2:4])
    # total number of 512-byte pages in executable
    nb_pages = struct.unpack('H', bin[4:6])
    # number of relocations
    nb_relocs = struct.unpack('H', bin[6:8])
    # header size in paragraphs
    hdr_sz = struct.unpack('H', bin[8:0xA])
    # minimum number of paragraphs allocated in addition to code size
    min_para = struct.unpack('H', bin[0xA:0xC])
    # maximum number of paragraphs allocated in addition to code size
    max_para = struct.unpack('H', bin[0xC:0xE])
    # initial ss relative to start of exe
    initial_ss = struct.unpack('H', bin[0xE:0x10])
    # initial sp
    initial_sp = struct.unpack('H', bin[0x10:0x12])
    # checksum of exe
    csum = struct.unpack('H', bin[0x12:0x14])
    # cs:ip relative to start of exe (entry point)
    mz_ip_val = struct.unpack('H', bin[0x14:0x16])
    mz_cs_val = struct.unpack('H', bin[0x16:0x18])
    # offset of relocation table; 40h for new-(NE,LE,LX,W3,PE)
    reloc_off = struct.unpack('H', bin[0x18:0x1a])
    # overlay number (0h = main program)
    overlay_num = struct.unpack('H', bin[0x1a:0x1c])
    # new executable header expansions
    # 4 bytes of unknown
    unk_1 = bin[0x1c:0x20]
    unk_2 = bin[0x20:0x22]
    reserved = bin[0x22:0x3c]
    ne_hdr_off = struct.unpack('L', bin[0x3c:0x40])[0]

    print'NE header offset: {0:04X}'.format(ne_hdr_off)

    ptr = ne_hdr_off
    ne_id = bin[ptr:ptr+2]
    linker_maj_vers = struct.unpack('B', bin[ptr+2:ptr+3])
    linker_minor_ver = struct.unpack('B', bin[ptr+3:ptr+4])
    # offset of entry table
    entry_tbl_off = struct.unpack('H', bin[ptr+4:ptr+6])
    entry_tbl_len = struct.unpack('H', bin[ptr+6:ptr+8])
    file_load_crc = struct.unpack('L', bin[ptr+8:ptr+0xC])
    # program flags, bitmapped; 0-1 DGroup type: 0 - none, 1 - single shared,
    # 2 - multiple, 3 - (null); 2 - global initialization;
    # 3 - protected mode only; 4 - 8086 instructions; 5 - 80286 instructions;
    # 6 - 80386 instructions; 7 - 80x87 instructions
    program_flags = struct.unpack('B', bin[ptr+0xC])
    # application flags, bitmapped: 0-2: application type: 1 - full screen,
    # 2 - compatible with windows/P.M. API, 3 - uses windows/P.M. API;
    # 3 - OS/2 family application; 4 - reserved?; 5 - errors in image/executable;
    # 6 - "non-conforming program"; 7 - DLL or driver (SS:SIP info invalid, CS:IP points at far init routine called with AX=module handle which returns AX=0000h
    application_flags = struct.unpack('B', bin[ptr+0xd])
    auto_data_seg_idx = struct.unpack('B', bin[ptr+0xe])
    init_local_heap_sz = struct.unpack('H', bin[ptr+0x10:ptr+0x12])
    init_stack_sz = struct.unpack('H', bin[ptr+0x12:ptr+0x14])
    # cs:ip cs is index into segment table
   #entry_pt = struct.unpack('L', bin[ptr+0x14:ptr+0x18])[0]
    ne_ip_val = struct.unpack('H', bin[ptr+0x14:ptr+0x16])[0]
    ne_cs_val = struct.unpack('H', bin[ptr+0x16:ptr+0x18])[0]
    print 'ne ip val: {0:04X}\nne cs val: {1:04X}'.format(ne_ip_val, ne_cs_val)
    # ss:sp ss is index into segment table
    init_stack_ptr = struct.unpack('L', bin[ptr+0x18:ptr+0x1c])
    seg_cnt = struct.unpack('H', bin[ptr+0x1c:ptr+0x1e])
    mod_ref_cnt = struct.unpack('H', bin[ptr+0x1e:ptr+0x20])
    seg_tbl_off = struct.unpack('H', bin[ptr+0x22:ptr+0x24])[0]
    print 'seg tbl: {0:04X}; count: {1}'.format(seg_tbl_off, seg_cnt)
    res_tbl_off = struct.unpack('H', bin[ptr+0x24:ptr+0x26])[0]
    print 'res tbl: {0:04X}'.format(res_tbl_off)
    resident_names_tbl_off = struct.unpack('H', bin[ptr+0x26:ptr+0x28])[0]
    print 'resident names tbl: {0:04X}'.format(resident_names_tbl_off)
    mod_ref_tbl_off = struct.unpack('H', bin[ptr+0x28:ptr+0x2a])[0]
    print 'mod ref tbl: {0:04X}'.format(mod_ref_tbl_off)
    imported_names_tbl_off = struct.unpack('H', bin[ptr+0x2a:ptr+0x2c])[0]
    print 'imported names tbl: {0:04X}'.format(imported_names_tbl_off)
    nonresident_names_tbl_off = struct.unpack('L', bin[ptr+0x2c:ptr+0x30])[0]
    print 'nonresident names tbl: {0:04X}'.format(nonresident_names_tbl_off)
    # count of moveable entry point listed in entry table
    mov_ep_cnt = struct.unpack('H', bin[ptr+0x30:ptr+0x32])
    # file align sz shift count
    fass_cnt = struct.unpack('H', bin[ptr+0x32:ptr+0x34])
    res_tbl_entry_cnt = struct.unpack('H', bin[ptr+0x34:ptr+0x36])
    # 0 - unk, 1 - os/2, 2 - win, 3 - euro ms-dos, 4 - win 386, 5 - boss
    target_os = struct.unpack('B', bin[ptr+0x36])
    # other os/2 exe flags; 0 - long filename spt; 1 - 2.x protected mode; 2 - 2.x proportional fonts; 3 - executable has gangload area
    os2_flags = struct.unpack('B', bin[ptr+0x37])
    # offset to return thunks or start of gangload area
    thunk_off = struct.unpack('H', bin[ptr+0x38:ptr+0x3a])
    # offset to seg ref tunks or len of gangload area
    seg_ref_thunk_off = struct.unpack('H', bin[ptr+0x3a:ptr+0x3c])
    # min code swap area siz
    min_code_swap_sz = struct.unpack('H', bin[ptr+0x3c:ptr+0x3e])
    # expected windows version number
    expected_win_vers = struct.unpack('2B', bin[ptr+0x3e:ptr+0x40])

    # parse the segment table
    seg_tbl_ptr = ptr + seg_tbl_off
    print "seg tbl ptr: {0:04X}".format(seg_tbl_ptr)

    

    print 'done'

    # relocation items 0000h = offset within segment, 0002h = segment of relocation
    # To get the position of the relocation within the file, you have to compute the
    # physical adress from the segment:offset pair, which is done by multiplying the
    # segment by 16 and adding the offset and then adding the offset of the binary
    # start. Note that the raw binary code starts on a paragraph boundary within the
    # executable file. All segments are relative to the start of the executable in
    # memory, and this value must be added to every segment if relocation is done
    # manually.



# ENTRY POINT #
if __name__ == '__main__'"" :
    run()


# END OF FILE #




















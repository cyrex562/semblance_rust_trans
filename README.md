# semblance_rust_trans
Rust port of Semblance PE/NE/COFF file dumper


INF: Executable-File Header Format                            [P_WinSDK]

3.00
WINDOWS
PSSONLY | Windows 3 Developers Notes softlib ENDUSER

Summary:

Note: This article is part of a set of seven articles, collectively
called the "Windows 3.00 Developer's Notes." More information about
the contents of the other articles, and procedures for ordering a
hard-copy set, can be found in the knowledge base article titled "INF:
The Windows 3.00 Developer's Notes" (Q65260).

This article can be found in the Software/Data Library by searching on
the word EXEFMT or S12688. EXEFMT was archived using the PKware
file-compression utility.

More Information:

Microsoft defined the segmented executable file format for Windows
applications and dynamic-link libraries (DLLs). This file format is
also referred to as the New Executable Format. This new format is an
extension of the existing MS-DOS .EXE format (old-style format). The
purpose of the segmented executable format is to provide the
information needed to support the dynamic linking and segmentation
capabilities of the Windows environment.

An executable file contains Microsoft Windows code and data, or
Windows code, data, and resources. Specific fields have been added to
the old-style .EXE format header to indicate the existence of the
segmented file format. The old-style header may contain a valid
executable program, called a stub program, that will be executed if
the program is run on MS-DOS (without Windows). This stub program
usually prints a message indicating that Microsoft Windows is required
to run the program. The segmented executable format extensions also
begin with a header that describes the contents and location of the
executable image in the file. The loader uses this header information
when it loads the executable segments in memory.


======================================================================
                     OLD-STYLE HEADER EXTENSIONS
======================================================================

The old-style header contains information the loader expects for a DOS
executable file. It describes a stub program (WINSTUB) the loader can
place in memory when necessary, it points to the new-style header, and
it contains the stub programs relocation table.

The following illustrates the distinct parts of the old-style
executable format:

        +-------------------------+
    00h |  Old-style header info  |
        +-------------------------+
    20h |        Reserved         |
        +-------------------------+
    3Ch |   Offset to segmented   |
        |       .EXE header       |
        +-------------------------+
    40h |  Relocation table and   |
        |    DOS stub program     |
        +-------------------------+
        |  Segmented .EXE Header  |
        |           .             |
        |           .             |
        |           .             |

The word at offset 18h in the old-style .EXE header contains the
relative byte offset to the stub program's relocation table. If this
offset is 40h, then the double word at offset 3Ch is assumed to be the
relative byte offset from the beginning of the file to the beginning
of the segmented executable header. A new-format .EXE file is
identified if the segmented executable header contains a valid
signature. If the signature is not valid, the file is assumed to be an
old-style format .EXE file. The remainder of the old-style format
header will describe a DOS program, the stub. The stub may be any
valid program but will typically be a program that displays an error
message.

======================================================================
                         SEGMENTED EXE FORMAT
======================================================================

Because Windows executable files are often larger than one segment
(64K), additional information (that does not appear in the old-style
header) is required so that the loader can load each segment properly.
The segmented EXE format was developed to provide the loader with this
information.

The segmented .EXE file has the following format:

        +-----------------+
    00h |  Old-style EXE  |
        |      Header     |
        +-----------------+
    20h |    Reserved     |
        +-----------------+
    3Ch |    Offset to    | ---+
        | Segmented Header|    |
        +-----------------+    |
    40h | Relocation Table|    |
        |  & Stub Program |    |
        +-----------------+    |
        |                 |    |
        +-----------------+    |
    xxh |  Segmented EXE  | <--+
        |      Header     |
        +-----------------+
        |  Segment Table  |
        +-----------------+
        | Resource Table  |
        +-----------------+
        |  Resident Name  |
        |      Table      |
        +-----------------+
        | Module Reference|
        |      Table      |
        +-----------------+
        | Imported Names  |
        |      Table      |
        +-----------------+
        |   Entry Table   |
        +-----------------+
        |  Non-Resident   |
        |   Name Table    |
        +-----------------+
        |   Seg #1 Data   |
        |   Seg #1 Info   |
        +-----------------+
                .
                .
                .
        +-----------------+
        |   Seg #n Data   |
        |   Seg #n Info   |
        +-----------------+


The following sections describe each of the components that make up
the segmented EXE format. Each section contains a description of the
component and the fields in the structures that make up that
component.

Note: All unused fields and flag bits are reserved for future use and
must contain 0 (zero) values.

======================================================================
                         SEGMENTED EXE HEADER
======================================================================

The segmented EXE header contains general information about the EXE
file and contains information on the location and size of the other
sections. The Windows loader copies this section, along with other
data, into the module table in the system data. The module table is
internal data used by the loader to manage the loaded executable
modules in the system and to support dynamic linking.

The following describes the format of the segmented executable header.
For each field, the offset is given relative to the beginning of the
segmented header, the size of the field is defined, and a description
is given.

    Offset Size Description
    ------ ---- -----------

    00h     DW  Signature word.
                "N" is low-order byte.
                "E" is high-order byte.

    02h     DB  Version number of the linker.

    03h     DB  Revision number of the linker.

    04h     DW  Entry Table file offset, relative to the beginning of
                the segmented EXE header.
    06h     DW  Number of bytes in the entry table.

    08h     DD  32-bit CRC of entire contents of file.
                These words are taken as 00 during the calculation.

    0Ch     DW  Flag word.
                0000h = NOAUTODATA
                0001h = SINGLEDATA (Shared automatic data segment)
                0002h = MULTIPLEDATA (Instanced automatic data
                        segment)
                2000h = Errors detected at link time, module will not
                        load.
                8000h = Library module.
                        The SS:SP information is invalid, CS:IP points
                        to an initialization procedure that is called
                        with AX equal to the module handle. This
                        initialization procedure must perform a far
                        return to the caller, with AX not equal to
                        zero to indicate success, or AX equal to zero
                        to indicate failure to initialize. DS is set
                        to the library's data segment if the
                        SINGLEDATA flag is set. Otherwise, DS is set
                        to the caller's data segment.

                        A program or DLL can only contain dynamic
                        links to executable files that have this
                        library module flag set. One program cannot
                        dynamic-link to another program.

    0Eh     DW  Segment number of automatic data segment.
                This value is set to zero if SINGLEDATA and
                MULTIPLEDATA flag bits are clear, NOAUTODATA is
                indicated in the flags word.

                A Segment number is an index into the module's segment
                table. The first entry in the segment table is segment
                number 1.

    10h     DW  Initial size, in bytes, of dynamic heap added to the
                data segment. This value is zero if no initial local
                heap is allocated.

    12h     DW  Initial size, in bytes, of stack added to the data
                segment. This value is zero to indicate no initial
                stack allocation, or when SS is not equal to DS.

    14h     DD  Segment number:offset of CS:IP.

    18h     DD  Segment number:offset of SS:SP.
                If SS equals the automatic data segment and SP equals
                zero, the stack pointer is set to the top of the
                automatic data segment just below the additional heap
                area.

                    +--------------------------+
                    | additional dynamic heap  |
                    +--------------------------+ <- SP
                    |    additional stack      |
                    +--------------------------+
                    | loaded auto data segment |
                    +--------------------------+ <- DS, SS

    1Ch     DW  Number of entries in the Segment Table.

    1Eh     DW  Number of entries in the Module Reference Table.
    20h     DW  Number of bytes in the Non-Resident Name Table.

    22h     DW  Segment Table file offset, relative to the beginning
                of the segmented EXE header.

    24h     DW  Resource Table file offset, relative to the beginning
                of the segmented EXE header.

    26h     DW  Resident Name Table file offset, relative to the
                beginning of the segmented EXE header.

    28h     DW  Module Reference Table file offset, relative to the
                beginning of the segmented EXE header.

    2Ah     DW  Imported Names Table file offset, relative to the
                beginning of the segmented EXE header.

    2Ch     DD  Non-Resident Name Table offset, relative to the
                beginning of the file.

    30h     DW  Number of movable entries in the Entry Table.

    32h     DW  Logical sector alignment shift count, log(base 2) of
                the segment sector size (default 9).

    34h     DW  Number of resource entries.

    36h     DB  Executable type, used by loader.
                  02h = WINDOWS

    37h-3Fh DB  Reserved, currently 0's.


======================================================================
                            SEGMENT TABLE
======================================================================

The segment table contains an entry for each segment in the executable
file. The number of segment table entries are defined in the segmented
EXE header. The first entry in the segment table is segment number 1.
The following is the structure of a segment table entry.

   Size Description
   ---- -----------

   DW   Logical-sector offset (n byte) to the contents of the segment
        data, relative to the beginning of the file. Zero means no
        file data.

   DW   Length of the segment in the file, in bytes. Zero means 64K.

   DW   Flag word.
        0007h = TYPE_MASK  Segment-type field.
        0000h = CODE       Code-segment type.
        0001h = DATA       Data-segment type.
        0010h = MOVEABLE   Segment is not fixed.
        0040h = PRELOAD    Segment will be preloaded; read-only if
                           this is a data segment.
        0100h = RELOCINFO  Set if segment has relocation records.
        F000h = DISCARD    Discard priority.

   DW   Minimum allocation size of the segment, in bytes. Total size
        of the segment. Zero means 64K.


======================================================================
                            RESOURCE TABLE
======================================================================

The resource table follows the segment table and contains entries for
each resource in the executable file. The resource table consists of
an alignment shift count, followed by a table of resource records. The
resource records define the type ID for a set of resources. Each
resource record contains a table of resource entries of the defined
type. The resource entry defines the resource ID or name ID for the
resource. It also defines the location and size of the resource. The
following describes the contents of each of these structures:

   Size Description
   ---- -----------

   DW   Alignment shift count for resource data.

   A table of resource type information blocks follows. The following
   is the format of each type information block:

        DW  Type ID. This is an integer type if the high-order bit is
            set (8000h); otherwise, it is an offset to the type string,
            the offset is relative to the beginning of the resource
            table. A zero type ID marks the end of the resource type
            information blocks.

        DW  Number of resources for this type.

        DD  Reserved.

        A table of resources for this type follows. The following is
        the format of each resource (8 bytes each):

            DW  File offset to the contents of the resource data,
                relative to beginning of file. The offset is in terms
                of the alignment shift count value specified at
                beginning of the resource table.

            DW  Length of the resource in the file (in bytes).

            DW  Flag word.
                0010h = MOVEABLE  Resource is not fixed.
                0020h = PURE      Resource can be shared.
                0040h = PRELOAD   Resource is preloaded.

            DW  Resource ID. This is an integer type if the high-order
                bit is set (8000h), otherwise it is the offset to the
                resource string, the offset is relative to the
                beginning of the resource table.

            DD  Reserved.

   Resource type and name strings are stored at the end of the
   resource table. Note that these strings are NOT null terminated and
   are case sensitive.

   DB   Length of the type or name string that follows. A zero value
        indicates the end of the resource type and name string, also
        the end of the resource table.

   DB   ASCII text of the type or name string.


======================================================================
                         RESIDENT-NAME TABLE
======================================================================

The resident-name table follows the resource table, and contains this
module's name string and resident exported procedure name strings. The
first string in this table is this module's name. These name strings
are case-sensitive and are not null-terminated. The following
describes the format of the name strings:

   Size Description
   ---- -----------

   DB   Length of the name string that follows. A zero value indicates
        the end of the name table.

   DB   ASCII text of the name string.

   DW   Ordinal number (index into entry table). This value is ignored
        for the module name.


======================================================================
                        MODULE-REFERENCE TABLE
======================================================================

The module-reference table follows the resident-name table. Each entry
contains an offset for the module-name string within the imported-
names table; each entry is 2 bytes long.

   Size Description
   ---- -----------

   DW   Offset within Imported Names Table to referenced module name
        string.


======================================================================
                         IMPORTED-NAME TABLE
======================================================================

The imported-name table follows the module-reference table. This table
contains the names of modules and procedures that are imported by the
executable file. Each entry is composed of a 1-byte field that
contains the length of the string, followed by any number of
characters. The strings are not null-terminated and are case
sensitive.

   Size Description
   ---- -----------

   DB   Length of the name string that follows.

   DB   ASCII text of the name string.


======================================================================
                             ENTRY TABLE
======================================================================

The entry table follows the imported-name table. This table contains
bundles of entry-point definitions. Bundling is done to save space in
the entry table. The entry table is accessed by an ordinal value.
Ordinal number one is defined to index the first entry in the entry
table. To find an entry point, the bundles are scanned searching for a
specific entry point using an ordinal number. The ordinal number is
adjusted as each bundle is checked. When the bundle that contains the
entry point is found, the ordinal number is multiplied by the size of
the bundle's entries to index the proper entry.

The linker forms bundles in the most dense manner it can, under the
restriction that it cannot reorder entry points to improve bundling.
The reason for this restriction is that other .EXE files may refer to
entry points within this bundle by their ordinal number. The following
describes the format of the entry table bundles.

   Size Description
   ---- -----------

   DB   Number of entries in this bundle. All records in one bundle
        are either moveable or refer to the same fixed segment. A zero
        value in this field indicates the end of the entry table.

   DB   Segment indicator for this bundle. This defines the type of
        entry table entry data within the bundle. There are three
        types of entries that are defined.

        000h = Unused entries. There is no entry data in an unused
               bundle. The next bundle follows this field. This is
               used by the linker to skip ordinal numbers.

        001h-0FEh = Segment number for fixed segment entries. A fixed
               segment entry is 3 bytes long and has the following
               format.

            DB  Flag word.
                01h = Set if the entry is exported.
                02h = Set if the entry uses a global (shared) data
                      segments.
                      The first assembly-language instruction in the
                      entry point prologue must be "MOV AX,data
                      segment number". This may be set only for
                      SINGLEDATA library modules.

            DW  Offset within segment to entry point.

        0FFH = Moveable segment entries. The entry data contains the
               segment number for the entry points. A moveable segment
               entry is 6 bytes long and has the following format.

            DB  Flag word.
                01h = Set if the entry is exported.
                02h = Set if the entry uses a global (shared) data
                      segments.

            INT 3FH.

            DB  Segment number.

            DW  Offset within segment to entry point.


======================================================================
                        NONRESIDENT-NAME TABLE
======================================================================

The nonresident-name table follows the entry table, and contains a
module description and nonresident exported procedure name strings.
The first string in this table is a module description. These name
strings are case-sensitive and are not null-terminated. The name
strings follow the same format as those defined in the resident name
table.


======================================================================
                           PER SEGMENT DATA
======================================================================

The location and size of the per-segment data is defined in the
segment table entry for the segment. If the segment has relocation
fixups, as defined in the segment table entry flags, they directly
follow the segment data in the file. The relocation fixup information
is defined as follows:


   Size Description
   ---- -----------

   DW   Number of relocation records that follow.

   A table of relocation records follows. The following is the format
   of each relocation record.

        DB  Source type.
            0Fh = SOURCE_MASK
            00h = LOBYTE
            02h = SEGMENT
            03h = FAR_ADDR (32-bit pointer)
            05h = OFFSET (16-bit offset)

        DB  Flags byte.
            03h = TARGET_MASK
            00h = INTERNALREF
            01h = IMPORTORDINAL
            02h = IMPORTNAME
            03h = OSFIXUP
            04h = ADDITIVE

        DW  Offset within this segment of the source chain.
            If the ADDITIVE flag is set, then target value is added to
            the source contents, instead of replacing the source and
            following the chain. The source chain is an 0FFFFh
            terminated linked list within this segment of all
            references to the target.

        The target value has four types that are defined in the flag
        byte field. The following are the formats for each target
        type:

        INTERNALREF

            DB  Segment number for a fixed segment, or 0FFh for a
                movable segment.

            DB  0

            DW  Offset into segment if fixed segment, or ordinal
                number index into Entry Table if movable segment.

        IMPORTNAME

            DW  Index into module reference table for the imported
                module.

            DW  Offset within Imported Names Table to procedure name
                string.

        IMPORTORDINAL

            DW  Index into module reference table for the imported
                module.
            DW  Procedure ordinal number.

        OSFIXUP

            DW  Operating system fixup type.
                Floating-point fixups.
                0001h = FIARQQ, FJARQQ
                0002h = FISRQQ, FJSRQQ
                0003h = FICRQQ, FJCRQQ
                0004h = FIERQQ
                0005h = FIDRQQ
                0006h = FIWRQQ

            DW  0

======================================================================

Microsoft is a registered trademark and Windows is a trademark of
Microsoft Corporation.

Additional reference words: 3.0


http://bytepointer.com/resources/win16_ne_exe_format_win3.0.htm
https://wiki.osdev.org/NE#NE_2
https://github.com/libyal/libexe/blob/master/documentation/Executable%20(EXE)%20file%20format.asciidoc
http://www.idea2ic.com/File_Formats/exe-win.pdf
http://www.fileformat.info/format/exe/corion-ne.htm

https://www.asciitable.com/

## ASCII Table

CODE	HEX	CHAR	
0	00	NUL	Null
1	01	SOH	Start of heading
2	02	STX	Start of text
3	03	ETX	End of text
4	04	EOT	End of trans.
5	05	ENQ	Enquiry
6	06	ACK	Ack.
7	07	BEL	Bell
8	08	BS	Back space
9	09	HT	Horizontal tab
10	0A	LF	Line feed
11	0B	VT	Vertical tab
12	0C	FF	Form feed
13	0D	CR	Carriage return
14	0E	SO	Shift out
15	0F	SI	Shift in
16	10	DLE	Data line escape
17	11	DC1	Device control 1
18	12	DC2	Device control 2
19	13	DC3	Device control 3
20	14	DC4	Device control 4
21	15	NAK	Negative ack.
22	16	SYN	Synchronous idle
23	17	ETB	End of block
24	18	CAN	Cancel
25	19	EM	End of medium
26	1A	SUB	Substitute
27	1B	ESC	Escape
28	1C	FS	File separator
29	1D	GS	Group separator
30	1E	RS	Record separator
31	1F	US	Unit separator
32	20		Space
33	21	!	
34	22	"	&quot;
35	23	#	
36	24	$	
37	25	%	
38	26	&	&amp;
39	27	'	
40	28	(	
41	29	)	
42	2A	*	
43	2B	+	
44	2C	,	
45	2D	-	
46	2E	.	
47	2F	/	
48	30	0	
49	31	1	
50	32	2	
51	33	3	
52	34	4	
53	35	5	
54	36	6	
55	37	7	
56	38	8	
57	39	9	
58	3A	:	
59	3B	;	
60	3C	<	&lt;
61	3D	=	
62	3E	>	&gt;
63	3F	?	
64	40	@	
65	41	A	
66	42	B	
67	43	C	
68	44	D	
69	45	E	
70	46	F	
71	47	G	
72	48	H	
73	49	I	
74	4A	J	
75	4B	K	
76	4C	L	
77	4D	M	
78	4E	N	
79	4F	O	
80	50	P	
81	51	Q	
82	52	R	
83	53	S	
84	54	T	
85	55	U	
86	56	V	
87	57	W	
88	58	X	
89	59	Y	
90	5A	Z	
91	5B	[	
92	5C	\	
93	5D	]	
94	5E	^	
95	5F	_	
96	60	`	
97	61	a	
98	62	b	
99	63	c	
100	64	d	
101	65	e	
102	66	f	
103	67	g	
104	68	h	
105	69	i	
106	6A	j	
107	6B	k	
108	6C	l	
109	6D	m	
110	6E	n	
111	6F	o	
112	70	p	
113	71	q	
114	72	r	
115	73	s	
116	74	t	
117	75	u	
118	76	v	
119	77	w	
120	78	x	
121	79	y	
122	7A	z	
123	7B	{	
124	7C	|	
125	7D	}	
126	7E	~	
127	7F		Delete
CODE	HEX	CHAR	
128	80	€	&euro;
129	81		
130	82	‚	&sbquo;
131	83	ƒ	&fnof;
132	84	„	&bdquo;
133	85	…	&hellip;
134	86	†	&dagger;
135	87	‡	&Dagger;
136	88	ˆ	&circ;
137	89	‰	&permil;
138	8A	Š	&Scaron;
139	8B	‹	&lsaquo;
140	8C	Œ	&OElig;
141	8D		
142	8E	Ž	
143	8F		
144	90		
145	91	‘	&lsquo;
146	92	’	&rsquo;
147	93	“	&ldquo;
148	94	”	&rdquo;
149	95	•	&bull;
150	96	–	&ndash;
151	97	—	&mdash;
152	98	˜	&tilde;
153	99	™	&trade;
154	9A	š	&scaron;
155	9B	›	&rsaquo;
156	9C	œ	&oelig;
157	9D		
158	9E	ž	
159	9F	ÿ	&yuml;
160	A0	 	&nbsp;
161	A1	¡	&iexcl;
162	A2	¢	&cent;
163	A3	£	&pound;
164	A4	¤	&curren;
165	A5	¥	&yen;
166	A6	¦	&brvbar;
167	A7	§	&sect;
168	A8	¨	&uml;
169	A9	©	&copy;
170	AA	ª	&ordf;
171	AB	«	&laquo;
172	AC	¬	&not;
173	AD	­	&shy;
174	AE	®	&reg;
175	AF	¯	&macr;
176	B0	°	&deg;
177	B1	±	&plusmn;
178	B2	²	&sup2;
179	B3	³	&sup3;
180	B4	´	&acute;
181	B5	µ	&micro;
182	B6	¶	&para;
183	B7	·	&middot;
184	B8	¸	&cedil;
185	B9	¹	&sup1;
186	BA	º	&ordm;
187	BB	»	&raquo;
188	BC	¼	&frac14;
189	BD	½	&frac12;
190	BE	¾	&frac34;
191	BF	¿	&iquest;
192	C0	À	&Agrave;
193	C1	Á	&Aacute;
194	C2	Â	&Acirc;
195	C3	Ã	&Atilde;
196	C4	Ä	&Auml;
197	C5	Å	&Aring;
198	C6	Æ	&AElig;
199	C7	Ç	&Ccedil;
200	C8	È	&Egrave;
201	C9	É	&Eacute;
202	CA	Ê	&Ecirc;
203	CB	Ë	&Euml;
204	CC	Ì	&Igrave;
205	CD	Í	&Iacute;
206	CE	Î	&Icirc;
207	CF	Ï	&Iuml;
208	D0	Ð	&ETH;
209	D1	Ñ	&Ntilde;
210	D2	Ò	&Ograve;
211	D3	Ó	&Oacute;
212	D4	Ô	&Ocirc;
213	D5	Õ	&Otilde;
214	D6	Ö	&Ouml;
215	D7	×	&times;
216	D8	Ø	&Oslash;
217	D9	Ù	&Ugrave;
218	DA	Ú	&Uacute;
219	DB	Û	&Ucirc;
220	DC	Ü	&Uuml;
221	DD	Ý	&Yacute;
222	DE	Þ	&THORN;
223	DF	ß	&szlig;
224	E0	à	&agrave;
225	E1	á	&aacute;
226	E2	â	&acirc;
227	E3	ã	&atilde;
228	E4	ä	&auml;
229	E5	å	&aring;
230	E6	æ	&aelig;
231	E7	ç	&ccedil;
232	E8	è	&egrave;
233	E9	é	&eacute;
234	EA	ê	&ecirc;
235	EB	ë	&euml;
236	EC	ì	&igrave;
237	ED	í	&iacute;
238	EE	î	&icirc;
239	EF	ï	&iuml;
240	F0	ð	&eth;
241	F1	ñ	&ntilde;
242	F2	ò	&ograve;
243	F3	ó	&oacute;
244	F4	ô	&ocirc;
245	F5	õ	&otilde;
246	F6	ö	&ouml;
247	F7	÷	&divide;
248	F8	ø	&oslash;
249	F9	ù	&ugrave;
250	FA	ú	&uacute;
251	FB	û	&ucirc;
252	FC	ü	&uuml;
253	FD	ý	&yacute;
254	FE	þ	&thorn;
255	FF	ÿ	&yuml;
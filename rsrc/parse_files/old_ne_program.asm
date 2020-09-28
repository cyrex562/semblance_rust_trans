;-----------------------------------------------
; DUMP.ASM -- OS/2 Hex Dump Program
; (c) 1988, Ziff Communications Co.
;-----------------------------------------------

EXTRN DosOpen:FAR, DosRead:FAR, DosWrite:FAR
EXTRN DosClose:FAR, DosExit:FAR

DOSSEG
.286
.MODEL SMALL
.STACK 200h
;--------------------------
.DATA ; Initialized Data Segment
;--------------------------

SyntaxMsg db 13, 10, "Syntax: DUMP filename"
db 13, 10, 10, "DUMP (c) 1988, Ziff Communications Co."
db 13, 10, "PC Magazine ", 254," Charles Petzold - 12/87"
db 13, 10
SyntaxMsgLen equ $ - SyntaxMsg

FileSpecMsg db "DUMP: File not found or cannot be opened"
FileSpecMsgLen equ $ - FileSpecMsg

Delimiters db 9, " ,;=", 0
Address dw 0, 0
;----------------------------
.DATA? ; Uninitialized Data Segment
;----------------------------
FileHandle dw ?
OpenAction dw ?
BytesRead dw ?
BytesWritten dw ?
InputBuffer db 16 dup (?)
OutputBuffer db 80 dup (?)
;--------------
.CODE ; Code Segment
;--------------

;----------------------------------------------
; Parse command line to get file specification
;----------------------------------------------

Entry: Push DS ; Data segment selector
Pop ES ; Transfer it to ES
Mov DS, AX ; DS = Environment selector
Mov SI, BX ; SI = Start of command line

SkipProgName: Lodsb ; Pull a command line byte
Or AL, AL ; Check if it's zero
Jnz SkipProgName ; If not, continue

SkipDelims: Lodsb ; Get byte from parameter
Or AL, AL ; Check if it's zero
Jz DisplaySyntax ; If so, exit with message

Mov DI, Offset Delimiters ; Check if delimiter
Mov CX, 5 ; There are 5 of them
Repne Scasb ; Scan the string
Jz SkipDelims ; If delimiter, loop back

Mov DX, SI ; Pointer to file name + 1
Dec DX ; Pointer to file name

SearchFileEnd: Lodsb ; Get a byte
Mov DI, Offset Delimiters ; Check if delimiter
Mov CX, 6 ; 6 of them now including 0
Repne Scasb ; Scan the string
Jnz SearchFileEnd ; If not delimiter, do it again

Mov Byte Ptr [SI - 1], 0 ; Terminate with zero byte
Jmp Short OpenFile

;------------
; Error Exit
;------------

DisplaySyntax: Push ES ; Otherwise, set DS to
Pop DS ; data segment
Mov DX, Offset SyntaxMsg ; Syntax message offset
Mov CX, SyntaxMsgLen ; Syntax message length

ErrorExit: Push 2 ; Standard error handle
Push DS ; Segment of string
Push DX ; Offset of string
Push CX ; Length of string
Push DS ; Segment for bytes written
Push Offset BytesWritten ; Offset for bytes written
Call DosWrite ; Display the message

Push 1 ; Terminte all threads
Push 1 ; Return code
Call DosExit ; Exit

;-------------------------
; Open file using DosOpen
;-------------------------

OpenFile: Push DS ; Exchange DS and ES
Push ES
Pop DS
Pop ES

Push ES ; Segment of file name
Push DX ; Offset of file name
Push DS ; Segment for file handle
Push Offset FileHandle ; Offset for file handle
Push DS ; Segment for action
Push Offset OpenAction ; Offset for action
Push 0 ; High size (not used)
Push 0 ; Low size (not used)
Push 0 ; Attribute (not used)
Push 1 ; Open flag (open if file exists)
Push 20h ; Open mode (read, deny write)
Push 0 ; Reserved
Push 0 ; Reserved
Call DosOpen ; Open the file

Or AX, AX ; Check for error
Jz ReadAndDump ; If none, continue

Mov DX, Offset FileSpecMsg ; Error message
Mov CX, FileSpecMsgLen ; Length of message

Jmp ErrorExit ; Print it and exit

;---------------------------------------------
; Read 16 bytes from file and display address
;---------------------------------------------

ReadAndDump: Push DS ; Set ES to DS
Pop ES

MainLoop: Push [FileHandle] ; File Handle
Push DS ; Segment of buffer
Push Offset InputBuffer ; Offset of buffer
Push 16 ; Length of buffer
Push DS ; Segment for bytes read
Push Offset BytesRead ; Offset for bytes read
Call DosRead ; Read the file

Cmp [BytesRead], 0 ; See if no bytes read
Jnz DoLine ; If bytes read, continue

Jmp AllDone ; If zero bytes read, end program

DoLine: Mov DI, Offset OutputBuffer ; Destination

Mov AX, [Address + 2] ; High address within file
Call HexWordOut ; Display the value
Mov AL, '-' ; A dash
Stosb
Mov AX, [Address] ; Low offset within file
Call HexWordOut ; Display it

Mov AL, ' ' ; Follow it with a blank
Stosb

;---------------------
; Prepare Hex Display
;---------------------

Mov SI, Offset InputBuffer ; Source
Mov CX, [BytesRead] ; Number of bytes read
Sub BX, BX ; Initialize counter

HexLoop: Mov AL, ' ' ; A blank normally precedes
Cmp BX, 8 ; But check if counter is 8
Jnz DoTheSpace ; If not, a blank is OK

Mov AL, '-' ; Otherwise, use a dash
DoTheSpace: Stosb ; Display it

Lodsb ; Get a byte
Call HexByteOut ; Convert it to hex

Inc BX ; Kick up the counter
Loop HexLoop ; And loop for next byte

Mov CX, 16 ; 16 possible bytes
Sub CX, BX ; Subtract those already done
IMul CX, 3 ; Multiply by 3
Add CX, 2 ; And add 2 for the end
Mov AL, ' ' ; This is all blank
Rep Stosb ; Store the blanks

;---------------------------
; Prepare Character Display
;---------------------------

Mov SI, Offset InputBuffer ; Source
Mov CX, [BytesRead] ; Number of bytes

CharLoop: Lodsb ; Get the character
Cmp AL, ' ' ; See if displayable ASCII
Jb DisplayPeriod ; If below space, use dot

Cmp AL, 7Fh ; If DEL or above, use dot
Jb DisplayChar

DisplayPeriod: Mov AL, '.' ; Display period

DisplayChar: Stosb ; Display character
Loop CharLoop ; Do next character

Mov AL, 13 ; Carriage return
Stosb
Mov AL, 10 ; And line feed
Stosb

;-----------------------------
; Display Line using DosWrite
;-----------------------------

Sub DI, Offset OutputBuffer ; Length of output line

Push 1 ; File Handle: Standard Output
Push DS ; Segment of buffer
Push Offset OutputBuffer ; Offset of buffer
Push DI ; Length of buffer
Push DS ; Segment for bytes written
Push Offset BytesWritten ; Offset for bytes written
Call DosWrite ; Display it

Add [Address], 16 ; Kick up address
Adc [Address + 2], 0
Jmp MainLoop ; Do another line

AllDone: Push [FileHandle] ; File handle
Call DosClose ; Close the file

Push 1 ; Exit program
Push 0
Call DosExit

;----------------------------------
; Subroutines to store line output
;----------------------------------

HexWordOut: Xchg AL, AH
Call HexByteOut ; Do high byte first
Xchg AL, AH
; Fall through
HexByteOut: Push AX
Shr AL, 4
Call HexDigitOut ; Do high digit first
Pop AX
And AL, 0Fh
; Fall through
HexDigitOut: Add AL, 90h
Daa ; Common trick to convert
Adc AL, 40h ; hex digit to ASCII
Daa

Stosb ; Store in line buffer
Ret

END Entry

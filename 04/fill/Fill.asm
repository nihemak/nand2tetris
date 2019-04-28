// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
(LOOP_KBD)
        // select color
        @KBD
        D=M
        @SELECT_WHITE
        D; JEQ
        @0
        D=A-1
        @SET_COLOR
        0; JMP
(SELECT_WHITE)
        @0
        D=A
(SET_COLOR)
        @color
        M=D

        @SCREEN
        D=A
        @pos
        M=D

        // 32 * 256 = 8192
        @8192
        D=A
        @n
        M=D

(LOOP_FILL)
        @n
        D=M
        @FILL_END
        D; JEQ

        // print color
        @color
        D=M
        @pos
        A=M
        M=D

        @pos
        M=M+1
        @n
        M=M-1

        @LOOP_FILL
        0; JMP
(FILL_END)
        @LOOP_KBD
        0; JMP
#!/bin/bash

cp -r ./nand2tetris/projects/12/MemoryTest 12/1_Memory/ && \
cp -r 12/1_Memory/Memory.jack 12/1_Memory/MemoryTest/

./nand2tetris/tools/JackCompiler.sh 12/1_Memory/MemoryTest

# ./nand2tetris/tools/VMEmulator.sh
#   12/1_Memory/MemoryTest/MemoryTest.tst

cp -r ./nand2tetris/projects/12/ArrayTest 12/2_Array/ && \
cp -r 12/2_Array/Array.jack 12/2_Array/ArrayTest/

./nand2tetris/tools/JackCompiler.sh 12/2_Array/ArrayTest

# ./nand2tetris/tools/VMEmulator.sh
#   12/2_Array/ArrayTest/ArrayTest.tst

cp -r ./nand2tetris/projects/12/MathTest 12/3_Math/ && \
cp -r 12/3_Math/Math.jack 12/3_Math/MathTest/

./nand2tetris/tools/JackCompiler.sh 12/3_Math/MathTest

# ./nand2tetris/tools/VMEmulator.sh
#   12/3_Math/MathTest/MathTest.tst

cp -r ./nand2tetris/projects/12/StringTest 12/4_String/ && \
cp -r 12/4_String/String.jack 12/4_String/StringTest/

./nand2tetris/tools/JackCompiler.sh 12/4_String/StringTest

# ./nand2tetris/tools/VMEmulator.sh
#   12/4_String/StringTest/

cp -r ./nand2tetris/projects/12/OutputTest 12/5_Output/ && \
cp -r 12/5_Output/Output.jack 12/5_Output/OutputTest/

./nand2tetris/tools/JackCompiler.sh 12/5_Output/OutputTest

# ./nand2tetris/tools/VMEmulator.sh
#   12/5_Output/OutputTest/

cp -r ./nand2tetris/projects/12/ScreenTest 12/6_Screen/ && \
cp -r 12/6_Screen/Screen.jack 12/6_Screen/ScreenTest/

./nand2tetris/tools/JackCompiler.sh 12/6_Screen/ScreenTest

# ./nand2tetris/tools/VMEmulator.sh
#   12/6_Screen/ScreenTest/

cp -r ./nand2tetris/projects/12/KeyboardTest 12/7_Keyboard/ && \
cp -r 12/7_Keyboard/Keyboard.jack 12/7_Keyboard/KeyboardTest/

./nand2tetris/tools/JackCompiler.sh 12/7_Keyboard/KeyboardTest

# ./nand2tetris/tools/VMEmulator.sh
#   12/7_Keyboard/KeyboardTest/

cp -r ./nand2tetris/projects/12/SysTest 12/8_Sys/ && \
cp -r 12/8_Sys/Sys.jack 12/8_Sys/SysTest/

./nand2tetris/tools/JackCompiler.sh 12/8_Sys/SysTest

# ./nand2tetris/tools/VMEmulator.sh
#   12/8_Sys/SysTest/

cp -r ./nand2tetris/projects/11/Pong 12/9_All/ && \
cp -r 12/1_Memory/Memory.jack 12/9_All/Pong/ && \
cp -r 12/2_Array/Array.jack 12/9_All/Pong/ && \
cp -r 12/3_Math/Math.jack 12/9_All/Pong/ && \
cp -r 12/4_String/String.jack 12/9_All/Pong/ && \
cp -r 12/5_Output/Output.jack 12/9_All/Pong/ && \
cp -r 12/6_Screen/Screen.jack 12/9_All/Pong/ && \
cp -r 12/7_Keyboard/Keyboard.jack 12/9_All/Pong/ && \
cp -r 12/8_Sys/Sys.jack 12/9_All/Pong/ && \

./nand2tetris/tools/JackCompiler.sh 12/9_All/Pong

# ./nand2tetris/tools/VMEmulator.sh
#   12/9_All/Pong/

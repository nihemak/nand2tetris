#!/bin/bash

cp ./nand2tetris/projects/7/StackArithmetic/SimpleAdd/* 07/VMtranslator1/
cp ./nand2tetris/projects/7/StackArithmetic/StackTest/* 07/VMtranslator1/

cd 07/VMtranslator1/

clang --std=c11 -Wall -Wextra -o VMtranslator main.c Parser.c ParserPrivate.c CodeWriter.c CodeWriterPrivate.c

./VMtranslator SimpleAdd.vm 
./VMtranslator StackTest.vm 

cd -

./nand2tetris/tools/CPUEmulator.sh 07/VMtranslator1/SimpleAdd.tst
./nand2tetris/tools/CPUEmulator.sh 07/VMtranslator1/StackTest.tst

cp ./nand2tetris/projects/7/StackArithmetic/SimpleAdd/* 07/VMtranslator2/
cp ./nand2tetris/projects/7/StackArithmetic/StackTest/* 07/VMtranslator2/
cp ./nand2tetris/projects/7/MemoryAccess/BasicTest/* 07/VMtranslator2/
cp ./nand2tetris/projects/7/MemoryAccess/PointerTest/* 07/VMtranslator2/
cp ./nand2tetris/projects/7/MemoryAccess/StaticTest/* 07/VMtranslator2/

cd 07/VMtranslator2/

clang --std=c11 -Wall -Wextra -o VMtranslator main.c Parser.c ParserPrivate.c CodeWriter.c CodeWriterPrivate.c

./VMtranslator SimpleAdd.vm 
./VMtranslator StackTest.vm 
./VMtranslator BasicTest.vm 
./VMtranslator PointerTest.vm 
./VMtranslator StaticTest.vm 

cd -

./nand2tetris/tools/CPUEmulator.sh 07/VMtranslator2/SimpleAdd.tst
./nand2tetris/tools/CPUEmulator.sh 07/VMtranslator2/StackTest.tst
./nand2tetris/tools/CPUEmulator.sh 07/VMtranslator2/BasicTest.tst
./nand2tetris/tools/CPUEmulator.sh 07/VMtranslator2/PointerTest.tst
./nand2tetris/tools/CPUEmulator.sh 07/VMtranslator2/StaticTest.tst

#!/bin/bash

cp ./nand2tetris/projects/7/StackArithmetic/SimpleAdd/* 08/VMtranslator3/
cp ./nand2tetris/projects/7/StackArithmetic/StackTest/* 08/VMtranslator3/
cp ./nand2tetris/projects/7/MemoryAccess/BasicTest/* 08/VMtranslator3/
cp ./nand2tetris/projects/7/MemoryAccess/PointerTest/* 08/VMtranslator3/
cp ./nand2tetris/projects/7/MemoryAccess/StaticTest/* 08/VMtranslator3/
cp ./nand2tetris/projects/8/ProgramFlow/BasicLoop/* 08/VMtranslator3/
cp ./nand2tetris/projects/8/ProgramFlow/FibonacciSeries/* 08/VMtranslator3/

cd 08/VMtranslator3/

clang --std=c11 -Wall -Wextra -o VMtranslator main.c Parser.c ParserPrivate.c CodeWriter.c CodeWriterPrivate.c

./VMtranslator SimpleAdd.vm 
./VMtranslator StackTest.vm 
./VMtranslator BasicTest.vm 
./VMtranslator PointerTest.vm 
./VMtranslator StaticTest.vm 
./VMtranslator BasicLoop.vm 
./VMtranslator FibonacciSeries.vm 

cd -

./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator3/SimpleAdd.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator3/StackTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator3/BasicTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator3/PointerTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator3/StaticTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator3/BasicLoop.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator3/FibonacciSeries.tst

cp ./nand2tetris/projects/7/StackArithmetic/SimpleAdd/* 08/VMtranslator4/
cp ./nand2tetris/projects/7/StackArithmetic/StackTest/* 08/VMtranslator4/
cp ./nand2tetris/projects/7/MemoryAccess/BasicTest/* 08/VMtranslator4/
cp ./nand2tetris/projects/7/MemoryAccess/PointerTest/* 08/VMtranslator4/
cp ./nand2tetris/projects/7/MemoryAccess/StaticTest/* 08/VMtranslator4/
cp ./nand2tetris/projects/8/ProgramFlow/BasicLoop/* 08/VMtranslator4/
cp ./nand2tetris/projects/8/ProgramFlow/FibonacciSeries/* 08/VMtranslator4/
cp ./nand2tetris/projects/8/FunctionCalls/SimpleFunction/* 08/VMtranslator4/

cd 08/VMtranslator4/

clang --std=c11 -Wall -Wextra -o VMtranslator main.c Parser.c ParserPrivate.c CodeWriter.c CodeWriterPrivate.c

./VMtranslator SimpleAdd.vm 
./VMtranslator StackTest.vm 
./VMtranslator BasicTest.vm 
./VMtranslator PointerTest.vm 
./VMtranslator StaticTest.vm 
./VMtranslator BasicLoop.vm 
./VMtranslator FibonacciSeries.vm 
./VMtranslator SimpleFunction.vm 

cd -

./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator4/SimpleAdd.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator4/StackTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator4/BasicTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator4/PointerTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator4/StaticTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator4/BasicLoop.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator4/FibonacciSeries.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator4/SimpleFunction.tst

cp -r ./nand2tetris/projects/8/FunctionCalls/FibonacciElement 08/VMtranslator5/
cp -r ./nand2tetris/projects/8/FunctionCalls/StaticsTest 08/VMtranslator5/
cp -r ./nand2tetris/projects/8/FunctionCalls/NestedCall 08/VMtranslator5/

cd 08/VMtranslator5/

clang --std=c11 -Wall -Wextra -o VMtranslator main.c Parser.c ParserPrivate.c CodeWriter.c CodeWriterPrivate.c

./VMtranslator FibonacciElement
./VMtranslator StaticsTest
./VMtranslator NestedCall

cd -

./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator5/FibonacciElement/FibonacciElement.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator5/StaticsTest/StaticsTest.tst
./nand2tetris/tools/CPUEmulator.sh 08/VMtranslator5/NestedCall/NestedCall.tst

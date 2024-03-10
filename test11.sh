#!/bin/bash

cp -r ./nand2tetris/projects/10/Square 11/JackCompiler/ && \
mkdir -p 11/JackCompiler/Square/expect && \
mv 11/JackCompiler/Square/*.xml 11/JackCompiler/Square/expect/
patch -p1 -d 11/JackCompiler/Square/expect < 11/JackCompiler/test/Square.patch

cp -r ./nand2tetris/projects/10/ExpressionLessSquare 11/JackCompiler/ && \
mkdir -p 11/JackCompiler/ExpressionLessSquare/expect && \
mv 11/JackCompiler/ExpressionLessSquare/*.xml 11/JackCompiler/ExpressionLessSquare/expect/
patch -p1 -d 11/JackCompiler/ExpressionLessSquare/expect < 11/JackCompiler/test/ExpressionLessSquare.patch

cp -r ./nand2tetris/projects/10/ArrayTest 11/JackCompiler/ && \
mkdir -p 11/JackCompiler/ArrayTest/expect && \
mv 11/JackCompiler/ArrayTest/*.xml 11/JackCompiler/ArrayTest/expect/
patch -p1 -d 11/JackCompiler/ArrayTest/expect < 11/JackCompiler/test/ArrayTest.patch

# memo: create path
# diff -u -w -r expect expect_fix > ArrayTest.patch
# https://mrgoofy.hatenablog.com/entry/20101019/1287500809
# https://ex1.m-yabe.com/archives/6357

cd 11/JackCompiler/

clang --std=c11 -Wall -Wextra -o JackCompiler main.c JackTokenizer.c JackTokenizerPrivate.c SymbolTable.c SymbolTablePrivate.c CompilationEngine.c

./JackCompiler Square
./JackCompiler ExpressionLessSquare
./JackCompiler ArrayTest

cd -

./nand2tetris/tools/TextComparer.sh 11/JackCompiler/Square/expect/Main.xml 11/JackCompiler/Square/Main.xml 
./nand2tetris/tools/TextComparer.sh 11/JackCompiler/Square/expect/Square.xml 11/JackCompiler/Square/Square.xml 
./nand2tetris/tools/TextComparer.sh 11/JackCompiler/Square/expect/SquareGame.xml 11/JackCompiler/Square/SquareGame.xml 
./nand2tetris/tools/TextComparer.sh 11/JackCompiler/ExpressionLessSquare/expect/Main.xml 11/JackCompiler/ExpressionLessSquare/Main.xml
./nand2tetris/tools/TextComparer.sh 11/JackCompiler/ExpressionLessSquare/expect/Square.xml 11/JackCompiler/ExpressionLessSquare/Square.xml
./nand2tetris/tools/TextComparer.sh 11/JackCompiler/ExpressionLessSquare/expect/SquareGame.xml 11/JackCompiler/ExpressionLessSquare/SquareGame.xml
./nand2tetris/tools/TextComparer.sh 11/JackCompiler/ArrayTest/expect/Main.xml 11/JackCompiler/ArrayTest/Main.xml 

cp -r ./nand2tetris/projects/11/Seven 11/JackCompiler2/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler2/Seven/

cd 11/JackCompiler2/

clang --std=c11 -Wall -Wextra -o JackCompiler main.c JackTokenizer.c JackTokenizerPrivate.c SymbolTable.c SymbolTablePrivate.c VMWriter.c CompilationEngine.c

./JackCompiler Seven

cd -

# ./nand2tetris/tools/VMEmulator.sh
#   11/JackCompiler2/Seven

cp -r ./nand2tetris/projects/11/Seven 11/JackCompiler3/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler3/Seven/

cp -r ./nand2tetris/projects/11/ConvertToBin 11/JackCompiler3/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler3/ConvertToBin/

cd 11/JackCompiler3/

clang --std=c11 -Wall -Wextra -o JackCompiler main.c JackTokenizer.c JackTokenizerPrivate.c SymbolTable.c SymbolTablePrivate.c VMWriter.c CompilationEngine.c

./JackCompiler Seven
./JackCompiler ConvertToBin

cd -

# ./nand2tetris/tools/VMEmulator.sh
#   11/JackCompiler3/Seven
#   11/JackCompiler3/ConvertToBin

cp -r ./nand2tetris/projects/11/Seven 11/JackCompiler4/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler4/Seven/

cp -r ./nand2tetris/projects/11/ConvertToBin 11/JackCompiler4/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler4/ConvertToBin/

cp -r ./nand2tetris/projects/11/Square 11/JackCompiler4/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler4/Square/

cd 11/JackCompiler4/

clang --std=c11 -Wall -Wextra -o JackCompiler main.c JackTokenizer.c JackTokenizerPrivate.c SymbolTable.c SymbolTablePrivate.c VMWriter.c CompilationEngine.c

./JackCompiler Seven
./JackCompiler ConvertToBin
./JackCompiler Square

cd -

# ./nand2tetris/tools/VMEmulator.sh
#   11/JackCompiler4/Seven
#   11/JackCompiler4/ConvertToBin
#   11/JackCompiler4/Square

cp -r ./nand2tetris/projects/11/Seven 11/JackCompiler5/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler5/Seven/

cp -r ./nand2tetris/projects/11/ConvertToBin 11/JackCompiler5/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler5/ConvertToBin/

cp -r ./nand2tetris/projects/11/Square 11/JackCompiler5/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler5/Square/

cp -r ./nand2tetris/projects/11/Average 11/JackCompiler5/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler5/Average/

cd 11/JackCompiler5/

clang --std=c11 -Wall -Wextra -o JackCompiler main.c JackTokenizer.c JackTokenizerPrivate.c SymbolTable.c SymbolTablePrivate.c VMWriter.c CompilationEngine.c

./JackCompiler Seven
./JackCompiler ConvertToBin
./JackCompiler Square
./JackCompiler Average

cd -

# ./nand2tetris/tools/VMEmulator.sh
#   11/JackCompiler5/Seven
#   11/JackCompiler5/ConvertToBin
#   11/JackCompiler5/Square
#   11/JackCompiler5/Average

cp -r ./nand2tetris/projects/11/Seven 11/JackCompiler6/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler6/Seven/

cp -r ./nand2tetris/projects/11/ConvertToBin 11/JackCompiler6/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler6/ConvertToBin/

cp -r ./nand2tetris/projects/11/Square 11/JackCompiler6/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler6/Square/

cp -r ./nand2tetris/projects/11/Average 11/JackCompiler6/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler6/Average/

cp -r ./nand2tetris/projects/11/Pong 11/JackCompiler6/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler6/Pong/

cp -r ./nand2tetris/projects/11/ComplexArrays 11/JackCompiler6/ && \
cp -r ./nand2tetris/tools/OS/* 11/JackCompiler6/ComplexArrays/

cd 11/JackCompiler6/

clang --std=c11 -Wall -Wextra -o JackCompiler main.c JackTokenizer.c JackTokenizerPrivate.c SymbolTable.c SymbolTablePrivate.c VMWriter.c CompilationEngine.c

./JackCompiler Seven
./JackCompiler ConvertToBin
./JackCompiler Square
./JackCompiler Average
./JackCompiler Pong
./JackCompiler ComplexArrays

cd -

# ./nand2tetris/tools/VMEmulator.sh
#   11/JackCompiler6/Seven
#   11/JackCompiler6/ConvertToBin
#   11/JackCompiler6/Square
#   11/JackCompiler6/Average
#   11/JackCompiler6/Pong
#   11/JackCompiler6/ComplexArrays

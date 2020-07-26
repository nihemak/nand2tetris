#!/bin/bash

cp -r ./nand2tetris/projects/10/Square 10/JackAnalyzer/ && \
mkdir -p 10/JackAnalyzer/Square/expect && \
mv 10/JackAnalyzer/Square/*.xml 10/JackAnalyzer/Square/expect/

cp -r ./nand2tetris/projects/10/ExpressionLessSquare 10/JackAnalyzer/ && \
mkdir -p 10/JackAnalyzer/ExpressionLessSquare/expect && \
mv 10/JackAnalyzer/ExpressionLessSquare/*.xml 10/JackAnalyzer/ExpressionLessSquare/expect/

cp -r ./nand2tetris/projects/10/ArrayTest 10/JackAnalyzer/ && \
mkdir -p 10/JackAnalyzer/ArrayTest/expect && \
mv 10/JackAnalyzer/ArrayTest/*.xml 10/JackAnalyzer/ArrayTest/expect/

cd 10/JackAnalyzer/

clang --std=c11 -Wall -Wextra -o JackAnalyzer main.c JackTokenizer.c JackTokenizerPrivate.c

./JackAnalyzer Square
./JackAnalyzer ExpressionLessSquare
./JackAnalyzer ArrayTest

cd -

./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer/Square/expect/MainT.xml 10/JackAnalyzer/Square/MainT.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer/Square/expect/SquareT.xml 10/JackAnalyzer/Square/SquareT.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer/Square/expect/SquareGameT.xml 10/JackAnalyzer/Square/SquareGameT.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer/ExpressionLessSquare/expect/MainT.xml 10/JackAnalyzer/ExpressionLessSquare/MainT.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer/ExpressionLessSquare/expect/SquareT.xml 10/JackAnalyzer/ExpressionLessSquare/SquareT.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer/ExpressionLessSquare/expect/SquareGameT.xml 10/JackAnalyzer/ExpressionLessSquare/SquareGameT.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer/ArrayTest/expect/MainT.xml 10/JackAnalyzer/ArrayTest/MainT.xml 


cp -r ./nand2tetris/projects/10/ExpressionLessSquare 10/JackAnalyzer2/ && \
mkdir -p 10/JackAnalyzer2/ExpressionLessSquare/expect && \
mv 10/JackAnalyzer2/ExpressionLessSquare/*.xml 10/JackAnalyzer2/ExpressionLessSquare/expect/

cd 10/JackAnalyzer2/

clang --std=c11 -Wall -Wextra -o JackAnalyzer main.c JackTokenizer.c JackTokenizerPrivate.c CompilationEngine.c

./JackAnalyzer ExpressionLessSquare

cd -

./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer2/ExpressionLessSquare/expect/Main.xml 10/JackAnalyzer2/ExpressionLessSquare/Main.xml
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer2/ExpressionLessSquare/expect/Square.xml 10/JackAnalyzer2/ExpressionLessSquare/Square.xml
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer2/ExpressionLessSquare/expect/SquareGame.xml 10/JackAnalyzer2/ExpressionLessSquare/SquareGame.xml


cp -r ./nand2tetris/projects/10/Square 10/JackAnalyzer3/ && \
mkdir -p 10/JackAnalyzer3/Square/expect && \
mv 10/JackAnalyzer3/Square/*.xml 10/JackAnalyzer3/Square/expect/

cp -r ./nand2tetris/projects/10/ExpressionLessSquare 10/JackAnalyzer3/ && \
mkdir -p 10/JackAnalyzer3/ExpressionLessSquare/expect && \
mv 10/JackAnalyzer3/ExpressionLessSquare/*.xml 10/JackAnalyzer3/ExpressionLessSquare/expect/

cp -r ./nand2tetris/projects/10/ArrayTest 10/JackAnalyzer3/ && \
mkdir -p 10/JackAnalyzer3/ArrayTest/expect && \
mv 10/JackAnalyzer3/ArrayTest/*.xml 10/JackAnalyzer3/ArrayTest/expect/

cd 10/JackAnalyzer3/

clang --std=c11 -Wall -Wextra -o JackAnalyzer main.c JackTokenizer.c JackTokenizerPrivate.c CompilationEngine.c

./JackAnalyzer Square
./JackAnalyzer ExpressionLessSquare
./JackAnalyzer ArrayTest

cd -

./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer3/Square/expect/Main.xml 10/JackAnalyzer3/Square/Main.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer3/Square/expect/Square.xml 10/JackAnalyzer3/Square/Square.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer3/Square/expect/SquareGame.xml 10/JackAnalyzer3/Square/SquareGame.xml 
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer3/ExpressionLessSquare/expect/Main.xml 10/JackAnalyzer3/ExpressionLessSquare/Main.xml
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer3/ExpressionLessSquare/expect/Square.xml 10/JackAnalyzer3/ExpressionLessSquare/Square.xml
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer3/ExpressionLessSquare/expect/SquareGame.xml 10/JackAnalyzer3/ExpressionLessSquare/SquareGame.xml
./nand2tetris/tools/TextComparer.sh 10/JackAnalyzer3/ArrayTest/expect/Main.xml 10/JackAnalyzer3/ArrayTest/Main.xml 

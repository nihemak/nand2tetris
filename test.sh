#!/bin/bash

./nand2tetris/tools/HardwareSimulator.sh 01/Not.tst
./nand2tetris/tools/HardwareSimulator.sh 01/And.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Or.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Xor.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Mux.tst
./nand2tetris/tools/HardwareSimulator.sh 01/DMux.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Not16.tst
./nand2tetris/tools/HardwareSimulator.sh 01/And16.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Or16.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Mux16.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Or8Way.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Mux4Way16.tst
./nand2tetris/tools/HardwareSimulator.sh 01/Mux8Way16.tst
./nand2tetris/tools/HardwareSimulator.sh 01/DMux4Way.tst
./nand2tetris/tools/HardwareSimulator.sh 01/DMux8Way.tst

./nand2tetris/tools/HardwareSimulator.sh 02/HalfAdder.tst
./nand2tetris/tools/HardwareSimulator.sh 02/FullAdder.tst
./nand2tetris/tools/HardwareSimulator.sh 02/Add16.tst
./nand2tetris/tools/HardwareSimulator.sh 02/Inc16.tst
./nand2tetris/tools/HardwareSimulator.sh 02/ALU-nostat.tst
./nand2tetris/tools/HardwareSimulator.sh 02/ALU.tst

./nand2tetris/tools/HardwareSimulator.sh 03/a/Bit.tst
./nand2tetris/tools/HardwareSimulator.sh 03/a/Register.tst
./nand2tetris/tools/HardwareSimulator.sh 03/a/RAM8.tst
./nand2tetris/tools/HardwareSimulator.sh 03/a/RAM64.tst
./nand2tetris/tools/HardwareSimulator.sh 03/b/RAM512.tst
./nand2tetris/tools/HardwareSimulator.sh 03/b/RAM4K.tst
./nand2tetris/tools/HardwareSimulator.sh 03/b/RAM16K.tst
./nand2tetris/tools/HardwareSimulator.sh 03/a/PC.tst

./nand2tetris/tools/Assembler.sh 04/mult/mult.asm
./nand2tetris/tools/CPUEmulator.sh 04/mult/Mult.tst
./nand2tetris/tools/Assembler.sh 04/fill/Fill.asm
# This is commented out because it can not be tested automatically.
# ./nand2tetris/tools/CPUEmulator.sh 04/fill/Fill.tst
./nand2tetris/tools/CPUEmulator.sh 04/fill/FillAutomatic.tst

# This is commented out because it can not be tested automatically.
# ./nand2tetris/tools/HardwareSimulator.sh 05/Memory.tst
./nand2tetris/tools/HardwareSimulator.sh 05/CPU.tst
./nand2tetris/tools/HardwareSimulator.sh 05/CPU-external.tst
./nand2tetris/tools/HardwareSimulator.sh 05/ComputerAdd.tst
./nand2tetris/tools/HardwareSimulator.sh 05/ComputerAdd-external.tst
./nand2tetris/tools/HardwareSimulator.sh 05/ComputerMax.tst
./nand2tetris/tools/HardwareSimulator.sh 05/ComputerMax-external.tst
./nand2tetris/tools/HardwareSimulator.sh 05/ComputerRect.tst
./nand2tetris/tools/HardwareSimulator.sh 05/ComputerRect-external.tst

cp ./nand2tetris/projects/06/max/MaxL.asm 06/AssemblerL/
./nand2tetris/tools/Assembler.sh 06/AssemblerL/MaxL.asm
mv 06/AssemblerL/MaxL.hack 06/AssemblerL/MaxLExpected.hack

cp ./nand2tetris/projects/06/rect/RectL.asm 06/AssemblerL/
./nand2tetris/tools/Assembler.sh 06/AssemblerL/RectL.asm
mv 06/AssemblerL/RectL.hack 06/AssemblerL/RectLExpected.hack

cp ./nand2tetris/projects/06/pong/PongL.asm 06/AssemblerL/
./nand2tetris/tools/Assembler.sh 06/AssemblerL/PongL.asm
mv 06/AssemblerL/PongL.hack 06/AssemblerL/PongLExpected.hack

cd 06/AssemblerL/

clang --std=c11 -Wall -Wextra -o AssemblerL main.c Parser.c Code.c

echo "MaxL"
./AssemblerL MaxL.asm
diff MaxLExpected.hack MaxL.hack
echo $?

echo "RectL"
./AssemblerL RectL.asm
diff RectLExpected.hack RectL.hack
echo $?

echo "PongL"
./AssemblerL PongL.asm
diff PongLExpected.hack PongL.hack
echo $?

cd -

cp ./nand2tetris/projects/06/add/Add.asm 06/Assembler/
./nand2tetris/tools/Assembler.sh 06/Assembler/Add.asm
mv 06/Assembler/Add.hack 06/Assembler/AddExpected.hack

cp ./nand2tetris/projects/06/max/Max.asm 06/Assembler/
./nand2tetris/tools/Assembler.sh 06/Assembler/Max.asm
mv 06/Assembler/Max.hack 06/Assembler/MaxExpected.hack

cp ./nand2tetris/projects/06/rect/Rect.asm 06/Assembler/
./nand2tetris/tools/Assembler.sh 06/Assembler/Rect.asm
mv 06/Assembler/Rect.hack 06/Assembler/RectExpected.hack

cp ./nand2tetris/projects/06/pong/Pong.asm 06/Assembler/
./nand2tetris/tools/Assembler.sh 06/Assembler/Pong.asm
mv 06/Assembler/Pong.hack 06/Assembler/PongExpected.hack

cd 06/Assembler/

clang --std=c11 -Wall -Wextra -o Assembler main.c Parser.c Code.c SymbolTable.c

echo "Add"
./Assembler Add.asm
diff AddExpected.hack Add.hack
echo $?

echo "Max"
./Assembler Max.asm
diff MaxExpected.hack Max.hack
echo $?

echo "Rect"
./Assembler Rect.asm
diff RectExpected.hack Rect.hack
echo $?

echo "Pong"
./Assembler Pong.asm
diff PongExpected.hack Pong.hack
echo $?

cd -

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

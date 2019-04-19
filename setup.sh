#!/bin/bash

rm -rf nand2tetris*
wget "https://drive.google.com/uc?export=download&id=1KcFPj8KQ_QAHheFmLCqs5iqC_0NCndvs" -O nand2tetris.zip
unzip nand2tetris.zip
chmod +x ./nand2tetris/tools/*.sh

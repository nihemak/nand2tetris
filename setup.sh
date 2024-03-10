#!/bin/bash

rm -rf nand2tetris*
wget "https://drive.google.com/uc?export=download&id=1xZzcMIUETv3u3sdpM_oTJSTetpVee3KZ" -O nand2tetris.zip
unzip nand2tetris.zip
chmod +x ./nand2tetris/tools/*.sh

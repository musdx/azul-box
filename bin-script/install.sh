#!/bin/bash

RED='\033[0;31m'
YELLOW='\033[1;32m'



echo -e ${YELLOW}copy content

sudo cp logo.png /usr/share/icons/azul_box.png

sudo cp azul-box /usr/bin/azulbox

sudo chmod +x /usr/bin/azulbox

sudo cp azul_box.desktop /usr/share/applications/

echo -e ${RED}Remember to install dependencies! Check https://github.com/musdx/azul-box/blob/master/README.md#dependencies for more info

#!/bin/bash

RED='\033[0;31m'
YELLOW='\033[1;32m'



echo -e ${YELLOW}copy content

sudo install -Dm 644 logo.png /usr/share/icons/azul_box.png

sudo install -Dm 755 azul-box /usr/bin/azulbox

sudo install -Dm 644 azul_box.desktop /usr/share/applications/

echo -e ${RED}Remember to install dependencies! Check https://github.com/musdx/azul-box/blob/master/README.md#dependencies for more info

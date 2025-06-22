#!/bin/bash

RED='\033[0;31m'
YELLOW='\033[1;32m'

echo -e ${YELLOW}make directory

cargo build --release

echo -e ${YELLOW}copy content

sudo cp -r assets/logo.png /usr/share/icons/azul_box.png

sudo cp target/release/azul-box /usr/bin/azulbox

sudo cp desktop/azul_box.desktop /usr/share/applications/

echo -e ${RED}Remember to install dependencies! Check https://github.com/musdx/azul-box/blob/master/README.md#dependencies for more info

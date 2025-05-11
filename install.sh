#!/bin/bash

echo "make directory"

sudo mkdir /opt/azul_box/

cargo build --release

echo "copy content"

sudo cp -r assets/ /opt/azul_box/

sudo cp target/release/azul-box /opt/azul_box/

sudo cp desktop/azul_box.desktop ~/.local/share/applications/

echo Remember to install dependencies check https://github.com/musdx/azul-box/blob/master/README.md#dependencies for more

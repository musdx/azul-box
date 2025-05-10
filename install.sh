#!/bin/bash

sudo mkdir /opt/azul_box/

cargo build --release

echo "make directory"

echo "copy content"

sudo cp -r assets/ /opt/azul_box/

sudo cp target/release/azul-box /opt/azul_box/

sudo cp desktop/azul_box.desktop ~/.local/share/applications/

#!/bin/sh

cargo build --release

mv stfu-notify-daemon.service ~/.config/systemd/user/

cd /target/release/

strip client
strip server
strip gui

sudo mv client /usr/bin/stfu-notify
sudo mv server /usr/bin/stfu-notify-daemon
sudo mv gui /usr/bin/stfu-notify-gui

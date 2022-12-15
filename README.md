# Keyboard Controller GUI
This repo contains a GUI for controlling clevo keyboard backlight

## Required Setup
The [Tuxedo Keyboard](https://github.com/tuxedocomputers/tuxedo-keyboard) Kernel module must be installed before either tool in this repo can be used.

Copy keyctl to /usr/bin `cp ./target/debug/keyctlgui /usr/bin/`

To create a desktop shortcut copy the included .desktop file into a location your system reconizes ex:
`cp ./keyctl.desktop ~/.local/share/applications`

## GUI
The GUI as a slider for red, green, blue, and brightness as well as a hex entry mechanism. 

## Operating System
These programs have only been tested on my computer which is running KDE Neon but will work on other opterating systems that install sys files to the same location.
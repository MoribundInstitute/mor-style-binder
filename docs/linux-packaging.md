# Linux Packaging Notes

MorStyleBinder can be packaged as a normal Linux desktop utility.

## Desktop file

Suggested desktop entry:

```text
packaging/linux/morstylebinder.desktop

Install path:

/usr/share/applications/morstylebinder.desktop
Binary

Suggested binary install path:

/usr/bin/morstylebinder

The current release binary is built with:

cargo build --release

Output:

target/release/style_binder

A package may rename or install this as:

morstylebinder
Icons

Suggested icon source folder:

assets/icons/icons/hicolor/

Suggested install paths:

/usr/share/icons/hicolor/16x16/apps/morstylebinder.png
/usr/share/icons/hicolor/24x24/apps/morstylebinder.png
/usr/share/icons/hicolor/32x32/apps/morstylebinder.png
/usr/share/icons/hicolor/48x48/apps/morstylebinder.png
/usr/share/icons/hicolor/64x64/apps/morstylebinder.png
/usr/share/icons/hicolor/128x128/apps/morstylebinder.png
/usr/share/icons/hicolor/256x256/apps/morstylebinder.png
/usr/share/icons/hicolor/512x512/apps/morstylebinder.png
/usr/share/icons/hicolor/scalable/apps/morstylebinder.svg


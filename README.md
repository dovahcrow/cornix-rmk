# Non-official RMK firmware for Cornix keyboard from Jezail Funder Studio

Basically I want more customizations so I made this.

The LED light is not working. Help is welcomed!

## Build firmware

```shell
cargo make uf2
```

You will find the uf2 files in the project root.

Then you can flash the keyboard by reseting the keyboard and drag & drop the uf2 file to the keyboard (which is shown as a USB device in your file explorer).

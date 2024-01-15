#!/bin/sh
OUT=""$1".gba"
/opt/devkitpro/devkitARM/arm-none-eabi/bin/objcopy -O binary "$1" "$OUT"
gbafix "$OUT"
visualboyadvance-m "$OUT"

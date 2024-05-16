# Microbit Neopixels for rust

The Micropython port for the micro:bit have a neopixel module. The Makecode micro:bit editor have an extension for neopixel. But the rust crate for [micro:bit](github.com/nrf-rs/microbit) doesn't have a way to interface with neopixels.

**UNTIL NOW!**

This crate allows you to interface with neopixels with rust and the micro:bit.

## Prerequists

- arm-none-eabi toolchain

To work with this crate you need to install the arm-none-eabi toolchain, because we need the arm-none-eabi-as assembler, to assemble a program to send data from the micro:bit to the neopixel. You can install it with your systems package manager or through [the website](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads).

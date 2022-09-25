# Fixsliced GIFT-128 implementation

**This is an experimental library that has not been confirmed to be secured. Use at your own discretion.**

This project contains a fixsliced implementation of the block cipher algorithm GIFT-128 in two versions: a base one and
one with first-order masking applied. This library was implemented to highlight upsides and downsides to working with
cryptographic implementation in Rust with a focus on side-channel leakage through differential power analysis.

See [the associated binaries used for experimentation on a bare-metal target](https://github.com/NVedsted/cortex-gift)
for more information.

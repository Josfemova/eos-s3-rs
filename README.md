# eos-s3-rs

# WIP

Bringing support for the EOS-S3 in Rust.

### Embedded-hal traits implemented:

- [ ] ADC traits
- [ ] digital IO  (but must decide what to do with #3)
- [ ] delay/timer 
- [ ] serial UART 
- [ ] I2C 
- [ ] SPI 
- [ ] watchdog 

Traits like CAN and PWN cannot be implemented for the M4 subsystem which is the app we can modify from Rust, but it would be nice to have examples in which such things are implemented on the eFPGA.

### About other features

- Implementing the FPGA loading sequence is already possible. [Qorc implementation](https://github.com/QuickLogic-Corp/qorc-sdk/tree/master/Libraries/FPGA) can be used as a guide. 
- Sensors in the devboards don't seem to have Rust drivers as of now, so that's also on the backlog
- I2S & PDM is somewhat low priority, hal traits are the first stuff that I'll be working on. 
- Contributions are welcome. 

### Additional notes

- Higher integration with the symbiflow workflow is desirable. Perhaps if this can be achieved with a cargo subcommand ergonomics for hybrid (hdl + rust) workflows, it would make of Rust a comfy language for this kind of thing, which could also lead to it being the de-facto "thing". 
- This should help in the clock implementation: https://qorc-sdk.readthedocs.io/en/latest/guides/clock-power/clock-power.html
- The SVD is ready, but it would be nice to have QuickLogic check everything is in order. 
- The SVD does not contain the core CM4F peripherals, as those don't seem to be required for pac generation. If you are an user of the SVD but not the rust libraries, feel free to contribute the transcription of the core peripherals in the [yaml files already in the repo](https://github.com/Josfemova/eos-s3-rs/tree/main/svd/patches/core)
- I cannot overstate how useful it is to have the tools/libraries mantained by the Rust Embedded Working Group. The ecosystem is amazing, if you are somewhat new to embedded rust and somehow got to this place before any of the other projects, please check [this repo](https://github.com/rust-embedded/awesome-embedded-rust) - it has a list of all the nice things Embedded Rust has to offer. 

## Credit

The work on this repository is based on the information found on the [EOS-S3](https://github.com/QuickLogic-Corp/EOS-S3) and [qork-sdk](https://github.com/QuickLogic-Corp/qorc-sdk) repositories. Both of these projects are licensed under the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0). QuickLogic does not provide an SVD file for the EOS-S3, but it does provide an excel file with the specification of the memory map.

## License

The code in this repository is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

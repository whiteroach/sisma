# SIGMA

Keyboard MIDI-controller's firmware based on STM32F1 (blue-pill).

---

#### **Description:**

This repository aims to put together a working firmware for a MIDI-controller keyboard based on an STM32F1 microcontroller.
With this project, I aim to build and design from the bottom up a working prototype. Therefore this repository is intended to be an experimental environment from which to learn by doing different aspects of electronics and embedded programming using the Rust programming language. <br/>
I'll link under resources materials that I think are relevant for my development and for others to understand the different topics on which this project is based.  
**Sisma**, in Italian, means _telluric motion_, _earth tremor_.

#### **Build & Flash:**

This repository is based on [knurling-rs/app-template](https://github.com/knurling-rs/app-template) and uses [_probe-run_](https://github.com/knurling-rs/probe-run) and [_defmt_](https://github.com/knurling-rs/defmt) as a logger. If you want to try and flash on your STM32F1 microcontroller, connect your debugger to your machine and run in the terminal:

```
cargo rb main
```

#### **Resources:**

- https://docs.rs/stm32f1xx-hal/latest/stm32f1xx_hal/
- https://docs.rust-embedded.org/
- https://cgit.pinealservo.com/BluePill_Rust/resources/issues/1
- https://github.com/joaocarvalhoopen/stm32_bluepill_in_rust__Template

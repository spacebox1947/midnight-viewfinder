# Midnight Viewfinder

## Overview
`midnight-viewfinder` combines an `esp32c3-mini` with `no_std` and a partner wasm app via `wgpu`. The final goal of the project is to replace an ancient Arch Linux + Arduino system with a modern, safe, and efficient Linux + ESP32 application.

## Physical Installation
- A large interactive digital *Viewfinder*, similar to one you would find on a road trip at a scenic overview. A visitor can rotate the viewfinder on its post. 
- There are two buttons on the handles of the viewfinder, and internally a rotating axle.
- As the viewfinder rotates, the visitor can rotate through six categories of Alaskan historical text and imagery.
- With a click of either button, the visitor can enter the category, and then rotate through the images by moving the viwefinder, or tapping a button.

## Project Goals

### ESP32-c3 super mini (Other C3 RISC-V boards will work fine)
**midnight-viewfinder**
- [ ] Impliment buttons, LEDs, and quadrature rotary encoder to replace internals of existing viewfinder.
- [ ] Impliment Wi-Fi connection.
- [ ] Impliment encryption and authentication as best as possible to protect the microcontroller and the Museum's internal network for the given gallery.
- [ ] In a partner crate, impliment testing features that require `std` to test an emulated version of the microcontroller.
- [ ] use `defmt` or another reasonable logging tool to enable local and remote debugging of the MCU.

### WGPU Display App of Image Carousel
- [ ] Impliment rendering via WGPU.
- [ ] Build a reasonably flexibly app that can be refined with designer's feedback
- [ ] Listen for MCU via network, connecting to its direct IP.
- [ ] Fall-back to receive serial data via USB (less desireable)
- [ ] Impliment unit testing

### Other Thoughts
- [ ] Basic axum API / Backend to allow remote management of images, categories, etc.

# Final Thoughts
This workspace is licensed under the GNU ... See `LICESNE.md` for more.

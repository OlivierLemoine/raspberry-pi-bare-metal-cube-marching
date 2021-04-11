#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc_error_handler)]

extern crate alloc;

//mod exception;
mod shapes;
mod start;

use hal::{eprintln, mailbox::*};

//const OBJS: &[&dyn shapes::Shape] = &[
//    //
//    &shapes::Sphere::new(0., 0., 0., 1.),
//];

fn main() {
    eprintln!("Hello world");

    //const FACTOR: usize = 20;
    //const WIDTH: usize = 10 * FACTOR;
    //const HEIGHT: usize = 6 * FACTOR;

    //let camera = shapes::Camera::new(WIDTH, HEIGHT);
    //camera.render_ascii(OBJS);

    let firmware_version = Message::new()
        .with(tag::GetFirmwareVersion)
        .commit()
        .unwrap();

    eprintln!("Firmware version {}", firmware_version);

    let screen_buffer = Message::new()
        .with(tag::SetPhysicalSize {
            width: 800,
            height: 600,
        })
        .with(tag::SetVirtualSize {
            width: 800,
            height: 600,
        })
        .with(tag::SetDepth(8))
        .with(tag::AllocateBuffer)
        .commit()
        .unwrap();

    let (physical_size, virtual_size, depth, pitch) = Message::new()
        .with(tag::GetPhysicalSize)
        .with(tag::GetVirtualSize)
        .with(tag::GetDepth)
        .with(tag::GetPitch)
        .commit()
        .unwrap();

    eprintln!(
        "Screen (PH){}x{} (VI){}x{} at {:X?}; {} bits per pixel and pitch = {}",
        physical_size.width,
        physical_size.height,
        virtual_size.height,
        virtual_size.height,
        screen_buffer.ptr as u64,
        depth,
        pitch
    );

    eprintln!("Done.");
}

#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc_error_handler)]

extern crate alloc;

//mod exception;
mod start;

mod shapes;

fn main() {
    hal::eprintln!("Hello world");

    const FACTOR: usize = 20;
    const WIDTH: usize = 10 * FACTOR;
    const HEIGHT: usize = 6 * FACTOR;

    let objs: &[&dyn shapes::Shape] = &[
        //
        &shapes::Sphere::new(0., 0., 0., 1.),
    ];
    let camera = shapes::Camera::new(WIDTH, HEIGHT);
    camera.render_ascii(&objs);
}

#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc_error_handler)]

//mod exception;
mod start;

mod shapes;

fn main() {
    hal::eprintln!("Hello world");

    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;

    let objs: &[&dyn shapes::Shape] = &[&shapes::Sphere::new(0., 0., 0., 1.)];
    let camera = shapes::Camera::new(WIDTH, HEIGHT);
    camera.render_ascii(&objs);
}

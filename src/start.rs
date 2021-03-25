static STACK_BASE: [u64; 4] = [0x800000, 0x700000, 0x600000, 0x500000];

extern "C" {
    #[link_name = "__start_size"]
    static START_SIZE: u8;
}

#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start() -> ! {
    asm!(
        // Thread ID
        "mrs x0, mpidr_el1",
        "and x0, x0, #0x3",
        "lsl x0, x0, #3",
        // Set stack base for thread
        "ldr x1, [{stack_base}, x0]",
        "mov SP, x1",
        // If thread id == 0, go to main else park
        "cbz x0, {main}",
        main = sym main_thread_start,
        stack_base = in(reg) &STACK_BASE,
    );
    // app_manager::wait_for_start()
    loop {}
}

unsafe extern "C" fn main_thread_start() -> ! {
    asm!(
        "mov x1, #0",
        //"msr VBAR_EL2, x1",
        //
        "ldr x1, =__bss_start",
        "ldr w2, =__bss_size",
        "lp: cbz w2, end",
        "str xzr, [x1], #8",
        "sub w2, w2, #1",
        "cbnz w2, lp",
        "end:",
    );

    heap::HEAP.init_allocator();

    let arm_mem = hal::memory();
    let arm_mem_end = arm_mem.as_ptr().offset(arm_mem.len() as isize);
    heap::HEAP.update_alloc_end(arm_mem_end);
    asm!("mov sp, {}", in(reg) arm_mem_end as usize);

    let start_size = &START_SIZE as *const u8 as u64;
    heap::HEAP.mark_custom_block(core::slice::from_raw_parts(
        0x80_0000 as *const u8,
        start_size as usize,
    ));

    super::main();

    loop {}
}

mod heap {
    #[alloc_error_handler]
    fn alloc_error(_layout: core::alloc::Layout) -> ! {
        loop {}
    }

    #[global_allocator]
    pub static mut HEAP: linked_list_alloc::Alloc = linked_list_alloc::Alloc::static_allocator();
}

mod panic_handler {
    use core::panic::PanicInfo;

    #[panic_handler]
    unsafe fn panic(p: &PanicInfo<'_>) -> ! {
        hal::eprintln!("Panic occured !\n\r{}", p);
        loop {}
    }
}

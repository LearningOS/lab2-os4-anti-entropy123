use crate::{config::*, mm::PhysAddr, task::Task};

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

static mut KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

pub fn get_kernel_stack_phyaddr(app_id: usize) -> PhysAddr {
    // todo: maybe should use UPSafeCell?
    PhysAddr::from(unsafe { KERNEL_STACK[app_id].data.as_ptr() as usize } + KERNEL_STACK_SIZE)
}

pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn get_app_elf(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}

pub fn setup_task_cx(app_id: usize) -> usize {
    usize::from(get_kernel_stack_phyaddr(app_id)) - core::mem::size_of::<Task>()
}

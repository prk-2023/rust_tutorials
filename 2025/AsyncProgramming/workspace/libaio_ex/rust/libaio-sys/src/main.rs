use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::ptr;

// 1. Manually define the libaio structures (FFI)
// #[repr(C)] ensures the memory layout matches the C 'struct iocb'
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iocb {
    pub data: u64,
    pub key: u32,
    pub rw_flags: i32,
    pub aio_lio_opcode: u16,
    pub aio_reqprio: i16,
    pub aio_fildes: u32,
    pub aio_buf: u64,
    pub aio_nbytes: u64,
    pub aio_offset: i64,
    pub reserved2: u64,
    pub aio_resfd: u32,
    pub aio_flags: u32,
}

#[repr(C)]
pub struct io_event {
    pub data: u64,
    pub obj: u64,
    pub res: i64,
    pub res2: i64,
}

// 2. Link against the system libaio library
// The block itself must now be marked unsafe
#[link(name = "aio")]
unsafe extern "C" {
    fn io_setup(maxevents: i32, ctxp: *mut *mut libc::c_void) -> i32;
    fn io_submit(ctx: *mut libc::c_void, nr: i64, iocbpp: *mut *mut iocb) -> i32;
    fn io_getevents(
        ctx: *mut libc::c_void,
        min_nr: i64,
        nr: i64,
        events: *mut io_event,
        timeout: *mut libc::timespec,
    ) -> i32;
    fn io_destroy(ctx: *mut libc::c_void) -> i32;
}

fn main() {
    // 3. Setup Context
    let mut ctx: *mut libc::c_void = ptr::null_mut();
    unsafe {
        if io_setup(10, &mut ctx) < 0 {
            panic!("io_setup failed. Make sure libaio is installed!");
        }
    }

    // 4. Open file with O_DIRECT
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .custom_flags(libc::O_DIRECT)
        .open("demo_rust_direct.bin")
        .expect(
            "Failed to open file. Note: O_DIRECT needs a real filesystem (ext4/xfs), not tmpfs.",
        );

    // 5. Aligned Buffer (Mandatory for O_DIRECT)
    let layout = std::alloc::Layout::from_size_align(4096, 4096).unwrap();
    let buf_ptr = unsafe { std::alloc::alloc(layout) };

    // 6. Initialize the I/O Control Block
    let mut cb: iocb = unsafe { std::mem::zeroed() };
    cb.aio_fildes = file.as_raw_fd() as u32;
    cb.aio_lio_opcode = 1; // IOCB_CMD_PWRITE
    cb.aio_buf = buf_ptr as u64;
    cb.aio_nbytes = 4096;
    cb.aio_offset = 0;

    let mut cbs = [&mut cb as *mut iocb];

    // 7. Submit and Wait
    unsafe {
        println!("Submitting...");
        io_submit(ctx, 1, cbs.as_mut_ptr());

        let mut events: [io_event; 1] = std::mem::zeroed();
        println!("Waiting for completion...");
        let n = io_getevents(ctx, 1, 1, events.as_mut_ptr(), ptr::null_mut());

        if n > 0 {
            println!("Success! Result: {} bytes written", events[0].res);
        }

        // Cleanup
        io_destroy(ctx);
        std::alloc::dealloc(buf_ptr, layout);
    }
}

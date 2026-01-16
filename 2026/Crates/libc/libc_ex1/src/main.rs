use libc;
use libc::printf;
use std::ffi::CString;
//---
use libc::{fstat, mmap, munmap, open};
use libc::{fstat as libc_fstat, MAP_PRIVATE, O_RDONLY, PROT_READ};
use std::ptr;

use libc::{c_int, signal, SIGINT, SIG_DFL, SIG_IGN};
use std::sync::atomic::{AtomicBool, Ordering};

// A global flag to track if we received a signal
static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);

// The actual C-compatible handler function
extern "C" fn handle_sigint(_sig: c_int) {
    GOT_SIGNAL.store(true, Ordering::SeqCst);
}

fn main() {
    let pid = unsafe { libc::getpid() };
    println!("Program PID: {}", pid);
    println!("----------------------");

    let msg = CString::new("Hello from Rust via C printf!").expect("CString::new failed");

    unsafe {
        // printf(const char *format, ....)
        // .as_ptr() gives us the raw pointer C requires
        printf(msg.as_ptr());
    }

    //--------------------------------------------
    // Second example
    let path = CString::new("test.txt").expect("CString Failed");

    unsafe {
        // 1. Open the file to get File descriptor (fd)
        let fd = open(path.as_ptr(), O_RDONLY);
        if fd < 0 {
            panic!("Could not open file... Double check");
        }

        // 2. get file size ( needed for mmap )
        let mut stat: libc::stat = std::mem::zeroed();
        if libc::fstat(fd, &mut stat) < 0 {
            panic!("Could not get file stats");
        }
        let size = stat.st_size as usize;

        // 3. map the file into memory
        // NULL: let the OS choose the address
        // PROT_READ: we only want to read
        // MAP_PRIVATE: Changes aren't written back
        let data = mmap(ptr::null_mut(), size, PROT_READ, MAP_PRIVATE, fd, 0);

        if data == libc::MAP_FAILED {
            panic!("mmap Failed...");
        }

        // 4. Access the data ( treating it like a slice )
        let slice = std::slice::from_raw_parts(data as *const u8, size);
        println!("File content from memory: {:?}", std::str::from_utf8(slice));

        // 5. Cleanup: Unmap and close
        munmap(data, size);
        libc::close(fd);
    }

    //Example 2
    //---------------------------------------

    unsafe {
        // Register our handler for SIGINT (Ctrl+C)
        // signal() returns the previous handler
        signal(SIGINT, handle_sigint as libc::sighandler_t);
    }

    println!("Waiting for Ctrl+C...");
    while !GOT_SIGNAL.load(Ordering::SeqCst) {
        // Do work...
    }
    println!("\nSignal received! Cleaning up...");
}

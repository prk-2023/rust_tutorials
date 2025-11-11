use std::rc::Rc;

// --- 1. The Shared Data Structure (Config) ---
// This struct will hold the data we want to share.
#[derive(Debug)]
struct Config {
    environment: String,
    log_level: u8,
}

// --- 2. Components That Use the Shared Config ---
struct WorkerA {
    // The worker holds a pointer (Rc) to the shared Config data.
    // It is an owner, but not the only one.
    config: Rc<Config>,
}

struct WorkerB {
    // This worker also holds its own pointer (Rc) to the EXACT same Config data.
    config: Rc<Config>,
}

impl WorkerA {
    fn report_status(&self) {
        // Since Rc implements Deref, we can access fields directly through the smart pointer.
        // This is Deref Coercion in action!
        println!(
            "  ⚙️ Worker A: Running in '{}' environment.",
            self.config.environment
        );
    }
}

impl WorkerB {
    fn execute_job(&self) {
        // Accessing the shared data via Rc.
        println!(
            "  🚀 Worker B: Executing job with log level {}.",
            self.config.log_level
        );
    }
}

fn main() {
    println!("--- Program Start ---");

    // --- Initial Creation ---
    // 1. Create the Config data on the heap inside the first Rc smart pointer.
    let shared_config = Rc::new(Config {
        environment: "Production".to_string(),
        log_level: 2,
    });

    // The reference count is now 1.
    println!(
        "(Info) Initial Ref Count: {}",
        Rc::strong_count(&shared_config)
    ); // Output: 1

    // --- Sharing Ownership ---
    // 2. Clone the Rc pointer. This is a shallow copy of the pointer, NOT the data.
    // This increments the reference count.
    let worker_a_config = Rc::clone(&shared_config);
    let worker_b_config = Rc::clone(&shared_config);

    // The reference count is now 3 (shared_config, worker_a_config, worker_b_config).
    println!(
        "(Info) Ref Count after cloning: {}",
        Rc::strong_count(&shared_config)
    ); // Output: 3

    // --- Instantiate Workers ---
    let worker_a = WorkerA {
        config: worker_a_config,
    };
    let worker_b = WorkerB {
        config: worker_b_config,
    };

    println!("-------------------------");

    // Workers access and use the shared data immutably.
    worker_a.report_status();
    worker_b.execute_job();

    println!("-------------------------");

    // The original `shared_config` is still in scope, keeping the count up.
    println!(
        "(Info) Ref Count before scope end: {}",
        Rc::strong_count(&shared_config)
    ); // Output: 3

    // The `worker_a` and `worker_b` variables go out of scope here.
    // Their respective Rc pointers are dropped, decrementing the count twice.
} // The `main` function ends.

// --- The Drop Trait in Action (Implicitly) ---
// The original `shared_config` variable goes out of scope here.
// The reference count drops from 1 to 0.
// Rc<Config>'s internal Drop implementation sees count = 0, and ONLY NOW frees the Config data from the heap.

// Output when the last Rc drops:
// (Info) The shared data is implicitly freed from the heap here, ensuring memory safety.

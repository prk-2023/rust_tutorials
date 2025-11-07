// Program that wraps the getpid() system call and provides a wrapper function that either uses a
// user-provided PID or falls back to the current process's PID.
use nix::unistd::{getpid, Pid};
use std::fmt;

#[allow(dead_code)]
// Custom type to represent a process management action
#[derive(Debug, PartialEq, Copy, Clone)]
enum PidAction {
    ReadStatus,
    SendSignal,
}

// Implement Display for nicer printing
impl fmt::Display for PidAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
struct PidManager {
    // In a real system, this might track a list of managed PIDs
    // but here it's just a placeholder to keep the structure similar.
    managed_pids: Vec<Pid>,
}

impl PidManager {
    /// The core logic: determines which PID to act upon.
    fn determine_target_pid(&self, preferred_pid: Option<Pid>) -> Pid {
        // If a preferred PID is provided (Some), use it.
        // If not (None), fall back to getting the current process's PID.
        preferred_pid.unwrap_or_else(|| self.get_current_pid())
    }

    /// Analogous to Inventory::most_stocked().
    /// This function performs a system call to get the current PID.
    fn get_current_pid(&self) -> Pid {
        // let current_pid = unistd::getpid();
        let current_pid = getpid();
        println!("[FALLBACK] No preferred PID given. Using current process's PID.");
        current_pid
    }
}

fn main() {
    let manager = PidManager {
        // Initialize with an empty list for this simplified example
        managed_pids: vec![],
    };

    // --- Scenario 1: User provides a specific PID ---
    // Note: We use a made-up PID (12345) for demonstration
    //let user_pref_pid_1 = Some(Pid::from_raw(12345));
    let user_pref_pid_1 = Some(Pid::from_raw(1));
    let target_pid_1 = manager.determine_target_pid(user_pref_pid_1);

    println!("--- Action 1 ---");
    println!(
        "User preferred PID: {:?}. Target PID for action: {}",
        user_pref_pid_1, target_pid_1
    );

    println!("----------------");

    // --- Scenario 2: User provides no PID ---
    let user_pref_pid_2 = None;
    let target_pid_2 = manager.determine_target_pid(user_pref_pid_2);

    println!("--- Action 2 ---");
    println!(
        "User preferred PID: {:?}. Target PID for action: {}",
        user_pref_pid_2, target_pid_2
    );
    // Print the actual current PID value to confirm the fallback
    //println!("Confirmed current PID used: {}", unistd::getpid());
    println!("Confirmed current PID used: {}", getpid());
}
// --- Programming Book example
// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }
//
// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }
//
// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }
//
//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;
//
//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }
//
// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };
//
//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );
//
//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }

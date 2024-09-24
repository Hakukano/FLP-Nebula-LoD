use std::time::Duration;

use sysinfo::{Pid, System};

pub async fn run(parent_pid: u32) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        if System::new_all()
            .process(Pid::from_u32(parent_pid))
            .is_none()
        {
            println!("The parent is dead! Exiting...");
            return;
        }
    }
}

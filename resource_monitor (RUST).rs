use sysinfo::{System, Components, SystemExt};
use notify_rust::Notification;
use std::thread;
use std::time::Duration;

fn notify(title: &str, message: &str) {
    Notification::new()
        .summary(title)
        .body(message)
        .timeout(5000) // 5 seconds
        .show()
        .unwrap();
}

fn main() {
    let mut sys = System::new_all();

    // Notify on startup
    notify("System Monitor", "Monitoring system resources...");

    loop {
        sys.refresh_all();

        let ram = sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0;
        let swap = sys.used_swap() as f64 / sys.total_swap() as f64 * 100.0;
        let cpu_usage = sys.global_cpu_usage();
        let sensors = Components::new_with_refreshed_list();

        if ram >= 80.0 {
            notify("⚠ High RAM Usage!", &format!("RAM at {:.2}%", ram));
        }

        if swap >= 30.0 {
            notify("⚠ High Swap Usage!", &format!("Swap at {:.2}%", swap));
        }

        if cpu_usage >= 85.0 {
            notify("⚠ High CPU Usage!", &format!("CPU at {:.2}%", cpu_usage));
        }

        for component in sensors {
            if component.temperature() >= 85.0 {
                notify(
                    "⚠ High Temperature!",
                    &format!("{} at {:.2}°C", component.label(), component.temperature()),
                );
            }
        }

        thread::sleep(Duration::from_secs(10)); // Check every 10 seconds
    }
}

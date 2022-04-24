use sysinfo::{NetworkExt, SystemExt, ProcessorExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = sysinfo::System::default();

    sys.refresh_all();

    sys.refresh_cpu();

    println!("=> system:");
    
    let memory_usage = sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0;

    // RAM and swap information:
    println!("memory : {:.2}%", memory_usage);
    println!("cpu    : {:.2}%", sys.global_processor_info().cpu_usage());

    // Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }

    // Components temperature:
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    Ok(())
}

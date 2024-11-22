enum DeviceType {
    Server,
    ATM,
    Workstation,
    Printer,
    PABX
}

enum serverStatus {
    Online,
    Offline
}

enum StorageStatus {
    Free,
    PartiallyUsed,
    HalfFull,
    AlmostFull,
    Full,
    NoSpace
}

struct Host {
    name: String,
    ip_address: String,
    port: u8,
    category: DeviceType
}

fn isAvailable(host: (&str, u8)) -> bool {
    todo!()
}

fn add_host() ->  

fn main() {
    let device_id : u32 = 120;
    println!("Supervise Terminal!");
    println!("Device ID: {device_id}");
}

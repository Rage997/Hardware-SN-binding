mod hwid;
mod mac_addr;

fn main() {
    println!("Getting hardware information...");
    let id = hwid::get_id();
    println!("The UUID of the machine is {}.", id);


    mac_addr::get();
}



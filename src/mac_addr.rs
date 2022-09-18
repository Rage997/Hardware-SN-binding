use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn get() {
    // On mac, you can get all the mac addresses by running /sbin/ifconfig | grep ether
    
    
	let output = Command::new("/sbin/ifconfig")
	.output()
	.expect("Failed to retrieve hardware information");

    assert!(output.status.success());
	let tmp =  String::from_utf8(output.stdout)
		.expect("Found invalid UTF-8");
	match tmp
		.lines()
		.find(|l| l.contains("ether"))// find the line containing the UUID
		.unwrap_or("")
		.split(' ')
		.nth(1)
		{
		None => panic!("IOPlatformUUID not found"),
		Some(id) => {
            println!(id);
			return id.to_string();
		}
	}    
}


#[cfg(target_os = "linux")]
pub fn get() {
    // On a linux machine, all network interfaces are stored inside /sys/class/net
    // You can get the mac address by running "cat /sys/class/net/<network_interface>/address"

    let net = Path::new("/sys/class/net");
    let entry = fs::read_dir(net).expect("Error");
    // a bit cubersome :/
    let ifaces = entry.filter_map(|p| p.ok())
                      .map(|p| p.path().file_name().expect("Error").to_os_string())
                      .filter_map(|s| s.into_string().ok())
                      .collect::<Vec<String>>();
    println!("Available interfaces: {:?}", ifaces);

    for iface in ifaces.iter(){
        let addr_path = net.join(iface.as_str()).join("address");
        let mut f = fs::File::open(addr_path).expect("Failed");
        let mut macaddr = String::new();
        f.read_to_string(&mut macaddr).expect("Error");
        println!("MAC address: {}", macaddr);
    }
    
}
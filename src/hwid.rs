use std::process::Command;
use std::string::String;
// use std::io::{self, Write};


#[cfg(target_os = "linux")]
pub fn get_id()-> String {
	// Get hardware UUID
	// On linux, a random 32 byte ID is generated at system installation
	// it is store at the location /etc/machine-id
	// I am not sure if this is true for **all** linux distribution
	// see: https://manpages.ubuntu.com/manpages/xenial/en/man1/systemd-machine-id-setup.1.html  

	println!("You are running a Linux distribution. Retrieving /etc/machine-id file...!");
	let id_path = "/etc/machine-id";
	// Open file
	let id_contents = std::fs::read_to_string(id_path);
	return id_contents.unwrap();
}


#[cfg(target_os = "macos")]
pub fn get_id()-> String {
	// Get hardware UUID
	// The command: ioreg -rd1 -c IOPlatformExpertDevice
	// lists a lot of information about the device.
	// This function runs: ioreg -rd1 -c IOPlatformExpertDevice | grep IOPlatformUUID
	// and returns the ID.
	// IOPlatformUUID is an hardware identifier. 
	// see: https://stackoverflow.com/questions/933460/unique-hardware-id-in-mac-os-x

	println!("You are running MacOs. Retrieving IOPlatformUUID...!");

	let output = Command::new("ioreg")
	.arg("-rd1")
	.arg("-c")
	.arg("IOPlatformExpertDevice")
	.output()
	.expect("Failed to retrieve hardware information");

	// println!("status: {}", output.status);
	// io::stdout().write_all(&output.stdout).unwrap();
	// io::stderr().write_all(&output.stderr).unwrap();

	assert!(output.status.success());
	let tmp =  String::from_utf8(output.stdout)
		.expect("Found invalid UTF-8");
	match tmp
		.lines()
		.find(|l| l.contains("IOPlatformUUID"))// find the line containing the UUID
		.unwrap_or("")
		.split('=')
		.nth(1)
		.unwrap_or("")
		.split('\"')
		.nth(1)
		{
		None => panic!("IOPlatformUUID not found"),
		Some(id) => {
			return id.to_string();
		}
	}
		// 	if id.is_empty() {
		// 		panic!("IOPlatformUUID not found")
		// 	} else {
		// 		Ok(id.unwrap().to_string())
		// }
		
	// .output()
	// .stdout;

	// let out = String::from_utf8(cmd);
	// let out = cmd.stdout;
	// return s;

}

// }


// pub fn get_id() -> std::string::String {
// 	unimplemented!("*BSD support is not implemented")
// }
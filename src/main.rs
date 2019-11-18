mod keycode;

use std::{mem, net::UdpSocket};

use keycode::map_key_code;
use winapi::um::winuser::{
	INPUT_u, MapVirtualKeyW, SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE,
};
fn main() -> std::io::Result<()> {
	{
		let mut socket = UdpSocket::bind("192.168.11.2:33333")?;

		let mut buf = [0; 64];
		let mut shit: bool = false;
		let mut alt: bool = false;

		let ctrl_prefix = vec!["14", "35", "0", "2", "3", "11", "45"];
		unsafe {
			let mut ip = mem::zeroed::<INPUT>();
			let mut ipu = mem::zeroed::<INPUT_u>();
			ip.type_ = INPUT_KEYBOARD;
			ipu.ki_mut().time = 0;
			ipu.ki_mut().wVk = 0;
			ipu.ki_mut().dwExtraInfo = 0;

			//bug回避用
			{
				send_key_up(0x1D);
				send_key_up(0x2A);
				send_key_up(map_key_code("58"));
			}
			loop {
				//フラグを入力にリセット
				ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE;
				let (amt, src) = socket.recv_from(&mut buf)?;
				let code = std::str::from_utf8(&buf[..amt]).unwrap();

				let comb: Vec<&str> = code.split(',').collect();

				if comb.len() == 2 {
					let code = map_key_code(comb[1]);
					match comb[0] {
						//Ctrl
						"ct" => {
							match comb[1] {
								"14" => send_key(0xE04F), // ctrl + e => end
								"35" => send_key(0xE048), // ctrl + p => up arrow
								"0" => send_key(0xE047),  // ctrl + a => Home
								"2" => send_key(0xE053),  // ctrl + a => delete
								"3" => send_key(0xE04D),  // ctrl + f => right arrow
								"11" => send_key(0xE04B), // ctrl + b => left arrow
								"45" => send_key(0xE050), // ctrl + b => down arrow
								_ => {}
							}
						}
						// Cmd
						"c" => {
							send_key_press(0x1D); // send the ctrl press
							send_key(code); //send key press and release\
							 // send_key_up(0x1D); //send the ctrl rerlease
						}
						// Shift
						"s" => {
							send_key_press(0x2A);
							send_key(code);
							// send_key_up(0x2A);
						}
						// ctrl + shift + hoge
						"d" => {
							send_key_press(0x1D); //Send the Left-Ctrl press
							send_key_press(0x2A); // Send the Left-Shift press
							send_key(code);
							// send_key_up(0x2A);
							// send_key_up(0x1D);
						}

						// alt
						"a" => {
							send_key_press(0xE038);
							send_key(code);
						}
						_ => {}
					}
				} else {
					let code = map_key_code(comb[0]);
					match comb[0] {
						"55" => send_key_up(code), // up cmd key,
						"56" => send_key_up(code), // up shift key,
						"58" => send_key_up(code), // up alt key
						"59" => send_key_up(code), // up ctrl key,
						_ => send_key(code),
					}
				}
				println!("{:?}", std::str::from_utf8(&buf[..amt]));
			}
		}
		// Redeclare `buf` as slice of the received data and send reverse data back to origin.
		// let buf = &mut buf[..amt];
		// buf.reverse();
		// socket.send_to(buf, &src)?;
	} // the socket is closed here
	Ok(())
}

unsafe fn send_key_up(code: u16) {
	println!("key up {}", code);
	let mut ip = mem::zeroed::<INPUT>();
	let mut ipu = mem::zeroed::<INPUT_u>();
	ip.type_ = INPUT_KEYBOARD;
	ipu.ki_mut().time = 0;
	ipu.ki_mut().wVk = 0;
	ipu.ki_mut().dwExtraInfo = 0;
	ipu.ki_mut().wScan = code;
	//Prepare a keyup event
	ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
	ip.u = ipu;
	SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
}

unsafe fn send_key_press(code: u16) {
	let mut ip = mem::zeroed::<INPUT>();
	let mut ipu = mem::zeroed::<INPUT_u>();
	ip.type_ = INPUT_KEYBOARD;
	ipu.ki_mut().time = 0;
	ipu.ki_mut().wVk = 0;
	ipu.ki_mut().dwExtraInfo = 0;
	ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE;
	ipu.ki_mut().wScan = code;
	ip.u = ipu;
	//Send the press
	SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
}
unsafe fn send_key(code: u16) {
	let mut ip = mem::zeroed::<INPUT>();
	let mut ipu = mem::zeroed::<INPUT_u>();
	ip.type_ = INPUT_KEYBOARD;
	ipu.ki_mut().time = 0;
	ipu.ki_mut().wVk = 0;
	ipu.ki_mut().dwExtraInfo = 0;
	ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE;
	ipu.ki_mut().wScan = code;
	ip.u = ipu;
	//Send the press
	SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
	//Prepare a keyup event
	ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
	ip.u = ipu;
	SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
}

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
		loop {
			let (amt, src) = socket.recv_from(&mut buf)?;
			let code = std::str::from_utf8(&buf[..amt]).unwrap();

			let comb: Vec<&str> = code.split(',').collect();
			unsafe {
				let mut ip = mem::zeroed::<INPUT>();
				let mut ipu = mem::zeroed::<INPUT_u>();
				ip.type_ = INPUT_KEYBOARD;
				ipu.ki_mut().time = 0;
				ipu.ki_mut().wVk = 0;
				ipu.ki_mut().dwExtraInfo = 0;
				ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE;

				if comb.len() == 2 {
					match comb[1] {
						"14" => send_key(0xE04F), // ctrl + e => end
						"35" => send_key(0xE048), // ctrl + p => up arrow
						"0" => send_key(0xE047),  // ctrl + a => Home
						"2" => send_key(0xE053),  // ctrl + a => delete
						"3" => send_key(0xE04D),  // ctrl + f => right arrow
						"11" => send_key(0xE04B), // ctrl + b => left arrow
						"45" => send_key(0xE050), // ctrl + b => down arrow
						_ => {
							match comb[0] {
								"c" => {
									//Send the ctrl press
									ipu.ki_mut().wScan = 0x1D; // Left-Ctrl
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);

									//send key
									ipu.ki_mut().wScan = map_key_code(comb[1]);
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);

									//Prepare a keyup key
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Prepare a keyup ctrl
									ipu.ki_mut().wScan = 0x1D; // Left-Ctrl
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
								}
								"s" => {
									//Send the Left-Shift press
									ipu.ki_mut().wScan = 0x2A; // Left-Shift
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//send key
									ipu.ki_mut().wScan = map_key_code(comb[1]);
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Prepare a keyup key
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Prepare a keyup shift
									ipu.ki_mut().wScan = 0x2A; // Left-Shift
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
								}
								// ctrl + shift + hoge
								"d" => {
									//Send the Left-Ctrl press
									ipu.ki_mut().wScan = 0x1D; // Left-Ctrl
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Send the Left-Shift press
									ipu.ki_mut().wScan = 0x2A; // Left-Shift
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//send key
									ipu.ki_mut().wScan = map_key_code(comb[1]);
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Prepare a keyup key
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Prepare a keyup shift
									ipu.ki_mut().wScan = 0x2A; // Left-Shift
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Prepare a keyup Ctrl
									ipu.ki_mut().wScan = 0x1D; // Left-Ctrl
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
								}

								// alt
								"a" => {
									//Send the Left-alt press
									ipu.ki_mut().wScan = 0xE038; // Left-alt
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//send key
									ipu.ki_mut().wScan = map_key_code(comb[1]);
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Prepare a keyup key
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
									//Prepare a keyup alt
									ipu.ki_mut().wScan = 0xE038; // Left-alt
									ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
									ip.u = ipu;
									SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
								}
								_ => {}
							}
						}
					}
				} else {
					ipu.ki_mut().wScan = map_key_code(comb[0]);
					ip.u = ipu;
					//Send the press
					SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
					//Prepare a keyup event
					ipu.ki_mut().dwFlags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
					ip.u = ipu;
					SendInput(1, &mut ip, mem::size_of::<INPUT>() as i32);
					// send_key(map_key_code(comb[0]));
				}
			}
			println!("{:?}", std::str::from_utf8(&buf[..amt]));
		}

		// Redeclare `buf` as slice of the received data and send reverse data back to origin.
		// let buf = &mut buf[..amt];
		// buf.reverse();
		// socket.send_to(buf, &src)?;
	} // the socket is closed here
	Ok(())
}

unsafe fn send_key(code: u16) {
	unsafe {
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
}

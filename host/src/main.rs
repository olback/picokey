use {
    aes_gcm_siv::{
        aead::{Aead, NewAead},
        Aes256GcmSiv, Key, Nonce,
    },
    error::Error,
    rand::Rng,
    serialport::{self, SerialPortInfo, SerialPortType},
    std::{
        io::{Read, Write},
        thread,
        time::Duration,
    },
};

mod error;

const KEY_IV: &[u8] = include_bytes!("../../keyiv");

const PICO_VID: u16 = 0x2e8a;
const PICO_PID: u16 = 0x000a;
const SERIAL_BAUD: u32 = 115200;

const KEY_LENGTH: usize = 32;
const IV_LENGTH: usize = 12;

fn get_pico_port() -> Option<serialport::SerialPortInfo> {
    let ports = serialport::available_ports().unwrap();
    for port in ports {
        match port.port_type {
            SerialPortType::UsbPort(ref up) if up.vid == PICO_VID && up.pid == PICO_PID => {
                return Some(port)
            }
            _ => {}
        }
    }
    None
}

fn auth(port_info: SerialPortInfo) -> Result<(), Error> {
    println!("=> Authenticating");

    let mut acm = serialport::new(port_info.port_name, SERIAL_BAUD)
        .data_bits(serialport::DataBits::Eight)
        .open()?;

    // Generate garbage
    let garbage = rand::thread_rng().gen::<[u8; 32]>();

    // Encrypt garbage
    let key = Key::from_slice(&KEY_IV[0..KEY_LENGTH]);
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(&KEY_IV[KEY_LENGTH..(KEY_LENGTH + IV_LENGTH)]);

    let data_out = [
        // base64::encode(cipher.encrypt(nonce, &garbage[..])?).as_bytes(),
        base64::encode(cipher.encrypt(nonce, &b"Hello"[..])?).as_bytes(),
        // base64::encode("Hello").as_bytes(),
        b"\n",
    ]
    .concat();

    acm.write(&data_out[..])?;

    // Wait until data is ready
    loop {
        match acm.bytes_to_read() {
            Ok(0) => (),
            Ok(_) => break,
            Err(e) => return Err(e.into()),
        }
        thread::sleep(Duration::from_millis(1));
    }

    let line = read_line(&mut acm, b"\r\n");
    println!("({}) {:?}", line.len(), line);
    let decoded = base64::decode(&line)?;
    check_error(&decoded)?;

    println!("({}) {:?}", garbage.len(), garbage);
    println!("({}) {:?}", decoded.len(), decoded);

    Ok(())
}

fn unlock() -> Result<(), ()> {
    println!("=> Unlocking");
    Ok(())
}

fn check_error(slice: &[u8]) -> Result<(), Error> {
    if slice.len() == 32 && slice[4..].iter().all(|v| v == &0) {
        use std::convert::TryInto;
        let code = i32::from_be_bytes((&slice[0..4]).try_into().unwrap());
        Err(Error::Code(code))
    } else {
        Ok(())
    }
}

fn main() {
    let mut run = true;
    loop {
        let port = get_pico_port();
        if port.is_some() && run {
            run = false;
            match auth(port.unwrap()) {
                Ok(_) => {
                    println!("=> Auth success");
                    match unlock() {
                        Ok(_) => println!("=> Unlocked"),
                        Err(e) => eprintln!("Unlock failed: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Auth failed: {:?}", e),
            }
        } else if port.is_none() && run == false {
            run = true;
        }

        thread::sleep(Duration::from_millis(100));
    }

    ////////////////////////////////////////////////////////////////////

    /*let mut acm0 = serialport::new("/dev/ttyACM0", 115200)
    .data_bits(serialport::DataBits::Eight)
    .open()
    .unwrap();*/

    /*let challange: [u8; 32] = rand::thread_rng().gen();
    let challange_b64 = base64::encode(&challange);
    println!("Expected: {}", challange_b64);

    acm0.write([&challange_b64.as_bytes(), &b"\n"[..]].concat().as_ref())
        .unwrap();*/

    /*acm0.write(b"hello\n").unwrap();

    loop {
        loop {
            match acm0.bytes_to_read() {
                Ok(0) => continue, //println!("0 bytes to read"),
                Ok(_) => break,
                Err(e) => eprintln!("{:#?}", e),
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        let res = read_line(&mut acm0, b'\n');
        let ret_str = String::from_utf8_lossy(&res);
        println!("{}", ret_str);
    }*/
}

fn read_line<R: Read>(r: &mut R, eol: &[u8]) -> Vec<u8> {
    let mut res = Vec::<u8>::with_capacity(64);
    loop {
        // println!("{:?}", res);
        let mut r_buf = [0u8; 1];
        if let Ok(_) = r.read_exact(&mut r_buf) {
            if eol.contains(&r_buf[0]) {
                break;
            } else {
                res.push(r_buf[0]);
            }
        }
    }
    res
}

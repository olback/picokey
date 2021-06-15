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
        process::Command,
        thread,
        time::Duration,
    },
};

mod config;
mod error;

const KEY_IV: &[u8] = include_bytes!("../../keyiv");

const PICO_VID: u16 = 0x2e8a;
const PICO_PID: u16 = 0x000a;
const SERIAL_BAUD: u32 = 115200;
const GARBAGE_LENGTH: usize = 32;
const GARBAGE_LENGTH_HALF: usize = GARBAGE_LENGTH / 2;
const UID_LENGTH: usize = 8;
const KEY_LENGTH: usize = 32;
const IV_LENGTH: usize = 12;

fn get_pico_port() -> Option<serialport::SerialPortInfo> {
    let ports = serialport::available_ports().unwrap();
    for port in ports {
        match port.port_type {
            SerialPortType::UsbPort(ref up) if up.vid == PICO_VID && up.pid == PICO_PID => {
                return Some(port);
            }
            _ => {}
        }
    }
    None
}

fn auth(port_info: SerialPortInfo, valid_uids: &[&[u8]]) -> Result<(), Error> {
    println!("=> Authenticating");

    let mut acm = serialport::new(port_info.port_name, SERIAL_BAUD)
        .data_bits(serialport::DataBits::Eight)
        .open()?;

    // Generate garbage
    let garbage = rand::thread_rng().gen::<[u8; GARBAGE_LENGTH]>();

    // Encrypt garbage
    let key = Key::from_slice(&KEY_IV[0..KEY_LENGTH]);
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(&KEY_IV[KEY_LENGTH..(KEY_LENGTH + IV_LENGTH)]);

    let encrypted = cipher.encrypt(nonce, &garbage[..])?;
    let encoded = base64::encode(&encrypted);
    let data_out = [encoded.as_bytes(), b"\n"].concat();
    acm.write(&data_out[..])?;

    println!("Garbage out:   ({}) {:?}", garbage.len(), garbage);
    println!("Encrypted out: ({}) {:?}", encrypted.len(), encrypted);
    println!("Encoded out:   ({}) {:?}", encoded.len(), encoded);
    println!("Data out:      ({}) {:?}", data_out.len(), data_out);

    // Wait until data is ready
    loop {
        match acm.bytes_to_read() {
            Ok(0) => (),
            Ok(_) => break,
            Err(e) => return Err(e.into()),
        }
        thread::sleep(Duration::from_millis(1));
    }

    // Read
    let line = read_line(&mut acm, b"\r\n");
    println!("Data in:     ({}) {:?}", line.len(), line);

    // Decode
    let decoded = base64::decode(&line)?;
    println!("Decoded in:  ({}) {:?}", decoded.len(), decoded);
    check_error(&decoded)?;

    // Decrypt
    let decrypted = cipher.decrypt(nonce, &decoded[..])?;
    println!("Decrypted in: ({}) {:?}", decrypted.len(), decrypted);

    // Check if received data is correct
    if decrypted.len() == GARBAGE_LENGTH + UID_LENGTH {
        if decrypted[0..GARBAGE_LENGTH_HALF] != garbage[0..GARBAGE_LENGTH_HALF]
            || decrypted[(GARBAGE_LENGTH_HALF + UID_LENGTH)..] != garbage[GARBAGE_LENGTH_HALF..]
            || !valid_uids
                .contains(&&decrypted[GARBAGE_LENGTH_HALF..(GARBAGE_LENGTH_HALF + UID_LENGTH)])
        {
            return Err(Error::Unauthorized);
        }
    } else {
        return Err(Error::Unauthorized);
    }

    Ok(())
}

fn unlock(cmd: &[&str]) -> Result<(), Error> {
    println!("=> Unlocking");
    Command::new(cmd[0]).args(&cmd[1..]).spawn()?;
    Ok(())
}

fn check_error(slice: &[u8]) -> Result<(), Error> {
    if slice.len() == 32 && slice[4..].iter().all(|v| v == &0) {
        use std::convert::TryInto;
        let code = i32::from_le_bytes((&slice[0..4]).try_into().unwrap());
        Err(Error::Code(code))
    } else {
        Ok(())
    }
}

fn main() {
    // Make sure we have a valid key/iv length
    assert_eq!(
        KEY_LENGTH + IV_LENGTH,
        KEY_IV.len(),
        "Key length + Iv length does not match keyiv content"
    );

    let conf = config::Config::load().expect("Failed to load config");
    // println!("{:#?}", conf);

    let mut run = true;
    loop {
        let port = get_pico_port();
        if port.is_some() && run {
            run = false;
            match auth(port.unwrap(), &conf.pico_ids()[..]) {
                Ok(_) => {
                    println!("=> Auth success");
                    match unlock(&conf.command()) {
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

use {
    rand::Rng,
    serialport,
    std::io::{Read, Write},
};

fn main() {
    let mut acm0 = serialport::new("/dev/ttyACM0", 115200)
        .data_bits(serialport::DataBits::Eight)
        .open()
        .unwrap();

    /*let challange: [u8; 32] = rand::thread_rng().gen();
    let challange_b64 = base64::encode(&challange);
    println!("Expected: {}", challange_b64);

    acm0.write([&challange_b64.as_bytes(), &b"\n"[..]].concat().as_ref())
        .unwrap();*/

    acm0.write(b"hello\n").unwrap();

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
    }
}

fn read_line<R: Read>(r: &mut R, eol: u8) -> Vec<u8> {
    let mut res = Vec::<u8>::with_capacity(64);
    loop {
        let mut r_buf = [0u8; 1];
        if let Ok(_) = r.read_exact(&mut r_buf) {
            if r_buf[0] == eol {
                break;
            } else {
                res.push(r_buf[0]);
            }
        }
    }
    res
}

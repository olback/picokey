#[derive(Debug, giftwrap::Wrap)]
pub enum Error {
    Io(std::io::Error),
    SerialPort(serialport::Error),
    Base64(base64::DecodeError),
    Aead(aes_gcm_siv::aead::Error),
    #[noWrap]
    Code(i32),
    #[noWrap]
    Unauthorized,
}

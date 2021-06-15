use {
    serde::Deserialize,
    std::{fs, io},
    toml,
};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "unlock-command")]
    unlock_command: String,
    #[serde(deserialize_with = "deserialize_pico_ids", rename = "pico-ids")]
    pico_ids: Vec<Vec<u8>>,
}

impl Config {
    pub fn load() -> Result<Self, io::Error> {
        let path = dirs::config_dir()
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Config dir not found",
            ))?
            .join("picokey")
            .join("Config.toml");
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn command(&self) -> Vec<&str> {
        self.unlock_command.split(' ').collect::<Vec<&str>>()
    }

    pub fn pico_ids(&self) -> Vec<&[u8]> {
        self.pico_ids.iter().map(|k| &k[..]).collect()
    }
}

fn deserialize_pico_ids<'de, D>(de: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let uid_strs: Vec<String> = serde::de::Deserialize::deserialize(de)?;
    let mut uids = Vec::with_capacity(uid_strs.len());
    for (n, uid_str) in uid_strs.into_iter().enumerate() {
        match base64::decode(uid_str) {
            Ok(uid) => uids.push(uid),
            Err(e) => eprintln!("Error parsing key {}: {}", n, e),
        }
    }
    Ok(uids)
}

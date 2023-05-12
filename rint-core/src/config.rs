use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::{fs::File, path::Path};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub ip: String,

    pub port: u16,
}

const DEFAULT_CONFIG_CONTENT: &str = r#"
ip = '127.0.0.1'

port = 8796
"#;

impl Config {
    pub fn parse_from(path: &str) -> Config {
        let mut default = toml::from_str(DEFAULT_CONFIG_CONTENT).expect("加载默认配置失败");

        let mut config = load_content(path, default).expect("加载配置文件错误");

        return config;
    }
}

pub fn write_content<T>(file: &mut File, t: &T) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let content = toml::to_string(t)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
pub fn load_content<T>(path: &str, default: Option<T>) -> Result<T, Box<dyn Error>>
where
    T: Serialize + DeserializeOwned,
{
    let content_file_path = Path::new(path);
    if !content_file_path.exists() {
        // 创建默认配置文件
        let mut file = File::create(content_file_path)?;
        // 写入基本配置
        if let Some(t) = default {
            write_content(&mut file, &t)?;
        }
    };
    let content = fs::read_to_string(content_file_path)?;
    let res = toml::from_str(&content)?;
    Ok(res)
}

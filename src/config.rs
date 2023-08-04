use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    version: u8,
    pub default_relays:Vec<String>,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self {
        version: 0,
        default_relays:vec!["ws://localhost:8080".to_string()],
    } }
}

pub fn load_config() -> MyConfig {
    confy::load("ngit", None)
        .expect("load_config should always load either previously saved custom config saved via confy or the default config")
}

pub fn save_conifg(cfg:&MyConfig) -> &MyConfig {
    confy::store("ngit",None, &cfg)
        .expect("save_conifg should always save confy custom config or defaults for ngit-cli and return it");
    cfg
}


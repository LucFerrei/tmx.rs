use serde::Deserialize;
use std::{error::Error, fs};

#[derive(Deserialize, Debug)]
struct SearchPath {
   path: String, 
}

#[derive(Deserialize, Debug)]
pub struct Config {
    paths: Vec<SearchPath>
}

fn get_paths_from_config() -> Result<Config, Box<dyn Error>>{
    let config_path = "/home/lucas/.config/tmx/tmx.toml";
    let content = fs::read_to_string(config_path)?;

    let config: Config = toml::from_str(&content)?;
    // println!("Config: {:#?}", config);
    Ok(config)
}

pub fn transform_config_to_vector() -> Vec<String> {
   let mut result: Vec<String> = Vec::new(); 

   if let Ok(config) = get_paths_from_config() {
       for search_path in config.paths {
           result.push(search_path.path);
       }
   }else {
       result.push("/home/lucas/".to_string());
   }

   result
}

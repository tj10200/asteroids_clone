extern crate serde;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_xml_rs;
use std::collections::HashMap;
use std::fs;
use std::io::Result;

pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Sprite {
    pub fn w_radius(&self) -> f32 {
        self.width as f32 / 2.0
    }

    pub fn h_radius(&self) -> f32 {
        self.height as f32 / 2.0
    }
}

// XML Structure
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TextureAtlas {
    #[serde(rename = "imagePath")]
    image_path: String,
    #[serde(rename = "SubTexture")]
    textures: Vec<SubTexture>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SubTexture {
    name: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Resource)]
pub struct XMLSpriteSheetLoader {
    pub file: String,
    map: HashMap<String, Sprite>,
}

impl Default for XMLSpriteSheetLoader {
    fn default() -> Self {
        XMLSpriteSheetLoader {
            file: String::new(),
            map: HashMap::new(),
        }
    }
}

impl XMLSpriteSheetLoader {
    pub fn build(sprite_sheet: String, map_file: String) -> Result<XMLSpriteSheetLoader> {
        let map_file = format!("assets/{map_file}");

        match fs::read_to_string(map_file) {
            Ok(xml_string) => {
                let atlas_map: TextureAtlas = serde_xml_rs::from_str(&xml_string).unwrap();
                let mut map: HashMap<String, Sprite> = HashMap::new();
                for entry in atlas_map.textures {
                    let sprite = Sprite {
                        x: entry.x,
                        y: entry.y,
                        width: entry.width,
                        height: entry.height,
                    };
                    map.insert(entry.name.clone(), sprite);
                }
                Ok(XMLSpriteSheetLoader {
                    file: sprite_sheet.to_string(),
                    map,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_sprite(&self, sprite_name: String) -> Option<&Sprite> {
        self.map.get(&sprite_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::sprite_loader::mapper::{Sprite, XMLSpriteSheetLoader};
    use std::collections::HashMap;

    #[test]
    fn build_from_xml() {
        const SPRITE_SHEET_STR: &str = "assets/sprites/sheet.png";
        const SPRITE_SHEET_XML: &str = "assets/sprites/sheet.xml";

        match XMLSpriteSheetLoader::build(
            SPRITE_SHEET_STR.to_string(),
            SPRITE_SHEET_XML.to_string(),
        ) {
            Ok(loader) => {
                assert_eq!(loader.file, SPRITE_SHEET_STR, "sprite sheet's not equal");
                assert_ne!(loader.map.len(), 0);
            }
            Err(e) => {
                panic!("build returned an error: {e}");
            }
        }
    }

    #[test]
    fn test_get_sprite() {
        let loader = XMLSpriteSheetLoader {
            file: "".to_string(),
            map: HashMap::from([
                (
                    "test_1".to_string(),
                    Sprite {
                        x: 1,
                        y: 1,
                        width: 1,
                        height: 1,
                    },
                ),
                (
                    "test_2".to_string(),
                    Sprite {
                        x: 2,
                        y: 2,
                        width: 2,
                        height: 2,
                    },
                ),
                (
                    "test_3".to_string(),
                    Sprite {
                        x: 3,
                        y: 3,
                        width: 3,
                        height: 3,
                    },
                ),
            ]),
        };

        for i in 1..=3 {
            let test_name = format!("test_{i}");
            match loader.get_sprite(test_name.clone()) {
                Some(sprite) => {
                    assert_eq!(sprite.x, i);
                    assert_eq!(sprite.y, i);
                    assert_eq!(sprite.width, i as u32);
                    assert_eq!(sprite.height, i as u32);
                }
                None => {
                    panic!("sprite not found for {test_name}");
                }
            }
        }

        let negative_test = "test_not_real".to_string();
        if let Some(_) = loader.get_sprite(negative_test) {
            panic!("expected nothing, but found something");
        }
    }
}

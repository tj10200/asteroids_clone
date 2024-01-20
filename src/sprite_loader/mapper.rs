extern crate serde;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Vect};
use image;
use image::Pixel;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;
use serde_xml_rs;
use std::collections::HashMap;
use std::fs;

pub struct Sprite {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Sprite {
    pub fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.height / 2.0
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
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Resource)]
pub struct XMLSpriteSheetLoader {
    pub file: String,
    map: HashMap<String, Sprite>,
    collisions: HashMap<String, SpriteShapes>,
}

impl Default for XMLSpriteSheetLoader {
    fn default() -> Self {
        XMLSpriteSheetLoader {
            file: String::new(),
            map: HashMap::new(),
            collisions: HashMap::new(),
        }
    }
}

impl XMLSpriteSheetLoader {
    pub fn build<'a>(
        sprite_sheet: &'a str,
        map_file: &'a str,
        collision_file: &'a str,
    ) -> Result<XMLSpriteSheetLoader, std::io::Error> {
        let map = XMLSpriteSheetLoader::build_sprite_map(map_file).unwrap();
        let collisions = XMLSpriteSheetLoader::build_sprite_collisions(collision_file).unwrap();
        Ok(XMLSpriteSheetLoader {
            file: sprite_sheet.to_string(),
            map,
            collisions,
        })
    }

    fn build_sprite_map(map_file: &str) -> Result<HashMap<String, Sprite>, std::io::Error> {
        let map_file = format!("assets/{map_file}");

        match fs::read_to_string(map_file) {
            Ok(xml_string) => {
                let atlas_map: TextureAtlas = serde_xml_rs::from_str(&xml_string).unwrap();
                let mut map: HashMap<String, Sprite> = HashMap::new();
                for entry in atlas_map.textures {
                    let sprite = Sprite {
                        name: entry.name.clone(),
                        x: entry.x,
                        y: entry.y,
                        width: entry.width,
                        height: entry.height,
                    };
                    map.insert(entry.name.clone(), sprite);
                }
                Ok(map)
            }
            Err(e) => Err(e),
        }
    }

    fn build_sprite_collisions(
        sprite_collision_file: &str,
    ) -> Result<HashMap<String, SpriteShapes>, std::io::Error> {
        let sprite_collision_file = format!("assets/{sprite_collision_file}");
        match fs::read_to_string(sprite_collision_file) {
            Ok(json_string) => {
                let sprites: Vec<SpriteShapes> = serde_json::from_str(&json_string).unwrap();
                let mut map: HashMap<String, SpriteShapes> = HashMap::from(
                    sprites
                        .into_iter()
                        .map(|s| (s.name.clone(), s.clone()))
                        .collect::<HashMap<String, SpriteShapes>>(),
                );
                Ok(map)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_sprite(&self, sprite_name: &str) -> Option<&Sprite> {
        self.map.get(&sprite_name.to_string())
    }

    pub fn get_sprite_collider(
        &self,
        sprite_name: &str,
        frame: usize,
        to_origin: bool,
    ) -> Option<Collider> {
        let sprite_name = &sprite_name.to_string();
        if let Some(sprite) = self.get_sprite(sprite_name) {
            if let Some(shapes) = self.collisions.get(sprite_name) {
                return shapes.collider(sprite, frame, to_origin);
            }
        }
        None
    }
}

#[derive(Serialize, Clone)]
struct Shape {
    name: String,
    points: Vec<Vect>,
}

impl<'de> Deserialize<'de> for Shape {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            name: String,
            points: Vec<Vec<i32>>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let points = helper
            .points
            .into_iter()
            .map(|p| Vect::new(p[0] as f32, p[1] as f32))
            .collect();
        Ok(Shape {
            name: helper.name,
            points,
        })
    }
}

impl Shape {
    pub fn get_points(&self, sprite: &Sprite, to_origin: bool) -> Vec<Vect> {
        if !to_origin {
            return self.points.clone();
        }
        self.points
            .clone()
            .into_iter()
            .map(|p| {
                Vec2::new(
                    p.x - sprite.x - sprite.half_width(),
                    -p.y + sprite.y + sprite.half_height(),
                )
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Frame {
    frame: usize,
    shapes: Vec<Shape>,
}

#[derive(Serialize, Clone)]
pub struct SpriteShapes {
    name: String,
    frames: HashMap<usize, Frame>,
}

impl<'de> Deserialize<'de> for SpriteShapes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            name: String,
            frames: Vec<Frame>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let set: HashMap<usize, Frame> = HashMap::from(
            helper
                .frames
                .into_iter()
                .map(|f| (f.frame, f))
                .collect::<HashMap<usize, Frame>>(),
        );
        Ok(SpriteShapes {
            name: helper.name,
            frames: set,
        })
    }
}

impl SpriteShapes {
    pub fn collider(&self, sprite: &Sprite, frame: usize, to_origin: bool) -> Option<Collider> {
        let mut compound_shapes = Vec::new();
        if let Some(frame) = self.frames.get(&frame) {
            for shape in &frame.shapes {
                if let Some(convex_shape) =
                    Collider::convex_polyline(shape.get_points(sprite, to_origin))
                {
                    // Assuming each shape is placed at the origin (0.0, 0.0) of the compound collider.
                    // You can adjust the position of each shape within the compound collider as needed.
                    compound_shapes.push((Vec2::new(0.0, 0.0), 0.0, convex_shape));
                }
            }
            if compound_shapes.len() > 0 {
                return Some(Collider::compound(compound_shapes));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::sprite_loader::mapper::*;
    use std::collections::HashMap;

    const SPRITE_SHEET_STR: &str = "sprites/sheet.png";
    const SPRITE_SHEET_XML: &str = "sprites/sheet.xml";
    const SPRITE_SHEET_COLLISIONS: &str = "sprites/sheet1-edges.json";

    #[test]
    fn build_from_xml() {
        match XMLSpriteSheetLoader::build(
            SPRITE_SHEET_STR,
            SPRITE_SHEET_XML,
            SPRITE_SHEET_COLLISIONS,
        ) {
            Ok(loader) => {
                assert_eq!(loader.file, SPRITE_SHEET_STR, "sprite sheet's not equal");
                assert_ne!(loader.map.len(), 0);
                assert_ne!(loader.collisions.len(), 0);
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
                        name: "test_1".to_string(),
                        x: 1.0,
                        y: 1.0,
                        width: 1.0,
                        height: 1.0,
                    },
                ),
                (
                    "test_2".to_string(),
                    Sprite {
                        name: "test_2".to_string(),
                        x: 2.0,
                        y: 2.0,
                        width: 2.0,
                        height: 2.0,
                    },
                ),
                (
                    "test_3".to_string(),
                    Sprite {
                        name: "test_3".to_string(),
                        x: 3.0,
                        y: 3.0,
                        width: 3.0,
                        height: 3.0,
                    },
                ),
            ]),
            collisions: Default::default(),
        };

        for i in 1..=3 {
            let test_name = format!("test_{i}");
            match loader.get_sprite(&test_name) {
                Some(sprite) => {
                    assert_eq!(sprite.x, i as f32);
                    assert_eq!(sprite.y, i as f32);
                    assert_eq!(sprite.width, i as f32);
                    assert_eq!(sprite.height, i as f32);
                }
                None => {
                    panic!("sprite not found for {test_name}");
                }
            }
        }

        let negative_test = "test_not_real";
        if let Some(_) = loader.get_sprite(negative_test) {
            panic!("expected nothing, but found something");
        }
    }

    #[test]
    fn test_get_sprite_collider() {
        let loader = XMLSpriteSheetLoader {
            file: "".to_string(),
            map: Default::default(),
            collisions: HashMap::from([(
                "test_1".to_string(),
                SpriteShapes {
                    name: "test_1".to_string(),
                    frames: HashMap::from([(
                        0,
                        Frame {
                            frame: 0,
                            shapes: vec![Shape {
                                name: "shape".to_string(),
                                points: vec![
                                    Vect::new(0., 0.),
                                    Vect::new(1., 0.),
                                    Vect::new(0., 1.),
                                    Vect::new(1., 1.),
                                ],
                            }],
                        },
                    )]),
                },
            )]),
        };

        match loader.get_sprite_collider(&"test_1", 0, false) {
            Some(collider) => {
                println!("collider returned");
            }
            None => {
                panic!("collider not generated");
            }
        }

        match loader.get_sprite_collider(&"test_1", 1, false) {
            Some(collider) => {
                panic!("collider returned for unexpected frame");
            }
            None => {}
        }

        match loader.get_sprite_collider(&"not_there", 0, false) {
            Some(collider) => {
                panic!("collider returned for unknown sprite");
            }
            None => {}
        }
    }
}

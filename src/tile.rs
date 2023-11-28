use std::collections::HashMap;
use bevy::prelude::{Commands, Component, Entity};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::Value::{Object};
use crate::utils::read_json_file;

#[derive(Component)]
struct TileDeck(Vec<Entity>);

#[derive(Copy, Clone)]
enum Sides {
    North,
    East,
    South,
    West,
}

impl Sides {
    fn from_string(s: &String) -> Self {
        match s.as_str() {
            "N" => Self::North,
            "E" => Self::East,
            "S" => Self::South,
            _ => Self::West,
        }
    }

    fn to_index(&self) -> usize {
        match self {
            Sides::North => 0,
            Sides::East => 1,
            Sides::South => 2,
            Sides::West => 3
        }
    }
}

enum Animal {
    Monkey,
    Elephant,
    Giraffe,
    Lion,
    Zebra,
    None
}

impl Animal {
    fn from_string(s: String) -> Self {
        match s.as_str() {
            "M" => Self::Monkey,
            "E" => Self::Elephant,
            "G" => Self::Giraffe,
            "L" => Self::Lion,
            "Z" => Self::Zebra,
            _ => Self::None
        }
    }
}

#[derive(Copy, Clone)]
enum Land {
    Trail,
    Bush,
    Savanna
}

#[derive(Component)]
pub struct TileName(String);

#[derive(Component)]
struct Baobab;

#[derive(Component)]
struct TileConnections([Land; 4]);

#[derive(Component)]
pub struct Trail {
    sides: Vec<Sides>,
    animal: Animal
}

#[derive(Component)]
pub struct Bush {
    sides: Vec<Sides>,
    animal: Option<Animal>,
    birds: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct BushJson {
    sides: Vec<String>,
    birds: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct TileJson {
    trails: Vec<Vec<String>>,
    bushes: Vec<BushJson>,
    baobab: bool,
    variations: Vec<Vec<String>>
}

impl TileJson {
    fn from_json(json: &Value) -> Option<Self> {
        serde_json::from_value(json.clone()).ok()
    }

    fn json_to_struct() -> HashMap<String, Self> {
        let json = read_json_file("./assets/tiles.json".to_string());


        let mut tiles: Vec<TileJson> = Vec::new();
        let mut tiles_names: Vec<String> = Vec::new();

        if let Some(tiles_json) = json.get("tiles") {
            if let Object(tiles_obj) = tiles_json {

                for (tile_name, tile_json) in tiles_obj {
                    if let Some(tile) = TileJson::from_json(tile_json) {
                        tiles.push(tile);
                        tiles_names.push(tile_name.clone());
                    }
                }
            }
        }

        return tiles_names.into_iter().zip(tiles.into_iter()).collect()
    }
}

pub fn spawn_all_tiles(mut commands: Commands) {
    let tiles_map = TileJson::json_to_struct();
    let mut deck: Vec<Entity> = Vec::with_capacity(tiles_map.len());

    for (name, tile) in tiles_map {
        println!("Loading tile {}", name);
        for var in tile.variations {
            println!("\t{}", var.clone().into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(""));
            // Add name
            let mut entity = commands.spawn(TileName(name.clone()));
            // Add baobab
            if tile.baobab {
                entity.insert(Baobab);
            }
            let mut i: usize = 0;
            let mut connections = [Land::Savanna; 4];
            // Add trails
            for trail in &tile.trails {
                let mut sides: Vec<Sides> = Vec::new();
                for side_str in trail {
                    let side = Sides::from_string(side_str);
                    sides.push(side);
                    connections[side.to_index()] = Land::Trail;
                }
                entity.insert(Trail {
                    sides,
                    animal: Animal::from_string(var[i].clone())
                });
                i += 1;
            }
            // Add bushes
            for bush in &tile.bushes {
                let mut sides: Vec<Sides> = Vec::new();
                for side_str in &bush.sides {
                    let side = Sides::from_string(&side_str);
                    sides.push(side);
                    connections[side.to_index()] = Land::Bush;
                }
                let mut animal: Option<Animal> = None;
                if let animal_str = var[i].clone() {
                    animal = Some(Animal::from_string(animal_str));
                    i += 1;
                }
                entity.insert(Bush {
                    sides,
                    animal,
                    birds: bush.birds,
                });
            }

            deck.push(entity.id())
        }
    }

    commands.spawn(TileDeck(deck));
}
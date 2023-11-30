use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::Value::{Object};
use crate::LockToCam;
use crate::utils::read_json_file;

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
pub struct Deck;

#[derive(Component)]
pub struct Board(Vec<Vec<Option<Entity>>>);

#[derive(Component)]
pub struct TileName(String);

#[derive(Component)]
struct TileSideUp(bool);

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

// Structs for reading the json file
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

    fn json_to_struct(key: &str) -> HashMap<String, Self> {
        let json = read_json_file("./assets/tiles.json".to_string());


        let mut tiles: Vec<TileJson> = Vec::new();
        let mut tiles_names: Vec<String> = Vec::new();

        if let Some(tiles_json) = json.get(key) {
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

fn pos_to_matrix_index(coord: (i32, i32), n: usize) -> (usize, usize) {
    let center = n as i32 / 2;
    println!("{}", center);

    let j = (coord.0 + center) as usize;
    let i = (center - coord.1) as usize;

    let j = j.clamp(0, n - 1);
    let i = i.clamp(0, n - 1);

    (i, j)
}

pub fn spawn_all_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tiles_map = TileJson::json_to_struct("tiles");
    let deck = commands.spawn((Deck, LockToCam, SpriteBundle {
        texture: asset_server.load("tile_back.png"),
        ..default()
    })).id();

    for (name, tile) in tiles_map {
        println!("Loading tile {}", name);
        for var in tile.variations {
            println!("\t{}", var.join(""));
            // Add name
            let mut tile_commands = commands.spawn((TileName(name.clone()), TileSideUp(false), SpriteBundle {
                texture: asset_server.load("tile_base.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                visibility: Visibility::Hidden,
                ..default()
            }));
            // Add baobab
            if tile.baobab {
                tile_commands.insert(Baobab);
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
                tile_commands.insert(Trail {
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
                tile_commands.insert(Bush {
                    sides,
                    animal,
                    birds: bush.birds,
                });
            }

            let tile_entity = tile_commands.id();
            commands.entity(deck).push_children(&[tile_entity]);
        }
    }
}

pub fn spawn_starting_tiles(mut commands: Commands, query: Query<&Deck, &TileName>) {
    let tiles_map = TileJson::json_to_struct("starting_tile");
    let n = query.iter().count() + tiles_map.len();
    let mut board = vec![vec![None; n]; n];

    for (name, tile) in tiles_map {
        println!("Loading tile {}", name);
        for var in tile.variations {
            println!("\t{}", var.join(""));
            // Add name
            let mut entity = commands.spawn((TileName(name.clone()), TileSideUp(true), TransformBundle::default()));
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

            let (i, j) = match name.as_str() {
                "CSS1" => pos_to_matrix_index((-1,0), n),
                "CSS2" => pos_to_matrix_index((0,0), n),
                _ => pos_to_matrix_index((1,0), n),
            };

            board[i][j] = Some(entity.id());
        }
    }

    commands.spawn(Board(board));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pos_to_matrix_index_tl() {
        assert_eq!(pos_to_matrix_index((-36, 36), 72), (0, 0));
    }

    #[test]
    fn test_pos_to_matrix_index_bl() {
        assert_eq!(pos_to_matrix_index((-36, -35), 72), (71, 0));
    }

    #[test]
    fn test_pos_to_matrix_index_tr() {
        assert_eq!(pos_to_matrix_index((35, 36), 72), (0, 71));
    }

    #[test]
    fn test_pos_to_matrix_index_br() {
        assert_eq!(pos_to_matrix_index((35, -35), 72), (71, 71));
    }
}
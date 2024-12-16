use std::collections::HashMap;
use std::convert::TryInto;
use crate::game::board::*;

/// Converts a `TileKind` to its corresponding character representation.
///
/// This implementation maps each tile type to a unique character:
/// - `TileKind::Grain` -> `'G'`
/// - `TileKind::Wool` -> `'W'`
/// - `TileKind::Brick` -> `'B'`
/// - `TileKind::Lumber` -> `'L'`
/// - `TileKind::Ore` -> `'O'`
/// - `TileKind::Nothing` -> `'N'`
///
/// Example usage:
/// ```rust
/// let tile = TileKind::Brick;
/// let tile_char: char = tile.into();
/// assert_eq!(tile_char, 'B');
/// ```
impl From<TileKind> for char {
    fn from(tile: TileKind) -> Self {
        match tile {
            TileKind::Grain => 'G',
            TileKind::Wool => 'W',
            TileKind::Brick => 'B',
            TileKind::Lumber => 'L',
            TileKind::Ore => 'O',
            TileKind::Nothing => 'N',
        }
    }
}


/// Attempts to convert a character into a `TileKind`.
///
/// This implementation maps specific characters to their corresponding `TileKind` enum:
/// - `'G'` -> `TileKind::Grain`
/// - `'W'` -> `TileKind::Wool`
/// - `'B'` -> `TileKind::Brick`
/// - `'L'` -> `TileKind::Lumber`
/// - `'O'` -> `TileKind::Ore`
/// - `'N'` -> `TileKind::Nothing`
///
/// Returns an error string if the character does not correspond to a valid `TileKind`.
///
/// # Type Parameters
/// - `Self::Error`: The error type, which is a static string slice (`&'static str`).
///
/// # Arguments
/// - `c`: The character to be converted.
///
/// # Returns
/// A `Result` containing the corresponding `TileKind` if the character is valid, or an error string if invalid.
///
/// # Example
/// ```rust
/// use std::convert::TryFrom;
///
/// let tile_kind = TileKind::try_from('G').unwrap();
/// assert_eq!(tile_kind, TileKind::Grain);
///
/// let invalid_tile = TileKind::try_from('X');
/// assert!(invalid_tile.is_err());
/// ```
impl TryFrom<char> for TileKind {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'G' => Ok(TileKind::Grain),
            'W' => Ok(TileKind::Wool),
            'B' => Ok(TileKind::Brick),
            'L' => Ok(TileKind::Lumber),
            'O' => Ok(TileKind::Ore),
            'N' => Ok(TileKind::Nothing),
            _ => Err("Invalid character for TileKind"),
        }
    }
}


/// Attempts to convert a character into a `BuildingKind`.
///
/// This implementation maps specific characters to their corresponding `BuildingKind` enum:
/// - `'S'` -> `BuildingKind::Settlement`
/// - `'C'` -> `BuildingKind::City`
///
/// Returns an error string if the character is invalid.
///
/// Example usage:
/// ```rust
/// use std::convert::TryFrom;
///
/// let building = BuildingKind::try_from('S').unwrap();
/// assert_eq!(building, BuildingKind::Settlement);
///
/// let invalid = BuildingKind::try_from('X');
/// assert!(invalid.is_err());
/// ```
impl TryFrom<char> for BuildingKind {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(BuildingKind::Settlement),
            'C' => Ok(BuildingKind::City),
            _ => Err("Invalid character for BuildingKind"),
        }
    }
}


/// Converts a `Player` to its corresponding character representation.
///
/// This implementation maps each player to a unique character:
/// - `Player::Red` -> `'R'`
/// - `Player::Blue` -> `'B'`
/// - `Player::White` -> `'W'`
///
/// Example usage:
/// ```rust
/// let player = Player::Red;
/// let player_char: char = player.into();
/// assert_eq!(player_char, 'R');
/// ```
impl From<Player> for char {
    fn from(player: Player) -> Self {
        match player {
            Player::Red => 'R',
            Player::Blue => 'B',
            Player::White => 'W',
        }
    }
}



/// Attempts to convert a `String` representation of a game state into a `Game` object.
///
/// This implementation parses the ASCII representation of the game board, extracting buildings,
/// roads, tiles, and the position of the robber, and initializes a `Game` object with the parsed state.
///
/// # Type Parameters
/// - `Self::Error`: The error type, which is a static string slice (`&'static str`).
///
/// # Arguments
/// - `board_str`: A string representing the state of the game, based on the `TEMPLATE`.
///
/// # Returns
/// A `Result` containing the parsed `Game` object if successful, or an error message if the input is invalid.
///
/// # Parsing Details
/// - Parses `building_coordinates` to identify building positions and their attributes.
/// - Parses `tile_coordinates` to extract tile dice values and resources.
/// - Parses `road_coordinates` to identify the location and ownership of roads.
/// - Ensures that the number of parsed elements matches the expected counts defined by constants
///   (e.g., `INTERSECTIONS`, `TILES`, `PATHS`).
///
/// # Example
/// ```rust
/// use std::convert::TryFrom;
///
/// let game_state = "
///           oo . oo . oo . oo . oo W oo W oo
///           .   10O   .   02W   .   09L   W
///      oo . oo . oo . RS R oo . oo B BS W oo . oo
///      .   12G   .   06B   .   04W   W   10B   .
/// oo . oo . oo W WS . oo . oo . oo . oo W oo . oo . oo
/// .   09G!  .   11L   .   00N   .   03L   W   08O   .
/// oo . oo . RS R oo . oo . oo . oo . oo . WS . oo . oo
///      .   08L   .   03O   .   04G   B   05W   .
///      oo . oo . RS B oo . oo . oo . RS . oo . oo
///           .   05B   .   06G   .   11W   .
///           oo . oo . oo . oo . oo . oo . oo
/// ".to_string();
///
/// let game: Game = Game::try_from(game_state).unwrap();
/// ```
///
/// # Errors
/// - Returns an error if the string does not match the expected format or fails during parsing.
/// - Ensures that mandatory elements such as buildings, tiles, and roads are properly defined.
///
/// # Notes
/// - The template used for parsing is defined in the constant `TEMPLATE`.
/// - Any discrepancies in the string's structure or missing elements will result in an error.
impl TryFrom<String> for Game {
    type Error = &'static str;

    fn try_from(board_str: String) -> Result<Self, Self::Error> {
        let mut building_coordinates = vec![];
        let mut tile_coordinates = vec![];
        let mut road_coordinates = vec![];
        for line in TEMPLATE.lines() {
            let line = line.trim_end();
            let mut building_line = vec![];
            let mut tile_line = vec![];
            let mut road_line = vec![];
            let chars: Vec<char> = line.chars().clone().collect();
            for (i, c) in chars.iter().enumerate() {
                if *c == 'B' && chars.get(i + 1) == Some(&'B') {
                    building_line.push(i);
                }
                if *c == 'T' && chars.get(i + 1) == Some(&'T') && chars.get(i + 2) == Some(&'T') && chars.get(i + 3) == Some(&'T') {
                    tile_line.push(i);
                }

                if *c == '*'  {
                    road_line.push(i);
                }
            }
            building_coordinates.push(building_line);
            tile_coordinates.push(tile_line);
            road_coordinates.push(road_line);
        }

        assert_eq!(building_coordinates.iter().map(|c| c.len()).sum::<usize>(), INTERSECTIONS);
        assert_eq!(tile_coordinates.iter().map(|t| t.len()).sum::<usize>(), TILES);
        assert_eq!(road_coordinates.iter().map(|t| t.len()).sum::<usize>(), PATHS);


        let mut id = 0;
        let mut buildings: Vec<Building> = vec![];
        for (i, line_coordinates) in building_coordinates.iter().enumerate() {
            let chars: Vec<char> = board_str.lines().nth(i).unwrap().chars().clone().collect();
            for coordinate in line_coordinates {
                let first_char = chars[*coordinate];
                let second_char = chars[coordinate + 1];
                if first_char != 'o' {
                    let building = Building{
                        intersection_id: IntersectionId(id),
                        kind: second_char.try_into()?,
                        player: first_char.try_into()?,
                    };
                    buildings.push(building);
                }
                id += 1;
            }
        }


        let mut id = 0;
        let mut roads: Vec<Road> = vec![];
        for (i, line_coordinates) in road_coordinates.iter().enumerate() {
            let chars: Vec<char> = board_str.lines().nth(i).unwrap().chars().clone().collect();
            for coordinate in line_coordinates {
                let first_char = chars[*coordinate];
                if first_char != '.' {
                    let road = Road{
                        id: PathId(id),
                        player: first_char.try_into()?,
                    };
                    roads.push(road);
                }
                id += 1;
            }
        }


        let mut id = 0;
        let mut tiles: Vec<Tile> = vec![];
        let mut robber: Option<RobberId> = None;

        for (i, line_coordinates) in tile_coordinates.iter().enumerate() {
            let chars: Vec<char> = board_str.lines().nth(i).unwrap().chars().clone().collect();
            for coordinate in line_coordinates {
                let first_char = chars[*coordinate];
                let second_char = chars[coordinate + 1];
                let third_char = chars[coordinate + 2];
                let fourth_char = chars[coordinate + 3];
                if fourth_char == '!' {
                    robber = Some(RobberId(id))
                }
                let kind: TileKind = TileKind::try_from(third_char)?;

                let dice = format!("{}{}", first_char, second_char).parse::<u8>().expect("Invalid tile dice number");
                tiles.push(Tile{ dice, kind });
                id += 1;
            }
        }

        let board:  Board = Board::new(tiles.try_into().expect("The board has not exactly 19 tiles"));

        Ok(
            Game{ board,
                state: State {
                    buildings,
                    roads,
                    robber: robber.unwrap(),
                } }
        )
    }
}

/// Converts a `Game` object into a string representation.
///
/// This implementation serializes the current state of the game into an ASCII representation
/// based on the predefined `TEMPLATE`. The string output includes details about the tiles,
/// buildings, roads, and the position of the robber.
///
/// # Arguments
/// - `game`: The `Game` object to be converted into a string representation.
///
/// # Returns
/// A string representing the state of the game, ready for display or further processing.
///
/// # Serialization Details
/// - **Tiles**: Replaces `TTTT` placeholders in the template with dice values, tile types, and the robber position.
/// - **Buildings**: Replaces `BB` placeholders with the player owning the building and the building type.
/// - **Roads**: Replaces `*` placeholders with the player owning the road, or `.` if no road exists.
///
/// # Example
/// ```rust
/// use std::convert::From;
///
/// let game: Game = ...; // Assume the game is already initialized
/// let serialized: String = game.into();
/// println!("{}", serialized);
/// ```
///
/// # Notes
/// - The function uses the `TEMPLATE` constant to define the structure of the serialized string.
/// - If a building or road is not present at a specific location, default placeholders (`oo` for buildings, `.` for roads) are used.
///
/// # Implementation Details
/// - The function uses maps to track buildings and roads for efficient replacement in the template.
/// - The robber's position is indicated with a `!` character appended to the tile description.
impl From<Game> for String {
    fn from(game: Game) -> Self {
        let mut output  = TEMPLATE.to_string();
        for (id, tile) in game.board.tiles.iter().enumerate() {
            let robber = if RobberId(id) == game.state.robber {
                '!'
            } else {
                ' '
            };
            let kind = char::from(tile.kind.clone());
            output = output.replacen("TTTT",  &format!("{:02}{}{}", tile.dice, kind, robber), 1);
        }

        let mut building_map = HashMap::new();
        for  int in  game.state.buildings.iter() {
            building_map.insert(&int.intersection_id, int);
        }

        for i in 0..INTERSECTIONS {
            let cell = match building_map.get(&IntersectionId(i)) {
                None => { "oo".to_string() }
                Some(int) => {
                    let player: char = int.player.into();
                    let kind = match int.kind {
                        BuildingKind::Settlement =>{ "S".to_string() }
                        BuildingKind::City => { "C".to_string() }
                    };
                    format!("{}{}", player, kind)
                }
            };

            output = output.replacen("BB", &cell, 1);
        }

        let mut road_map = HashMap::new();
        for  int in  game.state.roads.iter() {
            road_map.insert(&int.id, int);
        }

        for i in 0..PATHS {
            let cell = match road_map.get(&PathId(i)) {
                None => { ".".to_string() }
                Some(int) => char::from(int.player).into(),
            };

            output = output.replacen("*", &cell, 1);
        }

        output
    }
}

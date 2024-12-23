use std::cmp::PartialEq;
use std::convert::TryFrom;
use crate::game::resources::PlayerResourceCount;

/// An enumeration representing the players in the Settlers of Catan game.
///
/// Each player is identified by a color:
/// - `red`: The red player.
/// - `blue`: The blue player.
/// - `white`: The white player.
///
/// This enum is used to specify the owner of buildings, roads, or other player-specific attributes.
///
/// Example usage:
/// ```rust
/// let current_player = Player::red;
/// match current_player {
///     Player::red => println!("red player's turn"),
///     Player::blue => println!("blue player's turn"),
///     Player::white => println!("white player's turn"),
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    Red,
    Blue,
    White,
}


/// A unique identifier for an intersection in the Settlers of Catan game.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct IntersectionId(pub usize);

/// Represents a path connecting two intersections.
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Path(pub IntersectionId, pub IntersectionId);

/// Attempts to convert a character into a `Player`.
///
/// This implementation maps specific characters to their corresponding `Player` enum:
/// - `'R'` -> `Player::red`
/// - `'B'` -> `Player::blue`
/// - `'W'` -> `Player::white`
///
/// Returns an error string if the character is invalid.
///
/// Example usage:
/// ```rust
/// use std::convert::TryFrom;
///
/// let player = Player::try_from('R').unwrap();
/// assert_eq!(player, Player::red);
///
/// let invalid = Player::try_from('X');
/// assert!(invalid.is_err());
/// ```
impl TryFrom<char> for Player {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'R' => Ok(Player::Red),
            'B' => Ok(Player::Blue),
            'W' => Ok(Player::White),
            _ => Err("Invalid character for Player"),
        }
    }
}


/// An enumeration representing the different types of tiles in the Settlers of Catan game.
///
/// Each tile provides a specific resource or has no resource:
/// - `grain`: Produces grain.
/// - `wool`: Produces wool.
/// - `brick`: Produces brick.
/// - `lumber`: Produces lumber.
/// - `ore`: Produces ore.
/// - `Nothing`: Represents the desert or other non-productive tiles.
///
/// This enum is used to describe the resource type associated with each tile on the board.
///
/// Example usage:
/// ```rust
/// let tile = TileKind::grain;
/// match tile {
///     TileKind::grain => println!("This tile produces grain."),
///     TileKind::Nothing => println!("This is a desert tile."),
///     _ => println!("This tile produces another resource."),
/// }
/// ```
#[derive(Debug, Clone)]
pub enum  TileKind {
    Grain,
    Wool,
    Brick,
    Lumber,
    Ore,
    Nothing
}



/// A unique identifier for a tile in the Settlers of Catan game.
#[derive(Debug)]
struct TileId(usize);


/// A unique identifier for the position of the robber on the game board.
#[derive(Eq, PartialEq)]
pub struct RobberId(pub usize);


/// Represents a tile on the game board.
///
/// Each tile has a dice value and a resource type (`TileKind`).


/// Represents a tile on the game board.
///
/// Each tile has a dice value and a resource type (`TileKind`).
#[derive(Debug)]
pub struct Tile {
    pub dice: u8,
    pub kind: TileKind
}

/// An enumeration representing the types of buildings in the Settlers of Catan game.
///
/// - `Settlement`: A basic building that provides fewer points/resources.
/// - `City`: An upgraded building that provides more points/resources.
#[derive(Debug, Copy, Clone)]
pub enum BuildingKind {
    Settlement,
    City,
}

/// Converts a `BuildingKind` to its corresponding character representation.
///
/// This implementation maps each building type to a unique character:
/// - `BuildingKind::Settlement` -> `'S'`
/// - `BuildingKind::City` -> `'C'`
///
/// Example usage:
/// ```rust
/// let building = BuildingKind::City;
/// assert_eq!(building.to_char(), 'C');
/// ```
impl BuildingKind {
    fn to_char(&self) -> char {
        match self {
            BuildingKind::Settlement => 'S',
            BuildingKind::City => 'C',
        }
    }
}

/// Represents a building on the board, including its location (`IntersectionId`),
/// its type (`BuildingKind`), and the player who owns it.
#[derive(Debug, Copy, Clone)]
pub struct Building {
    pub intersection_id: IntersectionId,
    pub kind: BuildingKind,
    pub player: Player
}


/// Represents a road on the board, including its location (`PathId`)
/// and the player who owns it.
#[derive(Debug)]
pub struct Road {
    pub id: PathId,
    pub player: Player
}


/// A unique identifier for a path (road) in the Settlers of Catan game.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PathId(pub usize);

#[derive(Debug)]
pub struct Intersection {
    pub paths: Vec<PathId>,
    tiles: Vec<TileId>,
}

/// Represents an intersection on the game board.
///
/// Each intersection connects multiple paths (`PathId`) and may touch multiple tiles (`TileId`).
impl Intersection {
    /// Creates a new `Intersection` with the given paths and tiles.
    fn new(paths: Vec<PathId>, tiles: Vec<TileId>) -> Intersection {
        Self {
            paths,
            tiles,
        }
    }
}

pub const PATHS: usize = 72;
pub const INTERSECTIONS: usize = 54;
pub const TILES: usize = 19;

/// Represents the game board in Settlers of Catan.
///
/// The board consists of:
/// - `paths`: An array of roads (`Path`) connecting intersections.
/// - `intersections`: An array of intersections where buildings can be placed.
/// - `tiles`: An array of resource tiles on the board.
pub struct Board {
    pub paths: [Path; PATHS],
    pub intersections: [Intersection; INTERSECTIONS],
    pub tiles: [Tile; TILES]
}

impl Board {
    /// Creates a new `Board` with the given tiles.
    ///
    /// The paths and intersections are pre-defined for the standard Catan board.
    ///
    /// # Arguments
    /// - `tiles`: An array of 19 tiles representing the game board resources and their dice values.
    ///
    /// Example usage:
    /// ```rust
    /// let tiles = [
    ///     Tile { dice: 10, kind: TileKind::grain },
    ///     Tile { dice: 2, kind: TileKind::wool },
    ///     // ...remaining tiles...
    /// ];
    /// let board = Board::new(tiles);
    /// ```
    pub fn new(tiles: [Tile; 19]) -> Board {
        let paths: [Path;72] = [
            Path(IntersectionId(0), IntersectionId(1)), // 0
            Path(IntersectionId(1), IntersectionId(2)), // 1
            Path(IntersectionId(2), IntersectionId(3)), // 2
            Path(IntersectionId(3), IntersectionId(4)), // 3
            Path(IntersectionId(4), IntersectionId(5)), // 4
            Path(IntersectionId(5), IntersectionId(6)), // 5
            Path(IntersectionId(0), IntersectionId(8)), // 6
            Path(IntersectionId(2), IntersectionId(10)),// 7
            Path(IntersectionId(4), IntersectionId(12)),// 8
            Path(IntersectionId(6), IntersectionId(14)),// 9
            Path(IntersectionId(8), IntersectionId(7)), // 10
            Path(IntersectionId(8), IntersectionId(9)), // 11
            Path(IntersectionId(9), IntersectionId(10)), // 12
            Path(IntersectionId(10), IntersectionId(11)), // 13
            Path(IntersectionId(11), IntersectionId(12)), // 14
            Path(IntersectionId(12), IntersectionId(13)), // 15
            Path(IntersectionId(13), IntersectionId(14)), // 16
            Path(IntersectionId(14), IntersectionId(15)), // 17
            Path(IntersectionId(7), IntersectionId(17)), // 18
            Path(IntersectionId(9), IntersectionId(19)), // 19
            Path(IntersectionId(11), IntersectionId(21)), // 20
            Path(IntersectionId(13), IntersectionId(23)), // 21
            Path(IntersectionId(15), IntersectionId(25)), // 22
            Path(IntersectionId(16), IntersectionId(17)), // 23
            Path(IntersectionId(17), IntersectionId(18)), // 24
            Path(IntersectionId(18), IntersectionId(19)), // 25
            Path(IntersectionId(19), IntersectionId(20)), // 26
            Path(IntersectionId(20), IntersectionId(21)), // 27
            Path(IntersectionId(21), IntersectionId(22)), // 28
            Path(IntersectionId(22), IntersectionId(23)), // 29
            Path(IntersectionId(23), IntersectionId(24)), // 30
            Path(IntersectionId(24), IntersectionId(25)), // 31
            Path(IntersectionId(25), IntersectionId(26)), // 32
            Path(IntersectionId(16), IntersectionId(27)), // 33
            Path(IntersectionId(18), IntersectionId(29)), // 34
            Path(IntersectionId(20), IntersectionId(31)), // 35
            Path(IntersectionId(22), IntersectionId(33)), // 36
            Path(IntersectionId(24), IntersectionId(35)), // 37
            Path(IntersectionId(26), IntersectionId(37)), // 38
            Path(IntersectionId(27), IntersectionId(28)), // 39
            Path(IntersectionId(28), IntersectionId(29)), // 40
            Path(IntersectionId(29), IntersectionId(30)), // 41
            Path(IntersectionId(30), IntersectionId(31)), // 42
            Path(IntersectionId(31), IntersectionId(32)), // 43
            Path(IntersectionId(32), IntersectionId(33)), // 44
            Path(IntersectionId(33), IntersectionId(34)), // 45
            Path(IntersectionId(34), IntersectionId(35)), // 46
            Path(IntersectionId(35), IntersectionId(36)), // 47
            Path(IntersectionId(36), IntersectionId(37)), // 48
            Path(IntersectionId(28), IntersectionId(38)), // 49
            Path(IntersectionId(30), IntersectionId(40)), // 50
            Path(IntersectionId(32), IntersectionId(42)), // 51
            Path(IntersectionId(34), IntersectionId(44)), // 52
            Path(IntersectionId(36), IntersectionId(46)), // 53
            Path(IntersectionId(38), IntersectionId(39)), // 54
            Path(IntersectionId(39), IntersectionId(40)), // 55
            Path(IntersectionId(40), IntersectionId(41)), // 56
            Path(IntersectionId(41), IntersectionId(42)), // 57
            Path(IntersectionId(42), IntersectionId(43)), // 58
            Path(IntersectionId(43), IntersectionId(44)), // 59
            Path(IntersectionId(44), IntersectionId(45)), // 60
            Path(IntersectionId(45), IntersectionId(45)), // 61
            Path(IntersectionId(39), IntersectionId(47)), // 62
            Path(IntersectionId(41), IntersectionId(49)), // 63
            Path(IntersectionId(43), IntersectionId(51)), // 64
            Path(IntersectionId(45), IntersectionId(53)), // 65
            Path(IntersectionId(47), IntersectionId(48)), // 66
            Path(IntersectionId(48), IntersectionId(49)), // 67
            Path(IntersectionId(49), IntersectionId(50)), // 68
            Path(IntersectionId(50), IntersectionId(51)), // 69
            Path(IntersectionId(51), IntersectionId(52)), // 70
            Path(IntersectionId(52), IntersectionId(53)), // 71
        ];

        let intersections: [Intersection; 54] = [
            Intersection::new(vec![PathId(0), PathId(6)], vec![TileId(0)]), // 0
            Intersection::new(vec![PathId(0), PathId(1)], vec![TileId(0)]), // 1
            Intersection::new(vec![PathId(1), PathId(2), PathId(7)], vec![TileId(0), TileId(1)]), // 2
            Intersection::new(vec![PathId(2), PathId(3)], vec![TileId(1)]), // 3
            Intersection::new(vec![PathId(3), PathId(4), PathId(8)], vec![TileId(1), TileId(2)]), // 4
            Intersection::new(vec![PathId(4), PathId(5)], vec![TileId(2)]), // 5
            Intersection::new(vec![PathId(5), PathId(9)], vec![TileId(2)]), // 6
            Intersection::new(vec![PathId(10), PathId(18)], vec![TileId(3)]),  // 7
            Intersection::new(vec![PathId(10), PathId(6), PathId(11)], vec![TileId(3), TileId(0)]),  // 8
            Intersection::new(vec![PathId(11), PathId(12), PathId(19)], vec![TileId(3), TileId(4)]), // 9
            Intersection::new(vec![PathId(7), PathId(12), PathId(13)], vec![TileId(0), TileId(1), TileId(4)]), // 10
            Intersection::new(vec![PathId(13), PathId(14), PathId(20)], vec![TileId(1), TileId(4), TileId(5)]), // 11
            Intersection::new(vec![PathId(8), PathId(14), PathId(15)], vec![TileId(1), TileId(2), TileId(5)]), // 12
            Intersection::new(vec![PathId(15), PathId(16), PathId(21)], vec![TileId(2), TileId(5), TileId(6)]), // 13
            Intersection::new(vec![PathId(9), PathId(16), PathId(17)], vec![TileId(2), TileId(6)]), // 14
            Intersection::new(vec![PathId(17), PathId(22)], vec![TileId(6)]), // 15
            Intersection::new(vec![PathId(23), PathId(33)], vec![TileId(7)]), // 16
            Intersection::new(vec![PathId(18), PathId(23), PathId(24)], vec![TileId(3), TileId(7)]), // 17
            Intersection::new(vec![PathId(24), PathId(25), PathId(34)], vec![TileId(3), TileId(7), TileId(8)]), // 18
            Intersection::new(vec![PathId(19), PathId(25), PathId(26)], vec![TileId(3), TileId(4), TileId(8)]), // 19
            Intersection::new(vec![PathId(26), PathId(27), PathId(35)], vec![TileId(4), TileId(8), TileId(9)]), // 20
            Intersection::new(vec![PathId(20), PathId(27), PathId(28)], vec![TileId(4), TileId(5), TileId(9)]), // 21
            Intersection::new(vec![PathId(28), PathId(29), PathId(36)], vec![TileId(5), TileId(9), TileId(10)]), // 22
            Intersection::new(vec![PathId(21), PathId(29), PathId(30)], vec![TileId(5), TileId(6), TileId(10)]), // 23
            Intersection::new(vec![PathId(30), PathId(31), PathId(37)], vec![TileId(6), TileId(10), TileId(11)]), // 24
            Intersection::new(vec![PathId(22), PathId(31), PathId(32)], vec![TileId(6), TileId(11)]), // 25
            Intersection::new(vec![PathId(32), PathId(38)], vec![TileId(11)]), // 26
            Intersection::new(vec![PathId(33), PathId(39)], vec![TileId(7)]), // 27
            Intersection::new(vec![PathId(39), PathId(40), PathId(49)], vec![TileId(7), TileId(12)]), // 28
            Intersection::new(vec![PathId(34), PathId(40), PathId(41)], vec![TileId(7), TileId(8), TileId(12)]), // 29
            Intersection::new(vec![PathId(41), PathId(42), PathId(50)], vec![TileId(8), TileId(12), TileId(13)]), // 30
            Intersection::new(vec![PathId(35), PathId(42), PathId(43)], vec![TileId(8), TileId(9), TileId(13)]), // 31
            Intersection::new(vec![PathId(43), PathId(44), PathId(51)], vec![TileId(9), TileId(13), TileId(14)]), // 32
            Intersection::new(vec![PathId(36), PathId(44), PathId(45)], vec![TileId(9), TileId(10), TileId(14)]), // 33
            Intersection::new(vec![PathId(45), PathId(46), PathId(52)], vec![TileId(10), TileId(14), TileId(15)]), // 34
            Intersection::new(vec![PathId(37), PathId(46), PathId(47)], vec![TileId(10), TileId(11), TileId(15)]), // 35
            Intersection::new(vec![PathId(47), PathId(48), PathId(53)], vec![TileId(11), TileId(15)]), // 36
            Intersection::new(vec![PathId(38), PathId(48)], vec![TileId(11)]), // 37
            Intersection::new(vec![PathId(49), PathId(54)], vec![TileId(12)]), // 38
            Intersection::new(vec![PathId(54), PathId(55), PathId(62)], vec![TileId(12), TileId(16)]), // 39
            Intersection::new(vec![PathId(50), PathId(55), PathId(56)], vec![TileId(12), TileId(13), TileId(16)]), // 40
            Intersection::new(vec![PathId(56), PathId(57), PathId(63)], vec![TileId(13), TileId(16), TileId(17)]), // 41
            Intersection::new(vec![PathId(51), PathId(57), PathId(58)], vec![TileId(13), TileId(14), TileId(17)]), // 42
            Intersection::new(vec![PathId(58), PathId(59), PathId(64)], vec![TileId(14), TileId(17), TileId(18)]), // 43
            Intersection::new(vec![PathId(52), PathId(59), PathId(60)], vec![TileId(14), TileId(15), TileId(18)]), // 44
            Intersection::new(vec![PathId(60), PathId(61), PathId(65)], vec![TileId(15), TileId(18)]), // 45
            Intersection::new(vec![PathId(53), PathId(61)], vec![TileId(15)]), // 46
            Intersection::new(vec![PathId(62), PathId(66)], vec![TileId(16)]), // 47
            Intersection::new(vec![PathId(66), PathId(67)], vec![TileId(16)]), // 48
            Intersection::new(vec![PathId(63), PathId(67), PathId(68)], vec![TileId(16), TileId(17)]), // 49
            Intersection::new(vec![PathId(68), PathId(69)], vec![TileId(17)]), // 50
            Intersection::new(vec![PathId(64), PathId(69), PathId(70)], vec![TileId(17), TileId(18)]), // 51
            Intersection::new(vec![PathId(70), PathId(71)], vec![TileId(18)]), // 52
            Intersection::new(vec![PathId(65), PathId(71)], vec![TileId(18)]), // 53
        ];

        Self {
            paths,
            intersections,
            tiles,
        }
    }
}

/// Represents the state of the game, including:
/// - `buildings`: A list of all buildings on the board.
/// - `roads`: A list of all roads on the board.
/// - `robber`: The current position of the robber.
pub struct State {
    pub buildings: Vec<Building>,
    pub roads: Vec<Road>,
    pub robber: RobberId,
    pub resources: PlayerResourceCount,
}

/// Represents the overall game state, including the board and the state of all players.
pub struct Game {
    pub board: Board,
    pub state: State,
}


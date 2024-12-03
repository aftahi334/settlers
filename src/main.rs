use crate::Player::White;

#[derive(Debug)]
enum Player {
    Red,
    Blue,
    White,
}

#[derive(Debug)]
enum  TileKind {
    Grain,
    Wool,
    Brick,
    Lumber,
    Ore,
    Nothing
}


#[derive(Debug)]
struct TileId(usize);
#[derive(Debug)]
struct Tile {
    dice: u8,
    kind: TileKind
}

#[derive(Debug)]
enum BuildingKind {
    Settlement,
    City,
}

#[derive(Debug)]
struct Building {
    id: IntersectionId,
    kind: BuildingKind,
    player: Player
}


#[derive(Debug)]
struct Road {
    id: PathId,
    player: Player
}


#[derive(Debug)]
struct PathId(usize);

#[derive(Debug)]
struct Intersection {
    paths: Vec<PathId>,
    tiles: Vec<TileId>,
}

impl Intersection {
    fn new(paths: Vec<PathId>, tiles: Vec<TileId>) -> Intersection {
        Self {
            paths,
            tiles,
        }
    }
}

struct Board {
    paths: [Path; 72],
    intersections: [Intersection; 54],
    tiles: [Tile; 19]
}

impl Board {
    fn new(tiles: [Tile; 19]) -> Board {
        let paths: [Path;72] = [
            Path::new(vec![IntersectionId(0), IntersectionId(1)]), // 0
            Path::new(vec![IntersectionId(1), IntersectionId(2)]), // 1
            Path::new(vec![IntersectionId(2), IntersectionId(3)]), // 2
            Path::new(vec![IntersectionId(3), IntersectionId(4)]), // 3
            Path::new(vec![IntersectionId(4), IntersectionId(5)]), // 4
            Path::new(vec![IntersectionId(5), IntersectionId(6)]), // 5
            Path::new(vec![IntersectionId(0), IntersectionId(8)]), // 6
            Path::new(vec![IntersectionId(2), IntersectionId(10)]),// 7
            Path::new(vec![IntersectionId(4), IntersectionId(12)]),// 8
            Path::new(vec![IntersectionId(6), IntersectionId(14)]),// 9
            Path::new(vec![IntersectionId(8), IntersectionId(7)]), // 10
            Path::new(vec![IntersectionId(8), IntersectionId(9)]), // 11
            Path::new(vec![IntersectionId(9), IntersectionId(10)]), // 12
            Path::new(vec![IntersectionId(10), IntersectionId(11)]), // 13
            Path::new(vec![IntersectionId(11), IntersectionId(12)]), // 14
            Path::new(vec![IntersectionId(12), IntersectionId(13)]), // 15
            Path::new(vec![IntersectionId(13), IntersectionId(14)]), // 16
            Path::new(vec![IntersectionId(14), IntersectionId(15)]), // 17
            Path::new(vec![IntersectionId(7), IntersectionId(17)]), // 18
            Path::new(vec![IntersectionId(9), IntersectionId(19)]), // 19
            Path::new(vec![IntersectionId(11), IntersectionId(21)]), // 20
            Path::new(vec![IntersectionId(13), IntersectionId(23)]), // 21
            Path::new(vec![IntersectionId(15), IntersectionId(25)]), // 22
            Path::new(vec![IntersectionId(16), IntersectionId(17)]), // 23
            Path::new(vec![IntersectionId(17), IntersectionId(18)]), // 24
            Path::new(vec![IntersectionId(18), IntersectionId(19)]), // 25
            Path::new(vec![IntersectionId(19), IntersectionId(20)]), // 26
            Path::new(vec![IntersectionId(20), IntersectionId(21)]), // 27
            Path::new(vec![IntersectionId(21), IntersectionId(22)]), // 28
            Path::new(vec![IntersectionId(22), IntersectionId(23)]), // 29
            Path::new(vec![IntersectionId(23), IntersectionId(24)]), // 30
            Path::new(vec![IntersectionId(24), IntersectionId(25)]), // 31
            Path::new(vec![IntersectionId(25), IntersectionId(26)]), // 32
            Path::new(vec![IntersectionId(16), IntersectionId(27)]), // 33
            Path::new(vec![IntersectionId(18), IntersectionId(29)]), // 34
            Path::new(vec![IntersectionId(20), IntersectionId(31)]), // 35
            Path::new(vec![IntersectionId(22), IntersectionId(33)]), // 36
            Path::new(vec![IntersectionId(24), IntersectionId(35)]), // 37
            Path::new(vec![IntersectionId(26), IntersectionId(37)]), // 38
            Path::new(vec![IntersectionId(27), IntersectionId(28)]), // 39
            Path::new(vec![IntersectionId(28), IntersectionId(29)]), // 40
            Path::new(vec![IntersectionId(29), IntersectionId(30)]), // 41
            Path::new(vec![IntersectionId(30), IntersectionId(31)]), // 42
            Path::new(vec![IntersectionId(31), IntersectionId(32)]), // 43
            Path::new(vec![IntersectionId(32), IntersectionId(33)]), // 44
            Path::new(vec![IntersectionId(33), IntersectionId(34)]), // 45
            Path::new(vec![IntersectionId(34), IntersectionId(35)]), // 46
            Path::new(vec![IntersectionId(35), IntersectionId(36)]), // 47
            Path::new(vec![IntersectionId(36), IntersectionId(37)]), // 48
            Path::new(vec![IntersectionId(28), IntersectionId(38)]), // 49
            Path::new(vec![IntersectionId(30), IntersectionId(40)]), // 50
            Path::new(vec![IntersectionId(32), IntersectionId(42)]), // 51
            Path::new(vec![IntersectionId(34), IntersectionId(44)]), // 52
            Path::new(vec![IntersectionId(36), IntersectionId(46)]), // 53
            Path::new(vec![IntersectionId(38), IntersectionId(39)]), // 54
            Path::new(vec![IntersectionId(39), IntersectionId(40)]), // 55
            Path::new(vec![IntersectionId(40), IntersectionId(41)]), // 56
            Path::new(vec![IntersectionId(41), IntersectionId(42)]), // 57
            Path::new(vec![IntersectionId(42), IntersectionId(43)]), // 58
            Path::new(vec![IntersectionId(43), IntersectionId(44)]), // 59
            Path::new(vec![IntersectionId(44), IntersectionId(45)]), // 60
            Path::new(vec![IntersectionId(45), IntersectionId(45)]), // 61
            Path::new(vec![IntersectionId(39), IntersectionId(47)]), // 62
            Path::new(vec![IntersectionId(41), IntersectionId(49)]), // 63
            Path::new(vec![IntersectionId(43), IntersectionId(51)]), // 64
            Path::new(vec![IntersectionId(45), IntersectionId(53)]), // 65
            Path::new(vec![IntersectionId(47), IntersectionId(48)]), // 66
            Path::new(vec![IntersectionId(48), IntersectionId(49)]), // 67
            Path::new(vec![IntersectionId(49), IntersectionId(50)]), // 68
            Path::new(vec![IntersectionId(50), IntersectionId(51)]), // 69
            Path::new(vec![IntersectionId(51), IntersectionId(52)]), // 70
            Path::new(vec![IntersectionId(52), IntersectionId(53)]), // 71
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

struct RubberId(usize);

struct State {
    buildings: Vec<Building>,
    roads: Vec<Road>,
    rubber: RubberId,
}

#[derive(Debug)]
struct IntersectionId(usize);
#[derive(Debug)]
struct Path {
    intersections: Vec<IntersectionId>,
}

impl Path {
    fn new(intersections: Vec<IntersectionId>) -> Self {
        Self {
            intersections,
        }
    }
}

fn main() {
    let tiles = [
        Tile{ dice: 10, kind: TileKind::Ore},
        Tile{ dice: 02, kind: TileKind::Wool},
        Tile{ dice: 09, kind: TileKind::Lumber},
        Tile{ dice: 12, kind: TileKind::Grain},
        Tile{ dice: 06, kind: TileKind::Brick},
        Tile{ dice: 04, kind: TileKind::Wool},
        Tile{ dice: 10, kind: TileKind::Brick},
        Tile{ dice: 09, kind: TileKind::Grain},
        Tile{ dice: 11, kind: TileKind::Lumber},
        Tile{ dice: 00, kind: TileKind::Nothing},
        Tile{ dice: 03, kind: TileKind::Lumber},
        Tile{ dice: 08, kind: TileKind::Ore},
        Tile{ dice: 08, kind: TileKind::Lumber},
        Tile{ dice: 03, kind: TileKind::Ore},
        Tile{ dice: 04, kind: TileKind::Grain},
        Tile{ dice: 05, kind: TileKind::Wool},
        Tile{ dice: 05, kind: TileKind::Brick},
        Tile{ dice: 06, kind: TileKind::Grain},
        Tile{ dice: 11, kind: TileKind::Wool}];
    let board = Board::new(tiles);

    // let mut buildings = vec![
    //     Building{
    //         id: 10,
    //         kind: BuildingKind::Settlement,
    //         player: Player::Red,
    //     },
    //     Building{
    //         id: 13,
    //         kind: BuildingKind::Settlement,
    //         player: Player::Blue,
    //     },
    //     Building{
    //         id: 19,
    //         kind: BuildingKind::Settlement,
    //         player: Player::White,
    //     },
    //     Building{
    //         id: 35,
    //         kind: BuildingKind::Settlement,
    //         player: Player::White,
    //     },
    //     Building{
    //         id: 29,
    //         kind: BuildingKind::Settlement,
    //         player: Player::Red,
    //     },
    //     Building{
    //         id: 40,
    //         kind: BuildingKind::Settlement,
    //         player: Player::Red,
    //     },
    //     Building{
    //         id: 44,
    //         kind: BuildingKind::Settlement,
    //         player: Player::Red,
    //     },
    // ];
    // 
    // let roads = vec![
    //     Road{ id: PathId(13), player: Player::Red },
    //     Road{ id: 15, player: Player::Blue },
    //     Road{ id: 37, player: Player::White },
    //     Road{ id: 41, player: Player::Red },
    //     Road{ id: 56, player: Player::Blue },
    //     Road{ id: 52, player: Player::Blue },
    // ];


    let mut buildings: Vec<Building> = vec![];
    for i in 0..board.intersections.len() {
        buildings.push(Building{
            id: IntersectionId(i),
            kind: BuildingKind::Settlement,
            player: Player::White,
        })
    }

    let mut roads: Vec<Road> = vec![];
    for i in 0..board.paths.len() {
        roads.push(Road { id: PathId(i), player: White })
    }


    let state = State{
        buildings,
        roads,
        rubber: RubberId(9),
    };


    print_board_state(board, state);
}

fn print_board_state(board: Board, state: State) {
    /*
        10    02    09
          \RSR   \OSO
    12    06    04    10
      SW
09    11    NO    03    08
   \RSR              SW
    08    03    04 |R 05
       SB\B  SO\O  SB
        05    06    11

     */
    println!("      SSSSSSSSS SSSS");
    println!("        {:02}     {:02}     {:02}", board.tiles[0].dice, board.tiles[1].dice, board.tiles[2].dice);
    println!("");
    println!("    {:02}     {:02}     {:02}     {:02}", board.tiles[3].dice, board.tiles[4].dice, board.tiles[5].dice, board.tiles[6].dice);
    println!("");
    println!("{:02}     {:02}      {:02}     {:02}     {:02}", board.tiles[7].dice, board.tiles[8].dice, board.tiles[9].dice, board.tiles[10].dice, board.tiles[11].dice);
    println!("");
    println!("    {:02}     {:02}     {:02}     {:02}", board.tiles[12].dice, board.tiles[13].dice, board.tiles[14].dice, board.tiles[15].dice);
    println!("");
    println!("        {:02}     {:02}     {:02}", board.tiles[16].dice, board.tiles[17].dice, board.tiles[18].dice);
}

use std::cmp::{PartialEq};
use std::collections::{HashMap, HashSet};

const TEMPLATE: &str = "
          BB * BB * BB * BB * BB * BB * BB
          *   TTTT  *   TTTT  *   TTTT  *
     BB * BB * BB * BB * BB * BB * BB * BB * BB
     *   TTTT  *   TTTT  *   TTTT  *   TTTT  *
BB * BB * BB * BB * BB * BB * BB * BB * BB * BB * BB
*   TTTT  *   TTTT  *   TTTT  *   TTTT  *   TTTT  *
BB * BB * BB * BB * BB * BB * BB * BB * BB * BB * BB
     *   TTTT  *   TTTT  *   TTTT  *   TTTT  *
     BB * BB * BB * BB * BB * BB * BB * BB * BB
          *   TTTT  *   TTTT  *   TTTT  *
          BB * BB * BB * BB * BB * BB * BB   ";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    Red,
    Blue,
    White,
}

impl From<Player> for char {
    fn from(player: Player) -> Self {
        match player {
            Player::Red => 'R',
            Player::Blue => 'B',
            Player::White => 'W',
        }
    }
}

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


#[derive(Debug, Clone)]
enum  TileKind {
    Grain,
    Wool,
    Brick,
    Lumber,
    Ore,
    Nothing
}


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


#[derive(Debug)]
struct TileId(usize);
#[derive(Debug)]
struct Tile {
    dice: u8,
    kind: TileKind
}

#[derive(Debug, Copy, Clone)]
enum BuildingKind {
    Settlement,
    City,
}

impl BuildingKind {
    fn to_char(&self) -> char {
        match self {
            BuildingKind::Settlement => 'S',
            BuildingKind::City => 'C',
        }
    }
}

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

#[derive(Debug, Copy, Clone)]
struct Building {
    intersection_id: IntersectionId,
    kind: BuildingKind,
    player: Player
}


#[derive(Debug)]
struct Road {
    id: PathId,
    player: Player
}


#[derive(Debug, Eq, PartialEq, Hash)]
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

const PATHS: usize = 72;
const INTERSECTIONS: usize = 54;
const TILES: usize = 19;

struct Board {
    paths: [Path; PATHS],
    intersections: [Intersection; INTERSECTIONS],
    tiles: [Tile; TILES]
}

impl Board {
    fn new(tiles: [Tile; 19]) -> Board {
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

#[derive(Eq, PartialEq)]
struct RobberId(usize);

struct State {
    buildings: Vec<Building>,
    roads: Vec<Road>,
    robber: RobberId,
}

struct Game {
    board: Board,
    state: State,
}


impl Game {
    fn dfs(&self, node: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>, longest: usize) -> (usize, usize) {
        if visited.contains(&node) {
            return (0, 0);
        }
        visited.insert(node);
        let mut max1 = 0;
        let mut max2 = 0;
        for node2 in graph[&node].clone() {
            let (height, _) = self.dfs(node2, graph, visited, longest);
            if max1 < height {
                max2 = max1;
                max1 = height;
            } else if max2 < height {
                max2 = height
            }
        }
        let longest = if max1 + max2 > longest {
            max1 + max2
        } else {
            longest
        };
        (max1 + 1, longest)
    }
    pub(crate) fn longest_road(&self, player: Player) -> usize {
        let graph = self.road_graph(player);
        let (_, road_length) = self.dfs(6, &graph, &mut HashSet::new(), 0);

        road_length
    }

    fn road_graph(&self, player: Player) -> HashMap<usize, Vec<usize>> {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        for road in &self.state.roads {
            if road.player == player {
                let Path(IntersectionId(a), IntersectionId(b)) = self.board.paths[road.id.0];
                graph.entry(a).or_insert_with(Vec::new).push(b);
                graph.entry(b).or_insert_with(Vec::new).push(a);
            }
        }
        graph
    }

    pub(crate) fn possible_building_intersections(&self, player: Player) -> HashSet<IntersectionId> {
        let too_close_intersections= self.too_close_intersections();
        let mut possible_building_intersections: HashSet<IntersectionId> = HashSet::new();
        for road in self.state.roads.iter().filter(|road| road.player == player) {
            let Path(intersection_a, intersection_b) =  self.board.paths[road.id.0];
            if !too_close_intersections.contains(&intersection_a) {
                possible_building_intersections.insert(intersection_a);
            }
            if !too_close_intersections.contains(&intersection_b) {
                possible_building_intersections.insert(intersection_b);
            }
        }
        possible_building_intersections
    }

    pub(crate) fn possible_road_paths(&self, player: Player) -> HashSet<Path> {
        let graph = self.road_graph(player);
        let mut leaf_possible: HashSet<usize> = HashSet::new();
        let mut leaf_already_made: HashSet<usize> = HashSet::new();
        for key in graph.keys() {
            let values = graph.get(key).unwrap();
            if values.len() == 1 {
                leaf_already_made.insert(values[0]);
                leaf_possible.insert(*key);
            }
        }

        let mut possible_road_paths: HashSet<Path> = HashSet::new();
        for path in self.board.paths.iter() {
            let Path(IntersectionId(a), IntersectionId(b)) = path;
            if leaf_possible.contains(a) && !leaf_already_made.contains(b) {
                possible_road_paths.insert(path.clone());
            }

            if leaf_possible.contains(b) && !leaf_already_made.contains(a) {
                possible_road_paths.insert(path.clone());
            }
        }
        possible_road_paths
    }

    pub(crate) fn too_close_intersections(&self) -> HashSet<IntersectionId> {
        let mut too_close_intersections: HashSet<IntersectionId> = HashSet::new();
        for building in self.state.buildings.iter() {
            let Building{
                intersection_id,
                ..
            } = building;

            for path in  &self.board.intersections[intersection_id.0].paths {
                let Path(intersection_a, intersection_b) = self.board.paths[path.0];
                too_close_intersections.insert(intersection_a);
                too_close_intersections.insert(intersection_b);

            }
        }
        too_close_intersections
    }


}

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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct IntersectionId(usize);
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Path(IntersectionId, IntersectionId);


fn main() {
}


#[cfg(test)] // Ensures the test code is compiled only in test mode
mod tests {
    use super::*; // Import the functions from the parent module
    #[test]
    fn test_parse1() {
        let board = get_board();

        let buildings = vec![
            Building {
                intersection_id: IntersectionId(10),
                kind: BuildingKind::Settlement,
                player: Player::Red,
            },
            Building {
                intersection_id: IntersectionId(13),
                kind: BuildingKind::Settlement,
                player: Player::Blue,
            },
            Building {
                intersection_id: IntersectionId(19),
                kind: BuildingKind::Settlement,
                player: Player::White,
            },
            Building {
                intersection_id: IntersectionId(35),
                kind: BuildingKind::Settlement,
                player: Player::White,
            },
            Building {
                intersection_id: IntersectionId(29),
                kind: BuildingKind::Settlement,
                player: Player::Red,
            },
            Building {
                intersection_id: IntersectionId(40),
                kind: BuildingKind::Settlement,
                player: Player::Red,
            },
            Building {
                intersection_id: IntersectionId(44),
                kind: BuildingKind::Settlement,
                player: Player::Red,
            },
        ];

        let roads = vec![
            Road { id: PathId(13), player: Player::Red },
            Road { id: PathId(15), player: Player::Blue },
            Road { id: PathId(37), player: Player::White },
            Road { id: PathId(41), player: Player::Red },
            Road { id: PathId(56), player: Player::Blue },
            Road { id: PathId(52), player: Player::Blue },
        ];

        let state = State {
            buildings,
            roads,
            robber: RobberId(7),
        };

        let game1 = Game{ board, state };

        let string1 = String::from(game1);
        let game2: Game= string1.clone().try_into().unwrap();

        let string2 = String::from(game2);
        println!("{}", string1);
        assert_eq!(string1, string2);

    }

    fn get_board() -> Board {
        let tiles = [
            Tile { dice: 10, kind: TileKind::Ore },
            Tile { dice: 02, kind: TileKind::Wool },
            Tile { dice: 09, kind: TileKind::Lumber },
            Tile { dice: 12, kind: TileKind::Grain },
            Tile { dice: 06, kind: TileKind::Brick },
            Tile { dice: 04, kind: TileKind::Wool },
            Tile { dice: 10, kind: TileKind::Brick },
            Tile { dice: 09, kind: TileKind::Grain },
            Tile { dice: 11, kind: TileKind::Lumber },
            Tile { dice: 00, kind: TileKind::Nothing },
            Tile { dice: 03, kind: TileKind::Lumber },
            Tile { dice: 08, kind: TileKind::Ore },
            Tile { dice: 08, kind: TileKind::Lumber },
            Tile { dice: 03, kind: TileKind::Ore },
            Tile { dice: 04, kind: TileKind::Grain },
            Tile { dice: 05, kind: TileKind::Wool },
            Tile { dice: 05, kind: TileKind::Brick },
            Tile { dice: 06, kind: TileKind::Grain },
            Tile { dice: 11, kind: TileKind::Wool }];
        let board = Board::new(tiles);
        board
    }

    #[test]
    fn test_parse2() {
        let mut buildings: Vec<Building> = vec![];
        for i in 0..INTERSECTIONS {
            buildings.push(Building {
                intersection_id: IntersectionId(i),
                kind: BuildingKind::Settlement,
                player: Player::White,
            })
        }

        let mut roads: Vec<Road> = vec![];
        for i in 0..PATHS {
            roads.push(Road { id: PathId(i), player: Player::White })
        }


        let state = State {
            buildings,
            roads,
            robber: RobberId(8),
        };

        let board = get_board();
        let game1 = Game { board, state };

        let string1 = String::from(game1);
        let game2: Game = string1.clone().try_into().unwrap();

        let string2 = String::from(game2);
        println!("{}", &string1);

        assert_eq!(string1, string2);
    }
    
    #[test]
    fn test_longest_road() {
        let game: Game = "
          oo . oo . oo . oo . oo W oo W oo
          .   10O   .   02W   .   09L   W
     oo . oo . oo . RS R oo . oo B BS W oo . oo
     .   12G   .   06B   .   04W   W   10B   .
oo . oo . oo W WS . oo . oo . oo . oo W oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   W   08O   .
oo . oo . RS R oo . oo . oo . oo . oo . WS . oo . oo
     .   08L   .   03O   .   04G   B   05W   .
     oo . oo . RS B oo . oo . oo . RS . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo".to_string().try_into().unwrap();
        assert_eq!(game.longest_road(Player::White), 7);
    }

    #[test]
    fn test_possible_building_intersections() {
        let game: Game = "
          oo . oo . oo . oo . oo W oo W oo
          .   10O   .   02W   .   09L   W
     oo . oo . oo . RS R oo . oo B BS W oo . oo
     .   12G   .   06B   .   04W   W   10B   .
oo . oo . oo W WS . oo . oo . oo . oo W oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   W   08O   .
oo . oo . RS R oo . oo . oo . oo . oo . WS W oo . oo
     .   08L   .   03O   .   04G   B   05W   W
     oo . oo . RS B oo . oo . oo . RS . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo".to_string().try_into().unwrap();
        let s: HashSet<IntersectionId> = vec![IntersectionId(46), IntersectionId(6), IntersectionId(4), IntersectionId(5)].into_iter().collect();
         assert_eq!(game.possible_building_intersections(Player::White), s);
    }


    #[test]
    fn test_possible_possible_road_paths() {
        let game: Game = "
          oo . oo . oo . oo . oo W oo W oo
          .   10O   .   02W   .   09L   W
     oo . oo . oo . RS R oo . oo B BS W oo . oo
     .   12G   .   06B   .   04W   W   10B   .
oo . oo . oo W WS . oo . oo . oo . oo W oo . oo . oo
.   09G!  .   11L   .   00N   .   03L   W   08O   .
oo . oo . RS R oo . oo . oo . oo . oo . WS W oo . oo
     .   08L   .   03O   .   04G   B   05W   W
     oo . oo . RS B oo . oo . oo . RS . oo . oo
          .   05B   .   06G   .   11W   .
          oo . oo . oo . oo . oo . oo . oo".to_string().try_into().unwrap();
        let s: HashSet<Path> = vec![Path(IntersectionId(19), IntersectionId(20)), Path(IntersectionId(18), IntersectionId(29)), Path(IntersectionId(3), IntersectionId(4)), Path(IntersectionId(17), IntersectionId(18)), Path(IntersectionId(4), IntersectionId(12)), Path(IntersectionId(9), IntersectionId(19))].into_iter().collect();
        assert_eq!(s, game.possible_road_paths(Player::White));

    }
}
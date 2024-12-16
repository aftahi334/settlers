use crate::game::{Building, IntersectionId, Path, Player};
use std::collections::{HashMap, HashSet};
use crate::game::Game;
impl Game {
    /// Calculates the longest road using Depth-First Search (DFS).
    ///
    /// This method assumes that the graph is acyclic, making the problem analogous to finding the diameter
    /// of an N-ary tree. If the graph contains cycles, the problem becomes NP-complete, and this method 
    /// will not provide a valid result.
    ///
    /// # Methodology
    /// The function traverses the graph recursively to calculate the longest path by identifying
    /// the two longest branches extending from each node. These branches are used to compute the longest
    /// road as the sum of their lengths.
    ///
    /// # Arguments
    /// - `node`: The current node in the graph where the DFS starts.
    /// - `graph`: A `HashMap` representing the adjacency list of the graph. The keys are node indices,
    ///   and the values are vectors of connected node indices.
    /// - `visited`: A mutable reference to a `HashSet` that keeps track of visited nodes to prevent cycles
    ///   and redundant computations.
    /// - `longest`: The current longest path discovered during the DFS traversal.
    ///
    /// # Returns
    /// A tuple `(usize, usize)`:
    /// - The first element represents the height of the current DFS branch (i.e., the maximum depth from the `node`).
    /// - The second element represents the updated longest path across the graph.
    ///
    /// # Examples
    /// ```rust
    /// use std::collections::{HashMap, HashSet};
    ///
    /// // Example graph (acyclic tree structure)
    /// let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    /// graph.insert(0, vec![1, 2]);
    /// graph.insert(1, vec![0, 3, 4]);
    /// graph.insert(2, vec![0]);
    /// graph.insert(3, vec![1]);
    /// graph.insert(4, vec![1]);
    ///
    /// let mut visited = HashSet::new();
    /// let initial_longest = 0;
    ///
    /// // Assume `dfs` is implemented in the context of a struct with `self`
    /// let (branch_height, longest_path) = some_struct.dfs(0, &graph, &mut visited, initial_longest);
    ///
    /// println!("Branch height: {}", branch_height);
    /// println!("Longest path: {}", longest_path);
    /// ```
    ///
    /// # Complexity
    /// - **Time Complexity**: `O(V + E)`, where `V` is the number of vertices (nodes) and `E` is the number of edges in the graph.
    /// - **Space Complexity**: `O(V)` due to the `visited` set and recursion stack.
    ///
    /// # Assumptions
    /// - The graph is connected and acyclic.
    /// - Nodes are represented as `usize` indices.
    ///
    /// # Notes
    /// - If the graph contains cycles, the method will not compute the correct result.
    /// - To extend this method for cyclic graphs, additional logic (e.g., cycle detection or pruning) is required.
    ///
    /// # Limitations
    /// - The method does not verify if the graph is acyclic; it relies on the caller to provide a valid input.
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

    /// Calculates the longest road for a given player.
    ///
    /// Uses depth-first search (DFS) to find the longest connected path of roads owned by the player.
    ///
    /// # Arguments
    /// - `player`: The player whose roads are being evaluated.
    ///
    /// # Returns
    /// The length of the longest road owned by the player.
    ///
    /// Example usage:
    /// ```rust
    /// let longest_road = game.longest_road(Player::Red);
    /// println!("Longest road: {}", longest_road);
    /// ```
    pub(crate) fn longest_road(&self, player: Player) -> usize {
        let graph = self.road_graph(player);
        let (_, road_length) = self.dfs(6, &graph, &mut HashSet::new(), 0);

        road_length
    }

    /// Constructs a graph of roads owned by the given player.
    ///
    /// The graph is represented as a mapping of intersection IDs to lists of connected intersections.
    ///
    /// # Arguments
    /// - `player`: The player whose roads are being considered.
    ///
    /// # Returns
    /// A `HashMap` representing the graph of roads owned by the player.
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

    /// Identifies intersections where the given player can build a new building.
    ///
    /// Ensures that buildings are not placed too close to each other.
    ///
    /// # Arguments
    /// - `player`: The player attempting to build.
    ///
    /// # Returns
    /// A `HashSet` of `IntersectionId`s where the player can build.
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


    /// Determines all possible road paths a player can build on the board.
    ///
    /// This function identifies potential road locations based on the current state of the board and the roads
    /// already owned by the player. It ensures that new roads are connected to existing roads owned by the player
    /// and do not conflict with existing roads owned by other players.
    ///
    /// # Arguments
    /// - `player`: The `Player` whose potential road paths are being evaluated.
    ///
    /// # Returns
    /// A `HashSet` containing all possible `Path`s where the player can build new roads.
    ///
    /// # Methodology
    /// 1. Builds a road graph of intersections connected by roads owned by the player.
    /// 2. Identifies "leaf" intersections—intersections with exactly one connected road—where new roads can branch out.
    /// 3. Collects all valid paths where a new road can connect to these leaf intersections.
    ///
    /// # Example
    /// ```rust
    /// let possible_paths = game.possible_road_paths(Player::Red);
    /// for path in possible_paths {
    ///     println!("Possible path: {:?} -> {:?}", path.0, path.1);
    /// }
    /// ```
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

    /// Constructs a set of intersections that are too close to existing buildings.
    ///
    /// Buildings cannot be placed adjacent to other buildings.
    ///
    /// # Returns
    /// A `HashSet` of `IntersectionId`s that are too close to existing buildings.
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



#[cfg(test)] // Ensures the test code is compiled only in test mode
mod tests {
    use super::*; // Import the functions from the parent module
    use std::collections::HashSet;

    
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
}
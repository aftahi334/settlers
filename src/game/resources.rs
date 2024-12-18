#[cfg(test)] // Ensures the test code is compiled only in test mode
mod tests {
    use crate::game::{Game, PlayerResourceCount, ResourceCount};
    use std::convert::TryInto; // Import the functions from the parent module

    #[test]
    fn test_parse_resources() {
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
          oo . oo . oo . oo . oo . oo . oo
   G  W  B  L  O
W  1  2  3  4  5  
R  6  7  8  9  10 
B  11 12 13 14 15"
            .to_string()
            .try_into()
            .unwrap();
        let s = PlayerResourceCount {
            red: ResourceCount {
                grain: 6,
                wool: 7,
                brick: 8,
                lumber: 9,
                ore: 10,
            },
            blue: ResourceCount {
                grain: 11,
                wool: 12,
                brick: 13,
                lumber: 14,
                ore: 15,
            },
            white: ResourceCount {
                grain: 1,
                wool: 2,
                brick: 3,
                lumber: 4,
                ore: 5,
            },
        };
        assert_eq!(s, game.state.resources);
    }
}

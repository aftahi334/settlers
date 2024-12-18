# Settlers of Catan AI WebAssembly

## Overview
This project implements an AI bot for the board game **Settlers of Catan** using Rust and WebAssembly (Wasm). The AI bot evaluates the current game state and returns the optimal move based on the rules of the game. It is designed to run on the edge, enabling low-latency decision-making for multiplayer games hosted on distributed platforms.

## Features
- **AI-Powered Decision Making**: Uses algorithms like Minimax (planned) to compute the best possible move for the AI.
- **WebAssembly Support**: Highly portable Wasm module for execution in edge environments (e.g., Cloudflare Workers, Fastly Compute@Edge).
- **Game State Serialization**: Encodes and decodes the game state for seamless communication with the Wasm module.
- **Edge-Optimized Execution**: Designed to reduce latency and enhance scalability by running AI logic close to users.

## Project Structure
```
.
├── src
│   ├── main.rs           # Main Rust source file
│   ├── lib.rs            # Shared logic for board state and AI computation
├── Cargo.toml            # Rust dependencies and configuration
├── README.md             # Project documentation
├── tests                 # Unit and integration tests
├── wasm                  # Compiled WebAssembly binaries
└── examples              # Example game states and AI usage
```

## Requirements
- Rust programming language (1.70 or newer)
- `wasm32-unknown-unknown` target for WebAssembly compilation
- Edge computing platform with WebAssembly support (e.g., Cloudflare Workers)

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-username/settlers-of-catan-ai.git
   cd settlers-of-catan-ai
   ```

2. **Install Rust**:
   Follow the instructions on [Rust's official website](https://www.rust-lang.org/tools/install) to install Rust.

3. **Add Wasm Target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

4. **Build the Project**:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

5. **Run Tests**:
   ```bash
   cargo test
   ```

## Usage

### Input Format
The AI module expects the game state as a JSON string. Example:
```json
{
  "tiles": [
    {"dice": 10, "kind": "ore"},
    {"dice": 2, "kind": "wool"}
  ],
  "buildings": [
    {"id": 10, "kind": "Settlement", "player": "red"},
    {"id": 13, "kind": "City", "player": "blue"}
  ],
  "roads": [
    {"id": 13, "player": "red"},
    {"id": 15, "player": "blue"}
  ],
  "robber": 7
}
```

### Output Format
The AI module returns the next move as a JSON string. Example:
```json
{
  "action": "build_settlement",
  "position": 10
}
```

### Example Usage
```rust
use serde_json::json;

let game_state = json!({
  "tiles": [...],
  "buildings": [...],
  "roads": [...],
  "robber": 7
});

let next_move = ai_next_move(&game_state.to_string());
println!("AI suggests move: {}", next_move);
```

## Deployment

### Cloudflare Workers Example
1. **Upload Wasm Module**:
   Deploy the compiled `.wasm` file to Cloudflare.

2. **Create Worker Script**:
   ```javascript
   import wasm from './ai_module.wasm';

   async function handleRequest(request) {
       const gameState = await request.json();
       const nextMove = wasm.ai_next_move(JSON.stringify(gameState));
       return new Response(nextMove, { headers: { 'Content-Type': 'application/json' } });
   }

   addEventListener('fetch', event => {
       event.respondWith(handleRequest(event.request));
   });
   ```

3. **Test the Worker**:
   ```bash
   curl -X POST https://your-worker-url.workers.dev -d '{...}'
   ```

### Fastly Compute 

1. Fastly Compute 
   ```bash
    fastly compute serve --verbose
   ```

## Roadmap
- Implement Minimax or Monte Carlo Tree Search for AI decision-making.
- Add support for multiplayer scenarios.
- Optimize serialization/deserialization with binary formats.
- Extend functionality for full Settlers of Catan rules.

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request.

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments
Special thanks to the developers of Settlers of Catan for the inspiration and to the Rust and WebAssembly communities for their excellent tools and support.

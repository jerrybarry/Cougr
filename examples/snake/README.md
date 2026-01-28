# Snake On-Chain Game

A fully functional Snake game implemented as a Soroban smart contract using the `cougr-core` ECS (Entity-Component-System) framework. This example demonstrates how to build on-chain game logic on the Stellar blockchain.

## Overview

This implementation follows classic Snake game rules:
- The snake moves in a direction controlled by the player
- Eating food makes the snake grow and increases the score
- Hitting walls or the snake's own body ends the game

### Architecture

The game uses an Entity-Component-System (ECS) pattern:

- **Entities**: Snake head, snake body segments, and food
- **Components**: Position, Direction, SnakeHead, SnakeSegment, Food
- **Systems**: Movement, collision detection, growth, food spawning

## Prerequisites

Before you begin, ensure you have the following installed:

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. **WebAssembly target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Stellar CLI** (for contract deployment)
   ```bash
   cargo install --locked stellar-cli --features opt
   ```

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/salazarsebas/Cougr.git
cd Cougr/examples/snake
```

### 2. Build the Contract

```bash
# Standard Rust build
cargo build

# Build the Soroban WASM contract
stellar contract build
```

### 3. Run Tests

```bash
cargo test
```

Expected output:
```
running 23 tests
test components::tests::test_direction_opposite ... ok
test components::tests::test_direction_serialization ... ok
...
test result: ok. 23 passed; 0 failed; 0 ignored
```

## Contract Functions

### Game Initialization

| Function | Description |
|----------|-------------|
| `init_game()` | Initialize a new game with default 10x10 grid |
| `init_game_with_size(grid_size: i32)` | Initialize with custom grid size |

### Game Control

| Function | Description |
|----------|-------------|
| `change_direction(direction: u32)` | Change snake direction (0=Up, 1=Down, 2=Left, 3=Right) |
| `update_tick()` | Advance the game by one tick |

### Game State Queries

| Function | Returns | Description |
|----------|---------|-------------|
| `get_score()` | `u32` | Current player score |
| `check_game_over()` | `bool` | Whether the game has ended |
| `get_head_pos()` | `(i32, i32)` | Snake head position (x, y) |
| `get_snake_length()` | `u32` | Total snake length |
| `get_food_pos()` | `(i32, i32)` | Food position (x, y) |
| `get_snake_positions()` | `Vec<(i32, i32)>` | All snake segment positions |
| `get_grid_size()` | `i32` | Game grid size |

## Deployment to Testnet

### 1. Create a Test Account

Fund a test account using the Stellar Friendbot:
```bash
stellar keys generate --global alice --network testnet
stellar keys address alice
# Visit https://friendbot.stellar.org/?addr=<YOUR_ADDRESS>
```

### 2. Deploy the Contract

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/snake.wasm \
  --source alice \
  --network testnet
```

Save the returned contract ID for invoking functions.

### 3. Initialize and Play

```bash
# Initialize the game
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- init_game

# Make a move (change direction to Up)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- change_direction --direction 0

# Advance the game
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- update_tick

# Check the score
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- get_score
```

## Project Structure

```
examples/snake/
├── Cargo.toml           # Package configuration with cougr-core dependency
├── README.md            # This file
└── src/
    ├── lib.rs           # Main contract with Soroban entry points
    ├── components.rs    # Game components (Position, Direction, etc.)
    ├── systems.rs       # Game systems (movement, collision, etc.)
    └── simple_world.rs  # Simplified ECS world for Soroban
```

## How cougr-core Simplifies Development

The `cougr-core` package provides:

1. **Serialization-ready component patterns** - Components are designed for efficient on-chain storage
2. **Entity management** - Optimized entity spawning and despawning for Soroban's constraints
3. **Consistent architecture** - ECS pattern makes game logic modular and testable

### Example: Creating a Component

```rust
use crate::components::ComponentTrait;
use soroban_sdk::{Bytes, Env};

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl ComponentTrait for Position {
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_array(env, &self.x.to_be_bytes()));
        bytes.append(&Bytes::from_array(env, &self.y.to_be_bytes()));
        bytes
    }

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 { return None; }
        let x = i32::from_be_bytes([
            data.get(0)?, data.get(1)?, data.get(2)?, data.get(3)?
        ]);
        let y = i32::from_be_bytes([
            data.get(4)?, data.get(5)?, data.get(6)?, data.get(7)?
        ]);
        Some(Self { x, y })
    }
}
```

## Troubleshooting

### Common Issues

1. **Rust version errors**
   ```bash
   rustup update
   rustup default stable
   ```

2. **WASM target not installed**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Stellar CLI not found**
   ```bash
   cargo install --locked stellar-cli --features opt
   ```

4. **Dependency conflicts**
   ```bash
   cargo update
   cargo clean
   cargo build
   ```

## References

- [Soroban Smart Contracts Documentation](https://developers.stellar.org/docs/build/smart-contracts)
- [Stellar CLI Documentation](https://developers.stellar.org/docs/tools/cli)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cougr Repository](https://github.com/salazarsebas/Cougr)

## License

This example is part of the Cougr project and is available under the same license as the main repository.

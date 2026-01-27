# Space Invaders - On-Chain Game Example

A fully functional Space Invaders game implemented as a Soroban smart contract using the `cougr-core` ECS framework.

## Overview

This example demonstrates how to build on-chain game logic on the Stellar blockchain. The game focuses exclusively on smart contract logic (no graphical interface) and includes:

- Player ship management with left/right movement
- Invader grid with wave-based movement  
- Bullet mechanics for both player and enemies
- Collision detection and score/lives tracking
- Game over conditions

## Quick Start

### Prerequisites

- Rust 1.70.0+
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli)
- wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

### Build

```bash
# Standard Rust build
cargo build

# Build WASM for Soroban
stellar contract build
```

### Test

```bash
cargo test
```

## Contract Functions

| Function | Description | Returns |
|----------|-------------|---------|
| `init_game()` | Initialize a new game | - |
| `move_ship(direction: i32)` | Move ship (-1=left, 1=right) | New X position |
| `shoot()` | Fire a player bullet | `true` if fired |
| `update_tick()` | Advance game by one tick | `true` if still running |
| `get_score()` | Get current score | Score value |
| `get_lives()` | Get remaining lives | Lives count |
| `get_ship_position()` | Get ship X position | X coordinate |
| `check_game_over()` | Check if game ended | `true` if over |
| `get_active_invaders()` | Get remaining invaders | Count |

## Game Mechanics

### Invaders
- 4 rows × 8 columns = 32 invaders
- Three types: Squid (30pts), Crab (20pts), Octopus (10pts)
- Move horizontally, descend when reaching screen edge
- Game over if invaders reach player's row

### Player
- 3 lives initially
- Can move left/right within bounds
- Shoots bullets upward with cooldown
- Loses life when hit by enemy bullet

### Scoring
- Squid (top row): 30 points
- Crab (middle rows): 20 points
- Octopus (bottom row): 10 points

## Using Cougr-Core

This example showcases `cougr-core` integration:

```rust
use cougr_core::prelude::*;

// The game uses Soroban storage for persistence
// Future examples may use cougr-core's ECS World for entity management
```

## Deploy to Testnet

```bash
# Build WASM
stellar contract build

# Deploy (replace with your secret key)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/space_invaders.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet

# Initialize game
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <YOUR_SECRET_KEY> \
  --network testnet \
  -- init_game

# Play!
stellar contract invoke --id <CONTRACT_ID> -- move_ship --direction 1
stellar contract invoke --id <CONTRACT_ID> -- shoot
stellar contract invoke --id <CONTRACT_ID> -- update_tick
stellar contract invoke --id <CONTRACT_ID> -- get_score
```

## License

MIT OR Apache-2.0

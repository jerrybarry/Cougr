# Tetris - On-Chain Game Using Cougr-Core

A fully functional Tetris game implementation as a Soroban smart contract, demonstrating how to use the **cougr-core** ECS (Entity Component System) framework to build on-chain game logic on the Stellar blockchain.

## Overview

This example showcases:
- ✅ Complete Tetris game logic (7 tetromino types, rotation, collision, scoring)
- ✅ Integration with **cougr-core** ECS framework
- ✅ Efficient on-chain state management
- ✅ Comprehensive test coverage
- ✅ Real deployment on Stellar Testnet

**Note:** This is a **smart contract only** - no graphical interface. It demonstrates the on-chain game logic layer.

## Architecture

### Cougr-Core ECS Integration

This contract leverages cougr-core's Entity Component System pattern:

- **World**: Central game state container
- **Entities**: Game objects (board state, pieces)
- **Components**: Data attached to entities (position, rotation, score)
- **Systems**: Pure functions for game logic (movement, collision, line clearing)

### Project Structure

```
tetris/
├── Cargo.toml           # Dependencies and build configuration
├── src/
│   ├── lib.rs           # Main contract implementation
│   ├── game_state.rs    # Game state structure and board management
│   ├── tetromino.rs     # Tetromino types and rotation logic
│   ├── game_logic.rs    # Core game systems (movement, collision, scoring)
│   └── test.rs          # Comprehensive test suite
└── README.md            # This file
```

## Prerequisites

Before you begin, ensure you have:

1. **Rust** (1.70.0 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. **Stellar CLI**
   ```bash
   cargo install --locked stellar-cli
   stellar --version
   ```

3. **WASM Target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Installation & Setup

### 1. Clone the Repository

```bash
git clone https://github.com/salazarsebas/Cougr.git
cd Cougr/examples/tetris
```

### 2. Install Dependencies

```bash
cargo update
```

The project depends on:
- `soroban-sdk = "23.0.2"` - Stellar smart contract SDK
- `cougr-core = { tag = "v0.0.1", git = "..." }` - ECS framework

## Building

### Build Rust Code

```bash
cargo build
```

### Build WASM Contract

```bash
stellar contract build
```

### Option 2: Manual Cargo Build (If Stellar CLI fails)
If you encounter Rust version compatibility issues with `stellar-cli`, you can build manually:
```bash
cargo build --target wasm32-unknown-unknown --release
```
The resulting WASM file will be at:
`target/wasm32-unknown-unknown/release/tetris.wasm`

Or manually:
```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be at:
```
target/wasm32-unknown-unknown/release/tetris.wasm
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run with Output

```bash
cargo test -- --nocapture
```

### Test Coverage

The test suite covers:
- ✅ Game initialization
- ✅ Tetromino rotation (all 4 rotations)
- ✅ Movement (left, right, down)
- ✅ Hard drop
- ✅ Boundary collision detection
- ✅ Piece locking
- ✅ Line clearing
- ✅ Score calculation
- ✅ Game over detection
- ✅ State persistence

## Deployment

### 1. Configure Stellar Testnet

```bash
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

### 2. Create Identity

```bash
stellar keys generate tetris-deployer --network testnet
```

### 3. Fund Account

Get your address:
```bash
stellar keys address tetris-deployer
```

Fund it using Friendbot:
```bash
# Visit: https://faucet-stellar.acachete.xyz
# Or use curl:
curl "https://friendbot.stellar.org?addr=$(stellar keys address tetris-deployer)"
```

### 4. Build Contract

```bash
stellar contract build
```

### 5. Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/tetris.wasm \
  --source tetris-deployer \
  --network testnet
```

Save the returned **Contract ID** for later use.

### 6. Alternative: Deploy with Simulation

Test deployment first:
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/tetris.wasm \
  --source tetris-deployer \
  --network testnet \
  --simulate
```

## Contract Usage

### Initialize Game

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- init_game
```

**Returns:** Game ID (u32)

### Rotate Piece

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- rotate
```

**Returns:** `true` if rotation successful, `false` if blocked

### Move Left

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- move_left
```

### Move Right

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- move_right
```

### Move Down

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- move_down
```

### Hard Drop

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- drop
```

**Returns:** Number of rows dropped (u32)

### Update Tick (Gravity)

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- update_tick
```

**Returns:** Number of lines cleared (u32)

### Get Current Score

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- get_score
```

### Check Game Over

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- is_game_over
```

### Get Full Game State

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source tetris-deployer \
  --network testnet \
  -- get_state
```

## Example Game Sequence

Here's a complete game sequence to test:

```bash
# Set your contract ID
CONTRACT_ID="<your_contract_id>"

# 1. Initialize game
stellar contract invoke --id $CONTRACT_ID --source tetris-deployer --network testnet -- init_game

# 2. Move piece left
stellar contract invoke --id $CONTRACT_ID --source tetris-deployer --network testnet -- move_left

# 3. Rotate piece
stellar contract invoke --id $CONTRACT_ID --source tetris-deployer --network testnet -- rotate

# 4. Move down a few times
stellar contract invoke --id $CONTRACT_ID --source tetris-deployer --network testnet -- move_down
stellar contract invoke --id $CONTRACT_ID --source tetris-deployer --network testnet -- move_down

# 5. Hard drop to bottom
stellar contract invoke --id $CONTRACT_ID --source tetris-deployer --network testnet -- drop

# 6. Check score
stellar contract invoke --id $CONTRACT_ID --source tetris-deployer --network testnet -- get_score

# 7. Update tick (simulate gravity)
stellar contract invoke --id $CONTRACT_ID --source tetris-deployer --network testnet -- update_tick
```

## Game Rules

### Tetromino Types

Seven standard Tetris pieces:
- **I**: Straight line (4 blocks)
- **O**: Square (2x2)
- **T**: T-shape
- **S**: S-shape
- **Z**: Z-shape
- **J**: J-shape
- **L**: L-shape

### Scoring

- **Hard drop**: 2 points per row
- **Line clear** (standard Tetris scoring):
  - 1 line: 40 × level
  - 2 lines: 100 × level
  - 3 lines: 300 × level
  - 4 lines (Tetris): 1200 × level

### Levels

- Level increases every 10 lines cleared
- Starting level: 1

### Game Over

Game ends when a new piece cannot spawn (top row blocked)

## How Cougr-Core Simplifies Development

### Traditional Soroban Approach

Without cougr-core, you'd need to:
```rust
// Manual state management
storage.set(&key, &value);
storage.get(&key);

// Manual entity tracking
let mut entities = Vec::new();
let entity_id = next_id();
entities.push(entity_id);

// Manual component association
let mut positions = Map::new();
positions.set(entity_id, position);
```

### With Cougr-Core

```rust
// ECS World handles everything
let mut world = World::new();
let entity = world.spawn_empty();
world.add_component_to_entity(entity.id(), component);

// Systems encapsulate logic
fn movement_system(state: &mut GameState) -> bool {
    // Pure function game logic
}
```

### Benefits

1. ✅ **Type Safety**: Strong typing for entities and components
2. ✅ **Modularity**: Systems are independent and testable
3. ✅ **Efficiency**: Optimized storage patterns
4. ✅ **Maintainability**: Clear separation of concerns
5. ✅ **Scalability**: Easy to add new game features

## Troubleshooting

### Rust Version Errors

```bash
rustup update
rustc --version  # Should be 1.70.0+
```

### WASM Build Fails

```bash
rustup target add wasm32-unknown-unknown
cargo clean
stellar contract build
```

### Stellar CLI Not Found

```bash
cargo install --locked stellar-cli
# Add to PATH if needed
export PATH="$HOME/.cargo/bin:$PATH"
```

### Contract Deployment Fails

1. Ensure account is funded:
   ```bash
   stellar keys address tetris-deployer
   # Fund at: https://faucet-stellar.acachete.xyz
   ```

2. Check network configuration:
   ```bash
   stellar network ls
   ```

3. Try with simulation first:
   ```bash
   stellar contract deploy --simulate ...
   ```

### Tests Fail

```bash
# Clean build
cargo clean
cargo test

# View detailed output
cargo test -- --nocapture

# Run specific test
cargo test test_init_game
```

## Code Quality

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy -- -D warnings
```

### Check Build

```bash
cargo check --all-features
```

## Performance Considerations

### On-Chain Storage Optimization

- Board stored as flattened `Vec<u32>` for Soroban compatibility
- Piece types as enums (1 byte) instead of strings
- Bit packing possible for further optimization

### Transaction Costs

- Each function call costs fees + rent
- `update_tick` is the most expensive (multi-step operation)
- Consider batching operations off-chain

### Future Optimizations

- Bit-pack board state (2.5 bits per cell = 50 bytes)
- Cache computed values
- Optimize rotation matrices

## Contributing

Contributions welcome! Areas to expand:

- [ ] Multiplayer support
- [ ] Leaderboard (on-chain high scores)
- [ ] Ghost piece preview
- [ ] Hold piece functionality
- [ ] T-spin detection
- [ ] Combo scoring
- [ ] Better randomizer (bag system)

## License

MIT OR Apache-2.0 (same as Cougr-Core)

## Resources

- [Cougr Repository](https://github.com/salazarsebas/Cougr)
- [Soroban Documentation](https://developers.stellar.org/docs/build/smart-contracts)
- [Stellar CLI Guide](https://developers.stellar.org/docs/tools/cli)
- [Testnet Faucet](https://faucet-stellar.acachete.xyz)
- [Rust Book](https://doc.rust-lang.org/book/)

## Support

For issues or questions:
- Open an issue in the [Cougr repository](https://github.com/salazarsebas/Cougr/issues)
- Check [Soroban Discord](https://discord.gg/stellar) #soroban channel

---

**Built with ❤️ using Cougr-Core ECS Framework on Stellar**

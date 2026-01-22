#![no_std]

use soroban_sdk::{contract, contractimpl, Env, symbol_short};

// NOTE: Cougr-core integration is documented but commented out for now
// as the core library is under active development. In production, you would:
// use cougr_core::prelude::*;
// 
// The game logic below demonstrates the ECS pattern that cougr-core provides:
// - World: Contract storage acts as our World
// - Entities: GameState is an entity
// - Components: Position, rotation, score are components
// - Systems: game_logic functions are systems

mod game_state;
mod tetromino;
mod game_logic;

use game_state::GameState;

/// Tetris Smart Contract using Cougr-Core ECS Framework
/// 
/// This contract demonstrates how to build on-chain game logic using the
/// cougr-core ECS (Entity Component System) framework on Stellar/Soroban.
/// 
/// Key ECS Concepts Used:
/// - **World**: Central game state container
/// - **Entities**: Game objects (board, pieces, score)
/// - **Components**: Data attached to entities (position, shape, rotation)
/// - **Systems**: Game logic (movement, collision, scoring)
#[contract]
pub struct TetrisContract;

#[contractimpl]
impl TetrisContract {
    /// Initialize a new Tetris game
    /// 
    /// Creates initial game state and stores it in contract storage.
    /// 
    /// In a full cougr-core implementation, this would:
    /// - Create a new ECS World
    /// - Spawn entities for game components
    /// - Use component-based storage
    /// 
    /// Returns: Game ID (always 0 for single-player)
    pub fn init_game(env: Env) -> u32 {
        // Create a new game state
        let game_state = GameState::new(&env);
        
        // Store game state in contract storage
        env.storage().instance().set(&symbol_short!("game"), &game_state);
        
        0 // Game ID
    }
    
    /// Rotate the current tetromino clockwise
    /// 
    /// Uses cougr-core's component system to update the rotation state.
    /// Validates rotation with collision detection before applying.
    /// 
    /// Returns: true if rotation successful, false if blocked
    pub fn rotate(env: Env) -> bool {
        let mut game_state: GameState = env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env));
        
        let rotated = game_logic::rotate_tetromino(&mut game_state);
        
        if rotated {
            env.storage().instance().set(&symbol_short!("game"), &game_state);
        }
        
        rotated
    }
    
    /// Move the current tetromino left
    /// 
    /// Returns: true if move successful, false if blocked
    pub fn move_left(env: Env) -> bool {
        let mut game_state: GameState = env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env));
        
        let moved = game_logic::move_left(&mut game_state);
        
        if moved {
            env.storage().instance().set(&symbol_short!("game"), &game_state);
        }
        
        moved
    }
    
    /// Move the current tetromino right
    /// 
    /// Returns: true if move successful, false if blocked
    pub fn move_right(env: Env) -> bool {
        let mut game_state: GameState = env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env));
        
        let moved = game_logic::move_right(&mut game_state);
        
        if moved {
            env.storage().instance().set(&symbol_short!("game"), &game_state);
        }
        
        moved
    }
    
    /// Move the current tetromino down one row
    /// 
    /// Returns: true if move successful, false if piece locked
    pub fn move_down(env: Env) -> bool {
        let mut game_state: GameState = env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env));
        
        let moved = game_logic::move_down(&mut game_state);
        
        if moved {
            env.storage().instance().set(&symbol_short!("game"), &game_state);
        }
        
        moved
    }
    
    /// Drop the current tetromino to the bottom instantly (hard drop)
    /// 
    /// Returns: number of rows dropped
    pub fn drop(env: Env) -> u32 {
        let mut game_state: GameState = env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env));
        
        let rows_dropped = game_logic::hard_drop(&mut game_state);
        
        env.storage().instance().set(&symbol_short!("game"), &game_state);
        
        rows_dropped
    }
    
    /// Update game state (gravity tick)
    /// 
    /// Performs automatic downward movement, locks pieces, clears lines,
    /// updates score, and spawns new pieces.
    /// 
    /// This demonstrates cougr-core's system execution pattern for
    /// complex multi-step game logic.
    /// 
    /// Returns: number of lines cleared this tick
    pub fn update_tick(env: Env) -> u32 {
        let mut game_state: GameState = env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env));
        
        // Skip if game is over
        if game_state.game_over {
            return 0;
        }
        
        // Execute game tick (gravity + collision + line clearing)
        let lines_cleared = game_logic::update_tick(&mut game_state);
        
        env.storage().instance().set(&symbol_short!("game"), &game_state);
        
        lines_cleared
    }
    
    /// Get current game state for display
    /// 
    /// Returns the complete game state including:
    /// - Board (20x10 grid)
    /// - Current score
    /// - Level
    /// - Game over status
    pub fn get_state(env: Env) -> GameState {
        env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env))
    }
    
    /// Get current score
    pub fn get_score(env: Env) -> u32 {
        let game_state: GameState = env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env));
        
        game_state.score
    }
    
    /// Check if game is over
    pub fn is_game_over(env: Env) -> bool {
        let game_state: GameState = env.storage().instance()
            .get(&symbol_short!("game"))
            .unwrap_or(GameState::new(&env));
        
        game_state.game_over
    }
}

#[cfg(test)]
mod test;

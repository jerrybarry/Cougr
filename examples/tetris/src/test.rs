#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_init_game() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    let game_id = client.init_game();
    assert_eq!(game_id, 0);
}

#[test]
fn test_rotate() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Should be able to rotate
    let result = client.rotate();
    assert!(result);
}

#[test]
fn test_move_left() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Should be able to move left from center
    let result = client.move_left();
    assert!(result);
}

#[test]
fn test_move_right() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Should be able to move right from center
    let result = client.move_right();
    assert!(result);
}

#[test]
fn test_move_down() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Should be able to move down from top
    let result = client.move_down();
    assert!(result);
}

#[test]
fn test_hard_drop() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Hard drop should drop multiple rows
    let rows = client.drop();
    assert!(rows > 0);
}

#[test]
fn test_update_tick() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Initial tick should not clear lines
    let lines_cleared = client.update_tick();
    assert_eq!(lines_cleared, 0);
}

#[test]
fn test_get_score() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Initial score should be 0
    let score = client.get_score();
    assert_eq!(score, 0);
}

#[test]
fn test_game_over_status() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Game should not be over initially
    let game_over = client.is_game_over();
    assert!(!game_over);
}

#[test]
fn test_multiple_moves_sequence() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Perform a sequence of moves
    assert!(client.move_left());
    assert!(client.move_right());
    assert!(client.rotate());
    assert!(client.move_down());
    
    // Drop and check score updated
    client.drop();
    let score = client.get_score();
    assert!(score > 0); // Should have points from hard drop
}

#[test]
fn test_boundary_left() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Move all the way left
    for _ in 0..10 {
        client.move_left();
    }
    
    // Should not be able to move further left
    let result = client.move_left();
    assert!(!result);
}

#[test]
fn test_boundary_right() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Move all the way right
    for _ in 0..10 {
        client.move_right();
    }
    
    // Should not be able to move further right
    let result = client.move_right();
    assert!(!result);
}

#[test]
fn test_piece_locks_at_bottom() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Drop piece to bottom
    client.drop();
    
    // State should have a new piece now
    let state = client.get_state();
    // Check that board has some filled cells
    // Check that board has some filled cells
    let mut has_filled_cells = false;
    for i in 0..state.board.len() {
        if state.board.get(i).unwrap() != 0 {
            has_filled_cells = true;
            break;
        }
    }
    assert!(has_filled_cells);
}

#[test]
fn test_state_persistence() {
    let env = Env::default();
    let contract_id = env.register(TetrisContract, ());
    let client = TetrisContractClient::new(&env, &contract_id);
    
    client.init_game();
    
    // Make some moves
    client.move_left();
    client.rotate();
    
    // Get state
    let state = client.get_state();
    
    // Score should be 0 initially
    assert_eq!(state.score, 0);
    assert_eq!(state.level, 1);
    assert!(!state.game_over);
}

use crate::game_state::GameState;

/// Rotate tetromino clockwise with wall kick support
/// 
/// This demonstrates cougr-core's system pattern - pure functions that
/// operate on game state, making on-chain logic testable and predictable.
pub fn rotate_tetromino(state: &mut GameState) -> bool {
    if state.game_over {
        return false;
    }
    
    let original_rotation = state.current_piece.rotation;
    state.current_piece.rotate_cw();
    
    // Check if rotation is valid
    if state.is_valid_position(
        state.current_piece.x,
        state.current_piece.y,
        state.current_piece.rotation
    ) {
        return true;
    }
    
    // Try wall kicks (move left/right to accommodate rotation)
    let kicks = [(1, 0), (-1, 0), (2, 0), (-2, 0), (0, -1)];
    
    for (dx, dy) in kicks.iter() {
        if state.is_valid_position(
            state.current_piece.x + dx,
            state.current_piece.y + dy,
            state.current_piece.rotation
        ) {
            state.current_piece.x += dx;
            state.current_piece.y += dy;
            return true;
        }
    }
    
    // Rotation failed, revert
    state.current_piece.rotation = original_rotation;
    false
}

/// Move piece left
pub fn move_left(state: &mut GameState) -> bool {
    if state.game_over {
        return false;
    }
    
    if state.is_valid_position(
        state.current_piece.x - 1,
        state.current_piece.y,
        state.current_piece.rotation
    ) {
        state.current_piece.move_left();
        true
    } else {
        false
    }
}

/// Move piece right
pub fn move_right(state: &mut GameState) -> bool {
    if state.game_over {
        return false;
    }
    
    if state.is_valid_position(
        state.current_piece.x + 1,
        state.current_piece.y,
        state.current_piece.rotation
    ) {
        state.current_piece.move_right();
        true
    } else {
        false
    }
}

/// Move piece down
pub fn move_down(state: &mut GameState) -> bool {
    if state.game_over {
        return false;
    }
    
    if state.is_valid_position(
        state.current_piece.x,
        state.current_piece.y + 1,
        state.current_piece.rotation
    ) {
        state.current_piece.move_down();
        true
    } else {
        // Piece can't move down - lock it
        state.lock_piece();
        state.clear_lines();
        state.spawn_next_piece();
        state.check_game_over();
        false
    }
}

/// Hard drop - instant drop to bottom
pub fn hard_drop(state: &mut GameState) -> u32 {
    if state.game_over {
        return 0;
    }
    
    let mut rows_dropped = 0u32;
    
    // Keep moving down until blocked
    while state.is_valid_position(
        state.current_piece.x,
        state.current_piece.y + 1,
        state.current_piece.rotation
    ) {
        state.current_piece.move_down();
        rows_dropped += 1;
    }
    
    // Lock piece
    state.lock_piece();
    state.clear_lines();
    state.spawn_next_piece();
    state.check_game_over();
    
    // Bonus points for hard drop
    state.score += rows_dropped * 2;
    
    rows_dropped
}

/// Game tick - automatic downward movement and line clearing
/// 
/// This function demonstrates cougr-core's ability to handle complex
/// game logic updates in a single transaction. In a traditional approach,
/// this would require multiple contract calls.
pub fn update_tick(state: &mut GameState) -> u32 {
    if state.game_over {
        return 0;
    }
    
    // Try to move piece down
    if state.is_valid_position(
        state.current_piece.x,
        state.current_piece.y + 1,
        state.current_piece.rotation
    ) {
        state.current_piece.move_down();
        0
    } else {
        // Piece landed - lock it and clear lines
        state.lock_piece();
        let lines_cleared = state.clear_lines();
        state.spawn_next_piece();
        state.check_game_over();
        lines_cleared
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;
    
    #[test]
    fn test_rotate_tetromino() {
        let env = Env::default();
        let mut state = GameState::new(&env);
        let original_rotation = state.current_piece.rotation;
        
        assert!(rotate_tetromino(&mut state));
        assert_eq!(state.current_piece.rotation, (original_rotation + 1) % 4);
    }
    
    #[test]
    fn test_move_left() {
        let env = Env::default();
        let mut state = GameState::new(&env);
        let original_x = state.current_piece.x;
        
        assert!(move_left(&mut state));
        assert_eq!(state.current_piece.x, original_x - 1);
    }
    
    #[test]
    fn test_move_right() {
        let env = Env::default();
        let mut state = GameState::new(&env);
        let original_x = state.current_piece.x;
        
        assert!(move_right(&mut state));
        assert_eq!(state.current_piece.x, original_x + 1);
    }
    
    #[test]
    fn test_cannot_move_through_walls() {
        let env = Env::default();
        let mut state = GameState::new(&env);
        
        // Move to left wall
        for _ in 0..10 {
            move_left(&mut state);
        }
        
        // Should not be able to move further left
        assert!(!move_left(&mut state));
    }
    
    #[test]
    fn test_hard_drop() {
        let env = Env::default();
        let mut state = GameState::new(&env);
        let rows = hard_drop(&mut state);
        
        // Should have dropped some rows
        assert!(rows > 0);
    }
}

use soroban_sdk::{contracttype, Vec, Env};
use crate::tetromino::{Tetromino, TetrominoType};

///  Main game state structure
/// 
/// This structure uses cougr-core's component pattern to organize game data.
/// In a full ECS implementation, these would be separate components attached
/// to entities, but for Soroban's storage efficiency, we keep them together.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameState {
    /// Game board - flattened 200-element vector (20 rows x 10 columns)
    /// Each cell: 0 = empty, 1-7 = tetromino type
    /// Access via: board[y * 10 + x]
    pub board: Vec<u32>,
    
    /// Current active tetromino
    pub current_piece: Tetromino,
    
    /// Next tetromino to spawn
    pub next_piece: Tetromino,
    
    /// Current score
    pub score: u32,
    
    /// Current level (affects drop speed off-chain)
    pub level: u32,
    
    /// Total lines cleared
    pub lines_cleared: u32,
    
    /// Game over flag
    pub game_over: bool,
}

impl GameState {
    /// Create a new game state with initial configuration
    pub fn new(env: &Env) -> Self {
        // Create empty board
        let mut board = Vec::new(env);
        for _ in 0..200 {
            board.push_back(0u32);
        }
        
        Self {
            board,
            current_piece: Tetromino::new(TetrominoType::random(0)),
            next_piece: Tetromino::new(TetrominoType::random(1)),
            score: 0,
            level: 1,
            lines_cleared: 0,
            game_over: false,
        }
    }
    
    /// Get board value at (x, y)
    fn get_board(&self, x: usize, y: usize) -> u32 {
        if x >= 10 || y >= 20 {
            return 1; // Out of bounds = wall
        }
        self.board.get((y * 10 + x) as u32).unwrap_or(0)
    }
    
    /// Set board value at (x, y)
    fn set_board(&mut self, x: usize, y: usize, value: u32) {
        if x < 10 && y < 20 {
            self.board.set((y * 10 + x) as u32, value);
        }
    }
    
    /// Reset the game state
    pub fn reset(&mut self, env: &Env) {
        self.board = Vec::new(env);
        for _ in 0..200 {
            self.board.push_back(0u32);
        }
        self.current_piece = Tetromino::new(TetrominoType::random(0));
        self.next_piece = Tetromino::new(TetrominoType::random(1));
        self.score = 0;
        self.level = 1;
        self.lines_cleared = 0;
        self.game_over = false;
    }
    
    /// Spawn the next piece
    pub fn spawn_next_piece(&mut self) {
        self.current_piece = self.next_piece.clone();
        self.next_piece = Tetromino::new(TetrominoType::random(
            (self.lines_cleared + self.score) as u64
        ));
    }
    
    /// Lock current piece to the board
    pub fn lock_piece(&mut self) {
        let blocks = self.current_piece.get_blocks();
        let piece_type = (self.current_piece.piece_type as u32) + 1;
        
        for (dx, dy) in blocks.iter() {
            let x = (self.current_piece.x + dx) as usize;
            let y = (self.current_piece.y + dy) as usize;
            
            if y < 20 && x < 10 {
                self.set_board(x, y, piece_type);
            }
        }
    }
    
    /// Check if current position is valid (no collision)
    pub fn is_valid_position(&self, x: i32, y: i32, rotation: u32) -> bool {
        let mut temp_piece = self.current_piece.clone();
        temp_piece.x = x;
        temp_piece.y = y;
        temp_piece.rotation = rotation;
        
        let blocks = temp_piece.get_blocks();
        
        for (dx, dy) in blocks.iter() {
            let new_x = x + dx;
            let new_y = y + dy;
            
            // Check boundaries
            if new_x < 0 || new_x >= 10 || new_y >= 20 {
                return false;
            }
            
            // Check collision with existing blocks (only if piece is in play area)
            if new_y >= 0 {
                let board_x = new_x as usize;
                let board_y = new_y as usize;
                
                if self.get_board(board_x, board_y) != 0 {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Clear complete lines and return count
    pub fn clear_lines(&mut self) -> u32 {
        let mut count = 0u32;
        
        // Process from bottom to top
        let mut y = 19; // Start from bottom
        while y > 0 || (y == 0 && count == 0) {
            // Check if line is complete
            let mut is_complete = true;
            for x in 0..10 {
                if self.get_board(x, y) == 0 {
                    is_complete = false;
                    break;
                }
            }
            
            if is_complete {
                // Clear this line and shift down
                count += 1;
                
                // Shift all lines above down
                for shift_y in (1..=y).rev() {
                    for x in 0..10 {
                        let value = self.get_board(x, shift_y - 1);
                        self.set_board(x, shift_y, value);
                    }
                }
                
                // Clear top line
                for x in 0..10 {
                    self.set_board(x, 0, 0);
                }
                
                // Don't decrement y since we shifted down
            } else {
                // Move to next line up
                if y == 0 {
                    break;
                }
                y -= 1;
            }
        }
        
        // Update stats
        if count > 0 {
            self.lines_cleared += count;
            
            // Standard Tetris scoring
            let points = match count {
                1 => 40,
                2 => 100,
                3 => 300,
                4 => 1200,
                _ => 0,
            };
            self.score += points * self.level;
            
            // Level up every 10 lines
            self.level = (self.lines_cleared / 10) + 1;
        }
        
        count
    }
    
    /// Check if game is over (piece can't spawn)
    pub fn check_game_over(&mut self) {
        // Check if the newly spawned piece immediately collides
        if !self.is_valid_position(
            self.current_piece.x,
            self.current_piece.y,
            self.current_piece.rotation
        ) {
            self.game_over = true;
        }
    }
}

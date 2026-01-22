use soroban_sdk::contracttype;

/// Tetromino types (7 standard Tetris pieces)
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TetrominoType {
    I = 0, // Straight line
    O = 1, // Square
    T = 2, // T-shape
    S = 3, // S-shape
    Z = 4, // Z-shape
    J = 5, // J-shape
    L = 6, // L-shape
}

impl TetrominoType {
    /// Generate a "random" tetromino based on a seed
    /// In production, you'd use proper randomness, but for deterministic
    /// on-chain execution, we use a simple pseudo-random approach
    pub fn random(seed: u64) -> Self {
        match seed % 7 {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::J,
            _ => TetrominoType::L,
        }
    }
    
    /// Get the blocks for this tetromino at rotation 0
    pub fn get_base_blocks(&self) -> [(i32, i32); 4] {
        match self {
            // I piece: ####
            TetrominoType::I => [(0, 0), (1, 0), (2, 0), (3, 0)],
            
            // O piece: ##
            //          ##
            TetrominoType::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            
            // T piece:  #
            //          ###
            TetrominoType::T => [(1, 0), (0, 1), (1, 1), (2, 1)],
            
            // S piece:  ##
            //          ##
            TetrominoType::S => [(1, 0), (2, 0), (0, 1), (1, 1)],
            
            // Z piece: ##
            //           ##
            TetrominoType::Z => [(0, 0), (1, 0), (1, 1), (2, 1)],
            
            // J piece: #
            //          ###
            TetrominoType::J => [(0, 0), (0, 1), (1, 1), (2, 1)],
            
            // L piece:   #
            //          ###
            TetrominoType::L => [(2, 0), (0, 1), (1, 1), (2, 1)],
        }
    }
}

/// Tetromino instance with position and rotation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tetromino {
    pub piece_type: TetrominoType,
    pub x: i32,  // X position on board
    pub y: i32,  // Y position on board
    pub rotation: u32, // 0-3 (90 degree rotations)
}

impl Tetromino {
    /// Create a new tetromino at spawn position (top center)
    pub fn new(piece_type: TetrominoType) -> Self {
        Self {
            piece_type,
            x: 3, // Start at column 3 (center of 10-wide board)
            y: 0, // Start at top
            rotation: 0,
        }
    }
    
    /// Get the blocks for this tetromino with current rotation
    pub fn get_blocks(&self) -> [(i32, i32); 4] {
        let base_blocks = self.piece_type.get_base_blocks();
        
        // O piece doesn't rotate
        if matches!(self.piece_type, TetrominoType::O) {
            return base_blocks;
        }
        
        // Apply rotation transformation
        let mut rotated_blocks = [(0i32, 0i32); 4];
        
        for (i, &(x, y)) in base_blocks.iter().enumerate() {
            rotated_blocks[i] = match self.rotation % 4 {
                0 => (x, y),           // 0°
                1 => (y, -x + 2),      // 90° clockwise
                2 => (-x + 2, -y + 2), // 180°
                3 => (-y + 2, x),      // 270° clockwise
                _ => (x, y),
            };
        }
        
        rotated_blocks
    }
    
    /// Rotate clockwise
    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }
    
    /// Rotate counter-clockwise
    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 3) % 4;
    }
    
    /// Move left
    pub fn move_left(&mut self) {
        self.x -= 1;
    }
    
    /// Move right
    pub fn move_right(&mut self) {
        self.x += 1;
    }
    
    /// Move down
    pub fn move_down(&mut self) {
        self.y += 1;
    }
    
    /// Move up (for wall kick)
    pub fn move_up(&mut self) {
        self.y -= 1;
    }
}

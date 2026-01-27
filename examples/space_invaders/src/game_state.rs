//! Game state structures for Space Invaders
//! 
//! This module defines all the data structures needed to represent
//! the game state on-chain using Soroban's storage.

use soroban_sdk::contracttype;

/// Direction for ship movement
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

/// Type of invader (affects points and behavior)
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum InvaderType {
    /// Top row invaders - 30 points
    Squid = 0,
    /// Middle row invaders - 20 points
    Crab = 1,
    /// Bottom row invaders - 10 points
    Octopus = 2,
}

impl InvaderType {
    /// Get points for destroying this invader type
    pub fn points(&self) -> u32 {
        match self {
            InvaderType::Squid => 30,
            InvaderType::Crab => 20,
            InvaderType::Octopus => 10,
        }
    }
}

/// Represents a single invader in the grid
#[contracttype]
#[derive(Clone, Debug)]
pub struct Invader {
    /// X position (0-based grid position)
    pub x: i32,
    /// Y position (0-based grid position)
    pub y: i32,
    /// Type of invader
    pub invader_type: InvaderType,
    /// Whether the invader is still alive
    pub active: bool,
}

impl Invader {
    pub fn new(x: i32, y: i32, invader_type: InvaderType) -> Self {
        Self {
            x,
            y,
            invader_type,
            active: true,
        }
    }
}

/// Represents a bullet (player or enemy)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Bullet {
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Movement direction: -1 = up (player), 1 = down (enemy)
    pub direction: i32,
    /// Whether the bullet is still active
    pub active: bool,
}

impl Bullet {
    pub fn new(x: i32, y: i32, direction: i32) -> Self {
        Self {
            x,
            y,
            direction,
            active: true,
        }
    }
    
    /// Create a player bullet (moves up)
    pub fn player_bullet(x: i32, y: i32) -> Self {
        Self::new(x, y, -1)
    }
    
    /// Create an enemy bullet (moves down)
    pub fn enemy_bullet(x: i32, y: i32) -> Self {
        Self::new(x, y, 1)
    }
}

/// Main game state structure
#[contracttype]
#[derive(Clone, Debug)]
pub struct GameState {
    /// Player's ship X position (centered on game board)
    pub ship_x: i32,
    /// Player's current score
    pub score: u32,
    /// Player's remaining lives
    pub lives: u32,
    /// Whether the game is over
    pub game_over: bool,
    /// Current invader movement direction (1 = right, -1 = left)
    pub invader_direction: i32,
    /// Current game tick (for pacing)
    pub tick: u32,
    /// Cooldown for player shooting (ticks until can shoot again)
    pub shoot_cooldown: u32,
}

impl GameState {
    /// Create a new game state with default values
    pub fn new() -> Self {
        Self {
            ship_x: GAME_WIDTH / 2,
            score: 0,
            lives: 3,
            game_over: false,
            invader_direction: 1,
            tick: 0,
            shoot_cooldown: 0,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

// Game constants
/// Width of the game board
pub const GAME_WIDTH: i32 = 40;
/// Height of the game board
pub const GAME_HEIGHT: i32 = 30;
/// Number of invader columns
pub const INVADER_COLS: u32 = 8;
/// Number of invader rows
pub const INVADER_ROWS: u32 = 4;
/// Ship's Y position (fixed at bottom)
pub const SHIP_Y: i32 = GAME_HEIGHT - 2;
/// Y position where invaders cause game over
pub const INVADER_WIN_Y: i32 = SHIP_Y - 2;
/// Points needed for extra life
pub const EXTRA_LIFE_SCORE: u32 = 1000;
/// Shoot cooldown in ticks
pub const SHOOT_COOLDOWN: u32 = 3;
/// Bullet speed (positions per tick)
pub const BULLET_SPEED: i32 = 2;
/// Invader movement speed (ticks between moves)
pub const INVADER_MOVE_INTERVAL: u32 = 5;

/// Storage keys for Soroban persistent storage
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Main game state
    State,
    /// List of invaders
    Invaders,
    /// List of player bullets
    PlayerBullets,
    /// List of enemy bullets  
    EnemyBullets,
    /// Flag indicating if game has been initialized
    Initialized,
    /// Count of cougr-core entities (demonstrates ECS integration)
    EntityCount,
}


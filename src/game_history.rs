// Re-export the main types and functionality from the split modules
pub use crate::game_history_core::{GameHistory, GameOutcome, GameRound};

// The display and export functionality is implemented as trait extensions
// in their respective modules, so they're automatically available when
// those modules are imported

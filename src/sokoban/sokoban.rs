use std::collections::HashMap;

use tiny::math::{Direction, Position};

use crate::level::{parse_level, Cell, Grid, Level};

pub const END: &str = "#########################
#                       #
#       The End!        #
# All levels completed. #
#                       #
#   Congratulations!    #
#                       #
#   Any key to quit...  #
#                       #
#########################";

struct Move {
    player_move: Direction,
    box_move: Option<i32>,
}

pub struct GameState {
    pub player_position: Position,
    box_positions: HashMap<i32, Position>,
    level: Level,
    move_history: Vec<Move>,
}

impl GameState {
    fn new(level: Level) -> Self {
        GameState {
            player_position: level.start_position,
            box_positions: level.box_positions.clone(),
            level,
            move_history: vec![],
        }
    }

    pub fn load_level(level_string: &str) -> Result<Self, String> {
        let Ok(level) = parse_level(level_string) else {
            return Err("Failed parsing level.".to_string());
        };

        Ok(GameState::new(level))
    }

    pub fn render_grid(&self) -> Grid {
        let mut new_grid = self.level.grid.clone();

        new_grid.set_cell(self.player_position, Cell::Player);
        for (load_id, position) in &self.box_positions {
            new_grid.set_cell(*position, Cell::Box(*load_id));
        }

        new_grid
    }

    pub fn reset(&mut self) {
        let level = self.level.clone();
        *self = GameState::new(level);
    }

    pub fn undo(&mut self) {
        let Some(move_item) = self.move_history.pop() else {
            return;
        };

        self.player_position = self.player_position - move_item.player_move;

        if let Some(box_id) = move_item.box_move {
            if let Some(load_position) = self.box_positions.get_mut(&box_id) {
                let old_position = *load_position - move_item.player_move;
                *load_position = old_position;
            } else {
                panic!("Got an id of an unnexisting box.");
            }
        }
    }

    pub fn move_player(&mut self, grid: &Grid, direction: Direction) {
        assert!(grid.player_can_move(self.player_position, direction));

        let to_position = self.player_position + direction;

        let mut move_item = Move {
            player_move: direction,
            box_move: None,
        };

        // Move the load if there is one and it can move.
        if let Cell::Box(uid) = grid.cell_at(to_position) {
            if let Some(load_position) = self.box_positions.get_mut(&uid) {
                *load_position = to_position + direction;
                move_item.box_move = Some(uid);
            } else {
                panic!("Got an id of an unnexisting load.");
            }
        }

        self.player_position = to_position;
        self.move_history.push(move_item);
    }

    pub fn level_is_complete(&self) -> bool {
        for load_position in self.box_positions.values() {
            if self.level.grid.cell_at(*load_position) != Cell::Target {
                return false;
            }
        }

        true
    }
}

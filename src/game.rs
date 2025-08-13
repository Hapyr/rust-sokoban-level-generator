use crate::cell::Cell;
use crate::level::Level;
use crate::board::Board;
use rand::seq::SliceRandom;
use rand::Rng;


#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum Action {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}


#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct SokobanGame {
    pub board: Board,
    pub player_position: (usize, usize),
    pub previous_action: Option<Action>,
    pub search_depth: usize,
    pub max_search_depth: usize,
}

impl SokobanGame {
    pub fn new(level: Level, max_search_depth: usize) -> Self {
        let board = Board::new(level);
        let player_position = Self::get_player_position(&board);
        Self {
            board,
            player_position,
            previous_action: None,
            search_depth: 0,
            max_search_depth: max_search_depth,
        }
    }

    pub fn print_pretty(&self) {
        for y in 0..self.board.height {
            for x in 0..self.board.width {
                print!("{}", self.board.grid[y][x].to_char());
            }
            println!();
        }
    }

    // get successor of the game and the cost of the action for each transition
    pub fn get_successor(&self, random_subset: bool, random_subset_probability: f64, dead_end_limit: bool, all_actions: bool, all_actions_probability: f64) -> Vec<(Self, usize)> {
        // get possible actions
        let actions = self.get_possible_actions(all_actions, all_actions_probability);
        let mut successors = Vec::new();

        if self.search_depth >= self.max_search_depth {
            return successors;
        }

        for action in actions {
            let mut successor = self.clone();
            successor.step(action);

            if dead_end_limit {
                if !successor.is_dead_end() {
                    successor.search_depth += 1;
                    successors.push((successor, 1)); // Each move has a cost of 1
                }
            } else {
                successor.search_depth += 1;
                successors.push((successor, 1)); // Each move has a cost of 1
            }
        }

        if random_subset {
            let mut rng = rand::thread_rng();
            if !successors.is_empty() && rng.gen_bool(random_subset_probability) {
                let subset_size = rng.gen_range(1..=successors.len());
                successors.shuffle(&mut rng);
                successors.truncate(subset_size);
            }
        }

        successors
    }

    // Simple dead end detection. Dead end is when a box is surrounded by walls from 2 not perpendicular sides
    pub fn is_dead_end(&self) -> bool {
        for box_pos in self.board.boxes.iter() {
            // check if its a box not a boxongoal
            if self.board.grid[box_pos.pos.1][box_pos.pos.0] == Cell::Box {
                if self.board.grid[box_pos.pos.1 + 1][box_pos.pos.0] == Cell::Wall
                    && self.board.grid[box_pos.pos.1][box_pos.pos.0 + 1] == Cell::Wall
                {
                    return true;
                }
                if self.board.grid[box_pos.pos.1 - 1][box_pos.pos.0] == Cell::Wall
                    && self.board.grid[box_pos.pos.1][box_pos.pos.0 + 1] == Cell::Wall
                {
                    return true;
                }
                if self.board.grid[box_pos.pos.1 - 1][box_pos.pos.0] == Cell::Wall
                    && self.board.grid[box_pos.pos.1][box_pos.pos.0 - 1] == Cell::Wall
                {
                    return true;
                }
                if self.board.grid[box_pos.pos.1 + 1][box_pos.pos.0] == Cell::Wall
                    && self.board.grid[box_pos.pos.1][box_pos.pos.0 - 1] == Cell::Wall
                {
                    return true;
                }
            }
        }
        false
    }

    
    // simple heuristic to estimate the distance to the goal. 
    // distance is the sum of the distance to the nearest goal for each box
    // and the distance to the nearest box (not boxongoal) for the player
    pub fn distance_to_goal(&self) -> usize {
        // for each box calculate the distance to the nearst goal or boxongoal
        let mut distances = Vec::new();

        for box_pos in self.board.boxes.iter() {
            let mut min_distance = isize::MAX;
            for goal in self.board.goals.iter() {
                let distance = (box_pos.pos.0 as isize - goal.pos.0 as isize).abs()
                    + (box_pos.pos.1 as isize - goal.pos.1 as isize).abs();
                if distance < min_distance {
                    min_distance = distance;
                }
            }
            distances.push(min_distance);
        }
        let mut min_distance = isize::MAX;
        for box_pos in self.board.boxes.iter() {
            if box_pos.cell != Cell::BoxOnGoal {
                let distance = (self.player_position.0 as isize - box_pos.pos.0 as isize).abs()
                + (self.player_position.1 as isize - box_pos.pos.1 as isize).abs();
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }
        min_distance as usize + distances.iter().sum::<isize>() as usize
    }

    // check if the game is solved
    pub fn is_solved(&self) -> bool {
        // check if all boxes are on goals by counting number of Box. If they are zero, return true.
        let box_count = self
            .board
            .grid
            .iter()
            .flatten()
            .filter(|&cell| *cell == Cell::Box)
            .count();
        box_count == 0
    }

    pub fn get_player_position(board: &Board) -> (usize, usize) {
        for y in 0..board.height {
            for x in 0..board.width {
                if board.grid[y][x] == Cell::Player || board.grid[y][x] == Cell::PlayerOnGoal {
                    return (x, y);
                }
            }
        }
        panic!("No player found in level");
    }

    // step the game
    pub fn step(&mut self, action: Action) {
        let (x, y) = self.player_position;
        self.previous_action = Some(action.clone());

        if action == Action::UP {

            // check for invalid action
            if self.is_not_free_to_move_on(x, y - 1) {
                if y - 1 == 0 {
                    return;
                }else{
                    if  self.is_not_free_to_move_on(x, y - 2) {
                        return;
                    }
                }
                
                if self.board.grid[y - 1][x] == Cell::Wall {
                    return;
                }
            }

            // check for box
            if self.board.grid[y][x] == Cell::PlayerOnGoal {
                self.board.grid[y][x] = Cell::Goal;
            } else {
                self.board.grid[y][x] = Cell::Empty;
            }
            self.player_position = (x, y - 1);

            if self.is_free_to_move_on(x, y - 1) {
                if self.board.grid[y - 1][x] == Cell::Goal {
                    self.board.grid[y - 1][x] = Cell::PlayerOnGoal;
                } else {
                    self.board.grid[y - 1][x] = Cell::Player;
                }
            }
            if self.is_blocked_by_box(x, y - 1) {
                if self.board.grid[y - 1][x] == Cell::BoxOnGoal {
                    self.board.grid[y - 1][x] = Cell::PlayerOnGoal;
                } else {
                    self.board.grid[y - 1][x] = Cell::Player;
                }

                if self.board.grid[y - 2][x] == Cell::Goal {
                    self.board.grid[y - 2][x] = Cell::BoxOnGoal;
                } else {
                    self.board.grid[y - 2][x] = Cell::Box;
                }
                let box_index = self.board.box_object_positions[y - 1][x];
                self.board.boxes[box_index].pos = (x, y - 2);
                self.board.box_object_positions[y - 1][x] = usize::MAX;
                self.board.box_object_positions[y - 2][x] = box_index;
            }
        }

        if action == Action::DOWN {

            // check for invalid action
            if self.is_not_free_to_move_on(x, y + 1) {
                if y + 1 == self.board.height - 1 {
                    return;
                }else{
                    if self.is_not_free_to_move_on(x, y + 2) {
                        return;
                    }
                }
                if self.board.grid[y + 1][x] == Cell::Wall {
                    return;
                }
            }
            
            // check if player is on goal
            if self.board.grid[y][x] == Cell::PlayerOnGoal {
                self.board.grid[y][x] = Cell::Goal;
            } else {
                self.board.grid[y][x] = Cell::Empty;
            }
            self.player_position = (x, y + 1);

            if self.is_free_to_move_on(x, y + 1) {
                if self.board.grid[y + 1][x] == Cell::Goal {
                    self.board.grid[y + 1][x] = Cell::PlayerOnGoal;
                } else {
                    self.board.grid[y + 1][x] = Cell::Player;
                }
            }
            if self.is_blocked_by_box(x, y + 1) {
                if self.board.grid[y + 1][x] == Cell::BoxOnGoal {
                    self.board.grid[y + 1][x] = Cell::PlayerOnGoal;
                } else {
                    self.board.grid[y + 1][x] = Cell::Player;
                }

                if self.board.grid[y + 2][x] == Cell::Goal {
                    self.board.grid[y + 2][x] = Cell::BoxOnGoal;
                } else {
                    self.board.grid[y + 2][x] = Cell::Box;
                }

                let box_index = self.board.box_object_positions[y + 1][x];
                self.board.boxes[box_index].pos = (x, y + 2);
                self.board.box_object_positions[y + 1][x] = usize::MAX;
                self.board.box_object_positions[y + 2][x] = box_index;
            }
        }

        if action == Action::LEFT {

            // check for invalid action
            if self.is_not_free_to_move_on(x - 1, y) {
                if x - 1 == 0 {
                    return;
                }else{
                    if self.is_not_free_to_move_on(x - 2, y) {
                        return;
                    }
                }
                
                if self.board.grid[y][x - 1] == Cell::Wall {
                    return;
                }
            }

            // check if player is on goal
            if self.board.grid[y][x] == Cell::PlayerOnGoal {
                self.board.grid[y][x] = Cell::Goal;
            } else {
                self.board.grid[y][x] = Cell::Empty;
            }
            self.player_position = (x - 1, y);

            if self.is_free_to_move_on(x - 1, y) {
                if self.board.grid[y][x - 1] == Cell::Goal {
                    self.board.grid[y][x - 1] = Cell::PlayerOnGoal;
                } else {
                    self.board.grid[y][x - 1] = Cell::Player;
                }
            }
            if self.is_blocked_by_box(x - 1, y) {
                if self.board.grid[y][x - 1] == Cell::BoxOnGoal {
                    self.board.grid[y][x - 1] = Cell::PlayerOnGoal;
                } else {
                    self.board.grid[y][x - 1] = Cell::Player;
                }

                if self.board.grid[y][x - 2] == Cell::Goal {
                    self.board.grid[y][x - 2] = Cell::BoxOnGoal;
                } else {
                    self.board.grid[y][x - 2] = Cell::Box;
                }
                let box_index = self.board.box_object_positions[y][x - 1];
                self.board.boxes[box_index].pos = (x - 2, y);
                self.board.box_object_positions[y][x - 1] = usize::MAX;
                self.board.box_object_positions[y][x - 2] = box_index;
            }
        }

        if action == Action::RIGHT {

            // check for invalid action
            if self.is_not_free_to_move_on(x + 1, y) {
                if x + 1 == self.board.width - 1 {
                    return;
                }else{
                    if self.is_not_free_to_move_on(x + 2, y) {
                        return;
                    }
                }
                
                if self.board.grid[y][x + 1] == Cell::Wall {
                    return;
                }
            }
            
            if self.board.grid[y][x] == Cell::PlayerOnGoal {
                self.board.grid[y][x] = Cell::Goal;
            } else {
                self.board.grid[y][x] = Cell::Empty;
            }
            self.player_position = (x + 1, y);

            if self.is_free_to_move_on(x + 1, y) {
                if self.board.grid[y][x + 1] == Cell::Goal {
                    self.board.grid[y][x + 1] = Cell::PlayerOnGoal;
                } else {
                    self.board.grid[y][x + 1] = Cell::Player;
                }
            }
            if self.is_blocked_by_box(x + 1, y) {
                if self.board.grid[y][x + 1] == Cell::BoxOnGoal {
                    self.board.grid[y][x + 1] = Cell::PlayerOnGoal;
                } else {
                    self.board.grid[y][x + 1] = Cell::Player;
                }

                if self.board.grid[y][x + 2] == Cell::Goal {
                    self.board.grid[y][x + 2] = Cell::BoxOnGoal;
                } else {
                    self.board.grid[y][x + 2] = Cell::Box;
                }
                let box_index = self.board.box_object_positions[y][x + 1];
                self.board.boxes[box_index].pos = (x + 1, y);
                self.board.box_object_positions[y][x + 1] = usize::MAX;
                self.board.box_object_positions[y][x + 2] = box_index;
            }
        }
        /*
        // print grid object line by line
        for y in 0..self.board.height {
            for x in 0..self.board.width {
                // if usize::MAX, print space and grid board is wall print #
                if self.board.grid[y][x] == Cell::Wall {
                    print!("#");
                }else{
                    if self.board.grid_objects[y][x] != usize::MAX {
                        print!("{}", self.board.grid_objects[y][x]);

                    }else{
                        print!(" ");
                    }
                }
            }
            println!();
        }
        */
    }

    fn is_free_to_move_on(&self, x: usize, y: usize) -> bool {
        match self.board.grid[y][x] {
            Cell::Floor | Cell::Goal | Cell::SpecialFloor | Cell::Empty => true,
            _ => false,
        }
    }

    fn is_not_free_to_move_on(&self, x: usize, y: usize) -> bool {
        if x >= self.board.width || y >= self.board.height {
            return false;
        }
        match self.board.grid[y][x] {
            Cell::Wall | Cell::Box | Cell::BoxOnGoal => true,
            _ => false,
        }
    }

    fn is_blocked_by_box(&self, x: usize, y: usize) -> bool {
        match self.board.grid[y][x] {
            Cell::Box | Cell::BoxOnGoal => true,
            _ => false,
        }
    }


    pub fn get_possible_actions(&self, all_actions: bool, all_actions_probability: f64) -> Vec<Action> {
        let (x, y) = self.player_position;
        let mut actions = Vec::new();

        if all_actions {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(all_actions_probability) {
                actions.push(Action::UP);
                actions.push(Action::DOWN);
                actions.push(Action::LEFT);
                actions.push(Action::RIGHT);
                return actions;
            }
        }

        // up move
        if y > 0 && self.is_free_to_move_on(x, y - 1) {
            actions.push(Action::UP);
        }
        if y > 1 && self.is_blocked_by_box(x, y - 1) && self.is_free_to_move_on(x, y - 2) {
            actions.push(Action::UP);
        }

        // down move
        if y < self.board.height - 1 && self.is_free_to_move_on(x, y + 1) {
            actions.push(Action::DOWN);
        }
        if y < self.board.height - 2
            && self.is_blocked_by_box(x, y + 1)
            && self.is_free_to_move_on(x, y + 2)
        {
            actions.push(Action::DOWN);
        }
        
        // left move
        if x > 0 && self.is_free_to_move_on(x - 1, y) {
            actions.push(Action::LEFT);
        }
        if x > 1 && self.is_blocked_by_box(x - 1, y) && self.is_free_to_move_on(x - 2, y) {
            actions.push(Action::LEFT);
        }
        
        // right move
        if x < self.board.width - 1 && self.is_free_to_move_on(x + 1, y) {
            actions.push(Action::RIGHT);
        }
        if x < self.board.width - 2
            && self.is_blocked_by_box(x + 1, y)
            && self.is_free_to_move_on(x + 2, y)
        {
            actions.push(Action::RIGHT);
        }
        actions
    }
}

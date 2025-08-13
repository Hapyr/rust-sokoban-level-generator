use ndarray::{s, Array2};

use crate::cell::Cell;

/// Representation of a level in sokoban.
pub type Level = Array2<Cell>;

/// Generate a new level with the given dimensions and box count.
/// Height and width are the number of rooms in the level.
/// A room is 3x3 cells/fields.
pub fn generate_level(height: u8, width: u8, boxes: u8) -> Level {
    let rooms = loop {
        let rooms = generation::generate_rooms(height.into(), width.into());
        if requirements::level_meets_requirements(&rooms, boxes) {
            break rooms;
        }
    };

    let level = loop {
        let rooms = rooms.clone();
        // Place entities (goals, boxes, player) in the level.
        // If there is no backtrack map for the box positions, retry.
        if let Some(level) = entities::place_entities(rooms, boxes.into()) {
            break level;
        }
    };

    let h_end = (height * 3) as usize;
    let w_end = (width * 3) as usize;

    let mut framed_level = Array2::from_elem((h_end + 2, w_end + 2), Cell::Wall);
    let mut space = framed_level.slice_mut(s![1..=h_end, 1..=w_end]);
    space.assign(&level);

    framed_level
}

/// Create the string representation of a level.
/// This creates an exchangeable string that can be used
/// to share the levels with others.
///
/// The notation is from [JSoko Level Format](https://www.sokoban-online.de/sokoban/levell-format/).
///
/// This uses the "Runlength encoding":
/// `#### becomes 4#`
///
/// #### Example
/// ```text
/// #######
/// #.@ # #
/// #$* $ #
/// #   $ #
/// # ..  #
/// #  *  #
/// #######
/// ```
/// becomes
///
/// `7#|#.@-#-#|#$*-$-#|#3-$-#|#-..--#|#--*--#|7#`
pub fn encode_level(level: &Level) -> String {
    let mut encoding = String::new();
    let mut last_char = '\0';

    for row in level.rows() {
        let mut count = 0;
        for cell in row {
            let char = cell.to_encoding_char();
            if char == last_char {
                count += 1;
            } else {
                if last_char != '\0' {
                    encoding.push_str(&format!("{}{}", count, last_char));
                }
                count = 1;
                last_char = char;
            }
        }

        if last_char != '\0' {
            encoding.push_str(&format!("{}{}", count, last_char));
        }

        encoding.push('|');
    }

    encoding.pop();
    encoding
}

/// Print the level to a string with new-lines.
/// The notation is from [JSoko Level Format](https://www.sokoban-online.de/sokoban/levell-format/).
pub fn pretty_print_level(level: &Level) -> String {
    let (w, h) = level.dim();
    let mut print = String::with_capacity((w * h + h).into());

    for row in level.rows() {
        for cell in row {
            print.push(cell.to_char());
        }
        print.push('\n');
    }

    print
}

mod requirements {
    use ndarray::Array2;

    use crate::cell::Cell;
    use crate::level::Level;

    /// Checks the generated level against the requirements and returns
    /// false if the level should not be used.
    pub(super) fn level_meets_requirements(level: &Level, box_count: u8) -> bool {
        has_enough_space(level, box_count)
            && has_connectivity(level)
            && has_no_surrounded_floors(level)
            && has_no_large_spaces(level)
            && has_enough_goal_places(level, box_count)
    }

    /// Ensure that the level has enough space for the player, boxes, and one empty space.
    fn has_enough_space(cells: &Level, box_count: u8) -> bool {
        // count of boxes, plus player, plus 1 empty space
        let target_floors = (box_count + 2) as usize;

        cells.iter().filter(|&&c| c.is_floor()).count() >= target_floors
    }

    fn has_enough_goal_places(cells: &Level, box_count: u8) -> bool {
        super::entities::get_possible_goal_locations(cells).len() >= box_count as usize
    }

    /// Ensure that all floors in the level are connected.
    fn has_connectivity(cells: &Level) -> bool {
        // find starting point: first cell that is a floor.
        let start = cells.indexed_iter().find(|(_, &cell)| cell.is_floor());

        if start.is_none() {
            return false;
        }

        let ((start_x, start_y), _) = start.unwrap();
        let mut visited = Array2::from_elem(cells.dim(), false);
        let (width, height) = cells.dim();
        let mut stack = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            if visited[[x, y]] {
                continue;
            }

            visited[[x, y]] = true;

            // here, effectively floor is checked instead of "is_floor" because
            // one template is a special case
            if x > 0 && cells[[x - 1, y]] == Cell::Floor && !visited[[x - 1, y]] {
                stack.push((x - 1, y));
            }
            if x < width - 1 && cells[[x + 1, y]] == Cell::Floor && !visited[[x + 1, y]] {
                stack.push((x + 1, y));
            }
            if y > 0 && cells[[x, y - 1]] == Cell::Floor && !visited[[x, y - 1]] {
                stack.push((x, y - 1));
            }
            if y < height - 1 && cells[[x, y + 1]] == Cell::Floor && !visited[[x, y + 1]] {
                stack.push((x, y + 1));
            }
        }

        cells
            .iter()
            .zip(visited.iter())
            .filter(|(&cell, &visited)| cell == Cell::Floor && !visited)
            .count()
            == 0
    }

    /// Ensure that no floor tiles are surrounded by walls.
    fn has_no_surrounded_floors(cells: &Level) -> bool {
        let (width, height) = cells.dim();

        cells
            .indexed_iter()
            .filter(|(_, &c)| c.is_floor())
            .all(|((x, y), _)| {
                let mut surrounding_walls = 0;

                if (x > 0 && cells[[x - 1, y]] == Cell::Wall) || x == 0 {
                    surrounding_walls += 1;
                }
                if (x < width - 1 && cells[[x + 1, y]] == Cell::Wall) || x == width - 1 {
                    surrounding_walls += 1;
                }
                if (y > 0 && cells[[x, y - 1]] == Cell::Wall) || y == 0 {
                    surrounding_walls += 1;
                }
                if (y < height - 1 && cells[[x, y + 1]] == Cell::Wall) || y == height - 1 {
                    surrounding_walls += 1;
                }

                surrounding_walls < 3
            })
    }

    /// Ensures that no large spaces exist, they do not create interesting levels.
    fn has_no_large_spaces(cells: &Level) -> bool {
        for window in cells.windows((3, 4)) {
            if window.iter().all(|&c| c.is_floor()) {
                return false;
            }
        }

        for window in cells.windows((4, 3)) {
            if window.iter().all(|&c| c.is_floor()) {
                return false;
            }
        }

        true
    }
}

mod generation {
    use ndarray::{s, Array2};

    use crate::cell::Cell;
    use crate::level::Level;
    use crate::room::get_random_room;

    /// Generate a level with the given dimension of rooms (3x3 cells).
    /// Each room is randomly chosen and rotated. Then the room is
    /// checked against the surrounding rooms and placed if the room fits.
    ///
    /// The room fits if the outermost cells match the surroundings if they
    /// are not "EMPTY" cells.
    pub(super) fn generate_rooms(height: usize, width: usize) -> Level {
        // Create empty level (w*3, h*3, because every room is 3x3)
        let mut level = Array2::from_elem((height * 3, width * 3), Cell::Empty);
        let mut filled_height = 0;
        let mut filled_width = 0;

        // As long as there are empty rooms, fill them with random templates.
        while filled_width < width && filled_height < height {
            let chunk_parts =
                extract_surrounding_cells(&level, filled_height, filled_width, height, width);

            let mut chunk = level.slice_mut(s![
                filled_height * 3..filled_height * 3 + 3,
                filled_width * 3..filled_width * 3 + 3
            ]);

            loop {
                let new_room = get_random_room();

                let mut template_parts: Vec<Cell> = Vec::new();
                if filled_width != 0 {
                    template_parts.extend(new_room.slice(s![0..=0, 1..=3]));
                }
                if filled_width != width - 1 {
                    template_parts.extend(new_room.slice(s![4..=4, 1..=3]));
                }
                if filled_height != 0 {
                    template_parts.extend(new_room.slice(s![1..=3, 0..=0]));
                }
                if filled_height != height - 1 {
                    template_parts.extend(new_room.slice(s![1..=3, 4..=4]));
                }

                if !template_match(&chunk_parts, &template_parts) {
                    continue;
                }

                // if we get here, all parts match. we can place the template.
                chunk.assign(&new_room.slice(s![1..=3, 1..=3]));
                break;
            }

            // update the filled indices.
            if filled_width < width - 1 {
                filled_width += 1;
            } else {
                filled_width = 0;
                filled_height += 1;
            }
        }

        level
    }

    /// Checks the chunk and the surrounding cells. If they match, the template
    /// (room) can be placed.
    fn template_match(chunk_part: &Vec<Cell>, template_part: &Vec<Cell>) -> bool {
        for (&left, &right) in chunk_part.iter().zip(template_part.iter()) {
            if left == Cell::Empty || right == Cell::Empty {
                continue;
            }

            if left != right {
                return false;
            }
        }

        true
    }

    /// Generate a vector that contains the surrounding cells of the chunk.
    /// The edges of the level are not included.
    fn extract_surrounding_cells(
        cells: &Array2<Cell>,
        filled_height: usize,
        filled_width: usize,
        max_height: usize,
        max_width: usize,
    ) -> Vec<Cell> {
        let mut parts = Vec::new();

        let h_base = filled_height * 3;
        let w_base = filled_width * 3;

        if filled_width != 0 {
            parts.extend(cells.slice(s![(h_base)..(h_base + 3), (w_base - 1)..=(w_base - 1)]));
        }
        if filled_width != max_width - 1 {
            parts.extend(cells.slice(s![(h_base)..(h_base + 3), (w_base + 3)..=(w_base + 3)]));
        }
        if filled_height != 0 {
            parts.extend(cells.slice(s![(h_base - 1)..=(h_base - 1), (w_base)..(w_base + 3)]));
        }
        if filled_height != max_height - 1 {
            parts.extend(cells.slice(s![(h_base + 3)..=(h_base + 3), (w_base)..(w_base + 3)]));
        }

        parts
    }
}

mod entities {
    use std::collections::HashMap;

    use ndarray::Array2;
    use rand::prelude::*;

    use crate::cell::Cell;
    use crate::level::Level;

    type TrackingState = (Vec<(usize, usize)>, (usize, usize), Level, i32);
    type StepMap = Array2<i32>;
    type BacktrackMap = HashMap<String, (StepMap, Level)>;

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn make_move(
            &self,
            x: usize,
            y: usize,
            width: usize,
            height: usize,
        ) -> Option<(usize, usize)> {
            match self {
                Self::Up if y > 0 => Some((x, y - 1)),
                Self::Down if y < (height - 1) => Some((x, y + 1)),
                Self::Left if x > 0 => Some((x - 1, y)),
                Self::Right if x < (width - 1) => Some((x + 1, y)),
                _ => None,
            }
        }

        fn iterator() -> impl Iterator<Item = Direction> {
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .iter()
            .copied()
        }
    }

    /// Places goals, boxes and the player in the level.
    /// To create an interesting level, the following steps are performed:
    /// 1. Calculate all possible goal locations and shuffle them
    /// 2. Select the first x goal locations (x = box count)
    /// 3. Place the boxes on the selected goal locations
    /// 4. For all player positions, calculate the backtrack map to see
    ///    the farthest states that boxes can be pushed to
    /// 5. Return the farthest possible state
    pub(super) fn place_entities(mut level: Level, box_count: usize) -> Option<Level> {
        let goals = get_random_goal_locations(&level, box_count);

        // h = height (aka y), w = width (aka x)
        for (h, w) in goals {
            level[[h, w]] = Cell::BoxOnGoal;
        }

        let mut max_level = None;
        let mut max_pos = None;
        let mut max_steps = -1;

        for (_, (steps, level)) in create_box_backtrack_map(&level) {
            if let Some((index, &steps)) = steps.indexed_iter().max_by_key(|(_, &step)| step) {
                if steps > max_steps {
                    max_level = Some(level);
                    max_pos = Some(index);
                    max_steps = steps;
                }
            }
        }

        if max_level.is_none() || max_pos.is_none() {
            return None;
        }

        let (player_y, player_x) = max_pos.unwrap();
        let mut level = max_level.unwrap();
        level[[player_y, player_x]] = match level[[player_y, player_x]] {
            Cell::Goal => Cell::PlayerOnGoal,
            _ => Cell::Player,
        };

        Some(level)
    }

    /// Calculate all possible goal locations, shuffle them and fetch the first
    /// x locations.
    fn get_random_goal_locations(level: &Level, box_count: usize) -> Vec<(usize, usize)> {
        let mut possible_goals = get_possible_goal_locations(level);
        possible_goals.shuffle(&mut thread_rng());
        possible_goals.into_iter().take(box_count).collect()
    }

    /// Get all possible goal locations by checking if a certain position has
    /// 2 empty floors to either side. If any such side exists, the position
    /// is a viable goal location.
    pub fn get_possible_goal_locations(level: &Level) -> Vec<(usize, usize)> {
        let (height, width) = level.dim();

        level
            .indexed_iter()
            .filter(|(_, &cell)| cell.is_floor())
            .filter(|((h, w), _)| {
                let h = *h;
                let w = *w;

                // check if the cell has two collinear floors to any side

                // to the top.
                if h >= 2 && level[[h - 1, w]].is_floor() && level[[h - 2, w]].is_floor() {
                    return true;
                }

                // to the bottom.
                if h <= height - 3 && level[[h + 1, w]].is_floor() && level[[h + 2, w]].is_floor() {
                    return true;
                }

                // to the left.
                if w >= 2 && level[[h, w - 1]].is_floor() && level[[h, w - 2]].is_floor() {
                    return true;
                }

                // to the right.
                if w <= width - 3 && level[[h, w + 1]].is_floor() && level[[h, w + 2]].is_floor() {
                    return true;
                }

                false
            })
            .map(|(index, _)| index)
            .collect()
    }

    fn create_box_backtrack_map(level: &Level) -> BacktrackMap {
        let possible_player_positions = level
            .indexed_iter()
            .filter(|(_, &cell)| cell.is_floor())
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let initial_boxes = level
            .indexed_iter()
            .filter(|(_, &cell)| cell.is_box())
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let (height, width) = level.dim();
        let mut backtrack = HashMap::new();
        for (y, x) in possible_player_positions {
            let mut stack = vec![(initial_boxes.clone(), (x, y).clone(), level.clone(), 0)];

            while let Some(state) = stack.pop() {
                if check_for_cached_map(&state, &mut backtrack) {
                    continue;
                }

                let (boxes, player, level, step) = state;
                for (index, (box_y, box_x)) in boxes.iter().enumerate() {
                    for direction in Direction::iterator() {
                        // Move the box into the direction and see if it is still
                        // accessible.
                        let new_box_position = direction.make_move(*box_x, *box_y, width, height);
                        if new_box_position.is_none()
                            || !is_accessible(&level, player, new_box_position.unwrap())
                        {
                            continue;
                        }
                        let (box_new_x, box_new_y) = new_box_position.unwrap();

                        // Move the player to the same direction since the player
                        // must move the box in this direction.
                        let new_player_position =
                            direction.make_move(box_new_x, box_new_y, width, height);
                        if new_player_position.is_none()
                            || !is_accessible(&level, player, new_player_position.unwrap())
                        {
                            continue;
                        }
                        let new_player_position = new_player_position.unwrap();

                        let mut new_level = level.clone();
                        new_level[[*box_y, *box_x]] = match level[[*box_y, *box_x]] {
                            Cell::BoxOnGoal => Cell::Goal,
                            Cell::Box => Cell::Floor,
                            _ => panic!("Invalid box cell"),
                        };
                        new_level[[box_new_y, box_new_x]] = match level[[box_new_y, box_new_x]] {
                            Cell::Goal => Cell::BoxOnGoal,
                            Cell::Floor => Cell::Box,
                            _ => panic!("Invalid box cell"),
                        };

                        let mut new_boxes = boxes.clone();
                        new_boxes[index] = (box_new_y, box_new_x);

                        stack.push((new_boxes, new_player_position, new_level, step + 1));
                    }
                }
            }
        }

        backtrack
    }

    /// Check for a cached map. If there exists a map, update the used steps
    /// if needed and return true. Otherwise, create a cached map with steps 0
    /// and return false.
    fn check_for_cached_map(state: &TrackingState, backtrack_map: &mut BacktrackMap) -> bool {
        let (_, (player_x, player_y), level, step) = state;

        if *step == 0 {
            return false;
        }

        let id = level_identifier(level);

        if let Some((cached_map, _)) = backtrack_map.get_mut(&id) {
            let used_steps = &cached_map[[*player_y, *player_x]];
            if *used_steps > 0 {
                if *step < *used_steps {
                    update_backtrack_steps(level, cached_map, *player_x, *player_y, *step);
                }

                return true;
            }
        } else {
            backtrack_map.insert(
                id.clone(),
                (Array2::from_elem(level.dim(), 0), level.clone()),
            );
        }

        let (map, _) = backtrack_map.get_mut(&id).unwrap();
        update_backtrack_steps(level, map, *player_x, *player_y, *step);

        false
    }

    /// Update the backtrack map with the given step.
    /// Updates all walkable cells that have not the same step number.
    fn update_backtrack_steps(level: &Level, steps: &mut StepMap, x: usize, y: usize, step: i32) {
        let mut stack = vec![(x, y)];
        let (height, width) = level.dim();

        while let Some((x, y)) = stack.pop() {
            if steps[[y, x]] == step || !level[[y, x]].is_walkable() {
                continue;
            }

            steps[[y, x]] = step;

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < width - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < height - 1 {
                stack.push((x, y + 1));
            }
        }
    }

    /// Generate an identifier for the level. Concatenates all cells in the level.
    fn level_identifier(level: &Level) -> String {
        level.iter().map(|&c| c.to_char()).collect()
    }

    /// Check if the given position is accessible from another position.
    /// The position is accessible, if there exists a way from x1,y1 to x2,y2
    /// which is walkable.
    fn is_accessible(level: &Level, from: (usize, usize), to: (usize, usize)) -> bool {
        let (height, width) = level.dim();
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;

        if from_x >= width || to_x >= width || from_y >= height || to_y >= height {
            return false;
        }

        if !level[[from_y, from_x]].is_walkable() || !level[[to_y, to_x]].is_walkable() {
            return false;
        }

        let mut stack = vec![(from_x, from_y)];
        let mut visited = Array2::from_elem(level.dim(), false);

        while let Some((x, y)) = stack.pop() {
            if visited[[y, x]] {
                continue;
            }

            visited[[y, x]] = true;

            if x == to_x && y == to_y {
                return true;
            }

            if x > 0 && level[[y, x - 1]].is_walkable() && !visited[[y, x - 1]] {
                stack.push((x - 1, y));
            }
            if x < width - 1 && level[[y, x + 1]].is_walkable() && !visited[[y, x + 1]] {
                stack.push((x + 1, y));
            }
            if y > 0 && level[[y - 1, x]].is_walkable() && !visited[[y - 1, x]] {
                stack.push((x, y - 1));
            }
            if y < height - 1 && level[[y + 1, x]].is_walkable() && !visited[[y + 1, x]] {
                stack.push((x, y + 1));
            }
        }

        false
    }
}

use crate::cell::Cell;
use crate::level::Level;


// A grid object is either a box or a goal. It is used for faster iteration over boxes and goals.
#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct GridObject {
    pub pos: (usize, usize),
    pub cell: Cell,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct Board {
    pub grid: Vec<Vec<Cell>>,
    pub height: usize,
    pub width: usize,
    pub boxes: Vec<GridObject>,
    pub goals: Vec<GridObject>,
    pub box_object_positions: Vec<Vec<usize>>,
}

impl Board {
    pub fn new(level: Level) -> Self {
        let (height, width) = level.dim();

        // Create a grid of cells by coping the level onto the grid
        let mut grid = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(level[(y, x)]);
            }
            grid.push(row);
        }

        // list of boxes and goals
        let mut boxes = Vec::new();
        let mut goals = Vec::new();

        // for each position in the grid, if there is a box, save the index of the box from the boxes vector
        let mut box_object_positions = vec![vec![usize::MAX; width]; height];

        for y in 0..height {
            for x in 0..width {
                let cell = grid[y][x];
                if cell == Cell::Box || cell == Cell::BoxOnGoal {
                    boxes.push(GridObject { pos: (x, y), cell });
                    box_object_positions[y][x] = boxes.len() - 1;
                }
                if cell == Cell::Goal || cell == Cell::PlayerOnGoal || cell == Cell::BoxOnGoal {
                    goals.push(GridObject { pos: (x, y), cell });
                }
            }
        }
        Self {
            grid,
            height,
            width,
            boxes,
            goals,
            box_object_positions,
        }
    }

    pub fn get_one_line_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.grid[y][x].to_char());
            }
            //s.push('\n');
        }
        s
    }
}
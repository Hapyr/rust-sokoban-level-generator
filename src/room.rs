use crate::cell::Cell;
use ndarray::Array2;
use rand::Rng;

/// A template is a 5x5 array of cells.
/// It composes parts of a level and is randomly rotated.
type Template = [[Cell; 5]; 5];

/// Represents a randomly rotated template that is
/// used as a room inside the level.
pub(crate) type Room = Array2<Cell>;

/// Returns a randomly rotated template as a room.
pub(crate) fn get_random_room() -> Room {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..TEMPLATES.len());
    let mut template = TEMPLATES[index];

    let rotation = rng.gen_range(0..4);

    // TODO: maybe optimize this.
    for _ in 0..rotation {
        template = rotate_template(template);
    }

    Room::from(template.to_vec())
}

fn rotate_template(template: Template) -> Template {
    let mut new_template = [[Cell::Empty; 5]; 5];

    // transpose the template
    for y in 0..5 {
        for x in y..5 {
            new_template[y][x] = template[x][y];
            new_template[x][y] = template[y][x];
        }
    }

    // reverse the rows
    for y in 0..5 {
        new_template[y].reverse();
    }

    new_template
}

const TEMPLATES: [Template; 17] = [
    TEMPLATE_1,
    TEMPLATE_2,
    TEMPLATE_3,
    TEMPLATE_4,
    TEMPLATE_5,
    TEMPLATE_6,
    TEMPLATE_7,
    TEMPLATE_8,
    TEMPLATE_9,
    TEMPLATE_10,
    TEMPLATE_11,
    TEMPLATE_12,
    TEMPLATE_13,
    TEMPLATE_14,
    TEMPLATE_15,
    TEMPLATE_16,
    TEMPLATE_17,
];

const TEMPLATE_1: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_2: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_3: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_4: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_5: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_6: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Wall,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_7: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_8: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Wall,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_9: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Wall,
        Cell::Empty,
    ],
    [
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Wall,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
        Cell::Empty,
    ],
];

// This template is the exception for the
// connectivity check.
const TEMPLATE_10: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Wall,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::SpecialFloor,
        Cell::Floor,
        Cell::Floor,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_11: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_12: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_13: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_14: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];

const TEMPLATE_15: Template = [
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Wall,
        Cell::Floor,
        Cell::Wall,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
        Cell::Floor,
        Cell::Empty,
    ],
];

const TEMPLATE_16: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
];

const TEMPLATE_17: Template = [
    [
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
    [
        Cell::Floor,
        Cell::Floor,
        Cell::Wall,
        Cell::Floor,
        Cell::Floor,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
    ],
    [
        Cell::Empty,
        Cell::Floor,
        Cell::Floor,
        Cell::Empty,
        Cell::Empty,
    ],
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::Cell;

    #[test]
    fn rotate_90_degs() {
        let tpl = TEMPLATE_4;

        const EXPECTED: Template = [
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Floor,
                Cell::Floor,
                Cell::Wall,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Floor,
                Cell::Floor,
                Cell::Wall,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Floor,
                Cell::Floor,
                Cell::Wall,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ];

        let result = rotate_template(tpl);

        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn rotate_180_degs() {
        let tpl = TEMPLATE_4;

        const EXPECTED: Template = [
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Floor,
                Cell::Floor,
                Cell::Floor,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Floor,
                Cell::Floor,
                Cell::Floor,
                Cell::Empty,
            ],
            [Cell::Empty, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Empty],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ];

        let result = rotate_template(rotate_template(tpl));

        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn rotate_270_degs() {
        let tpl = TEMPLATE_4;

        const EXPECTED: Template = [
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Wall,
                Cell::Floor,
                Cell::Floor,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Wall,
                Cell::Floor,
                Cell::Floor,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Wall,
                Cell::Floor,
                Cell::Floor,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ];

        let result = rotate_template(rotate_template(rotate_template(tpl)));

        assert_eq!(result, EXPECTED);
    }
}

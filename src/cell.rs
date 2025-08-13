#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Cell {
    Empty,
    Wall,
    Player,
    PlayerOnGoal,
    Box,
    BoxOnGoal,
    Goal,
    Floor,
    SpecialFloor,
}

impl Cell {
    pub(crate) fn to_char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Box => '$',
            Self::BoxOnGoal => '*',
            Self::Goal => '.',
            Self::Player => '@',
            Self::PlayerOnGoal => '+',
            Self::Empty | Self::Floor | Self::SpecialFloor => ' ',
        }
    }

    pub(crate) fn to_encoding_char(&self) -> char {
        match self {
            Self::Empty | Self::Floor | Self::SpecialFloor => '-',
            _ => self.to_char(),
        }
    }

    pub(crate) fn is_floor(&self) -> bool {
        match self {
            Self::Floor | Self::SpecialFloor => true,
            _ => false,
        }
    }

    pub(crate) fn is_box(&self) -> bool {
        match self {
            Self::Box | Self::BoxOnGoal => true,
            _ => false,
        }
    }

    pub(crate) fn is_walkable(&self) -> bool {
        match self {
            Self::Floor | Self::SpecialFloor | Self::Goal => true,
            _ => false,
        }
    }
}

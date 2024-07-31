use ratatui::style::Color;

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum RailShape {
    Empty = 0,
    Horizontal = 1,
    Vertical = 2,
    TurnTopRight = 3,
    TurnBottomRight = 4,
    TurnBottomLeft = 5,
    TurnTopLeft = 6,
    TTop = 7,
    TRight = 8,
    TBottom = 9,
    TLeft = 10,
    Cross = 11,
    EndTop = 12,
    EndRight = 13,
    EndBottom = 14,
    EndLeft = 15,
}

#[derive(Copy, Clone)]
pub struct MapTile {
    shape: RailShape,
}

impl From<RailShape> for MapTile {
    fn from(value: RailShape) -> Self {
        Self { shape: value }
    }
}

impl From<RailShape> for char {
    fn from(value: RailShape) -> Self {
        match value {
            RailShape::Empty => ' ',
            RailShape::Horizontal => '─',
            RailShape::Vertical => '│',
            RailShape::TurnTopRight => '╰',
            RailShape::TurnBottomRight => '╭',
            RailShape::TurnBottomLeft => '╮',
            RailShape::TurnTopLeft => '╯',
            RailShape::TTop => '┴',
            RailShape::TRight => '├',
            RailShape::TBottom => '┬',
            RailShape::TLeft => '┤',
            RailShape::Cross => '┼',
            RailShape::EndTop => '╵',
            RailShape::EndRight => '╶',
            RailShape::EndBottom => '╷',
            RailShape::EndLeft => '╴',
        }
    }
}

impl RailShape {
    pub fn get_train_char(&self) -> char {
        match self {
            RailShape::Empty => ' ',
            RailShape::Horizontal => '═',
            RailShape::Vertical => '║',
            RailShape::TurnTopRight => '╚',
            RailShape::TurnBottomRight => '╔',
            RailShape::TurnBottomLeft => '╗',
            RailShape::TurnTopLeft => '╝',
            RailShape::TTop => '╩',
            RailShape::TRight => '╠',
            RailShape::TBottom => '╦',
            RailShape::TLeft => '╣',
            RailShape::Cross => '╬',
            RailShape::EndTop => '╨',
            RailShape::EndRight => '╞',
            RailShape::EndBottom => '╥',
            RailShape::EndLeft => '╡',
        }
    }
}

impl MapTile {
    pub fn get_train_char(&self) -> char {
        self.shape.get_train_char()
    }

    pub fn shape(&self) -> RailShape {
        self.shape
    }
}

impl From<MapTile> for char {
    fn from(value: MapTile) -> Self {
        char::from(value.shape)
    }
}

impl From<MapTile> for Color {
    fn from(_value: MapTile) -> Self {
        Color::Rgb(128, 128, 128)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonType {
    Pad { x: u8, y: u8 },
    Master(u8),
    Arrow(Direction),
    Mode(ModeType),
    Parameter(ParameterType),
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum ModeType {
    Clip = 0,
    One,
    Two,
    Set,
}

#[derive(Clone, Copy, Debug)]
pub enum ParameterType {
    Volume = 0,
    SendA,
    SendB,
    Pan,
    Control1,
    Control2,
    Control3,
    Control4,
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonEventEdge {
    PosEdge,
    NegEdge,
}

#[derive(Clone, Copy, Debug)]
pub struct ButtonEvent {
    pub btn: ButtonType,
    pub event: ButtonEventEdge,
}

static PARAMETER_TYPES: [ParameterType; 8] = [
    ParameterType::Volume,
    ParameterType::SendA,
    ParameterType::SendB,
    ParameterType::Pan,
    ParameterType::Control1,
    ParameterType::Control2,
    ParameterType::Control3,
    ParameterType::Control4,
];

static DIRECTION_TYPES: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

static MODE_TYPES: [ModeType; 4] = [ModeType::Clip, ModeType::One, ModeType::Two, ModeType::Set];

impl ButtonEvent {
    pub fn new(row: u8, col: u8, event: ButtonEventEdge) -> ButtonEvent {
        let btn = match col {
            0..=7 => {
                let x = row;
                let y = col;

                ButtonType::Pad { x, y }
            }
            8 => ButtonType::Master(row + 1),
            9 => match row {
                0..=3 => ButtonType::Arrow(DIRECTION_TYPES[row as usize]),
                4..=7 => ButtonType::Mode(MODE_TYPES[row as usize - 4]),
                _ => panic!("This should never happen!"),
            },
            10 => ButtonType::Parameter(PARAMETER_TYPES[row as usize]),
            _ => panic!("This should never happen!"),
        };

        ButtonEvent { btn, event }
    }
}

pub struct LedEvent {
    btn: ButtonType,
    event: LedEventType,
}

#[derive(Clone, Copy, Debug)]
pub enum LedEventType {
    Switch(bool),
    SwitchColor { r: bool, g: bool, b: bool },
}

impl LedEvent {
    pub fn new(btn: ButtonType, event: LedEventType) -> LedEvent {
        LedEvent { btn, event }
    }

    pub fn apply_to_banks(&self, banks: [u32; 8]) -> [u32; 8] {
        let mut new_banks = banks;

        match (self.event, self.btn) {
            (LedEventType::Switch(s), ButtonType::Master(i)) => {
                let bank = 7;
                let bit = 32 - i;

                if s {
                    new_banks[bank] |= 1 << bit;
                } else {
                    new_banks[bank] &= !(1 << bit);
                }
            }
            (LedEventType::Switch(s), ButtonType::Arrow(d)) => {
                let bank = 6;
                let bit = 31 - d as u8;

                if s {
                    new_banks[bank] |= 1 << bit;
                } else {
                    new_banks[bank] &= !(1 << bit);
                }
            }
            (LedEventType::Switch(s), ButtonType::Mode(m)) => {
                let bank = 6;
                let bit = 27 - m as u8;

                if s {
                    new_banks[bank] |= 1 << bit;
                } else {
                    new_banks[bank] &= !(1 << bit);
                }
            }
            (LedEventType::Switch(s), ButtonType::Parameter(p)) => {
                let bank = 6;
                let bit = 23 - p as u8;

                if s {
                    new_banks[bank] |= 1 << bit;
                } else {
                    new_banks[bank] &= !(1 << bit);
                }
            }
            (LedEventType::SwitchColor { r, g, b }, ButtonType::Pad { x, y }) => {
                let (bank_r, bank_g, bank_b) = if y < 4 { (1, 2, 0) } else { (4, 5, 3) };

                let bit = 31 - (((y % 4) * 8) + x);

                if r {
                    new_banks[bank_r] |= 1 << bit;
                } else {
                    new_banks[bank_r] &= !(1 << bit);
                }

                if g {
                    new_banks[bank_g] |= 1 << bit;
                } else {
                    new_banks[bank_g] &= !(1 << bit);
                }

                if b {
                    new_banks[bank_b] |= 1 << bit;
                } else {
                    new_banks[bank_b] &= !(1 << bit);
                }
            }
            _ => panic!("Invalid LED and button types!"),
        };

        new_banks
    }
}
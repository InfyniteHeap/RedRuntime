use super::*;

#[derive(Default)]
pub enum ComparatorMode {
    #[default]
    Compare,
    Subtract,
}

impl ComparatorMode {
    pub fn get_mode(id: &str) -> Self {
        match id {
            "compare" => Self::Compare,
            "subtract" => Self::Subtract,
            _ => panic!("Invaild comparator mode!"),
        }
    }

    pub fn get_id(self) -> &'static str {
        match self {
            Self::Compare => "compare",
            Self::Subtract => "subtract",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Compare => Self::Subtract,
            Self::Subtract => Self::Compare,
        }
    }
}

pub struct RedstoneComparator {
    pub facing: BlockDirection,
    pub mode: ComparatorMode,
    pub powered: bool,
}

impl RedstoneComparator {
    pub fn new(facing: BlockDirection, mode: ComparatorMode, powered: bool) -> Self {
        Self {
            facing,
            mode,
            powered,
        }
    }
}

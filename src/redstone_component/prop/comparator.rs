use super::*;

#[derive(Default)]
enum ComparatorMode {
    #[default]
    Compare,
    Subtract,
}

impl ComparatorMode {
    pub(crate) fn get_mode(id: &str) -> Self {
        match id {
            "compare" => Self::Compare,
            "subtract" => Self::Subtract,
            _ => panic!("Invaild comparator mode!"),
        }
    }

    pub(crate) fn get_id(self) -> &'static str {
        match self {
            Self::Compare => "compare",
            Self::Subtract => "subtract",
        }
    }

    pub(crate) fn toggle(self) -> Self {
        match self {
            Self::Compare => Self::Subtract,
            Self::Subtract => Self::Compare,
        }
    }
}

struct RedstoneComparator {
    facing: BlockDirection,
    mode: ComparatorMode,
    powered: bool,
}

impl RedstoneComparator {
    pub(crate) fn new(facing: BlockDirection, mode: ComparatorMode, powered: bool) -> Self {
        Self {
            facing,
            mode,
            powered,
        }
    }
}

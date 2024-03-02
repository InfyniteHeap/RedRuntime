use super::*;

pub(crate) struct Wire {
    pub(crate) north_connection: bool,
    pub(crate) east_connection: bool,
    pub(crate) south_connection: bool,
    pub(crate) west_connection: bool,

    pub(crate) upper_extended: WireConnectionDeirection,
}

pub(crate) enum WireConnectionDeirection {
    North,
    East,
    South,
    West,
    None,
}

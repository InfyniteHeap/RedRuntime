pub mod prop;

// RedRuntime sorts each block and pushs into specific vector, all according to these traits.
pub(crate) trait SignalSource {}
pub(crate) trait Capacitor {}
pub(crate) trait PoweredBlock {}
pub(crate) trait Container {}
pub(crate) trait Conductor {}

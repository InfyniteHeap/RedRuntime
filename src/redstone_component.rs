pub mod prop;

// Block that was tagged with this trait will be processed by RedRuntime, or will be discarded.
/// A tag that assists to help `RedRuntime` tell whether a block should be processed.
pub trait RedstoneComponent {}

// RedRuntime sorts each block and pushs into specific vector, all according to these traits.
pub(crate) trait SignalSource: RedstoneComponent {}
pub(crate) trait Capacitor: RedstoneComponent {}
pub(crate) trait PoweredBlock: RedstoneComponent {}
pub(crate) trait Container: RedstoneComponent {}
pub(crate) trait Conductor: RedstoneComponent {}

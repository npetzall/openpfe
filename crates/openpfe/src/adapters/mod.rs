//! Adapters for workspace crates (`openpfe-ipc`, detached spawn).

pub mod ipc;
pub mod mock;
pub mod spawn;

pub use ipc::IpcProjectControl;
pub use spawn::ReExecSpawn;

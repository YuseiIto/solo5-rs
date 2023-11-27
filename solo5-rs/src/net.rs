/// Network Protocol Stack.
///
/// Here is the implementation of the protocol stack.
/// The core of the protocol stack utilizes smoltcp, and here, It is implemented only the hardware abstraction layer for smoltcp.

/// The essential types and utiliy functions are defined at commmon.
pub mod raw;
pub mod smoltcp;

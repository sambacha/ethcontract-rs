#![deny(missing_docs)]

//! TODO(nlordell): documentation with examples.

pub mod contract;
pub mod errors;
mod future;
pub mod sign;
pub mod transaction;
pub mod transport;

pub use ethcontract_common::truffle;
pub use ethcontract_derive::contract;
pub use serde_json as json;
pub use web3;

pub use crate::contract::Instance;
pub use crate::transport::DynTransport;

/// Type alias for a contract `Instance` with an underyling `DynTransport`.
pub type DynInstance = Instance<DynTransport>;

#[doc(hidden)]
pub mod foreign {
    //! Foreign types that we re-export to be used internally by the procedural
    //! macro but do not appear on public interfaces.

    pub use lazy_static::lazy_static;
}

#[cfg(test)]
#[allow(missing_docs)]
mod test {
    pub mod macros;
    pub mod prelude;
    pub mod transport;
}

#[cfg(feature = "samples")]
#[allow(missing_docs)]
pub mod samples {
    //! Samples of derived contracts for documentation purposes in roder to
    //! illustrate what the generated API. This module should not be used and is
    //! should only be included when generating documentation.

    crate::contract!(
        "examples/truffle/build/contracts/DocumentedContract.json",
        crate = crate
    );
    crate::contract!(
        "examples/truffle/build/contracts/SimpleLibrary.json",
        crate = crate
    );
    crate::contract!(
        "examples/truffle/build/contracts/LinkedContract.json",
        crate = crate
    );
    crate::contract!(
        "examples/truffle/build/contracts/IERC20.json",
        crate = crate
    );
}

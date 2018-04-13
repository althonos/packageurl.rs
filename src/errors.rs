//! Error type and other helpers using the [`error-chain`] crate.
//!
//! [`error-chain`]: https://docs.rs/error-chain/

error_chain! {
    errors {
        MissingName {
            description("missing name")
            display("missing scheme")
        }
        MissingScheme {
            description("missing scheme")
            display("missing scheme")
        }
    }
}

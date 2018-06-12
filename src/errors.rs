//! Error type and other helpers using the [`error-chain`] crate.
//!
//! [`error-chain`]: https://docs.rs/error-chain/

error_chain! {
    errors {
        InvalidScheme(s: String) {
            description("invalid scheme")
            display("invalid scheme: '{}'", s)
        }
        MissingName {
            description("missing name")
            display("missing name")
        }
        MissingScheme {
            description("missing scheme")
            display("missing scheme")
        }
        MissingType {
            description("missing type")
            display("missing type")
        }
    }
}

//! Procedural Macros

use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}

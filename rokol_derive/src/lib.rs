mod layout;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LayoutDesc)]
pub fn layout_desc(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(layout::impl_vertex_layout(ast))
}

use {proc_macro::TokenStream, proc_macro2::TokenStream as TokenStream2, quote::*, syn::*};

pub fn impl_vertex_layout(ast: DeriveInput) -> TokenStream {
    let ty_name = &ast.ident;

    let input = match ast.data {
        Data::Struct(ref data) => data,
        _ => panic!("`VertexLayout` is for structs"),
    };

    // force `#[repr(C)]`
    // ast.attrs.iter().any(|a|

    let fields = match input.fields {
        Fields::Named(ref fields) => fields,
        _ => unimplemented!("Fiel`VertexLayout` is only for struct with named fields"),
    };

    let f: syn::Type = syn::parse_str("f32").unwrap();
    let f2: syn::Type = syn::parse_str("[f32; 2]").unwrap();
    let f3: syn::Type = syn::parse_str("[f32; 3]").unwrap();
    let f4: syn::Type = syn::parse_str("[f32; 4]").unwrap();
    let u4: syn::Type = syn::parse_str("[u8; 4]").unwrap();

    let formats = fields.named.iter().map(|field| {
        match &field.ty {
            ty if *ty == f => quote! { rg::VertexFormat::Float },
            ty if *ty == f2 => quote! { rg::VertexFormat::Float2 },
            ty if *ty == f3 => quote! { rg::VertexFormat::Float3 },
            ty if *ty == f4 => quote! { rg::VertexFormat::Float4 },
            ty if *ty == u4 => quote! { rg::VertexFormat::UByte4N },
            // TODO: support more types?
            _ => {
                // get the field type name (as tokens)
                let mut field_ty_tokens = TokenStream2::new();
                field.ty.to_tokens(&mut field_ty_tokens);

                panic!(
                    "Field `{}: {}` of type `{}` has unsupported type by `VertexLayout`",
                    field.ident.as_ref().unwrap(),
                    field_ty_tokens,
                    ty_name,
                )
            }
        }
    });

    let i = 0usize..fields.named.len();
    let gen_desc = quote! {
        let mut desc = rokol::gfx::LayoutDesc::default();
        #(
            desc.attrs[#i].format = #formats as u32;
        )*
        desc
    };

    TokenStream::from(quote! {
        impl #ty_name {
            pub const fn layout_desc() -> rokol::gfx::LayoutDesc {
                #gen_desc
            }
        }
    })
}

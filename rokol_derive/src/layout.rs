use {proc_macro::TokenStream, quote::*, syn::*};

// implements `fn layout_desc`
pub fn impl_vertex_layout(ast: DeriveInput) -> TokenStream {
    let ty_name = &ast.ident;

    let input = match ast.data {
        Data::Struct(ref data) => data,
        _ => panic!("`#[derive(VertexLayout)]` is for structs"),
    };

    // force `#[repr(C)]`
    let repr: syn::Attribute = parse_quote!(#[repr(C)]);
    assert!(
        ast.attrs.iter().any(|a| *a == repr),
        "`#[repr(C)]` is required to derive `VertexLayout`"
    );

    let fields = match input.fields {
        Fields::Named(ref fields) => fields,
        _ => unimplemented!("`#[derive(VertexLayout)]` is only for struct with named fields"),
    };

    let format_decls = [
        ("f32", quote! { rg::VertexFormat::Float }),
        ("[f32; 2]", quote! { rg::VertexFormat::Float2 }),
        ("[f32; 3]", quote! { rg::VertexFormat::Float3 }),
        ("[f32; 4]", quote! { rg::VertexFormat::Float4 }),
        ("[u8; 4]", quote! { rg::VertexFormat::UByte4N }),
    ];

    let format_defs = format_decls
        .iter()
        .map(|(s, quote)| (syn::parse_str::<syn::Type>(s).unwrap(), quote));

    let formats = fields.named.iter().map(|field| {
        format_defs
            .clone()
            .find_map(|(ty, tokens)| if field.ty == ty { Some(tokens) } else { None })
            .unwrap_or_else(|| {
                // not found from the list

                panic!(
                    "Field `{}: {}` of type `{}` has unsupported type by `#[derive(VertexLayout)]`",
                    field.ident.as_ref().unwrap(),
                    field.ty.to_token_stream(),
                    ty_name,
                )
            })
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
            pub fn layout_desc() -> rokol::gfx::LayoutDesc {
                #gen_desc
            }
        }
    })
}

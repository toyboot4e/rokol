use {proc_macro2::TokenStream as TokenStream2, quote::*, syn::*};

// implements `fn layout_desc`
pub fn impl_vertex_layout(ast: DeriveInput) -> TokenStream2 {
    let ty_name = &ast.ident;

    let input = match ast.data {
        Data::Struct(ref data) => data,
        _ => panic!("`#[derive(LayoutDesc)]` is only for structs"),
    };

    // force `#[repr(C)]`
    let repr: syn::Attribute = parse_quote!(#[repr(C)]);
    assert!(
        ast.attrs.iter().any(|a| *a == repr),
        "`#[repr(C)]` is required to derive `LayoutDesc`"
    );

    let fields = match input.fields {
        Fields::Named(ref fields) => fields,
        _ => unimplemented!("`#[derive(LayoutDesc)]` is only for struct with named fields"),
    };

    let format_decls = [
        (
            "f32",
            quote! {
                rokol::ffi::gfx::sg_vertex_format::SG_VERTEXFORMAT_FLOAT
            },
        ),
        (
            "[f32; 2]",
            quote! {
                rokol::ffi::gfx::sg_vertex_format::SG_VERTEXFORMAT_FLOAT2
            },
        ),
        (
            "[f32; 3]",
            quote! {
                rokol::ffi::gfx::sg_vertex_format::SG_VERTEXFORMAT_FLOAT3
            },
        ),
        (
            "[f32; 4]",
            quote! {
                rokol::ffi::gfx::sg_vertex_format::SG_VERTEXFORMAT_FLOAT4
            },
        ),
        (
            "[u8; 4]",
            quote! {
                rokol::ffi::gfx::sg_vertex_format::SG_VERTEXFORMAT_UBYTE4N
            },
        ),
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
                    "Field `{}: {}` of type `{}` has unsupported type by `#[derive(LayoutDesc)]`",
                    field.ident.as_ref().unwrap(),
                    field.ty.to_token_stream(),
                    ty_name,
                )
            })
    });

    let i = 0usize..fields.named.len();

    quote! {
        impl #ty_name {
            pub fn layout_desc() -> rokol::gfx::LayoutDesc {
                let mut desc = rokol::gfx::LayoutDesc::default();
                #(
                    desc.attrs[#i].format = #formats;
                )*
                desc
            }
        }
    }
}

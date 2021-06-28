
pub fn get_fty(num_bits: usize) -> syn::Ident {
    quote::format_ident!("f{}", num_bits)
}

pub fn get_uty(num_bits: usize) -> syn::Ident {
    quote::format_ident!("u{}", num_bits)
}

pub fn get_one(num_bits: usize) -> proc_macro2::TokenStream {
    if num_bits == 32 { quote::quote!(0x3f800000_u32) } else { quote::quote!(0x3ff0000000000000_u64) }
}

pub fn get_escale(num_bits: usize) -> proc_macro2::TokenStream {
    if num_bits == 32 { quote::quote!(0x00800000_u32) } else { quote::quote!(0x0010000000000000_u64) }
}

pub fn get_suffix(num_bits: usize) -> String {
    format!("f{}", num_bits)
}

pub fn get_quadrant_terms(num_bits: usize) -> usize {
    if num_bits == 32 { 8 } else { 12 }
}

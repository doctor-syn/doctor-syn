pub fn get_fty(num_bits: usize) -> syn::Ident {
    quote::format_ident!("f{}", num_bits)
}

pub fn get_uty(num_bits: usize) -> syn::Ident {
    quote::format_ident!("u{}", num_bits)
}

pub fn get_ity(num_bits: usize) -> syn::Ident {
    quote::format_ident!("i{}", num_bits)
}

pub fn get_one(num_bits: usize) -> proc_macro2::TokenStream {
    if num_bits == 32 {
        quote::quote!(0x3f800000_u32)
    } else {
        quote::quote!(0x3ff0000000000000_u64)
    }
}

pub fn get_escale(num_bits: usize) -> proc_macro2::TokenStream {
    if num_bits == 32 {
        quote::quote!(0x00800000_u32)
    } else {
        quote::quote!(0x0010000000000000_u64)
    }
}

pub fn get_quadrant_terms(num_bits: usize) -> usize {
    if num_bits == 32 {
        8
    } else {
        24
    }
}

pub fn get_single_pass_terms(num_bits: usize) -> usize {
    if num_bits == 32 {
        16
    } else {
        24
    }
}

pub fn get_tan_terms(num_bits: usize) -> usize {
    if num_bits == 32 {
        16
    } else {
        24
    }
}

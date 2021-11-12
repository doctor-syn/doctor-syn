pub struct Config {
    functions: Vec<String>,
    num_bits: usize,
    number_type: String,
    language: String,
    generate_tests: bool,
    prefix: String,
}

impl Config {
    pub fn new(
        num_bits: usize,
        number_type: &str,
        language: &str,
        generate_tests: bool,
        prefix: &str,
    ) -> Self {
        Self {
            functions: Vec::new(),
            num_bits,
            number_type: number_type.into(),
            language: language.into(),
            generate_tests,
            prefix: prefix.into(),
        }
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits
    }

    pub fn number_type(&self) -> &str {
        self.number_type.as_str()
    }

    pub fn language(&self) -> &str {
        self.language.as_str()
    }

    pub fn generate_tests(&self) -> bool {
        self.generate_tests
    }

    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    pub fn enabled(&self, function_name: &str) -> bool {
        self.functions.contains(&function_name.into())
    }

    pub fn add_function(&mut self, function_name: &str) {
        self.functions.push(function_name.into());
    }

    pub fn get_fty(&self) -> syn::Ident {
        quote::format_ident!("f{}", self.num_bits())
    }
    
    pub fn get_uty(&self) -> syn::Ident {
        quote::format_ident!("u{}", self.num_bits())
    }
    
    pub fn get_ity(&self) -> syn::Ident {
        quote::format_ident!("i{}", self.num_bits())
    }
    
    // pub fn get_one(&self) -> proc_macro2::TokenStream {
    //     if self.num_bits() == 32 {
    //         let value = 0x3f800000_u32 as f32;
    //         quote::quote!(#value)
    //     } else {
    //         let value = 0x3ff0000000000000_u64 as f64;
    //         quote::quote!(#value)
    //     }
    // }
    
    // pub fn get_escale(&self) -> proc_macro2::TokenStream {
    //     if self.num_bits() == 32 {
    //         let value = 0x00800000_u32 as f32;
    //         quote::quote!(#value)
    //     } else {
    //         let value = 0x0010000000000000_u64 as f64;
    //         quote::quote!(#value)
    //     }
    // }
    
    pub fn get_one(&self) -> proc_macro2::TokenStream {
        if self.num_bits() == 32 {
            quote::quote!(0x3f800000_u32)
        } else {
            quote::quote!(0x3ff0000000000000_u64)
        }
    }
    
    pub fn get_escale(&self) -> proc_macro2::TokenStream {
        if self.num_bits() == 32 {
            quote::quote!(0x00800000_u32)
        } else {
            quote::quote!(0x0010000000000000_u64)
        }
    }
    
    pub fn get_shift(&self) -> proc_macro2::TokenStream {
        if self.num_bits() == 32 {
            quote::quote!(23)
        } else {
            quote::quote!(52)
        }
    }
    
    pub fn get_eoffset(&self) -> proc_macro2::TokenStream {
        if self.num_bits() == 32 {
            quote::quote!(0x7f)
        } else {
            quote::quote!(0x3ff)
        }
    }
    
    pub fn get_quadrant_terms(&self) -> usize {
        if self.num_bits() == 32 {
            8
        } else {
            24
        }
    }
    
    pub fn get_single_pass_terms(&self) -> usize {
        if self.num_bits() == 32 {
            16
        } else {
            24
        }
    }
    
    pub fn get_tan_terms(&self) -> usize {
        if self.num_bits() == 32 {
            16
        } else {
            24
        }
    }
}


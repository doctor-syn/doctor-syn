pub struct Config {
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

    pub fn num_digits(&self) -> i64 {
        if self.num_bits == 32 {
            20
        } else {
            40
        }
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
            quote::quote!(127)
        } else {
            quote::quote!(1023)
        }
    }
}

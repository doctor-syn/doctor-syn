use std::path::PathBuf;

pub struct Config {
    options: crate::Opt,
}

impl Config {
    pub fn new(
        options: crate::Opt,
    ) -> Self {
        Self {
            options,
        }
    }

    pub fn num_bits(&self) -> usize {
        self.options.num_bits
    }

    pub fn num_digits(&self) -> i64 {
        if self.num_bits() == 32 {
            20
        } else {
            40
        }
    }

    pub fn number_type(&self) -> &str {
        self.options.number_type.as_str()
    }

    pub fn language(&self) -> &str {
        self.options.language.as_str()
    }

    pub fn generate_tests(&self) -> bool {
        self.options.generate_tests
    }

    pub fn generate_plots(&self) -> bool {
        self.options.generate_plots
    }

    pub fn prefix(&self) -> &str {
        if self.language() == "c" {
            if self.num_bits() == 32 {
                "ds32_"
            } else {
                "ds64_"
            }
        } else {
            ""
        }
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

    pub fn output(&self) -> Option<&std::path::PathBuf> {
        self.options.output.as_ref()
    }
}

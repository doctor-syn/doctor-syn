use proc_macro2::{Delimiter, TokenStream, TokenTree};

fn indent(depth: usize) -> &'static str {
    let i = "                                                            ";
    &i[0..depth * 2]
}

fn append_token(text: &mut String, tt: &TokenTree, depth: usize) {
    if let TokenTree::Group(g) = &tt {
        match g.delimiter() {
            Delimiter::Parenthesis => {
                text.extend(tt.to_string().chars());
            }
            Delimiter::Brace => {
                text.extend("{\n".chars());
                text.extend(indent(depth + 1).chars());
                for tt in g.stream() {
                    append_token(text, &tt, depth + 1);
                }
                text.extend("\n".chars());
                text.extend(indent(depth).chars());
                text.extend("}".chars());
                text.extend("\n".chars());
                text.extend(indent(depth).chars());
                if depth == 0 {
                    text.extend("\n".chars());
                }
            }
            Delimiter::Bracket => {
                text.extend(tt.to_string().chars());
            }
            Delimiter::None => {
                text.extend(tt.to_string().chars());
            }
        }
    } else {
        let tok = tt.to_string();
        if tok == ";" {
            text.extend(tok.chars());
            text.extend("\n".chars());
            text.extend(indent(depth).chars());
        } else {
            text.extend(tok.chars());
            text.extend(" ".chars());
        }
    }
}

/// Pretty (ish) print a token stream.
pub fn format_token_stream(tokens: TokenStream) -> String {
    let mut text = String::new();
    for tt in tokens {
        append_token(&mut text, &tt, 0);
    }
    text
}

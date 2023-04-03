use proc_macro::TokenStream;

#[proc_macro]
pub fn non_existing_std_lint_name_from_proc_macro(_: TokenStream) -> TokenStream {
    // The same whether it's a (nonexisting) "clippy::"" lint, or a (nonexisting) lint with no prefix.
    "#[allow(clippy::non_existing_std_lint_from_proc_macro)] fn _f() {}"
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn non_existing_clippy_lint_name_from_proc_macro(_: TokenStream) -> TokenStream {
    // The same whether it's a (nonexisting) "clippy::"" lint, or a (nonexisting) lint with no prefix.
    "#[allow(clippy::non_existing_clippy_lint_from_proc_macro)] fn _ff() {}"
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn non_existing_lint_prefix_from_proc_macro(_: TokenStream) -> TokenStream {
    "#[allow(non_existing_lint_prefix_from_proc_macro::all)] fn _fff() {}"
        .parse()
        .unwrap()
}

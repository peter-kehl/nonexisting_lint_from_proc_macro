#![allow(unstable_features)]
#![feature(proc_macro_def_site)]
#![deny(unknown_lints)]
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

// The incorrect lint name `non_existing_std_lint_from_proc_macro` (in proc macro below) doesn't get
// reported, regardless of which `Span` we use:
fn span() -> Span {
    Span::call_site()
    //Span::mixed_site()
    //Span::def_site()
}

/// Consune, mutate (apply `f`), return.
fn with_mut<T, F: Fn(&mut T)>(mut t: T, f: F) -> T {
    f(&mut t);
    t
}

/// Consume, set span to `span()`, return.
fn with_span(t: TokenTree) -> TokenTree {
    with_mut(t, |t| t.set_span(span()))
}

#[proc_macro]
pub fn non_existing_std_lint_name_from_proc_macro(_: TokenStream) -> TokenStream {
    /*if false {
        //return "#[deny(unknown_lints)] #[allow(non_existing_std_lint_from_proc_macro)] fn _f() {}".parse().unwrap();
        return "#[allow(non_existing_std_lint_from_proc_macro)] fn _f() {}"
            .parse()
            .unwrap();
    }*/

    // If span() returns Span::call_site(), then (all?) the following `with_span` calls are most
    // likely redundant, and some `.set_span(span())` calls are also redundant.
    let mut hash = Punct::new('#', Spacing::Joint);
    hash.set_span(span());

    let mut group_parens_lint_name = Group::new(
        Delimiter::Parenthesis,
        TokenStream::from(with_span(TokenTree::Ident(Ident::new(
            "non_existing_std_lint_from_proc_macro",
            span(),
        )))),
    );
    group_parens_lint_name.set_span(span());

    let mut group_bracket = Group::new(
        Delimiter::Bracket,
        TokenStream::from_iter([
            with_span(TokenTree::Ident(Ident::new("allow", span()))),
            with_span(TokenTree::Group(group_parens_lint_name)),
        ]),
    );
    group_bracket.set_span(span());

    let mut group_parens = Group::new(Delimiter::Parenthesis, TokenStream::new());
    group_parens.set_span(span());

    let mut group_braces = Group::new(Delimiter::Brace, TokenStream::new());
    group_braces.set_span(span());

    let stream = TokenStream::from_iter([
        with_span(TokenTree::Punct(hash)),
        with_span(TokenTree::Group(group_bracket)),
        with_span(TokenTree::Ident(Ident::new("fn", span()))),
        with_span(TokenTree::Ident(Ident::new("_f", span()))),
        with_span(TokenTree::Group(group_parens)),
        with_span(TokenTree::Group(group_braces)),
    ]);
    eprintln!("Output: {:?}", stream);
    stream
}

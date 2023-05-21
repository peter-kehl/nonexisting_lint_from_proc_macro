#![forbid(unknown_lints)]

fn _macro_rules_composing_through_proc_macro_doesnt_report_non_existing_lint() {
    #![forbid(unknown_lints)]
    // The following incorrect "standard" (prefixless) lint doesn't get reported at all.
    proc_mac::non_existing_std_lint_name_from_proc_macro!();
}

fn _paste_proc_macro_does_report_non_existing_lint() {
    #![forbid(unknown_lints)]
    // The following incorrect "standard" (prefixless) lint DOES get reported.
    ::paste::paste! {
        #[allow([<non_existing_std_lint_name_from_paste>])]
        const _C: () = ();
    }
}

fn _paste_proc_macro_composed_does_report_non_existing_lint() {
    #![forbid(unknown_lints)]
    // The following incorrect "standard" (prefixless) lint DOES get reported.
    ::paste::paste! {
        #[allow([<non_existing_ std_ lint_ name_ composed_ with_ paste>])]
        const _C: () = ();
    }
}

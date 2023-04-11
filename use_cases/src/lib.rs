#![deny(unknown_lints)]

fn _macro_rules_composing_through_proc_macro_doesnt_report_non_existing_lint() {
    #![deny(unknown_lints)]
    // The following incorrect "standard" (prefixless) lint doesn't get reported at all.
    proc_mac::non_existing_std_lint_name_from_proc_macro!();
}

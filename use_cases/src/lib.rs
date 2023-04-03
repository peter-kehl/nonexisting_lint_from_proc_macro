macro_rules! pure_macro_rules_simply_allow {
    ($lint_path:path, $code:block) => {
        #[allow($lint_path)]
        $code
    };
}

fn _pure_macro_rules_reports_non_existing_lints_fine() {
    // The following gets reported
    // - only by `cargo clippy --all`
    // - but NOT by `cargo check --all`
    // Ok. So be it.
    pure_macro_rules_simply_allow!(
        clippy::non_existing_lint_here_gets_reported_and_we_are_happy,
        {}
    );

    // The following gets reported by both
    // - `cargo check --all`
    // - `cargo clippy --all`
    pure_macro_rules_simply_allow!(std_non_existing_lint_here_gets_reported_and_we_are_happy, {
    });
    
    // The following gets reported by NEITHER
    // - `cargo check --all`
    // - `cargo clippy --all`
    pure_macro_rules_simply_allow!(
        rustdoc::non_existing_lint_here_gets_reported_and_we_are_happy,
        {}
    );
}

fn _macro_rules_composing_through_proc_macro_doesnt_report_non_existing_lint() {
    // The following incorrect "standard" (prefixless) lint doesn't get reported at all.
    proc_mac::non_existing_std_lint_name_from_proc_macro!();

    // The following incorrect clippy (prefixless) lint doesn't get reported at all.
    proc_mac::non_existing_clippy_lint_name_from_proc_macro!();

    // The following incorrect lint PREFIX does get reported.
    proc_mac::non_existing_lint_prefix_from_proc_macro!();
}

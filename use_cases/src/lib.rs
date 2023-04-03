macro_rules! pure_macro_rules_simply_allow {
    ($lint_path:path, $code:block) => {
        #[allow($lint_path)]
        $code
    };
}

fn _pure_macro_rules_reports_non_existing_lints_fine() {
    //pure_macro_rules_simply_allow!(clippy::non_existing_lint_here_gets_reported_and_we_are_happy, {});
    pure_macro_rules_simply_allow!(std_non_existing_lint_from_proc_macro, {});
}

fn _macro_rules_composing_through_proc_macro_doesnt_report_non_existing_lint() {
    proc_mac::non_existing_std_lint_name_from_proc_macro!();

    proc_mac::non_existing_clippy_lint_name_from_proc_macro!();

    proc_mac::non_existing_lint_prefix_from_proc_macro!();
}

pub const STEP_BUTTON: &str = "Step";

pub const fn auto_button(auto: bool) -> &'static str {
    if auto {
        "Auto: True"
    } else {
        "Auto: False"
    }
}

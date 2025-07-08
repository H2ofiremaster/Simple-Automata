use vizia::{
    binding::{Lens, ResGet},
    context::Context,
    view::{Handle, View},
};

use crate::ruleset::{Rule, Ruleset};

pub struct RuleDisplay<L: Lens<Target = Ruleset>> {
    ruleset: L,
    index: usize,
}
impl<L: Lens<Target = Ruleset>> RuleDisplay<L> {}
impl<L1: Lens<Target = Ruleset>> View for RuleDisplay<L1> {}

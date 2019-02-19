//! Mutator for boolean literals.

use crate::MutagenRuntimeConfig;

pub struct MutatorLitBool {
    mutator_id: u32,
    original_lit: bool,
}

impl MutatorLitBool {
    pub fn new(mutator_id: u32, original_lit: bool) -> MutatorLitBool {
        MutatorLitBool {
            mutator_id,
            original_lit,
        }
    }

    pub fn run_mutator(self, runtime: &MutagenRuntimeConfig) -> bool {
        if runtime.mutation_id != self.mutator_id {
            self.original_lit
        } else {
            !self.original_lit
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::MutagenRuntimeConfig;

    #[test]
    pub fn mutator_lit_bool_false_inactive() {
        let mutator = MutatorLitBool::new(1, false);
        let result = mutator.run_mutator(&MutagenRuntimeConfig::with_mutation_id(0));
        assert_eq!(result, false)
    }
    #[test]
    pub fn mutator_lit_bool_true_inactive() {
        let mutator = MutatorLitBool::new(1, true);
        let result = mutator.run_mutator(&MutagenRuntimeConfig::with_mutation_id(0));
        assert_eq!(result, true)
    }
    #[test]
    pub fn mutator_lit_bool_false_active() {
        let mutator = MutatorLitBool::new(1, false);
        let result = mutator.run_mutator(&MutagenRuntimeConfig::with_mutation_id(1));
        assert_eq!(result, true)
    }
    #[test]
    pub fn mutator_lit_bool_true_active() {
        let mutator = MutatorLitBool::new(1, true);
        let result = mutator.run_mutator(&MutagenRuntimeConfig::with_mutation_id(1));
        assert_eq!(result, false)
    }
}
use crate::domain::RANDOM;
use crate::repr::feature::Feature;
use crate::mixins::verifiable::Verifiable;

pub trait Gene<T, G>: Feature<T, G> + Verifiable
where
    G: Gene<T, G>,
{
    fn generate(&self, value: &T, rng: &mut impl rand::Rng) -> T;

    fn mutate(&self) -> G {
        let new_value = self.generate(self.get_value(), &mut *RANDOM.lock().unwrap());
        self.duplicate_with_value(new_value)
    }

    fn flatten(&self) -> Vec<&T> {
        vec![self.get_value()]
    }
}

#[derive(Clone, Debug)]
struct SimpleGene {
    value: i32,
    is_valid: bool,
}

impl Feature<i32, SimpleGene> for SimpleGene {
    fn get_value(&self) -> &i32 {
        &self.value
    }

    fn duplicate_with_value(&self, value: i32) -> SimpleGene {
        SimpleGene {
            value,
            is_valid: self.is_valid,
        }
    }
}

impl Verifiable for SimpleGene {
    fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl Gene<i32, SimpleGene> for SimpleGene {
    fn generate(&self, value: &i32, _rng: &mut impl rand::Rng) -> i32 {
        value + 1
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        fn gene_can_be_mutated(gene in simple_gene_default_strategy()) {
            let mutated_gene = gene.mutate();
            assert_ne!(gene.get_value(), mutated_gene.get_value());
        }

        #[test]
        fn gene_can_be_flattened(gene in simple_gene_default_strategy()) {
            let flattened = gene.flatten();
            assert_eq!(flattened.len(), 1);
            assert_eq!(flattened[0], gene.get_value());
        }
    }

    fn simple_gene_strategy(
        is_valid: impl Strategy<Value=bool> + 'static
    ) -> impl Strategy<Value=SimpleGene> {
        (any::<i32>(), is_valid)
            .prop_map(|(value, is_valid)| SimpleGene { value, is_valid })
    }

    fn simple_gene_default_strategy() -> impl Strategy<Value=SimpleGene> {
        simple_gene_strategy(Just(true))
    }
}

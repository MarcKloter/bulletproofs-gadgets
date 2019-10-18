use bulletproofs::r1cs::{ConstraintSystem, Variable, LinearCombination};
use curve25519_dalek::scalar::Scalar;
use gadget::Gadget;

// Gadget proving equality LEFT = RIGHT, where LEFT is a witness and RIGHT can be either a witness or instance variable
pub struct Equality {
    right_hand: Vec<LinearCombination>
}

impl Gadget for Equality {
    fn preprocess(&self, _: &Vec<Scalar>) -> Vec<Scalar> {
        Vec::new()
    }

    fn assemble(
        &self, 
        cs: &mut ConstraintSystem, 
        left_hand: &Vec<Variable>, 
        _: &Vec<(Option<Scalar>, Variable)>
    ) {
        assert!(self.right_hand.len() == left_hand.len(), "left and right hand side are not the same length");

        for i in 0..left_hand.len() {
            let right_lc : LinearCombination = self.right_hand.get(i).unwrap().clone();
            let left_lc : LinearCombination = (*left_hand.get(i).unwrap()).into();

            // constrain left - right = 0 <=> left = right
            cs.constrain(left_lc - right_lc);
        }
    }
}

impl Equality {
    pub fn new(right_hand: Vec<LinearCombination>) -> Equality {
        Equality {
            right_hand: right_hand
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use commitments::{commit, verifier_commit};
    use bulletproofs::{BulletproofGens, PedersenGens};
    use merlin::Transcript;
    use bulletproofs::r1cs::{Prover, Verifier};
    use conversions::{be_to_scalar};

    #[test]
    fn test_equality_gadget_1() {
        let right: Vec<LinearCombination> = vec![be_to_scalar(&vec![
            0x05, 0x22, 0xa6, 0x4d, 0x7b, 0x93, 0x1e, 0x21, 
            0x76, 0x0c, 0xf9, 0x55, 0xa1, 0x5f, 0xcc, 0x79, 
            0x3e, 0x8a, 0x52, 0xb4, 0x2a, 0x56, 0xab, 0x03, 
            0xaf, 0xdd, 0xec, 0x8b, 0xeb, 0x66, 0x87, 0x49
        ]).into()];

        let left: Vec<u8> = vec![
            0x05, 0x22, 0xa6, 0x4d, 0x7b, 0x93, 0x1e, 0x21, 
            0x76, 0x0c, 0xf9, 0x55, 0xa1, 0x5f, 0xcc, 0x79, 
            0x3e, 0x8a, 0x52, 0xb4, 0x2a, 0x56, 0xab, 0x03, 
            0xaf, 0xdd, 0xec, 0x8b, 0xeb, 0x66, 0x87, 0x49
        ];

        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(1, 1);

        let mut prover_transcript = Transcript::new(b"Equality");
        let mut prover = Prover::new(&pc_gens, &mut prover_transcript);

        let gadget = Equality::new(right);
        let (scalars, witness_commitments, variables) = commit(&mut prover, &left);
        let derived_commitments = gadget.prove(&mut prover, &scalars, &variables);
        let proof = prover.prove(&bp_gens).unwrap();

        let mut verifier_transcript = Transcript::new(b"Equality");
        let mut verifier = Verifier::new(&mut verifier_transcript);
        let witness_vars: Vec<Variable> = verifier_commit(&mut verifier, witness_commitments);
        let derived_vars: Vec<Variable> = verifier_commit(&mut verifier, derived_commitments);
        
        gadget.verify(&mut verifier, &witness_vars, &derived_vars);
        assert!(verifier.verify(&proof, &pc_gens, &bp_gens).is_ok());
    }
}
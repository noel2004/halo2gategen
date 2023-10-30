#![allow(dead_code)]
use halo2_proofs::{circuit::AssignedCell, halo2curves::ff::PrimeField};

#[derive(Clone, Debug)]
pub struct Limb<F: PrimeField> {
    pub cell: Option<AssignedCell<F, F>>,
    pub value: F,
}

impl<F: PrimeField> Limb<F> {
    pub fn new(cell: Option<AssignedCell<F, F>>, value: F) -> Self {
        Limb { cell, value }
    }
    pub fn get_the_cell(&self) -> AssignedCell<F, F> {
        self.cell.as_ref().unwrap().clone()
    }
}

#[derive(Debug)]
pub struct GateCell {
    pub cell: [usize; 3],
    pub name: String,
}

pub mod macros;

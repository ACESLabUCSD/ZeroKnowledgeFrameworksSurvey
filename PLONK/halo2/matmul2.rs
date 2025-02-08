use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Circuit, ConstraintSystem, Error},
    poly::Rotation
};
use ff::Field;
use pasta_curves::Fp;

struct MatMulCircuit<F: Field> {
    a: [[Fp; 128]; 128],
    b: [[Fp; 128]; 128],
    c: [[Fp; 128]; 128],
}

impl<F: Field> Circuit<F> for MatMulCircuit<F> {
    type Config = plonk::Config;
    type FloorPlanner = SimpleFloorPlanner<F>;
    fn without_witnesses(&self) -> Self {
        Self {
            a: [[Fp::zero(); 128]; 128],
            b: [[Fp::zero(); 128]; 128],
            c: [[Fp::zero(); 128]; 128],
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let mut cs = ConstraintSystem::<F>::new();

        let a = cs.alloc_input::<F>((128 * 128).into());
        let b = cs.alloc_input::<F>((128 * 128).into());
        let c = cs.alloc_output::<F>((128 * 128).into());

        cs.enable_equality(a.clone());
        cs.enable_equality(b.clone());

        for i in 0..128 {
            for j in 0..128 {
                let mut tmp = Fp::zero();
                for k in 0..128 {
                    tmp += a[i * 128 + k] * b[k * 128 + j];
                }
                cs.enforce(|| "matmul", |lc| lc + c[i * 128 + j], |lc| lc + tmp);
            }
        }

        Self {
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
        }
    }
}

fn main() -> Result<(), Error> {
    type F = pasta_curves::Fp;
    let mut layouter = SimpleFloorPlanner::<F>::new();
    let circuit = MatMulCircuit::<F> {
        a: Default::default(),
        b: Default::default(),
        c: Default::default(),   
    };

    circuit.synthesize(&mut layouter)?;

    Ok(())
}
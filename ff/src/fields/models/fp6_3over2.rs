use super::cubic_extension::{CubicExtConfig, CubicExtField};
use crate::fields::{CyclotomicMultSubgroup, Fp2, Fp2Config, SqrtPrecomputation};
use core::marker::PhantomData;

pub trait Fp6Config: 'static + Send + Sync + Copy {
    type Fp2Config: Fp2Config;

    const NONRESIDUE: Fp2<Self::Fp2Config>;

    /// Determines the algorithm for computing square roots.
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp6<Self>>> = None;

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP6_C1: &[Fp2<Self::Fp2Config>];
    const FROBENIUS_COEFF_FP6_C2: &[Fp2<Self::Fp2Config>];

    #[inline(always)]
    fn mul_fp2_by_nonresidue_in_place(fe: &mut Fp2<Self::Fp2Config>) -> &mut Fp2<Self::Fp2Config> {
        *fe *= &Self::NONRESIDUE;
        fe
    }
    #[inline(always)]
    fn mul_fp2_by_nonresidue(mut fe: Fp2<Self::Fp2Config>) -> Fp2<Self::Fp2Config> {
        Self::mul_fp2_by_nonresidue_in_place(&mut fe);
        fe
    }
}

pub struct Fp6ConfigWrapper<P: Fp6Config>(PhantomData<P>);

impl<P: Fp6Config> CubicExtConfig for Fp6ConfigWrapper<P> {
    type BasePrimeField = <P::Fp2Config as Fp2Config>::Fp;
    type BaseField = Fp2<P::Fp2Config>;
    type FrobCoeff = Fp2<P::Fp2Config>;

    const SQRT_PRECOMP: Option<SqrtPrecomputation<CubicExtField<Self>>> = P::SQRT_PRECOMP;

    const DEGREE_OVER_BASE_PRIME_FIELD: usize = 6;

    const NONRESIDUE: Self::BaseField = P::NONRESIDUE;

    const FROBENIUS_COEFF_C1: &[Self::FrobCoeff] = P::FROBENIUS_COEFF_FP6_C1;
    const FROBENIUS_COEFF_C2: &[Self::FrobCoeff] = P::FROBENIUS_COEFF_FP6_C2;

    #[inline(always)]
    fn mul_base_field_by_nonresidue_in_place(fe: &mut Self::BaseField) -> &mut Self::BaseField {
        P::mul_fp2_by_nonresidue_in_place(fe)
    }

    fn mul_base_field_by_frob_coeff(
        c1: &mut Self::BaseField,
        c2: &mut Self::BaseField,
        power: usize,
    ) {
        *c1 *= &Self::FROBENIUS_COEFF_C1[power % Self::DEGREE_OVER_BASE_PRIME_FIELD];
        *c2 *= &Self::FROBENIUS_COEFF_C2[power % Self::DEGREE_OVER_BASE_PRIME_FIELD];
    }
}

pub type Fp6<P> = CubicExtField<Fp6ConfigWrapper<P>>;

impl<P: Fp6Config> Fp6<P> {
    pub fn mul_assign_by_fp2(&mut self, other: Fp2<P::Fp2Config>) {
        self.c0 *= &other;
        self.c1 *= &other;
        self.c2 *= &other;
    }

    pub fn mul_by_fp(&mut self, element: &<P::Fp2Config as Fp2Config>::Fp) {
        self.c0.mul_assign_by_fp(element);
        self.c1.mul_assign_by_fp(element);
        self.c2.mul_assign_by_fp(element);
    }

    pub fn mul_by_fp2(&mut self, element: &Fp2<P::Fp2Config>) {
        self.c0 *= element;
        self.c1 *= element;
        self.c2 *= element;
    }

    pub fn mul_by_1(&mut self, c1: &Fp2<P::Fp2Config>) {
        let mut b_b = self.c1;
        b_b *= c1;

        let mut t1 = *c1;
        {
            let mut tmp = self.c1;
            tmp += &self.c2;

            t1 *= &tmp;
            t1 -= &b_b;
            P::mul_fp2_by_nonresidue_in_place(&mut t1);
        }

        let mut t2 = *c1;
        {
            let mut tmp = self.c0;
            tmp += &self.c1;

            t2 *= &tmp;
            t2 -= &b_b;
        }

        self.c0 = t1;
        self.c1 = t2;
        self.c2 = b_b;
    }

    pub fn mul_by_01(&mut self, c0: &Fp2<P::Fp2Config>, c1: &Fp2<P::Fp2Config>) {
        let mut a_a = self.c0;
        let mut b_b = self.c1;
        a_a *= c0;
        b_b *= c1;

        let mut t1 = *c1;
        {
            let mut tmp = self.c1;
            tmp += &self.c2;

            t1 *= &tmp;
            t1 -= &b_b;
            P::mul_fp2_by_nonresidue_in_place(&mut t1);
            t1 += &a_a;
        }

        let mut t3 = *c0;
        {
            let mut tmp = self.c0;
            tmp += &self.c2;

            t3 *= &tmp;
            t3 -= &a_a;
            t3 += &b_b;
        }

        let mut t2 = *c0;
        t2 += c1;
        {
            let mut tmp = self.c0;
            tmp += &self.c1;

            t2 *= &tmp;
            t2 -= &a_a;
            t2 -= &b_b;
        }

        self.c0 = t1;
        self.c1 = t2;
        self.c2 = t3;
    }
}

// We just use the default algorithms; there don't seem to be any faster ones.
impl<P: Fp6Config> CyclotomicMultSubgroup for Fp6<P> {}

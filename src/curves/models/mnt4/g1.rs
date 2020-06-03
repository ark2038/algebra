use crate::curves::models::mnt4::{MNT4Parameters, MNT4p};
use crate::curves::short_weierstrass_projective::{GroupAffine, GroupProjective};
use crate::{Fp2, ToBytes, AffineCurve, FromBytes};
use std::io::{Write, Result as IoResult, Read};
use std::io;

pub type G1Affine<P> = GroupAffine<<P as MNT4Parameters>::G1Parameters>;
pub type G1Projective<P> = GroupProjective<<P as MNT4Parameters>::G1Parameters>;

#[derive(Derivative)]
#[derivative(
Copy(bound = "P: MNT4Parameters"),
Clone(bound = "P: MNT4Parameters"),
Debug(bound = "P: MNT4Parameters"),
PartialEq(bound = "P: MNT4Parameters"),
Eq(bound = "P: MNT4Parameters")
)]
pub struct G1Prepared<P: MNT4Parameters> {
    pub p:                  G1Affine<P>,
    pub py_twist_squared:   Fp2<P::Fp2Params>,
}

impl<P: MNT4Parameters> ToBytes for G1Prepared<P> {
    fn write<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.p.write(&mut writer)?;
        self.py_twist_squared.write(&mut writer)?;
        Ok(())
    }
}

impl<P: MNT4Parameters> FromBytes for G1Prepared<P> {
    fn read<R: Read>(mut reader: R) -> IoResult<Self> {
        let p = G1Affine::<P>::read(&mut reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let py_twist_squared = Fp2::<P::Fp2Params>::read(&mut reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(G1Prepared{p, py_twist_squared})
    }
}

impl<P: MNT4Parameters> G1Prepared<P> {
    pub fn from_affine(point: &G1Affine<P>) -> Self {
        MNT4p::<P>::ate_precompute_g1(&point)
    }
}

impl<P: MNT4Parameters> Default for G1Prepared<P> {
    fn default() -> Self {
        Self::from_affine(&G1Affine::<P>::prime_subgroup_generator())
    }
}
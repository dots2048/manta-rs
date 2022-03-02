// Copyright 2019-2022 Manta Network.
// This file is part of manta-rs.
//
// manta-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-rs.  If not, see <http://www.gnu.org/licenses/>.

//! Manta Pay Transfer Testing

use crate::config::{
    FullParameters, MerkleTreeConfiguration, Mint, PrivateTransfer, ProofSystem, Reclaim,
};
use manta_crypto::{
    constraint::{measure::Measure, ProofSystem as _},
    merkle_tree,
    rand::{Rand, SeedableRng},
};
use rand_chacha::ChaCha20Rng;

type UtxoAccumulator = merkle_tree::full::FullMerkleTree<MerkleTreeConfiguration>;

/// Tests the generation of proving/verifying contexts for [`Mint`].
#[test]
fn sample_mint_context() {
    let mut rng = ChaCha20Rng::from_entropy();
    let cs = Mint::unknown_constraints(FullParameters::new(&rng.gen(), &rng.gen()));
    println!("Mint: {:?}", cs.measure());
    ProofSystem::generate_context(&(), cs, &mut rng).unwrap();
}

/// Tests the generation of proving/verifying contexts for [`PrivateTransfer`].
#[test]
fn sample_private_transfer_context() {
    let mut rng = ChaCha20Rng::from_entropy();
    let cs = PrivateTransfer::unknown_constraints(FullParameters::new(&rng.gen(), &rng.gen()));
    println!("PrivateTransfer: {:?}", cs.measure());
    ProofSystem::generate_context(&(), cs, &mut rng).unwrap();
}

/// Tests the generation of proving/verifying contexts for [`Reclaim`].
#[test]
fn sample_reclaim_context() {
    let mut rng = ChaCha20Rng::from_entropy();
    let cs = Reclaim::unknown_constraints(FullParameters::new(&rng.gen(), &rng.gen()));
    println!("Reclaim: {:?}", cs.measure());
    ProofSystem::generate_context(&(), cs, &mut rng).unwrap();
}

/// Tests the generation of a [`Mint`].
#[test]
fn mint() {
    let mut rng = ChaCha20Rng::from_entropy();
    assert!(
        Mint::sample_and_check_proof(
            &(),
            &rng.gen(),
            &mut UtxoAccumulator::new(rng.gen()),
            &mut rng
        )
        .expect("Random Mint should have successfully produced a proof."),
        "The Mint proof should have been valid."
    );
}

/// Tests the generation of a [`PrivateTransfer`].
#[test]
fn private_transfer() {
    let mut rng = ChaCha20Rng::from_entropy();
    assert!(
        PrivateTransfer::sample_and_check_proof(
            &(),
            &rng.gen(),
            &mut UtxoAccumulator::new(rng.gen()),
            &mut rng
        )
        .expect("Random PrivateTransfer should have successfully produced a proof."),
        "The PrivateTransfer proof should have been valid."
    );
}

/// Tests the generation of a [`Reclaim`].
#[test]
fn reclaim() {
    let mut rng = ChaCha20Rng::from_entropy();
    assert!(
        Reclaim::sample_and_check_proof(
            &(),
            &rng.gen(),
            &mut UtxoAccumulator::new(rng.gen()),
            &mut rng
        )
        .expect("Random Reclaim should have successfully produced a proof."),
        "The Reclaim proof should have been valid."
    );
}
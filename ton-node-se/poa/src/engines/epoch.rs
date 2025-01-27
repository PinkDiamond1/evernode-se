// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Epoch verifiers and transitions.

//use ethereum_types::H256;

use rlp::{Encodable, Decodable, DecoderError, RlpStream, Rlp};
use engines::authority_round::subst::{H256, Machine};

/// A full epoch transition.
#[derive(Debug, Clone)]
pub struct Transition {
	/// Block hash at which the transition occurred.
	pub block_hash: H256,
	/// Block number at which the transition occurred.
	pub block_number: u64,
	/// "transition/epoch" proof from the engine combined with a finality proof.
	pub proof: Vec<u8>,
}

impl Encodable for Transition {
	fn rlp_append(&self, s: &mut RlpStream) {
		s.begin_list(3)
			.append(&self.block_hash)
			.append(&self.block_number)
			.append(&self.proof);
	}
}

impl Decodable for Transition {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		Ok(Transition {
			block_hash: rlp.val_at(0)?,
			block_number: rlp.val_at(1)?,
			proof: rlp.val_at(2)?,
		})
	}
}

/// An epoch transition pending a finality proof.
/// Not all transitions need one.
pub struct PendingTransition {
	/// "transition/epoch" proof from the engine.
	pub proof: Vec<u8>,
}

impl Encodable for PendingTransition {
	fn rlp_append(&self, s: &mut RlpStream) {
		s.append(&self.proof);
	}
}

impl Decodable for PendingTransition {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		Ok(PendingTransition {
			proof: rlp.as_val()?,
		})
	}
}

/// Verifier for all blocks within an epoch with self-contained state.
//pub trait EpochVerifier<M: ::parity_machine::Machine>: Send + Sync {
pub trait EpochVerifier<M: Machine>: Send + Sync {
	/// Lightly verify the next block header.
	/// This may not be a header belonging to a different epoch.
	fn verify_light(&self, header: &M::Header) -> Result<(), M::Error>;

	/// Perform potentially heavier checks on the next block header.
	fn verify_heavy(&self, header: &M::Header) -> Result<(), M::Error> {
		self.verify_light(header)
	}

	/// Check a finality proof against this epoch verifier.
	/// Returns `Some(hashes)` if the proof proves finality of these hashes.
	/// Returns `None` if the proof doesn't prove anything.
	fn check_finality_proof(&self, _proof: &[u8]) -> Option<Vec<H256>> {
		None
	}
}

/// Special "no-op" verifier for stateless, epoch-less engines.
pub struct NoOp;

impl<M: Machine> EpochVerifier<M> for NoOp {
	fn verify_light(&self, _header: &M::Header) -> Result<(), M::Error> { Ok(()) }
}

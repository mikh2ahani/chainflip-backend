#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![doc = include_str!("../../cf-doc-head.md")]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod tests;

use bitvec::prelude::*;
use cf_traits::{EpochIndex, EpochInfo, EpochTransitionHandler};
use codec::FullCodec;
use frame_support::{
	dispatch::{
		DispatchResult, DispatchResultWithPostInfo, GetDispatchInfo, UnfilteredDispatchable,
	},
	ensure,
	pallet_prelude::Member,
	traits::{EnsureOrigin, Get},
	Hashable,
};
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::prelude::*;
use utilities::success_threshold_from_share_count;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use cf_traits::EpochIndex;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The outer Origin needs to be compatible with this pallet's Origin
		type Origin: From<RawOrigin>;

		/// The overarching call type.
		type Call: Member
			+ FullCodec
			+ From<frame_system::Call<Self>>
			+ UnfilteredDispatchable<Origin = <Self as Config>::Origin>
			+ GetDispatchInfo;

		type ValidatorId: Member
			+ FullCodec
			+ From<<Self as frame_system::Config>::AccountId>
			+ Into<<Self as frame_system::Config>::AccountId>;

		type EpochInfo: EpochInfo<ValidatorId = Self::ValidatorId>;

		type Amount: Parameter + Default + Eq + Ord + Copy + AtLeast32BitUnsigned;

		/// Benchmark stuff
		type WeightInfo: WeightInfo;
	}

	/// A hash to index the call by.
	pub(super) type CallHash = [u8; 32];

	/// Convenience alias for a collection of bits representing the votes of each validator.
	pub(super) type VoteMask = BitSlice<Msb0, u8>;

	/// The type used for tallying votes.
	pub(super) type VoteCount = u32;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// A lookup mapping (epoch, call_hash) to a bitmask representing the votes for each validator.
	#[pallet::storage]
	pub type Votes<T: Config> =
		StorageDoubleMap<_, Twox64Concat, EpochIndex, Identity, CallHash, Vec<u8>>;

	/// Defines a unique index for each validator for every epoch.
	#[pallet::storage]
	pub(super) type ValidatorIndex<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		EpochIndex,
		Blake2_128Concat,
		<T as frame_system::Config>::AccountId,
		u16,
	>;

	/// Track epochs and their associated validator count
	#[pallet::storage]
	pub type EpochValidatorCount<T: Config> = StorageMap<_, Twox64Concat, EpochIndex, u32>;

	/// No hooks are implemented for this pallet.
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some external event has been witnessed [call_sig, who, num_votes]
		WitnessReceived(CallHash, T::ValidatorId, VoteCount),

		/// The witness threshold has been reached [call_sig, num_votes]
		ThresholdReached(CallHash, VoteCount),

		/// A witness call has been executed [call_sig, result].
		WitnessExecuted(CallHash, DispatchResult),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// CRITICAL: The validator index is out of bounds. This should never happen.
		ValidatorIndexOutOfBounds,

		/// Witness is not a validator.
		UnauthorisedWitness,

		/// A witness vote was cast twice by the same validator.
		DuplicateWitness,

		/// The epoch has expired
		EpochExpired,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Called as a witness of some external event.
		///
		/// The provided `call` will be dispatched when the configured threshold number of validtors
		/// have submitted an identical transaction. This can be thought of as a vote for the
		/// encoded [Call](Config::Call) value.
		///
		/// ## Events
		///
		/// - [WitnessReceived](Event::WitnessReceived)
		/// - [ThresholdReached](Event::ThresholdReached)
		/// - [WitnessExecuted](Event::WitnessExecuted)
		///
		/// ## Errors
		///
		/// - [UnauthorisedWitness](Error::UnauthorisedWitness)
		/// - [ValidatorIndexOutOfBounds](Error::ValidatorIndexOutOfBounds)
		/// - [DuplicateWitness](Error::DuplicateWitness)
		#[pallet::weight(T::WeightInfo::witness().saturating_add(call.get_dispatch_info().weight))]
		pub fn witness(
			origin: OriginFor<T>,
			call: Box<<T as Config>::Call>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_witness(who, *call)
		}

		/// Called as a witness of some external event.
		///
		/// The provided `call` will be dispatched when the configured threshold number of validtors
		/// have submitted an identical transaction. This can be thought of as a vote for the
		/// encoded [Call](Config::Call) value.
		///
		/// ## Events
		///
		/// - [WitnessReceived](Event::WitnessReceived)
		/// - [ThresholdReached](Event::ThresholdReached)
		/// - [WitnessExecuted](Event::WitnessExecuted)
		///
		/// ## Errors
		///
		/// - [UnauthorisedWitness](Error::UnauthorisedWitness)
		/// - [ValidatorIndexOutOfBounds](Error::ValidatorIndexOutOfBounds)
		/// - [DuplicateWitness](Error::DuplicateWitness)
		#[pallet::weight(T::WeightInfo::witness().saturating_add(call.get_dispatch_info().weight))]
		pub fn witness_at_epoch(
			origin: OriginFor<T>,
			call: Box<<T as Config>::Call>,
			epoch_index: EpochIndex,
			block_number: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::do_witness_at_epoch(who, *call, epoch_index, block_number)
		}
	}

	/// Witness pallet origin
	#[pallet::origin]
	pub type Origin = RawOrigin;

	/// The raw origin enum for this pallet.
	#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode)]
	pub enum RawOrigin {
		WitnessThreshold,
	}
}

impl<T: Config> Pallet<T> {
	/// Do the actual witnessing.
	///
	/// Think of this a vote for some action (represented by a runtime `call`) to be taken. At a
	/// high level:
	///
	/// 1. Ensure we are not submitting a witness for an expired epoch
	/// 2. Look up the account id in the list of validators.
	/// 3. Get the list of votes for the epoch and call, or an empty list if this is the first vote.
	/// 4. Add the account's vote to the list.
	/// 5. Check the number of votes against the required threshold.
	/// 6. If the threshold is exceeded, execute the voted-on `call`.
	///
	/// This implementation uses a bitmask whereby each index to the bitmask represents a validator
	/// account ID in the current Epoch.
	///
	/// **Note:**
	/// This implementation currently allows voting to continue even after the vote threshold is
	/// reached.
	fn do_witness_at_epoch(
		who: <T as frame_system::Config>::AccountId,
		call: <T as Config>::Call,
		epoch_index: EpochIndex,
		_block_number: T::BlockNumber,
	) -> DispatchResultWithPostInfo {
		// Ensure the epoch has not yet expired
		ensure!(
			epoch_index >
				<<T::EpochInfo as EpochInfo>::LastExpiredEpoch as Get<EpochIndex>>::get(),
			Error::<T>::EpochExpired
		);

		// Look up the signer in the list of validators
		let index = ValidatorIndex::<T>::get(&epoch_index, &who)
			.ok_or(Error::<T>::UnauthorisedWitness)? as usize;

		// The number of validators for the epoch
		let num_validators = EpochValidatorCount::<T>::get(epoch_index)
			.expect("This value is updated alongside ValidatorIndex, so if we have a validator, we have a validator count.");

		// Register the vote
		let call_hash = Hashable::blake2_256(&call);
		let num_votes = Votes::<T>::try_mutate::<_, _, _, Error<T>, _>(
			&epoch_index,
			&call_hash,
			|buffer| {
				// If there is no storage item, create an empty one.
				if buffer.is_none() {
					let empty_mask = BitVec::<Msb0, u8>::repeat(false, num_validators as usize);
					*buffer = Some(empty_mask.into_vec())
				}

				let bytes = buffer
					.as_mut()
					.expect("Checked for none condition above, this will never panic;");

				// Convert to an addressable bitmask
				let bits = VoteMask::from_slice_mut(bytes)
				.expect("Only panics if the slice size exceeds the max; The number of validators should never exceed this;");

				let mut vote_count = bits.count_ones();

				// Get a reference to the existing vote.
				let mut vote = bits.get_mut(index).ok_or(Error::<T>::ValidatorIndexOutOfBounds)?;

				// Return an error if already voted, otherwise set the indexed bit to `true` to
				// indicate a vote.
				if *vote {
					return Err(Error::<T>::DuplicateWitness)
				}

				vote_count += 1;
				*vote = true;

				Ok(vote_count)
			},
		)?;

		Self::deposit_event(Event::<T>::WitnessReceived(
			call_hash,
			who.into(),
			num_votes as VoteCount,
		));

		// Check if threshold is reached and, if so, apply the voted-on Call.
		if num_votes == success_threshold_from_share_count(num_validators) as usize {
			Self::deposit_event(Event::<T>::ThresholdReached(call_hash, num_votes as VoteCount));
			let result = call.dispatch_bypass_filter((RawOrigin::WitnessThreshold).into());
			Self::deposit_event(Event::<T>::WitnessExecuted(
				call_hash,
				result.map(|_| ()).map_err(|e| e.error),
			));
		}

		Ok(().into())
	}

	fn do_witness(
		who: <T as frame_system::Config>::AccountId,
		call: <T as Config>::Call,
	) -> DispatchResultWithPostInfo {
		Self::do_witness_at_epoch(who, call, T::EpochInfo::epoch_index(), Default::default())
	}
}

impl<T: pallet::Config> cf_traits::Witnesser for Pallet<T> {
	type AccountId = T::ValidatorId;
	type Call = <T as pallet::Config>::Call;
	type BlockNumber = T::BlockNumber;

	fn witness(who: Self::AccountId, call: Self::Call) -> DispatchResultWithPostInfo {
		Self::do_witness(who.into(), call)
	}

	fn witness_at_epoch(
		who: Self::AccountId,
		call: Self::Call,
		epoch: EpochIndex,
		block_number: Self::BlockNumber,
	) -> DispatchResultWithPostInfo {
		Self::do_witness_at_epoch(who.into(), call, epoch, block_number)
	}
}

/// Simple struct on which to implement EnsureOrigin for our pallet's custom origin type.
///
/// # Example:
///
/// ```ignore
/// if let Ok(()) = EnsureWitnessed::ensure_origin(origin) {
///     log::debug!("This extrinsic was called as a result of witness threshold consensus.");
/// }
/// ```
pub struct EnsureWitnessed;

impl<OuterOrigin> EnsureOrigin<OuterOrigin> for EnsureWitnessed
where
	OuterOrigin: Into<Result<RawOrigin, OuterOrigin>> + From<RawOrigin>,
{
	type Success = ();

	fn try_origin(o: OuterOrigin) -> Result<Self::Success, OuterOrigin> {
		match o.into() {
			Ok(o) => match o {
				RawOrigin::WitnessThreshold => Ok(()),
			},
			Err(o) => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn successful_origin() -> OuterOrigin {
		RawOrigin::WitnessThreshold.into()
	}
}

impl<T: Config> EpochTransitionHandler for Pallet<T> {
	type ValidatorId = T::ValidatorId;
	type Amount = T::Amount;

	fn on_new_epoch(
		_old_validators: &[Self::ValidatorId],
		new_validators: &[Self::ValidatorId],
		_new_bond: Self::Amount,
	) {
		let epoch = T::EpochInfo::epoch_index();

		let mut total = 0;
		for (i, v) in new_validators.iter().enumerate() {
			ValidatorIndex::<T>::insert(&epoch, (*v).clone().into(), i as u16);
			total += 1;
		}

		EpochValidatorCount::<T>::insert(epoch, total);
	}
}

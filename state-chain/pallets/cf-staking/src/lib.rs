#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::{error::BadOrigin, traits::EnsureOrigin};
use frame_system::pallet_prelude::OriginFor;
pub use pallet::*;

use codec::FullCodec;
use sp_runtime::{traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, One}};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use cf_traits::Witnesser;
	use frame_support::{Callable, dispatch::WithPostDispatchInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_system::pallet::Account;

	type AccountId<T> = <T as frame_system::Config>::AccountId;

	#[pallet::config]
	pub trait Config: frame_system::Config
	{
		/// Standard Event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Standard Call type. We need this so we can use it as a constraint in `Witnesser`.
		type Call: From<Call<Self>> + IsType<<Self as frame_system::Config>::Call>;
	
		/// Numeric type based on the `Balance` type from `Currency` trait. Defined inline for now, but we
		/// might want to consider using the `Balances` pallet in future.
		type StakedAmount: Member
			+ FullCodec
			+ Copy
			+ Default
			+ AtLeast32BitUnsigned
			+ MaybeSerializeDeserialize
			+ CheckedSub;
		
		type EthereumAddress: Member + FullCodec;

		type Nonce: Member
			+ FullCodec
			+ Copy
			+ Default
			+ AtLeast32BitUnsigned
			+ MaybeSerializeDeserialize
			+ CheckedSub;

		type EnsureWitnessed: EnsureOrigin<Self::Origin>;

		type Witnesser: cf_traits::Witnesser<
			Call=<Self as Config>::Call, 
			AccountId=<Self as frame_system::Config>::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	pub type Stakes<T: Config> = StorageMap<_, Identity, AccountId<T>, T::StakedAmount, ValueQuery>;

	#[pallet::storage]
	pub type PendingClaims<T: Config> = StorageMap<
		_, 
		Identity, 
		AccountId<T>, 
		T::StakedAmount, 
		OptionQuery>;

	#[pallet::storage]
	pub type Nonces<T: Config> = StorageMap<_, Identity, AccountId<T>, T::Nonce, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	{
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	{
		/// Witness that a `Staked` event was emitted by the `StakeManager` smart contract.
		#[pallet::weight(10_000)]
		pub fn witness_staked(
			origin: OriginFor<T>,
			staker_account_id: AccountId<T>,
			amount: T::StakedAmount,
			refund_address: T::EthereumAddress,
		) -> DispatchResultWithPostInfo {
			let who =  ensure_signed(origin)?;
			let call = Call::staked(staker_account_id, amount, refund_address);

			T::Witnesser::witness(who, call.into())?;

			Ok(().into())
		}

		/// Funds have been staked to an account via the StakeManager smart contract. 
		///
		/// **This is a MultiSig call**
		#[pallet::weight(10_000)]
		pub fn staked(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			amount: T::StakedAmount,
			refund_address: T::EthereumAddress,
		) -> DispatchResultWithPostInfo {
			Self::ensure_witnessed(origin)?;

			if Account::<T>::contains_key(&account_id) {
				let total_stake = Self::add_stake(&account_id, amount)?;
				Self::deposit_event(Event::Staked(account_id, amount, total_stake));
			} else {
				// Account doesn't exist.
				debug::info!("Unknown staking account id {:?}, proceeding to refund.", account_id);
				Self::deposit_event(Event::Refund(amount, refund_address));
			}
			
			Ok(().into())
		}

		/// Get FLIP that is held for me by the system, signed by my validator key.
		///
		/// *QUESTION: should we burn a small amount of FLIP here to disincentivize spam?*
		#[pallet::weight(10_000)]
		pub fn claim(
			origin: OriginFor<T>,
			amount: T::StakedAmount,
			claim_address: T::EthereumAddress,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// If a claim already exists, return an error. The validator must either redeem their claim voucher
			// or wait until expiry before creating a new claim.
			ensure!(!PendingClaims::<T>::contains_key(&who), Error::<T>::PendingClaim);

			// Throw an error if the validator tries to claim too much. Otherwise decrement the stake by the 
			// amount claimed.
			Stakes::<T>::try_mutate::<_,_,Error::<T>,_>(&who, |stake| {
				*stake = stake.checked_sub(&amount).ok_or(Error::<T>::InsufficientStake)?;
				Ok(())
			})?;

			// Don't check for overflow here - we don't expect more than 2^32 claims.
			let nonce = Nonces::<T>::mutate(&who, |nonce| {
				*nonce += T::Nonce::one();
				*nonce
			});
			
			// Emit the event requesting that the CFE to generate the claim voucher.
			Self::deposit_event(Event::<T>::ClaimSigRequested(claim_address, nonce, amount));

			// Assume for now that the siging process is successful and simply insert this claim into
			// the pending claims. 
			//
			// TODO: This should be inserted by the CFE signer process including a valid signature.
			PendingClaims::<T>::insert(&who, amount);

			Ok(().into())
		}

		/// Witness that a `Claimed` event was emitted by the `StakeManager` smart contract. 
		///
		/// This implies that a valid claim has been 
		#[pallet::weight(10_000)]
		pub fn witness_claimed(
			origin: OriginFor<T>,
			account_id: AccountId<T>,
			claimed_amount: T::StakedAmount,
		) -> DispatchResultWithPostInfo {
			let who =  ensure_signed(origin)?;
			let call = Call::claimed(account_id, claimed_amount);

			T::Witnesser::witness(who, call.into())?;

			Ok(().into())
		}

		/// Previously staked funds have been reclaimed.
		///
		/// Note that calling this doesn't initiate any protocol changes - the `claim` has already been authorised
		/// by validator multisig. This merely signals that the claimant has in fact redeemed their funds via the 
		/// `StakeManager` contract. 
		///
		/// If the claimant tries to claim more funds than are available, we set the claimant's balance to 
		/// zero and raise an error. 
		///
		/// **This is a MultiSig call**
		#[pallet::weight(10_000)]
		pub fn claimed(
			origin: OriginFor<T>,
			account_id: AccountId<T>,
			claimed_amount: T::StakedAmount,
		) -> DispatchResultWithPostInfo {
			Self::ensure_witnessed(origin)?;

			let pending_claim_amount = PendingClaims::<T>::get(&account_id).ok_or(Error::<T>::NoPendingClaim)?;
			
			ensure!(claimed_amount == pending_claim_amount, Error::<T>::InvalidClaimAmount);

			PendingClaims::<T>::remove(&account_id);

			Self::deposit_event(Event::Claimed(account_id, claimed_amount));

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config>
	{
		/// A validator has staked some FLIP on the Ethereum chain. [validator_id, stake_added, total_stake]
		Staked(AccountId<T>, T::StakedAmount, T::StakedAmount),

		/// A validator has claimed their FLIP on the Ethereum chain. [validator_id, claimed_amount]
		Claimed(AccountId<T>, T::StakedAmount),

		/// The staked amount should be refunded to the provided Ethereum address. [refund_amount, address]
		Refund(T::StakedAmount, T::EthereumAddress),

		/// A claim request has been made to provided Ethereum address. [address, nonce, amount]
		ClaimSigRequested(T::EthereumAddress, T::Nonce, T::StakedAmount),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The account to be staked is not known.
		UnknownAccount,

		/// An invalid claim has been witnessed: the account has no pending claims.
		NoPendingClaim,

		/// An invalid claim has been witnessed: the amount claimed does not match the pending claim amount.
		InvalidClaimAmount,

		/// The claimant doesn't exist.
		InsufficientStake,

		/// The claimant tried to claim despite having a claim already pending.
		PendingClaim,

		/// The claimant tried to claim more funds than were available. 
		ClaimOverflow,

		/// Stake amount caused overflow on addition. Should never happen.
		StakeOverflow,
	}
}

impl<T: Config> Module<T> {
	fn add_stake(account_id: &T::AccountId, amount: T::StakedAmount) -> Result<T::StakedAmount, Error<T>> {
		Stakes::<T>::try_mutate(
			account_id, 
			|stake| {
				*stake = stake
					.checked_add(&amount)
					.ok_or(Error::<T>::StakeOverflow)?;
				
				Ok(*stake)
			})
	}

	fn ensure_witnessed(origin: OriginFor<T>) -> Result<<T::EnsureWitnessed as EnsureOrigin<OriginFor<T>>>::Success, BadOrigin> {
		T::EnsureWitnessed::ensure_origin(origin)
	}
}
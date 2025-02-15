use frame_support::construct_runtime;

construct_runtime! {
	pub struct Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system expanded::{}::{Pallet},
		Balance: balances::<Instance1> expanded::{}::{Event},
	}
}

fn main() {}

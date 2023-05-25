use frame_support::log::error;
use frame_system::RawOrigin;
use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use sp_runtime::{traits::StaticLookup, DispatchError};

#[derive(Default)]
pub struct AssetExtension;

impl<T> ChainExtension<T> for AssetExtension
where
	T: pallet_contracts::Config + pallet_assets::Config,
	<T as SysConfig>::RuntimeOrigin: From<RawOrigin<<T as SysConfig>::AccountId>>,
	<T as pallet_assets::Config>::AssetIdParameter: From<u32>,
	<<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
	<T as pallet_assets::Config>::Balance: From<u128>,
{
	fn call<E>(&mut self, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
	where
		E: Ext<T = T>,
	{
		let mut env = env.buf_in_buf_out();

		let func_id = env.func_id();
		match func_id {
			1 => {
				let origin = env.ext().address().clone();
				let target = env.ext().caller().clone();
				let asset_id = 0u32;
				let amount: u128 = 1;
				//
				pallet_assets::Pallet::<T>::transfer(
					RawOrigin::Signed(origin).into(),
					asset_id.into(),
					target.into(),
					amount.into(),
				)
				.unwrap();
			},

			_ => {
				error!("Called an unregistered `func_id`: {:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"))
			},
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}

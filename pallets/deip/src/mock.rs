use crate as pallet_deip;
use frame_support::{parameter_types, traits::Get};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, IdentityLookup},
};

pub const DEFAULT_ACCOUNT_ID: <Test as system::Config>::AccountId = 123;
pub const ALICE_ACCOUNT_ID: <Test as system::Config>::AccountId = 124;
pub const BOB_ACCOUNT_ID: <Test as system::Config>::AccountId = 125;

pub const INIT_TIMESTAMP: u64 = 30_000;
pub const BLOCK_TIME: u64 = 1_000;

pub type Extrinsic = TestXt<Call, ()>;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
		Deip: pallet_deip::{Module, Call, Storage, Event<T>, Config},
		Assets: pallet_assets::{Module, Storage, Event<T>},
		DeipAssets: pallet_deip_assets::{Module, Storage, Call},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxLocks: u32 = 1024;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl pallet_deip::traits::DeipAssetSystem<u64> for Test {
	type Balance = u64;
	type AssetId = u32;

	fn try_get_tokenized_project(id: &Self::AssetId) -> Option<super::ProjectId> {
		DeipAssets::try_get_tokenized_project(id)
	}

    fn transactionally_reserve(
        account: &u64,
        project_id: super::ProjectId,
        security_tokens_on_sale: &[(Self::AssetId, Self::Balance)],
    ) -> Result<(), ()> {
		DeipAssets::transactionally_reserve(account, project_id, security_tokens_on_sale)
	}

	fn transactionally_unreserve(project_id: super::ProjectId, account: &u64) -> Result<(), ()> {
		DeipAssets::transactionally_unreserve(project_id, account)
	}

	fn transfer(
		project_id: super::ProjectId,
		who: &u64,
		id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<(), ()> {
		DeipAssets::transfer_from_project(project_id, who, id, amount)
	}
}

impl pallet_deip::Config for Test {
	type Event = Event;
	type DeipAccountId = Self::AccountId;
	type Currency = Balances;
	type AssetSystem = Self;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const AssetDepositBase: Balance = 0;
	pub const AssetDepositPerZombie: Balance = 0;
	pub const ApprovalDeposit: Balance = 0;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 0;
	pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Test {
	type Event = Event;
	type Balance = u64;
	type AssetId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type AssetDepositBase = AssetDepositBase;
	type AssetDepositPerZombie = AssetDepositPerZombie;
	type StringLimit = StringLimit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Test>;
}

impl pallet_deip_assets::traits::DeipProjectsInfo for Test {
	type ProjectId = pallet_deip::ProjectId;

	fn exists(id: &Self::ProjectId) -> bool {
		let projects = &Deip::projects();
		projects.binary_search_by_key(&id, |&(ref p, _)| p).is_ok()
	}
}

impl pallet_deip_assets::Config for Test {
	type ProjectsInfo = Self;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}

pub fn new_test_ext2() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(
				DEFAULT_ACCOUNT_ID,
				(1000 * <ExistentialDeposit as Get<u64>>::get()).into(),
			),
			(
				ALICE_ACCOUNT_ID,
				(2000 * <ExistentialDeposit as Get<u64>>::get()).into(),
			),
			(
				BOB_ACCOUNT_ID,
				(2500 * <ExistentialDeposit as Get<u64>>::get()).into(),
			),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
	});
	ext
}

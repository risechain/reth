// cargo install --debug --locked --path bin/reth --bin reth
// reth stage drop execution
// reth node --debug.tip 0x4e3a3754410177e6937ef1f84bba68ea139e8d1a2258c5f85db9f1cd715a1bdd
// reth node --debug.tip 0xaf674ef208df140303da41ba04be5a8689a7d842b548035872c09a8a1c258658

use std::sync::Arc;

use alloy_primitives::{Address, B256, U256};
use reth_provider::OriginalValuesKnown;
use reth_revm::{db::{BundleState, DBErrorMarker}, state::{AccountInfo, Bytecode}, Database};

use qmdb::{config::Config, tasks::TasksManager, test_helper::SimpleTask, AdsCore, AdsWrap, SharedAdsWrap, ADS, utils::hasher};

pub(crate) struct Qmdb {
    ads: AdsWrap<SimpleTask>, // TODO: Better task type?
    shared_ads: SharedAdsWrap,
    height: i64 // TODO: Better block number management?
}

impl Qmdb {
    // TODO: Why upstream height is signed??
    pub(crate) fn new(height: i64) -> Self {
        // TODO: Handle more configs, better
        let config = Config::from_dir("qmdb");
        AdsCore::init_dir(&config);
        let mut ads = AdsWrap::new(&config);
        // TODO: Confirm this behavior, and this default should be `Default` upstream
        ads.start_block(height, Arc::new(TasksManager::default()));
        let shared_ads = ads.get_shared();
        Qmdb { ads, shared_ads, height }
    }

    // TODO: Make sure we write everything we need
    pub(crate) fn write_state(&self, bundle: &BundleState) {
        let state =
            bundle.to_plain_state(OriginalValuesKnown::Yes);
        for (address, info) in state.accounts {
            self.write_account(address, info)
        }
    }

    // `None` to remove
    pub(crate) fn write_account(&self, address: Address, info: Option<AccountInfo>) {
        dbg!(address, &info);
        if let Some(info) = info {
            // TODO: Write the account
        } else {
            // TODO: Remove the account
        }
    }

    pub(crate) fn flush(&mut self) {
        self.ads.flush();
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum QmdbError {}

impl DBErrorMarker for QmdbError {}

impl Database for &Qmdb{
    type Error = QmdbError;

    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        // Do we need this extra SHA or can just use the address itself as hash?
        let hash = hasher::hash(address);
        // TODO: Find the minimum number of bytes required here
        let mut buf = [];
        // TODO: Gradually abstracting a better API for all reads
        self.shared_ads.read_entry(self.height, &hash, address.as_slice(), &mut buf);
        // TODO: Deserialise an account info out of `buf`
        Ok(None)
    }

    fn code_by_hash(&mut self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        todo!()
    }

    fn storage(&mut self, _address: Address, _index: U256) -> Result<U256, Self::Error> {
        todo!()
    }

    fn block_hash(&mut self, _number: u64) -> Result<B256, Self::Error> {
        todo!()
    }
}

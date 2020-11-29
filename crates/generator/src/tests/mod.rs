use crate::dummy_state::DummyState;
use crate::traits::CodeStore;
use gw_common::blake2b::new_blake2b;
use gw_types::{bytes::Bytes, packed::BlockInfo, prelude::*};
use lazy_static::lazy_static;
use std::{fs, io::Read, path::PathBuf};

mod examples;
mod sudt;

const BINS_DIR: &'static str = "../../c/build";
const EXAMPLES_DIR: &'static str = "../../c/build/examples";
const SUM_BIN_NAME: &'static str = "sum.so";
const PROXY_BIN_NAME: &'static str = "proxy.so";
const SUDT_BIN_NAME: &'static str = "sudt.so";

lazy_static! {
    static ref SUM_PROGRAM: Bytes = {
        let mut buf = Vec::new();
        let mut path = PathBuf::new();
        path.push(&EXAMPLES_DIR);
        path.push(&SUM_BIN_NAME);
        let mut f = fs::File::open(&path).expect("load program");
        f.read_to_end(&mut buf).expect("read program");
        Bytes::from(buf.to_vec())
    };
    static ref SUM_PROGRAM_CODE_HASH: [u8; 32] = {
        let mut buf = [0u8; 32];
        let mut hasher = new_blake2b();
        hasher.update(&SUM_PROGRAM);
        hasher.finalize(&mut buf);
        buf
    };
    static ref PROXY_PROGRAM: Bytes = {
        let mut buf = Vec::new();
        let mut path = PathBuf::new();
        path.push(&EXAMPLES_DIR);
        path.push(&PROXY_BIN_NAME);
        let mut f = fs::File::open(&path).expect("load program");
        f.read_to_end(&mut buf).expect("read program");
        Bytes::from(buf.to_vec())
    };
    static ref PROXY_PROGRAM_CODE_HASH: [u8; 32] = {
        let mut buf = [0u8; 32];
        let mut hasher = new_blake2b();
        hasher.update(&PROXY_PROGRAM);
        hasher.finalize(&mut buf);
        buf
    };
    static ref SUDT_PROGRAM: Bytes = {
        let mut buf = Vec::new();
        let mut path = PathBuf::new();
        path.push(&BINS_DIR);
        path.push(&SUDT_BIN_NAME);
        let mut f = fs::File::open(&path).expect("load program");
        f.read_to_end(&mut buf).expect("read program");
        Bytes::from(buf.to_vec())
    };
    static ref SUDT_PROGRAM_CODE_HASH: [u8; 32] = {
        let mut buf = [0u8; 32];
        let mut hasher = new_blake2b();
        hasher.update(&SUDT_PROGRAM);
        hasher.finalize(&mut buf);
        buf
    };
}

pub fn new_block_info(aggregator_id: u32, number: u64, timestamp: u64) -> BlockInfo {
    BlockInfo::new_builder()
        .aggregator_id(aggregator_id.pack())
        .number(number.pack())
        .timestamp(timestamp.pack())
        .build()
}

pub fn build_dummy_state() -> DummyState {
    let mut tree = DummyState::default();
    tree.insert_code(SUM_PROGRAM_CODE_HASH.clone().into(), SUM_PROGRAM.clone());
    tree.insert_code(
        PROXY_PROGRAM_CODE_HASH.clone().into(),
        PROXY_PROGRAM.clone(),
    );
    tree.insert_code(SUDT_PROGRAM_CODE_HASH.clone().into(), SUDT_PROGRAM.clone());
    tree
}

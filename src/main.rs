extern crate cannyls;
extern crate rand;
#[macro_use]
extern crate trackable;

use cannyls::block::BlockSize;
use cannyls::lump::*;
use cannyls::nvm::FileNvm;
use cannyls::storage::{Storage, StorageBuilder};
use cannyls::Error;

use rand::prelude::*;

fn main() -> Result<(), Error> {
    let mut rng = rand::thread_rng();

    let (nvm, new) = track!(FileNvm::create_if_absent(
        "test.lusf",
        BlockSize::min().ceil_align(10 * 1024 * 1024)
    ))?;

    let mut storage = if !new {
        track!(Storage::open(nvm))?
    } else {
        track!(StorageBuilder::new().journal_region_ratio(0.8).create(nvm))?
    };

    let ss = track!(storage.journal_snapshot())?;
    println!(
        "unH = {}, H = {}, T = {}",
        ss.unreleased_head, ss.head, ss.tail
    );

    let now = std::time::Instant::now();

    for i in 0..10000 {
        let len: usize = rng.gen_range(0, 1024);
        let lump_id = LumpId::new(rng.gen_range(0, i + 1));
        let embed_data = track!(LumpData::new_embedded(vec![42; len]))?;
        track!(storage.put(&lump_id, &embed_data))?;
    }

    let elapsed = now.elapsed();
    println!(
        "elapsed {}.{}sec",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    // simulate crash
    std::mem::forget(storage);

    Ok(())
}

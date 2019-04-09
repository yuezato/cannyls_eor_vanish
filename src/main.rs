extern crate cannyls;
extern crate rand;
#[macro_use]
extern crate trackable;
extern crate structopt;

use cannyls::block::BlockSize;
use cannyls::lump::*;
use cannyls::nvm::FileNvm;
use cannyls::storage::{Storage, StorageBuilder};
use cannyls::Error;

use rand::prelude::*;

use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "cannyls_eor_vanish")]
struct Opt {
    #[structopt(long = "seed")]
    seed: Option<u64>,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut rng = if let Some(seed) = opt.seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_rng(rand::thread_rng()).unwrap()
    };

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
        "unH = {}, H = {}, T = {}, num of journal entries = {}",
        ss.unreleased_head, ss.head, ss.tail, ss.entries.len()
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

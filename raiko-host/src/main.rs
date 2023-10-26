// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod prover;
use anyhow::Result;
use clap::Parser;
use prover::server::serve;
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, require_equals = true)]
    /// Server bind address, e.g. "0.0.0.0:8080"
    bind: String,

    #[clap(short, long, require_equals = true, num_args = 0..=1, default_missing_value = "raiko-host/testdata")]
    /// Use a local directory as a cache for RPC calls. Accepts a custom directory.
    /// [default: raiko-host/testdata]
    cache: Option<String>,

    #[clap(short, long, require_equals = true, num_args = 0..=1, default_missing_value = "raiko-host/guests")]
    /// The guests path
    /// [default: raiko-host/guests]
    guest: Option<String>,
}

// Prerequisites:
//
//   $ rustup default
//   nightly-x86_64-unknown-linux-gnu (default)
//
// Go to /host directory and compile with:
//   $ cargo build
//
// Create /tmp/ethereum directory and run with:
//
//   $ RUST_LOG=info cargo run -- --rpc-url="https://rpc.internal.taiko.xyz/" --block-no=169 --cache=/tmp
//
// from target/debug directory

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    serve(&args.bind, &args.guest.unwrap(), &args.cache.unwrap()).await?;
    Ok(())
}

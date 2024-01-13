@echo off

set tmpdir=%cd%

git clone https://github.com/rust-random/rand.git
cd rand
cargo build --release

cd %tmpdir%

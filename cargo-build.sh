#!/bin/bash

cargo build --release
cargo install --path .
rust-sysmon
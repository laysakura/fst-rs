//! FST (Fast Succinct Trie) implementation in Rust.
//!
//! [Master API Docs](https://laysakura.github.io/fst-rs/fst_rs/)
//! |
//! [Released API Docs](https://docs.rs/crate/fst-rs)
//! |
//! [Benchmark Results](https://laysakura.github.io/fst-rs/criterion/report/)
//! |
//! [Changelog](https://github.com/laysakura/fst-rs/blob/master/CHANGELOG.md)
//!
//! [![Build Status](https://travis-ci.com/laysakura/fst-rs.svg?branch=master)](https://travis-ci.com/laysakura/fst-rs)
//! [![Crates.io Version](https://img.shields.io/crates/v/fst-rs.svg)](https://crates.io/crates/fst-rs)
//! [![Crates.io Downloads](https://img.shields.io/crates/d/fst-rs.svg)](https://crates.io/crates/fst-rs)
//! [![Minimum rustc version](https://img.shields.io/badge/rustc-1.33+-lightgray.svg)](https://github.com/laysakura/fst-rs#rust-version-supports)
//! [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/fst-rs/blob/master/LICENSE-MIT)
//! [![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/fst-rs/blob/master/LICENSE-APACHE)
//!
//! FST is a building block of SuRF (Succinct Range Filter).
//! SuRF and FST is first introduced in the [SIGMOD 2018 best paper](http://www.pdl.cmu.edu/PDL-FTP/Storage/surf_sigmod18.pdf).
//!
//! FST is a memory efficient and high performance trie.
//! FST is like LOUDS based trie (e.g. [trie-rs](https://github.com/laysakura/trie-rs)) but does not use pure LOUDS.
//! It instead uses LOUDS-DS (abbrev of LOUDS-Dense & LOUDS-Sparse), which uses bitmap-based encoding (fast but fat) for upper levels of a trie tree and LOUDS-based encoding (slow but memory-efficient) for lower levels.
//!
//! # Quickstart
//!
//! To use fst-rs, add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! fst-rs = "0.1"  # NOTE: Replace to latest minor version.
//! ```
//!
//! ## Usage Overview
//! (TBD)
//!
//! # Features
//! (TBD)

//! A convenient wrapper for the [`c-ares`](http://c-ares.haxx.se) library.
//!
//! The [`c-ares` crate](https://crates.io/crates/c-ares) provides a safe wrapper around the
//! underlying C library, but it's relatively hard work to use: the user needs to drive the polling
//! of file descriptors according to `c-ares` demands, which likely involves writing something
//! close to a full-blown event loop.
//!
//! This crate does that hard work for you so that the presented API is much more straightforward.
//! Simply create a `Resolver`, and make your query - providing a callback to be called when the
//! query completes.
//!
//! This crate also provides a `FutureResolver`.  Queries on this object return `futures::Future`
//! objects, and don't use callbacks.
//!
//! Additionally, this crate provides a `BlockingResolver`.  Usually if you're using `c-ares`, it's
//! because you care about high-performance, asynchronous code.  But sometimes you'd just like to
//! make a query with as little ceremony as possible, and you're willing to have your code block
//! while you do it.  In such cases the `BlockingResolver` is the most convenient option.
//!
//! On all resolvers:
//!
//! -  methods like `query_xxx` correspond to the `c-ares` function `ares_query`, which "initiates
//! a single-question DNS query"
//!
//! -  methods like `search_xxx` correspond to the `c-ares` function `ares_search`, which
//! "initiates a series of single-question DNS queries ... using the channel's search domains as
//! well as a host alias file given by the HOSTALIAS environment variable".
//!
//! See [`c-ares` documentation](https://c-ares.haxx.se/docs.html) for more details.
//!
//! # Example
//!
//! ```rust
//! extern crate c_ares_resolver;
//! extern crate tokio_core;
//!
//! fn main() {
//!     let resolver = c_ares_resolver::FutureResolver::new().unwrap();
//!     let query = resolver.query_a("google.com");
//!     let mut event_loop = tokio_core::reactor::Core::new().unwrap();
//!     let result = event_loop.run(query).unwrap();
//!     println!("{}", result);
//! }
//! ```
//!
//! Further examples showing how to use the library can be found
//! [here](https://github.com/dimbleby/c-ares-resolver/tree/master/examples).
#![deny(missing_docs)]
extern crate c_ares;
extern crate futures;
extern crate mio_extras;

#[cfg(unix)]
extern crate mio;

#[cfg(windows)]
extern crate winapi;

mod blockingresolver;
mod error;
mod eventloop;
mod futureresolver;
mod host;
mod nameinfo;
mod resolver;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

pub use blockingresolver::BlockingResolver;
pub use error::Error;
pub use futureresolver::{CAresFuture, FutureResolver};
pub use host::HostResults;
pub use nameinfo::NameInfoResult;
pub use resolver::{Options, Resolver};

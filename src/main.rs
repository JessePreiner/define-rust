extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
  pretty_env_logger::init();

	let url = match env::args().nth(1) {
	  Some(url) => url,
		None => {
			println!("Usage: client <url>");
				return;
				}
	};

	let url = url.parse::<hyper::Uri>().unwrap();
	if url.scheme() != Some("http") {
		println!("This example only works with 'http' URLs.");
		return;
	}
}

//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate pretty_env_logger;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod settings;

use settings::Settings;
use std::io::{self, Write};
use std::env;
use futures::{Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {
	let settings = Settings::new();
	println!("{:?}", settings);
	let url = match env::args().nth(1) {
	  Some(url) => url,
		None => {
			println!("Usage: client <url>");
			return;
		}
	};

	let url = url.parse::<hyper::Uri>().unwrap();
	let mut core = Core::new().unwrap();
	let client = Client::configure()
		.connector(HttpsConnector::new(4, &core.handle()).unwrap())
		.build(&core.handle());

	let work = client.get(url).and_then(|res| {
		println!("Response: {}", res.status());
		res.body().for_each(|chunk| {
			io::stdout()
			.write_all(&chunk)
			.map_err(From::from)
		})
	});

	core.run(work).unwrap();
}

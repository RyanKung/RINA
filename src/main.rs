extern crate futures;
extern crate libp2p_ping;
extern crate libp2p_core;
extern crate libp2p_tcp_transport;
extern crate tokio;
extern crate dotenv;
use std::env;

use futures::{Future, Stream};
use libp2p_ping::protocol::{Ping, PingOutput};
use libp2p_core::Transport;
use tokio::runtime::current_thread::Runtime;
use libp2p_tcp_transport::TcpConfig;

fn main() {
    dotenv::dotenv().ok();
    let addr = env::var("ADDRESS")
        .expect("Need set ADDRESS in you .env")
        .parse::<libp2p_core::Multiaddr>()
        .expect("invalid multiaddr");

    let tcp = TcpConfig::new();

    let ping_finished_future = tcp
        .with_upgrade(Ping::default())
        .dial(addr).expect("can't dial")
        .and_then(|out| {
            match out {
                PingOutput::Ponger(processing) => Box::new(processing) as Box<Future<Item = _, Error = _> + Send>,
                PingOutput::Pinger(mut pinger) => {
                    pinger.ping(());
                    let f = pinger.into_future().map(|_| ()).map_err(|(err, _)| err);
                    Box::new(f) as Box<Future<Item = _, Error = _> + Send>
                },
            }
        });

    // Runs until the ping arrives.
    let mut rt = Runtime::new().unwrap();
    let _ = rt.block_on(ping_finished_future)
        .unwrap_or_else(
        |e| {
                print!("{:}", e)
            }
        );
}

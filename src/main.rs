extern crate futures;
extern crate libp2p_ping;
extern crate libp2p_core;
extern crate libp2p_tcp_transport;
extern crate tokio;
extern crate dotenv;

use std::env;

use futures::sync::oneshot;
use futures::{Future, Stream};
use libp2p_ping::protocol::{Ping, PingOutput};
use libp2p_ping::PingListenBehaviour;
use libp2p_core::nodes::Swarm;
use libp2p_core::transport;
use tokio::runtime::current_thread::Runtime;
use libp2p_tcp_transport::TcpConfig;
use libp2p::{Transport, core::upgrade, secio, mplex};

fn main() {
    dotenv::dotenv().ok();
    let address = env::var("ADDRESS")
        .expect("Need set ADDRESS in you .env")
        .parse::<libp2p_core::Multiaddr>()
        .expect("invalid multiaddr");
    let local_key = secio::SecioKeyPair::ed25519_generated().unwrap();
    let local_peer_id = local_key.to_peer_id();
    println!("Local peer id: {:?}", local_peer_id);

   let transport = libp2p::CommonTransport::new()
        .with_upgrade(secio::SecioConfig::new(local_key))
        .and_then(move |out, endpoint| {
            let peer_id = out.remote_key.into_peer_id();
            let upgrade = upgrade::map(mplex::MplexConfig::new(), move |muxer| (peer_id, muxer));
            upgrade::apply(out.stream, upgrade, endpoint.into())
        });

    // let transport = libp2p_tcp_transport::TcpConfig::new()
    //     .with_upgrade(Ping::default())
    //     .dial(addr).unwrap()
    //     .and_then(|out| {
    //         match out {
    //             PingOutput::Ponger(processing) => Box::new(processing) as Box<Future<Item = _, Error = _> + Send>,
    //             PingOutput::Pinger(mut pinger) => {
    //                 pinger.ping(());
    //                 let f = pinger.into_future().map(|_| ()).map_err(|(err, _)| err);
    //                 Box::new(f) as Box<Future<Item = _, Error = _> + Send>
    //             },
    //         }
    //     });


    let mut swarm = {
        let mut behaviour = PingListenBehaviour::new();
        Swarm::new(transport, behaviour, libp2p::core::topology::MemoryTopology::empty())
    };

    let addr = Swarm::listen_on(&mut swarm, address).unwrap();
    println!("Listening on {:?}", addr);
    // Runs until the ping arrives.
    let mut rt = Runtime::new().unwrap();

}

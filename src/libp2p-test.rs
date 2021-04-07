//https://blog.logrocket.com/libp2p-tutorial-build-a-peer-to-peer-app-in-rust/

use std::io;
use std::ops::Range;
use std::iter::Map;
use libp2p::{Transport, tcp::TcpConfig, identity, PeerId, mplex};
use libp2p::core::{Multiaddr, upgrade};
use libp2p::futures::future::Lazy;
use libp2p::floodsub::{Topic, Floodsub};
use tokio::sync::mpsc;
use libp2p::noise::{X25519Spec, Keypair};
use std::borrow::Borrow;

const STORAGE_FILE_PATH: &str = "./recipes.json";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;


type Recipes = Vec<Recipe>;

use serde::{Serialize, Deserialize};
use libp2p::tcp::TokioTcpConfig;
use libp2p::mdns::TokioMdns;

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    id: usize,
    name: String,
    ingredients: String,
    instructions: String,
    public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct ListRequest {
    mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse {
    mode: ListMode,
    data: Recipes,
    receiver: String,
}

enum EventType {
    Response(ListResponse),
    Input(String),
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keys.public());
    let topic = Topic::new("recipes");

    println!("Peer Id: {}", peer_id.clone());
    // let (response_sender, mut response_rcv) = mpsc::unbounded_channel();

    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&keys)
        .expect("can create auth keys");

    let transp = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let mut behaviour = RecipeBehaviour {
        floodsub: Floodsub::new(PEER_ID.clone()),
        mdns: TokioMdns::new().expect("can create mdns"),
        response_sender,
    };

    behaviour.floodsub.subscribe(TOPIC.clone());
}
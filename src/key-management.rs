use std::env;
use std::fs;
use std::io::{Error, Write, BufReader, BufRead};
use async_std::{io, task};
use env_logger::{Builder, Env};
// use futures::prelude::*;
use libp2p::gossipsub::MessageId;
use libp2p::gossipsub::{
    GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity, ValidationMode,
};
use libp2p::identity::ed25519::Keypair;
use libp2p::{gossipsub, identity, PeerId};
use serde::de::IntoDeserializer;
use std::borrow::Borrow;
use libp2p::core::PublicKey;


fn create_and_save_newkey(filename: &str) -> Keypair {
    let local_keypair = Keypair::generate();
    let mut file = fs::File::create(filename).expect("Writing key to file.");
    let encoded = &local_keypair.encode();
    eprintln!("encoded size is {}", encoded.len());
    file.write(encoded).expect("Writing to file.");
    eprintln!("file size is {}", fs::metadata(filename).unwrap().len());
    local_keypair
}

fn get_key(filename: &str) -> Keypair {
    let local_keypair;
    match fs::File::open(filename) {
        Ok(mut file) => {
            let mut buff: Box<[u8]>;
            {
                let mut bufreader = BufReader::new(file);
                buff = Box::from(bufreader.fill_buf().expect("Get file buffer").clone());
                if buff.len() != 64
                {   eprintln!("Size not right, creating a new key");
                    fs::copy("./key", "./key.backup");
                    return create_and_save_newkey(filename); }
                (&(*buff)).consume(64);
            }


            eprintln!("Found \"{}\" with contents \"{:?}\", creating local_key from file.", filename, &buff[..]);
            let mut buffer2 = Box::from(buff.clone());
            match Keypair::decode(&mut buffer2) {
                Ok(key) => {
                    local_keypair = key;
                }
                Err(_) => {
                    eprint!("Could not decode, creating and saving..");
                    local_keypair = create_and_save_newkey(filename);
                }
            }
        }
        Err(_) => {
            eprintln!("File \"{}\" not found, creating local_key from memory and saving.", filename);
            local_keypair = create_and_save_newkey(filename);
        }
    }
    local_keypair
}

fn main() {
    let filename = "./key";
    let local_keypair = get_key(filename);
    let mut file = fs::File::create("private_key").unwrap();
    file.write(local_keypair.secret().as_ref());
    let local_peer = PeerId::from(libp2p::core::PublicKey::Ed25519(local_keypair.public()));
    eprintln!("With text:{:?}", local_peer);
}
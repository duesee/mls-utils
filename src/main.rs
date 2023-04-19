use std::io::{BufRead, BufReader};

use openmls::framing::MlsMessageIn;
use tls_codec::Deserialize;

fn main() {
    loop {
        let mut reader = BufReader::new(std::io::stdin());

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        if let Ok(hex) = hex::decode(line.trim()) {
            if let Ok(msg) = MlsMessageIn::tls_deserialize_exact(hex) {
                println!("{:#?}", msg);
            } else {
                println!("Error during `MlsMessageIn` decoding.");
            }
        } else {
            println!("Error during hex decoding.");
        }

        println!("{}", "-".repeat(80));
    }
}

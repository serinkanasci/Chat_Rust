use std::io::{self, Write};
use std::net:: TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const MSG_SIZE: usize = 32;

fn main () {
    println!("Avec quelle adresse souhaitez vous communiquer ? (Adresse IP : Port)");
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("Erreur de lecture stdin pour addr IP");
    // println!("Quel est votre pseudonyme ?");
    // let mut pseudonyme = String::new();
    // io::stdin().read_line(&mut pseudonyme).expect("Erreur de lecture stdin pour pseudonyme");
    let local: &str = &buff.trim().to_string();
    let mut client = TcpStream::connect(local).expect("TCP stream n'a pas pu se connecter");
    client.set_nonblocking(true).expect("Echec initialisation du non-blocking");
    let (tx, rx) = mpsc::channel::<String>();
    // tx.send(pseudonyme);
    
    thread::spawn(move || loop {

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Erreurn socket");
                println!("Message envoyé : {:?}", msg);
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Ecrivez votre message :");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Erreur de lecture stdin");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
    println!("Fin de la conversation .");
}

// 192.168.40.129:6000
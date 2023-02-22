use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const HOST: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(HOST).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 msg");

                println!("message received {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("Message sent {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a message:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Reading from stding failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("bye")
}

// use std::io::{ErrorKind, Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::sync::mpsc::{self, Sender};
// use std::thread;
// use std::time::Duration;

// const HOST: &str = "127.0.0.1:6000";
// const MSG_SIZE: usize = 32;

// fn main() {
//     let server = TcpListener::bind(HOST).expect("Listener failed to bind");
//     server
//         .set_nonblocking(true)
//         .expect("Failed to initialize non-blocking");
//     let mut clients: Vec<TcpStream> = vec![];
//     let (tx, rx) = mpsc::channel::<String>();

//     loop {
//         if let Ok((mut socket, addr)) = server.accept() {
//             println!("Client {} connected", addr);
//             // let tx = tx.clone();
//             clients.push(socket.try_clone().expect("Failed to clone client"));
//             thread::spawn(move || loop {
//                 let mut buff = vec![0; MSG_SIZE];
//                 match socket.read_exact(&mut buff) {
//                     Ok(_) => {
//                         let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
//                         let msg = String::from_utf8(msg).expect("Invalid utf8 msg");

//                         println!("{}: {:?}", addr, msg);
//                     }
//                     Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
//                     Err(_) => {
//                         println!("closing connction with {}", addr);
//                         break;
//                     }
//                 }
//                 sleep();
//             });
//         }
//         if let Ok(msg) = rx.try_recv() {
//             clients = clients
//                 .into_iter()
//                 .filter_map(|mut client| {
//                     let mut buff = msg.clone().into_bytes();
//                     buff.resize(MSG_SIZE, 0);
//                     client.write_all(&buff).map(|_| client).ok()
//                 })
//                 .collect::<Vec<_>>();
//         }
//         sleep();
//     }
// }

// fn sleep() {
//     thread::sleep(Duration::from_millis(100));
// }

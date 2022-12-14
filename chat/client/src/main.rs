use std::io::{ self, ErrorKind, Read, Write };
use std::net::TcpStream;
use std::sync::mpsc::{ self, TryRecvError };
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initiate non-blocking"); 

    print!(".........................................................................\r\n");
    print!("............***********************************************..............\r\n");
    print!("............***********CHAT APP By Louis Bertson***********..............\r\n");
    print!("............***********************************************..............\r\n");
    print!(".........................................................................\r\n");
    

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("Message recv {:?}", msg);
        
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing to the socket failed");
                println!("Message sent: {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break

        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Please write your message");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":q" || tx.send(msg).is_err() {break}
    }
    // this is the main program that is called all the time o
    println!("Bye Bye!");
}

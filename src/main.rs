use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// This Function handles a single incoming connection.
/// It reads bytes from th stream and writes them into a file.
fn handle_client(mut stream: TcpStream) {
    println!("Connection established!", stream.peer_addr().unwrap());

    // Create or overwrite a file name "received_file.bin"
    let mut file = File::create("received_file.bin").expect("Unable to create file");

    // Temporary buffer to hold data as it comes in
    let mut buffer = [0u8; 1024]; // 1024 bytes at a time

    // Keep reading until the client disconnects or we hit an error
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection CLosed
                println!("Finished Received File. Connection closed." );
                break;
            }
            Ok(n) => {
                 // Write the n bytes we read into the file
                file.write_all(&buffer[..n]).expect("Unable to write to file");
            }
            Err(e) => {
                eprintln!("Error rreading stream: {}", e);
                break;
            }
        }
    }
}

/// This is the main entry point of the program.
fn main() {
    println!("RustDrop receiver is running on port 8080...");

    // Create a TCP server that listens on port 8080 on all interfaces (0.0.0.0)
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Unable to bind to port");

    // This loop awaits for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // We could use threads here if we wanted parallelism
                handle_client(stream);
            }
            Err(e) => {
                eprintln!(" Failed to establish connection: {}", e);
            }
        }
    }









}
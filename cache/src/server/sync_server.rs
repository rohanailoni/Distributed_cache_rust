
use std::ffi::CString;
use std::{io, thread, time};
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Debug,PartialEq)]
enum Operation{
    GET,
    PUT
}
struct Command{
    op:Operation,
    key:String,
    value:String
}
impl Default for Command {
    fn default() -> Self {
        Command {
            op: Operation::GET,  // Set a default value for op
            key: String::new(),  // Set a default value for key
            value: String::new(),  // Set a default value for value
        }
    }
}
fn read_command(mut stream: TcpStream) -> Result<String,&'static str>{
    let mut buffer=[0;1024]; //1MB readable buffer
    let mut stream = stream;
    let bytes_read = stream.read(&mut buffer).unwrap();
    if bytes_read == 0 {
        Err("Connection closed by client.")
    }else{
        let command = String::from_utf8_lossy(&buffer[0..bytes_read]).to_string();
        Ok(command)
    }
}
fn write_response(mut stream: TcpStream, response: &str) -> io::Result<()> {
    stream.write(response.as_bytes())?;
    Ok(())
}

fn process_command(command:String)-> Result<Command, &'static str> {
    let splitrings:Vec<&str>=command.split_whitespace().collect();
    if splitrings.len() > 3 || splitrings.len()<2{
        return Err("Invalid command format.");
    }
    let op=match splitrings[0]{
        "GET"=> Operation::GET,
        "PUT"=> Operation::PUT,
        _ => return Err("Invalid operation."),
    };
    if op==Operation::GET{
        //this is the get command;
        Ok(Command {
            op,
            key: splitrings[1].to_string(),
            ..Default::default()

        })
    }else if op==Operation::PUT{

        Ok(Command {
            op,
            key: splitrings[1].to_string(),
            value: splitrings[2].to_string(),
        })
    }else{
        Err("Invalid Operation")
    }




}

pub(crate) fn runSyncTCPServer(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let now = time::Instant::now();
        match stream{
            Ok(mut stream) => {
                match read_command(stream.try_clone().unwrap()){
                    Ok(command) => {
                        match process_command(command) {
                            Ok(cmd) => {
                                // Process the command further if needed.
                                println!("Operation: {:?}, Key: {}, Value: {}", cmd.op, cmd.key, cmd.value);
                                let response = "Command processed successfully.";
                                if let Err(err) = write_response(stream, response) {
                                    eprintln!("Error writing response: {}", err);
                                }
                            }
                            Err(err) => eprintln!("Error processing command: {}", err),
                        }
                    }
                    Err(_) => {}

                }
            },
            Err(_)=>{}
        }


        thread::sleep( time::Duration::from_secs(4));
        println!("The elasped time in {:?}",now.elapsed());

    }
}
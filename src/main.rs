use std::fs::File;
use std::io::Read;

pub fn read_file_string(read_path: &str) -> Result<String,  std::io::Error> {
    let mut file = File::open(read_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    Ok(file_contents)
}

fn main() {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();
    let data_result = read_file_string("/home/rustam/cpp-transport-catalogue/process_request.json");

    let data = match data_result {
        Ok(file_contents) => file_contents,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    for request_nbr in 0..1 {
        println!("Sending Hello {}...", request_nbr);
        requester.send(&data, 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}

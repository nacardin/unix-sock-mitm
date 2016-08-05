extern crate clap;

use clap::App;
use std::os::unix::net::UnixDatagram;

fn main() {

    let matches = App::new("unix-sock-mitm")
        .version("0,1.0")
        .author("Nick Cardin <nick@cardin.email>")
        .about("Unix domain socket datagram intercept proxy")
        .args_from_usage("<path> 'Path for proxy socket'
            \
            <target-path> 'Path for target socket'")
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let target_path = matches.value_of("target-path").unwrap();

    let socket = UnixDatagram::bind(path).unwrap();

    let mut client_path = String::new();

    loop {
        let mut buf = [0; 256];
        let (count, address) = socket.recv_from(&mut buf).unwrap();

        let address_as_string = address.as_pathname().unwrap().to_str().unwrap();

        let buf_slice = &buf[0..count];

        let result = std::str::from_utf8(buf_slice).unwrap();
        println!("from: {:?}", address_as_string);

        if (address_as_string == target_path) {
            println!("to {:?}", client_path.as_str());
            socket.send_to(buf_slice, client_path.as_str()).unwrap();
        } else {
            client_path.clear();
            client_path.push_str(address_as_string);
            println!("to {:?}", target_path);
            socket.send_to(buf_slice, target_path).unwrap();
        }
        println!("count: {:?}", count);
        println!("utf-8 {:?} \n", result);

    }
}

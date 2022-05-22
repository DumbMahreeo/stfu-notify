use std::{env::args, io::Write, net::Shutdown, os::unix::net::UnixStream};

fn main() {
    let mut socket_path = "/tmp/stfu-notify".to_string();

    let mut text = String::new();
    let mut timeout = 0;

    let mut is_socket = false;
    let mut is_timeout = false;
    let mut has_timeout_been_set = false;

    let mut arguments = args();
    arguments.next();

    for arg in arguments {
        if is_socket {
            is_socket = false;
            socket_path.push('-');
            socket_path.push_str(&arg);
        } else if is_timeout {
            is_timeout = false;
            has_timeout_been_set = true;
            timeout = arg.parse().expect("Given timeout is not a valid number");
        } else {
            match arg.as_ref() {
                "-s" | "--socket" => is_socket = true,
                "-t" | "--timeout" => is_timeout = true,
                _ => {
                    text.push_str(&arg);
                    text.push(' ')
                }
            }
        }
    }

    let text = text.trim();

    if !has_timeout_been_set {
        timeout = 2000;
    }

    socket_path.push_str(".socket");

    let mut unix_stream = UnixStream::connect(socket_path).expect("Couldn't find socket");
    unix_stream
        .write(format!("{timeout}¦@¦{text}").as_bytes())
        .expect("Couldn't write");
    unix_stream
        .shutdown(Shutdown::Both)
        .expect("Couldn't shut down");
}

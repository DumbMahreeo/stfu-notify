use std::{
    env::args, io::Read, os::unix::net::UnixListener, thread, time::Duration, process::Command,
};

fn spawn_notification(timeout: u64, text: &str) {
    let mut gui = Command::new("stfu-notify-gui")
        .arg(text)
        .spawn()
        .unwrap();

    thread::sleep(Duration::from_millis(timeout));

    gui.kill().unwrap();
}

fn main() {
    let mut socket_path = "/tmp/stfu-notify".to_string();

    let mut arguments = args();
    arguments.next();

    if let Some(arg) = arguments.next() {
        socket_path.push('-');
        socket_path.push_str(&arg);
    }

    socket_path.push_str(".socket");

    let socket_path = socket_path.as_str();

    if let Ok(_) = std::fs::metadata(socket_path) {
        println!("A socket is already present. Deleting...");
        std::fs::remove_file(socket_path).unwrap();
    };

    println!("{socket_path}");
    let unix_listener = UnixListener::bind(socket_path).expect("Couldn't bind");
    for stream in unix_listener.incoming() {
        if let Ok(mut stream) = stream {
            thread::spawn(move || {
                let mut buf = vec![];
                if let Ok(_) = stream.read_to_end(&mut buf) {
                    let data = String::from_utf8(buf).unwrap();

                    let mut data = data.splitn(2, "¦@¦");

                    let mut timeout = 2000;
                    let mut text = "[Empty]";

                    if let Some(v) = data.next() {
                        timeout = v.parse().expect("Received timeout is not a valid number");
                    }

                    if let Some(v) = data.next() {
                        text = v;
                    }

                    spawn_notification(timeout, text);
                }
            });
        }
    }
}

use bufstream::BufStream;
use serde_json::to_vec;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::str;
use std::time::Duration;

fn send_command(
    addr: &str,
    timeout: i64,
    json: &serde_json::Value,
) -> Result<String, std::io::Error> {
    let timeout_duration = Duration::new(timeout as u64, 0);
    let saddr = addr.to_socket_addrs().unwrap().next().unwrap();

    let stream = TcpStream::connect_timeout(&saddr, timeout_duration)?;
    stream.set_read_timeout(Some(timeout_duration))?;
    stream.set_write_timeout(Some(timeout_duration))?;
    stream.set_nodelay(true)?;

    let mut buf = BufStream::new(stream);
    let jsonvec = to_vec(&json)?;
    buf.write_all(&jsonvec[..])?;
    buf.flush()?;

    let mut vec = Vec::new();
    buf.read_to_end(&mut vec)?;

    // If the last character is null byte (0x00), drop it
    if vec.last() == Some(&b'\0') {
        vec.truncate(vec.len() - 1);
    }

    let s: String = str::from_utf8(&vec).unwrap().to_owned();
    Ok(s)
}

pub fn cmd(addr: &str, timeout: i64, command: &str) -> Result<String, String> {
    let json = json!({ "command": command });

    match send_command(addr, timeout, &json) {
        Ok(result) => Ok(result),
        Err(error) => Err(format!("IO error: {}", error)),
    }
}

pub fn cmdp(addr: &str, timeout: i64, command: &str, parameter: &str) -> Result<String, String> {
    let json = json!({ "command": command, "parameter": parameter });

    match send_command(addr, timeout, &json) {
        Ok(result) => Ok(result),
        Err(error) => Err(format!("IO error: {}", error)),
    }
}

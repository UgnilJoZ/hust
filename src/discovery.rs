use crate::bridge::Bridge;
use crate::error::Result;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use std::net::UdpSocket;
use std::time::{Duration, Instant};

/// SSDP service discovery request
const DISCOVERY_TEXT: &[u8] = b"M-SEARCH * HTTP/1.1
HOST: 239.255.255.250:1900
MAN: ssdp:discover
MX: 10
ST: ssdp:all

";

/// Receives one bridge URL.
/// 
/// Before, the discovery text should have been sent on the socket via multicast.
fn receive_answer(socket: &UdpSocket) -> std::io::Result<String> {
    let mut buf = [0; 8192];
    let (answer_size, _) = socket.recv_from(&mut buf)?;
    let answer = String::from_utf8_lossy(&buf[0..answer_size]);
    let mut answer_lines = answer.lines();
    if let Some(firstline) = answer_lines.next() {
        if !firstline.starts_with("HTTP/1.1 200 OK") {
            return Err(Error::from(ErrorKind::InvalidData))?;
        }
        for line in answer_lines {
            if let Some(url) = line.strip_prefix("LOCATION: ") {
                return Ok(String::from(url));
            }
        }
    }
    Err(Error::from(ErrorKind::InvalidData))?
}


/// An iterator over the bridges in this network
pub struct BridgeFinder {
    /// Birthday of the bridge finder, needed for timeout
    pub start: Instant,
    /// The socket on which the responses are exepected
    pub socket: UdpSocket,
    /// Timeout after which the iteration will end
    pub timeout: Duration,
    /// Enables deduplication of the received URLs
    pub seen_urls: HashSet<String>,
}

impl BridgeFinder {
    pub fn new(timeout: Duration) -> std::io::Result<Self> {
        let start = Instant::now();
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.send_to(DISCOVERY_TEXT, "239.255.255.250:1900")?;
        Ok(BridgeFinder {
            start,
            socket,
            timeout,
            seen_urls: HashSet::new(),
        })
    }
}

impl Iterator for BridgeFinder {
    type Item = Result<Bridge>;

    fn next(&mut self) -> Option<Result<Bridge>> {
        let time_spent = self.start.elapsed();
        if time_spent > self.timeout {
            return None;
        }
        if let Err(e) = self
            .socket
            .set_read_timeout(Some(self.timeout - time_spent))
        {
            return Some(Err(e.into()));
        }
        match receive_answer(&self.socket) {
            Ok(url) => {
                if self.seen_urls.contains(&url) {
                    self.next()
                } else {
                    self.seen_urls.insert(url.clone());
                    Some(Bridge::from_description_url(url))
                }
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    self.next()
                } else {
                    Some(Err(e.into()))
                }
            }
        }
    }
}

/// Yield all Hue bridges you can find in the network within `timeout`.
/// 
/// Example:
/// ```
/// use std::time::Duration;
/// use hust::{find_bridges, Result};
/// 
/// fn main() -> Result<()> {
///     for bridge in find_bridges(Duration::from_secs(2))? {
///         println!("{:?}", bridge?);
///     }
///     Ok(())
/// }
/// ```
pub fn find_bridges(timeout: Duration) -> std::io::Result<impl Iterator<Item=Result<Bridge>>> {
    BridgeFinder::new(timeout)
}

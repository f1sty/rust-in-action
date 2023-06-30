use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::phy::{wait as phy_wait, TunTapInterface};
use smoltcp::socket::tcp::{Socket, SocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, HardwareAddress, IpAddress, IpCidr, Ipv4Address};
use std::fmt;
use std::net::IpAddr;
use std::os::unix::io::AsRawFd;
use url::Url;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    TcpConnect(smoltcp::socket::tcp::ConnectError),
    TcpSend(smoltcp::socket::tcp::SendError),
    TcpRecv(smoltcp::socket::tcp::RecvError),
    InvalidUrl,
    Content(std::str::Utf8Error),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<smoltcp::socket::tcp::ConnectError> for UpstreamError {
    fn from(error: smoltcp::socket::tcp::ConnectError) -> Self {
        UpstreamError::TcpConnect(error)
    }
}

impl From<smoltcp::socket::tcp::SendError> for UpstreamError {
    fn from(error: smoltcp::socket::tcp::SendError) -> Self {
        UpstreamError::TcpSend(error)
    }
}

impl From<smoltcp::socket::tcp::RecvError> for UpstreamError {
    fn from(error: smoltcp::socket::tcp::RecvError) -> Self {
        UpstreamError::TcpRecv(error)
    }
}

impl From<std::str::Utf8Error> for UpstreamError {
    fn from(error: std::str::Utf8Error) -> Self {
        UpstreamError::Content(error)
    }
}

fn random_port() -> u16 {
    49152 + rand::random::<u16>() % 16384
}

pub fn get(
    mut tap: TunTapInterface,
    mac: EthernetAddress,
    addr: IpAddr,
    url: Url,
) -> Result<(), UpstreamError> {
    let domain_name = url.host_str().ok_or(UpstreamError::InvalidUrl)?;
    let fd = tap.as_raw_fd();
    let default_gateway = Ipv4Address::new(192, 168, 42, 100);

    let config = Config::new(HardwareAddress::Ethernet(mac));
    let mut iface = Interface::new(config, &mut tap, Instant::now());

    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(192, 168, 42, 1), 24))
            .unwrap();
    });

    iface
        .routes_mut()
        .add_default_ipv4_route(default_gateway)
        .unwrap();

    let tcp_rx_buffer = SocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = SocketBuffer::new(vec![0; 1024]);
    let tcp_socket = Socket::new(tcp_rx_buffer, tcp_tx_buffer);
    let mut sockets = SocketSet::new(vec![]);
    let tcp_handle = sockets.add(tcp_socket);

    let http_header = format!(
        "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path(),
        domain_name
    );

    let mut state = HttpState::Connect;

    'http: loop {
        let timestamp = Instant::now();

        iface.poll(timestamp, &mut tap, &mut sockets);
        {
            let socket: &mut Socket<'_> = sockets.get_mut::<Socket>(tcp_handle);

            state = match state {
                HttpState::Connect if !socket.is_active() => {
                    eprintln!("connecting");
                    socket.connect(iface.context(), (addr, 80), random_port())?;
                    HttpState::Request
                }
                HttpState::Request if socket.may_send() => {
                    eprintln!("sending request");
                    socket.send_slice(http_header.as_ref())?;
                    HttpState::Response
                }
                HttpState::Response if socket.can_recv() => {
                    socket.recv(|raw_data| {
                        let output = String::from_utf8_lossy(raw_data);
                        println!("{output}");
                        (raw_data.len(), ())
                    })?;
                    HttpState::Response
                }
                HttpState::Response if !socket.may_recv() => {
                    eprintln!("received complete response");
                    break 'http;
                }
                _ => state,
            }
        }

        phy_wait(fd, iface.poll_delay(timestamp, &sockets)).expect("wait error");
    }
    Ok(())
}

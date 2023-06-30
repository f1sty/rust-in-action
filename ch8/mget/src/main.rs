use clap::Parser;
use smoltcp::phy::{Medium, TunTapInterface};
use url::Url;

mod dns;
mod ethernet;
mod http;

#[derive(Parser)]
#[command(about = "GET a webpage, manually")]
#[command(name = "mget")]
struct Cli {
    #[arg(required = true)]
    url: String,
    #[arg(required = true)]
    tap_device: String,
    #[arg(default_value_t = String::from("1.1.1.1"))]
    dns_server: String,
}

fn main() {
    let cli = Cli::parse();
    let url = Url::parse(&cli.url).expect("error: unable to parse <url> as a URL");

    if url.scheme() != "http" {
        eprintln!("error: only HTTP protocol supported");
        return;
    }

    let tap = TunTapInterface::new(&cli.tap_device, Medium::Ethernet)
        .expect("error: unable to use <tap_device> as a network interface");
    let domain_name = url.host_str().expect("domain name required");
    let _dns_server: std::net::Ipv4Addr = cli
        .dns_server
        .parse()
        .expect("error: unable to parse <dns_server> as an IPv4 address");
    let addr = dns::resolve(&cli.dns_server, domain_name).unwrap().unwrap();
    let mac = ethernet::MacAddress::new().into();

    http::get(tap, mac, addr, url).unwrap();
}

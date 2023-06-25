#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

pub fn main() {
    let sat_a = CubeSat {id: 0};
    let sat_b = CubeSat {id: 1};
    let sat_c = CubeSat {id: 2};

    let status_a = check_status(sat_a);
    let status_b = check_status(sat_b);
    let status_c = check_status(sat_c);

    println!("a: {status_a:?}, b: {status_b:?}, c: {status_c:?}");

    // "waiting" ...
    let status_a = check_status(sat_a);
    let status_b = check_status(sat_b);
    let status_c = check_status(sat_c);

    println!("a: {status_a:?}, b: {status_b:?}, c: {status_c:?}");
}

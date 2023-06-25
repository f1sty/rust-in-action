#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

pub fn main() {
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;

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

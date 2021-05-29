fn main() {
    route1(get_enum());
    route2(get_struct());
    get_message();
    get_options();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

fn get_enum() -> IpAddrKind {
    return IpAddrKind::V4;
}

fn route1(ip_kind: IpAddrKind)
{
    println!("{:?}", ip_kind);
}
#[derive(Debug)]
struct IpAddr1 {
    kind: IpAddrKind,
    address: String
}

fn get_struct() -> IpAddr1 {
    return IpAddr1 {
        kind: get_enum(),
        address: String::from("127.0.0.1")
    };
}

fn route2(ip_address: IpAddr1){
    println!("{:?}", ip_address);
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self){
        println!("{}", "Call");
    }
}

fn get_message() {
    let m = Message::Write(String::from("Ok"));
    m.call();
}

fn get_options() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}", some_number, absent_number);
}
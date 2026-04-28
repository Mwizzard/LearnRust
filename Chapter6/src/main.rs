fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind : IpAddrKind::V4,
        address : String::from("160.0.0.1"),
    };

    let loopback = IpAddr {
        kind : IpAddrKind::V6,
        address : String::from("A3:22:43:2F:4E"),
    };

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    } 

    println!("Value of a Dime : {}", value_in_cents(Coin::Dime));

}

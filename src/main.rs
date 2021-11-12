fn print_vector(v: &Vec<i32>) {
    for (pos, e) in v.iter().enumerate() {
        if pos == 0 {
            print!("[ ");
        }
        print!("{}", e);
        if pos != v.len() - 1 {
            print!(", ");
        } else {
            println!(" ]");
        }
    }
}

fn increment_vector(v: &mut Vec<i32>) {
    for i in v {
        *i += 1;
    }
}

mod ip_mod;
use ip_mod::IpAddr;

fn main() {
    println!("Hello rust!");
    println!("Play with vectors!");

    let mut v = vec![1, 2, 3, 4, 5];

    print_vector(&v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(4) {
        Some(third) => println!("The fifth element is {}", third),
        None => println!("There is no fifth element."),
    }

    match v.get(5) {
        Some(third) => println!("The sixth element is {}", third),
        None => println!("There is no sixth element."),
    }

    increment_vector(&mut v);

    print_vector(&v);

    println!("Play with strings!");

    let data = "initial contents";

    println!("{}", data);

    let _s = data.to_string();
    // the method also works on a literal directly:
    let _s = "initial contents".to_string();
    let mut s = String::from("initial contents");
    println!("{}", &s);
    s.push_str("and bar");
    println!("{}", &s);
    let s2 = " bar";
    s.push_str(&s2);
    println!("{}", &s);
    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    s1 += &s2;
    println!("{}", &s1);

    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    println!("{}", ipv4);
    let ipv6 = IpAddr::V6(456, 634, 785, 522, 877, 76, 21, 65);
    println!("{}", ipv6);
}

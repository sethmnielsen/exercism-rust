#![allow(unused_variables)]

pub fn verse(n: u32) -> String {
    let mut s = String::with_capacity(130);
    let n_str = String::from(&n.to_string());
    let n_str_next: String = match n {
        1 => String::new(),
        _ => String::from(&(n-1).to_string()),
    };

    let w1 = match n {
        1 => String::from(" bottle "),
        _ => String::from(" bottles "),
    };

    let w1_next = match n {
        2 => String::from(" bottle "),
        1 => String::new(),
        _ => w1.clone()
    };

    let w2 = match n {
        1 => String::from(" it "),
        _ => String::from(" one ")
    };

    let c1: String = String::from("of beer on the wall");
    let c0: String = String::from("no more bottles of beer on the wall");

    let c2: &String = match n {
        1 => &c0,
        _ => &c1,
    };

    let s1: String = n_str.clone() + &w1.clone() + &c1 + ", ";
    let s2: String = n_str.clone() + &w1.clone() + &String::from("of beer.\nTake");
    let s3: String = w2 + &String::from("down and pass it around, ");
    let s4: String = w1_next + c2 + ".\n";

    s.push_str(&s1);
    s.push_str(&s2);
    s.push_str(&s3);
    s.push_str(&n_str_next);
    s.push_str(&s4);
    s
}

pub fn sing(start: u32, end: u32) -> String {
    unimplemented!("sing verses {} to {}, inclusive", start, end)
}

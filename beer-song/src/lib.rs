#![allow(unused_variables)]

pub fn verse(n: u32) -> String {
    let mut s = String::with_capacity(130);

    if n != 0 {
        let n_str = String::from(&n.to_string());

        let n_str_next: String = match n {
            1 => String::new(),
            _ => String::from(&(n - 1).to_string()),
        };

        let w1 = match n {
            1 => String::from(" bottle "),
            _ => String::from(" bottles "),
        };

        let w1_next = match n {
            2 => String::from(" bottle "),
            1 => String::new(),
            _ => w1.clone(),
        };

        let w2 = match n {
            1 => String::from(" it "),
            _ => String::from(" one "),
        };

        let c1: String = String::from("of beer on the wall");
        let c0: String = String::from("no more bottles of beer on the wall");

        let c2: &String = match n {
            1 => &c0,
            _ => &c1,
        };

        // (n) bottle(s) of beer on the wall,
        let s1: String = n_str.clone() + &w1.clone() + &c1 + ", ";
        // (n) bottle(s) of beer.\nTake
        let s2: String = n_str + &w1 + &String::from("of beer.\nTake");
        // (it/one) down and pass it around,
        let s3: String = w2 + &String::from("down and pass it around, ");
        // (no more) bottle(s) of beer on the wall.\n
        let s4: String = w1_next + c2 + ".\n";

        s.push_str(&s1);
        s.push_str(&s2);
        s.push_str(&s3);
        s.push_str(&n_str_next);
        s.push_str(&s4);
    } else {
        let s1: String = String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        s.push_str(&s1);
    }
    s
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result: String = String::new();
    let real_start = start + 1;
    for i in (end..real_start).rev() {
        result.push_str(&verse(i));
        let eol = match i {
            fin if fin == end => String::from(""),
            _ => String::from("\n"),
        };
        result.push_str(&eol);
    }
    result
}

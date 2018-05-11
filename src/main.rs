extern crate sqlite;
extern crate regex;
extern crate chrono;

// use std::path::Path;
// use sqlite::State;
use regex::Regex;
// use std::cmp::Ordering;
use std::string::String;
use chrono::prelude::*;

fn main() {
    let id = "  42032319930606629x  ";
    validate(id);
}


/*
struct Division {
    id: i32,
    name: String,
    using: bool,
}
*/


fn db() {
    let connetion = sqlite::open("../db/id.sqlite").unwrap();
}


fn validate(id: &str) {
    let _id = id.trim().to_uppercase();
    println!("identity-card is {}", _id);
    if regex_match_id(&_id) == true {
        println!("regex passed!");
        if pass_checksum(&_id) == true {
            println!("checksum passed!");
        } else {
            println!("id checksum error!");
        }
        let date : String = _id.chars().skip(6).take(8).collect();
        let new_date = date.to_string() + " 00:00:00";
        // println!("date {:?}, new_date {:?}", date, new_date);
        let check_date = Utc.datetime_from_str(&new_date, "%Y%m%d %H:%M:%S").is_ok();
        // println!("{:?}", check_date);
        if check_date == true {
            println!("date passed!");
        } else {
            println!("date illegal!");
        }
    } else {
        println!("id illegal!");
    }
}


fn regex_match_id(id: &str) -> bool {
    let re = Regex::new(r"^\d{6}(18|19|20)\d{2}(0[1-9]|1[012])(0[1-9]|[12]\d|3[01])\d{3}(\d|X)$").unwrap();
    re.is_match(id)
}


fn pass_checksum(id: &str) -> bool {
    let mut _id : [char; 18] = ['0'; 18];
    for (index, c) in (id.chars()).enumerate() {
        _id[index] = c;
    }
    let _factor : [u8; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let _verify : [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
    let mut _checksum : u32 = 0;
    let mut _i : u8 = 0;
    let _last = _id[17];
    for _i in 0..17 {
        let _elm = _id[_i];
        let _t1 = (_elm.to_digit(10).unwrap()) as u32;
        let _t2 = _factor[_i] as u32;
        let _t3 = (_t1*_t2) as u32;
        // println!("_elm {:?}, _i {}, _checksum {} , _t1 {} , _t2 {} , _t3 {}", _elm, _i, _checksum, _t1, _t2, _t3);
        _checksum = _checksum + _t3;
    }
    let _mod = (_checksum%11) as usize;
    let verify = _verify[_mod];
    // println!("_checksum {}, _mod {}, verify {}, _last {}", _checksum, _mod, verify, _last);
    verify == _last
}

/*
fn check_birthday(year : i32, month : i32, day : i32) -> bool {
    let dt = Utc.ymd(year, month, day);
    let birthday = dt.format("%Y-%m-%d").to_string();
    true
}
*/

extern crate sqlite;
extern crate regex;
extern crate chrono;
extern crate dotenv;


use sqlite::Value;
use regex::Regex;
use std::string::String;
use chrono::prelude::*;
use std::io;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    println!("please input id the identity card (eg 42032319930606629x ): ");
    let mut id = String::new();  // mutable 可改变的

    io::stdin().read_line(&mut id)
            .expect("Fail to read line");

    id = id.trim().parse().unwrap();
    if id.is_empty() == true {
        println!("empty identity card, give you a default case: 42032319930606629x");
        id = "42032319930606629x".to_string();
    }
    let identity = &id;
    println!("----------");
    println!("identity card number: {}", identity);
    let is_ok = validate(identity).is_ok();
    println!("validation passed: {:?}", is_ok);

    /*
    match validate(identity) {
        Ok(ok) => println!("validation passed: {:?}", ok),
        Err(e) => println!("error: {:?}", e),
    };
    */

    if is_ok == true {
        let constellation = get_constellation(identity);
        println!("constellation: {}", constellation);
        let gender = get_gender(identity);
        println!("gender: {}", gender);
        let birthday = get_birth(identity);
        println!("birthday: {}", birthday);
        let age = get_age(identity);
        println!("age: {}", age);
        let area = get_area(identity);
        println!("area: {:?}", area);
    } else {
        let e = validate(identity).err().unwrap();
        println!("error: {:?}", e);
    }
}

#[derive(Debug)]
pub struct Area {
    status: bool,
    result: String,
    province: String,
    city: String,
    county: String,
    using: u8
}

pub fn trim(id: &str) -> String {
    let identity = id.trim().to_uppercase();
    identity
}

pub fn validate(id: &str) -> Result<bool, &'static str> {
    let identity = trim(&id);
    if regex_match_id(&identity) == false {
        return Err("identity card illegal!");
    }
    if pass_checksum(&identity) == false {
        return Err("identity card checksum error!");
    }
    let date : String = identity.chars().skip(6).take(8).collect();
    let new_date = date.to_string() + " 00:00:00";
    let check_date = Utc.datetime_from_str(&new_date, "%Y%m%d %H:%M:%S").is_ok();
    if check_date == true {
        return Ok(true);
    } else {
        return Err("identity card with wrong date!");
    }
}


fn regex_match_id(id: &str) -> bool {
    let re = Regex::new(r"^\d{6}(18|19|20)\d{2}(0[1-9]|1[012])(0[1-9]|[12]\d|3[01])\d{3}(\d|X)$").unwrap();
    re.is_match(id)
}


fn pass_checksum(id: &str) -> bool {
    let mut identity : [char; 18] = ['0'; 18];
    for (index, c) in (id.chars()).enumerate() {
        identity[index] = c;
    }
    let factor : [u8; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let verify : [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
    let mut checksum : u32 = 0;
    let last = identity[17];
    for i in 0..17 {
        let elm = identity[i];
        let t1 = (elm.to_digit(10).unwrap()) as u32;
        let t2 = factor[i] as u32;
        let t3 = (t1*t2) as u32;
        checksum = checksum + t3;
    }
    let m = (checksum%11) as usize;
    verify[m] == last
}


pub fn get_constellation(id: &str) -> String {
    let identity = trim(&id);
    let month : String = identity.chars().skip(10).take(2).collect();
    let day : String = identity.chars().skip(12).take(2).collect();
    let edge_days : [u32; 12] = [21, 20, 21, 20, 21, 22, 23, 23, 23, 24, 22, 21];
    let constellations = [
                            "水瓶座",  // 1.21-2.19 [Aquarius]
                            "双鱼座",  // 2.20-3.20 [Pisces]
                            "白羊座",  // 3.21-4.19 [Aries]
                            "金牛座",  // 4.20-5.20 [Taurus]
                            "双子座",  // 5.21-6.21 [Gemini]
                            "巨蟹座",  // 6.22-7.22 [Cancer]
                            "狮子座",  // 7.23-8.22 [Leo]
                            "处女座",  // 8.23-9.22 [Virgo]
                            "天秤座",  // 9.23-10.23 [Libra]
                            "天蝎座",  // 10.24-11.21 [Scorpio]
                            "射手座",  // 11.22-12.20 [Sagittarius]
                            "魔羯座",  // 12.21-1.20 [Capricorn]
                        ];
    let mut m = month.parse::<i32>().unwrap();
    m = m - 1;
    let mut idx = m as usize;
    let d = day.parse::<u32>().unwrap();
    if d < edge_days[idx] {
        m = m - 1;
    }
    if m >= 0 {
        idx = m as usize;
        constellations[idx].to_string()
    } else {
        constellations[11].to_string()
    }
}

pub fn get_gender(id: &str) -> String {
    let identity = trim(&id);
    let gender : String = identity.chars().skip(16).take(1).collect();
    let g = gender.parse::<u8>().unwrap();
    if g%2 == 0 {
        "f".to_string()
    } else {
        "m".to_string()
    }
}


pub fn get_area(id: &str) -> Area {
    let identity = trim(&id);
    let province : String = identity.chars().take(2).collect();
    let city : String = identity.chars().take(4).collect();
    let sufix_province : String = province.to_string() + "0000";
    // let sufix_province = "677612";
    let sufix_city : String = city.to_string() + "00";
    let county : String = identity.chars().take(6).collect();
    let mut area = Area {
        status: validate(&identity).is_ok(),
        result: String::from(""),
        province: String::from(""),
        city: String::from(""),
        county: String::from(""),
        using: 0
    };

    let province_re = db_query(sufix_province);
    let location : String = province_re.0;

    if location.is_empty() {
        return area;
    } else {
        area.province = location;
        let city_re = db_query(sufix_city);
        let county_re = db_query(county);
        area.city = city_re.0.to_string();
        area.county = county_re.0.to_string();
        area.result = area.province.to_string() + " " + &area.city + &area.county;
        area.using = county_re.1;
        return area;
    }
}

fn db_query(division_id: String) -> (String, u8) {
    let mut name = String::from("");
    let mut using : u8 = 0;
    let path = dotenv::var("SQLITE_DB_PATH").unwrap();
    let connection = sqlite::open(path).unwrap();
    let mut cursor = connection
            .prepare("SELECT * FROM divisions WHERE id = ? limit 1")
            .unwrap()
            .cursor();
    cursor.bind(&[Value::String(division_id.to_string())]).unwrap();
    while let Some(row) = cursor.next().unwrap() {
        name = row[1].as_string().unwrap().to_string();
        using = row[2].as_integer().unwrap() as u8;
    }
    let tuple = (name, using);
    return tuple;
}

pub fn get_age(id: &str) -> i32 {
    let identity = trim(&id);
    let year : String = identity.chars().skip(6).take(4).collect();
    let month : String = identity.chars().skip(10).take(2).collect();
    let day : String = identity.chars().skip(12).take(2).collect();
    let y = year.parse::<i32>().unwrap();
    let m = month.parse::<u32>().unwrap();
    let d = day.parse::<u32>().unwrap();
    let birth : DateTime<Utc> = Utc.ymd(y, m, d).and_hms(0, 0, 0);
    let t2 = birth.timestamp();
    let now: DateTime<Utc> = Utc::now();
    let t1 = now.timestamp();
    let t3 = t1 - t2;
    let diff : i32 = (t3/3_153_6000) as i32;
    return diff;
}

pub fn get_birth(id: &str) -> String {
    let identity = trim(&id);
    let birthday : String = identity.chars().skip(6).take(8).collect();
    birthday
}


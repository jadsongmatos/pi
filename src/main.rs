use std::env;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};

use num::integer::Roots;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

//mod check_palindrome;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

mod palindrome;

const size: usize = 37;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut file = File::open(&args[1]).expect("File open");

    let mut buf: [u8; 40] = [0; 40];

    let mut i = start_ycd(&args[1]);
    let mut num: u128 = bin_to_int(&buf);
    println!("{:?} i:{:?} n:{}", args, i, &num);

    let mut file_r = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("./result.txt")
        .unwrap();

    loop {
        file.seek(SeekFrom::Start(i)).expect("seek failed");

        file.read_exact(&mut buf).expect("File read");

        let shift = palindrome::check_palindrome(&buf);
        if shift == 0 {
            //s_buf = std::str::from_utf8(&buf).unwrap();

            //println!("palindrome: ${} {}", i, &s_buf);
            //let num = u128::from_str_radix(s_buf, 10).unwrap();

            num = bin_to_int(&buf);

            if let Err(e) = writeln!(file_r, "palindrome: ${} {}", i, &num) {
                eprintln!("Couldn't write to file: {}", e);
            }

            if is_prime(&num) {
                println!("palindrome primo: ${} {}", i, &num);
                if let Err(e) = writeln!(file_r, "palindrome primo: ${} {}", i, &num) {
                    eprintln!("Couldn't write to file: {}", e);
                }
                break;
            }
            i = i + 1;
        } else {
            i = i + shift;
        }

        if i % 1000000 == 0 {
            num = bin_to_int(&buf);
            println!("${} {}", i, num);
        }
    }
}

fn start_ycd(path: &String) -> u64 {
    let mut file = BufReader::new(File::open(path).expect("File open"));
    let mut buf = Vec::<u8>::new();
    file.read_until(0, &mut buf).unwrap() as u64
}

fn is_prime(value: &u128) -> bool {
    if value % 2 == 0 || value % 3 == 0 {
        return false;
    }

    let max_fac = (value).sqrt() + 1;
    let mut test_fac = 5;
    while test_fac <= max_fac {
        if value % test_fac == 0 || value % (test_fac + 2) == 0 {
            return false;
        }
        test_fac += 6;
    }

    true
}

fn bin_to_int(palavra: &[u8; 40]) -> u128 {
    let mut p1 = LittleEndian::read_u64(&palavra[0..8]).to_string();
    p1.push_str(LittleEndian::read_u64(&palavra[8..16]).to_string().as_str());
    p1.push_str(
        LittleEndian::read_u64(&palavra[16..24])
            .to_string()
            .as_str(),
    );
    p1.push_str(
        LittleEndian::read_u64(&palavra[24..32])
            .to_string()
            .as_str(),
    );
    p1.push_str(
        LittleEndian::read_u64(&palavra[32..40])
            .to_string()
            .as_str(),
    );
    println!("{}", p1);
    0
    //u128::from_str_radix(&p1[0..37], 10).unwrap()
}

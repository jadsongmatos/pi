use std::thread;
use std::thread::available_parallelism;

use std::env;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{BufReader, Read, Seek};
use std::io::prelude::*;


use num::integer::Roots;

fn main() {
    let num_cpus = available_parallelism().unwrap().get();

    let args: Vec<String> = env::args().collect();
    println!("num_cpus: {} {:?}", num_cpus, args);

    let file = File::open(&args[1]).expect("File open error");
    let mut reader = BufReader::new(file);

    for i in 0..num_cpus {
        thread::spawn(move || {
            let mut buf = [0; 21];
            let mut index = i as u64;
            let mut cont: usize = 0;
            let mut s_buf = std::str::from_utf8(&buf);
            loop {
                reader.seek(SeekFrom::Start(index)).expect("seek failed");

                match reader.read_exact(&mut buf) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("File read {}", e);
                        break;
                    }
                }

                if check_palindrome(&buf) {
                    s_buf = std::str::from_utf8(&buf);
                    println!("${} {:?}", i, s_buf);
                    break;
                }

                index = index + 21;
                cont = cont + 1;

                println!("${} #{} {}", i, index, cont);

                if cont % 100000 == 0 {
                    s_buf = std::str::from_utf8(&buf);
                    println!("${} {:?}", i, s_buf);
                }

                if index % 100000000 / num_cpus as u64 == 0 {
                    s_buf = std::str::from_utf8(&buf);
                    println!("${} {:?}", i, s_buf);
                    //break;
                }
            }
        });
    }
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

fn check_palindrome(palavra: &[u8; 21]) -> bool {
    if palavra[0] == palavra[20] {
        if palavra[1] == palavra[19] {
            if palavra[2] == palavra[18] {
                if palavra[3] == palavra[17] {
                    if palavra[4] == palavra[16] {
                        if palavra[5] == palavra[15] {
                            if palavra[6] == palavra[14] {
                                if palavra[7] == palavra[13] {
                                    if palavra[8] == palavra[12] {
                                        if palavra[9] == palavra[11] {
                                            let n_str = std::str::from_utf8(palavra).unwrap();
                                            println!("palindrome: {:?}", &palavra);

                                            match u128::from_str_radix(n_str, 10) {
                                                Ok(n) => {
                                                    if is_prime(&n) {
                                                        println!(
                                                            "palindrome primo: {:?}",
                                                            &palavra
                                                        );
                                                        return true;
                                                    }
                                                }
                                                Err(_) => {
                                                    println!("error from_str_radix")
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

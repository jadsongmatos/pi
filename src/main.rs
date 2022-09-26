use std::env;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};

use num::integer::Roots;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut file = File::open(&args[2]).expect("File open");

    let mut buf = [0; 21];
    let index = u64::from_str_radix(&args[1], 10).unwrap();
    let mut i = index;
    let mut s_buf = std::str::from_utf8(&buf);
    loop {
        if (i / 8) % 8 == index {
            file.seek(SeekFrom::Start(i)).expect("seek failed");

            file.read_exact(&mut buf).expect("File read");

            if check_palindrome(&buf) {
                s_buf = std::str::from_utf8(&buf);
                println!("${} {:?}", i, s_buf);
                break;
            }
        }

        i = i + 1;

        if i % 100000 == 0 {
            s_buf = std::str::from_utf8(&buf);
            println!("${} #{} {:?}", i, &args[1], s_buf);
        }
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

use std::thread;

use std::io::{Read, Seek, BufReader, BufRead};
use std::fs::File;
use std::io::SeekFrom;
use std::env;

fn main() {
    let num_cpus = thread::available_parallelism().unwrap().get();
    let args: Vec<String> = env::args().collect();
    println!("{} {:?}", num_cpus, args);

    if args.len() == 4 {
        let mut file;
        match File::open(&args[3]) {
            Ok(s) => {
                file = s;
            }
            Err(e) => {
                println!("File open {}", e);
                return;
            }
        }

        //let mut reader = BufReader::with_capacity(num_cpus*(21*1024),file);

        let mut buf = [0; 21];
        let mut i: u64 = args[1].parse().unwrap();
        loop {
            file.seek(SeekFrom::Start(i)).expect("seek failed");

            match file.read_exact(&mut buf) {
                Ok(_) => {}
                Err(e) => {
                    println!("File read {}", e);
                    break;
                }
            }

            let check = check_palindrome(&buf);
            if check == 0 {
                let palavra = std::str::from_utf8(&buf).unwrap();
                println!("palindrome: ${} #{:?} {:?}", i, thread::current().id(), &palavra);
                if rust_math::num::is_prime(palavra.parse().unwrap()) {
                    println!("palindrome primo: ${} #{:?} {:?}", i, thread::current().id(), &palavra);
                    break;
                }
                i = i + 1;
            } else {
                i = i + check as u64;
            }


            if i % 100000 == 0 {
                println!("${} {:?}", i, std::str::from_utf8(&buf));
            }

            if i == args[2].parse().unwrap(){
                break;
            }
        }
    }
}


fn check_palindrome(palavra: &[u8; 21]) -> u8 {
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
                                            return 0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if palavra[2] == palavra[20] {
        if palavra[3] == palavra[19] {
            if palavra[4] == palavra[18] {
                if palavra[5] == palavra[17] {
                    if palavra[6] == palavra[16] {
                        if palavra[7] == palavra[15] {
                            if palavra[8] == palavra[14] {
                                if palavra[9] == palavra[13] {
                                    if palavra[10] == palavra[12] {
                                        return 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if palavra[4] == palavra[20] {
        if palavra[5] == palavra[19] {
            if palavra[6] == palavra[18] {
                if palavra[7] == palavra[17] {
                    if palavra[8] == palavra[16] {
                        if palavra[9] == palavra[15] {
                            if palavra[10] == palavra[14] {
                                if palavra[11] == palavra[13] {
                                    return 2;
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if palavra[6] == palavra[20] {
        if palavra[7] == palavra[19] {
            if palavra[8] == palavra[18] {
                if palavra[9] == palavra[17] {
                    if palavra[10] == palavra[16] {
                        if palavra[11] == palavra[15] {
                            if palavra[12] == palavra[14] {
                                return 3;
                            }
                        }
                    }
                }
            }
        }
    } else if palavra[8] == palavra[20] {
        if palavra[9] == palavra[19] {
            if palavra[10] == palavra[18] {
                if palavra[11] == palavra[17] {
                    if palavra[12] == palavra[16] {
                        if palavra[13] == palavra[15] {
                            return 4;
                        }
                    }
                }
            }
        }
    } else if palavra[10] == palavra[20] {
        if palavra[11] == palavra[19] {
            if palavra[12] == palavra[18] {
                if palavra[13] == palavra[17] {
                    if palavra[14] == palavra[16] {
                        return 5;
                    }
                }
            }
        }
    } else if palavra[12] == palavra[20] {
        if palavra[13] == palavra[19] {
            if palavra[14] == palavra[18] {
                if palavra[15] == palavra[17] {
                    return 6;
                }
            }
        }
    } else if palavra[14] == palavra[20] {
        if palavra[15] == palavra[19] {
            if palavra[16] == palavra[18] {
                return 7;
            }
        }
    } else if palavra[16] == palavra[20] {
        if palavra[17] == palavra[19] {
            return 8;
        }
    } else if palavra[18] == palavra[20] {
        return 9;
    }

    return 10;
}

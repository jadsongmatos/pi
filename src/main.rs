use std::thread;

use std::io::{Read, Seek};
use std::fs::File;
use std::io::SeekFrom;

fn main() {
    let num_cpus = thread::available_parallelism().unwrap().get();

    println!("Hello, world! {}", num_cpus);


    for threadId in 0..num_cpus {
        thread::spawn(move || {
            let mut file;
            match File::open("/home/labtec/Downloads/pi/dec-pi-0.txt") {
                Ok(s) => {
                    file = s;
                }
                Err(e) => {
                    println!("File open {}", e);
                    return;
                }
            }

            let mut i = threadId as u64;

            let mut buf = [0; 21];

            loop {

                file.seek(SeekFrom::Start(i)).expect("seek failed");

                match file.read(&mut buf) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("File read error {}", e);
                        break;
                    }
                }

                if check_palindrome(&buf) == true {
                    println!("palindrome {} {:?}", i, &buf);
                }

                //if i % 100000 == 0 {
                    println!("${} #{}", i,threadId);
                //}
                i = i + 21;
            }
        });
    }
}

fn check_palindrome(palavra: &[u8;21]) -> bool {
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
                                            return true;
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

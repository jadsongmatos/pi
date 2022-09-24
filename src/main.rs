use std::thread;

use std::env;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{stdout, BufRead, BufReader, Read, Seek, Write};

use curl::easy::Easy;

fn main() {
    let num_cpus = thread::available_parallelism().unwrap().get();
    println!("{}", num_cpus);

    //let mut reader = BufReader::with_capacity(num_cpus*(21*1024),file);
    let mut easy = Easy::new();
    let mut buf: [u8; 21] = [0; 21];
    let mut i: u64 = 0;
    let mut url = format!("https://api.pi.delivery/v1/pi?start={i}&numberOfDigits=1000&radix=10");
    let mut certo = false;

    loop {
        if certo == false {
            println!("url: {}", &url);
            // Write the contents of rust-lang.org to stdout
            url = format!("https://api.pi.delivery/v1/pi?start={i}&numberOfDigits=1000&radix=10");
            i = i + 1000;
            easy.url(url.as_str()).unwrap();
            easy.write_function(move |data| {
                for cem in 0..978 {
                    for index in 0..21 {
                        buf[index] = data[index+cem];
                        //println!("n{:?}", std::str::from_utf8(&buf));
                    }

                    if check_palindrome(&buf) {
                        let palavra = std::str::from_utf8(&buf).unwrap();

                        println!(
                            "palindrome: ${} #{:?} {:?}",
                            i,
                            thread::current().id(),
                            &palavra
                        );
                        let num = u128::from_str_radix(palavra, 10).unwrap();
                        if prime(&num) {
                            println!(
                                "palindrome primo: ${} #{:?} {:?}",
                                i,
                                thread::current().id(),
                                &palavra
                            );
                            certo = true;
                        }
                    }

                  
                }

                println!("${} {:?}", i, std::str::from_utf8(&buf));
                Ok(data.len())
            })
            .unwrap();
            easy.perform().unwrap();
        }
    }
}

fn prime(y: &u128) -> bool {
    let mut aux = 2;
    let mut div = 0;

    loop {
        if aux <= y / 2 {
            if y % aux == 0 {
                div = div + 1;
            }
        } else {
            break;
        };
        aux = aux + 1;
    }

    if div == 0 {
        return true;
    }

    false
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

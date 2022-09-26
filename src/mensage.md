use crossbeam_channel::unbounded;
//use crossbeam_utils::thread;

use std::thread;
use std::thread::available_parallelism;

use std::env;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{BufRead, BufReader, Read, Seek};

use num::integer::Roots;

use std::process;

struct Mensagem {
    tipo: String,
    dado: [u8; 21],
}
fn main() {
    let num_cpus = available_parallelism().unwrap().get();
    let (s1, r1) = unbounded();
    //let (s2, r2) = (s1.clone(), r1.clone());

    let args: Vec<String> = env::args().collect();
    println!("num_cpus: {} {:?}", num_cpus, args);

    for i in 0..num_cpus {
        let (s2, r2) = (s1.clone(), r1.clone());
        thread::spawn(move || loop {
            let mensagem: Mensagem = r2.recv().unwrap();

            if mensagem.tipo == "c" {
                if check_palindrome(&mensagem.dado) {
                    s2.send(Mensagem {
                        tipo: String::from("o"),
                        dado: mensagem.dado,
                    })
                    .unwrap();
                }
            }
        });
    }

    let mut file;
    match File::open(&args[1]) {
        Ok(s) => {
            file = s;
        }
        Err(e) => {
            println!("File open {}", e);
            return;
        }
    }

    //let mut reader = BufReader::with_capacity(num_cpus*(21*1024),file);

    thread::spawn(move || loop {
        let res: Mensagem = r1.recv().unwrap();

        if res.tipo == "o" {
            println!("{:?}", std::str::from_utf8(&res.dado));
            process::exit(0x0000);
        }
    });

    let mut buf = [0; 21];
    let mut i: u64 = 2;
    let mut s_buf = std::str::from_utf8(&buf);
    loop {
        file.seek(SeekFrom::Start(i)).expect("seek failed");

        match file.read_exact(&mut buf) {
            Ok(_) => {}
            Err(e) => {
                println!("File read {}", e);
                break;
            }
        }

        // Send a message and then receive one.
        s1.send(Mensagem {
            tipo: String::from("c"),
            dado: buf,
        })
        .unwrap();

        i = i + 1;

        if i % 100000000 == 0 {
            s_buf = std::str::from_utf8(&buf);
            println!("${} {:?}", i, s_buf);
            break;
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

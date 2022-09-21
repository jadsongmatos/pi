use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

use std::sync::mpsc;
use std::thread;

fn main() {
    let default_parallelism_approx = thread::available_parallelism().unwrap().get();
    println!("Hello, world! {}", default_parallelism_approx);

    let mut file;
    match File::open("teste") {
        Ok(s) => {
            file = s;
        }
        Err(e) => {
            println!("File open {}", e);
            return;
        }
    }

    let mut palavra = [0; 21];

    match file.read(&mut palavra) {
        Ok(n) => {
            println!("File read {}", n);
            println!("The bytes: {:?}", &palavra);
        }
        Err(e) => {
            println!("File read {}", e);
            return;
        }
    }

    println!("check_palindrome {}", check_palindrome(palavra));
}

fn check_palindrome(palavra: [u8; 21]) -> u8 {
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


use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::Read;

use std::sync::mpsc;
use std::thread;


fn main() {
let default_parallelism_approx = thread::available_parallelism().unwrap().get();
println!("Hello, world! {}",default_parallelism_approx);

    let (tx, rx) = mpsc::channel();

    loop {
        

        let mut file;
        match File::open("teste"){
            Ok(s) => {
                file = s;
            },
            Err(e) => {
                println!("File open {}",e);
                return;
            }
        }

        let mut palavra:[u8;4]  = [0; 4];
        let mut parte1:[u8;2] = [0; 2];


        match file.read(&mut palavra){
            Ok(n) => {
                println!("File read {}",n);
                println!("The bytes: {:?}", &palavra);
            },
            Err(e) => {
                println!("File read {}",e);
                return;
            }
        }
    }

    let mut worker = Vec::new();
    for i in 0..default_parallelism_approx {
        let sender = tx.clone();
        //worker.push(sender);
        worker.push(thread::spawn(move|| {
            //worker[i].send(i).unwrap();
            sender.send("1808").unwrap();
        }))
    }

    println!("Hello, world! {}",default_parallelism_approx);

    println!("w {:?}" , worker[0]);

    for i in 0..default_parallelism_approx {
        // receive each value and wait between each one
        println!("Got: {}", rx.recv().unwrap());
        //println!("rx: {:?}", rx);
    }


    thread::spawn(move || {
        tx.send("hi").unwrap();
    });



}

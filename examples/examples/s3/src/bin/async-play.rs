
use std::time::Duration;
use std::thread;
use std::fs::{metadata, File};
use std::io::{Read, Seek, SeekFrom};
use std::env::args;

use futures::executor::block_on;
use async_std::task;

async fn say_hi_one() {
    println!( " hello " );
    tokio::time::sleep(Duration::from_millis(3000)).await;
    //task::sleep(Duration::from_millis( 3000 )).await;
    //thread::sleep( Duration::from_millis( 3000 ) );
    println!( " good bye " );
} // ()

async fn say_hi_two() {
    println!( " hello " );
} // ()

#[tokio::main]
/*
async fn main() {

    let fut1 = say_hi_one();

    let fut2 = say_hi_two();

    futures::join!( fut1, fut2 );
} // ()
*/

/*
async fn main() {
    block_on( main_async() );
} // ()
*/


async fn main() {
    let start = std::time::Instant::now();
    // your code here

    let args: Vec<String> = args().collect();
    let path: &String = &args[1];
    let threads = (&args[2]).parse::<usize>().unwrap();
    let length: usize = metadata(path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Couldn't convert len from u64 to usize");

    const BLOCK_SIZE: usize = 16_777_216; //16M
    //const THREADSCONST: usize = 10;
    // How much each thread should read
    let mut division: usize = ((length / threads) as f64).ceil() as usize;

    // Use scoped threads to keep things simpler
    thread::scope(|scope| {
        for i in 0..threads {
            scope.spawn(move || {
                // Open a file handle per thread

                let start_thread = std::time::Instant::now();
                let mut thread_file = File::open(&path).expect("Unable to open file");
                let mut contents = vec![0_u8; BLOCK_SIZE];
                // Can't be zero since that's the EOF condition from read()
                let mut read_length: usize = 1;
                let mut read_total: usize = 0;
                let offset: u64 = (i * division) as u64;
                //division =division/ (i+1);
                let start_offset = std::time::Instant::now();
                thread_file
                    .seek(SeekFrom::Start(offset))
                    .expect("Couldn't seek to position in file");

                eprintln!("Thread = {}, Time={:?}", i, start_offset.elapsed());
                let start_content_read = std::time::Instant::now();
                while (read_total < division) && (read_length != 0) {
                    // Handle the case when the bytes remaining to be read are
                    // less than the block size
                    if read_total + BLOCK_SIZE > division {
                        contents.truncate(division - read_total);
                    }
                    read_length = thread_file.read(&mut contents).expect("Couldn't read file");
                    read_total += read_length;
                }
                eprintln!("Thread Content Read = {}, Total Bytes Read = {}, Time={:?}", i, read_total, start_content_read.elapsed());
                eprintln!("Thread Total Time = {}, Time={:?}", i, start_thread.elapsed());
               // println!("{} {}", read_total, length);
            });
        }
    });
    eprintln!("{:?}", start.elapsed());
}
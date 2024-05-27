#![allow(clippy::result_large_err)]

use std::time::Duration;
use std::thread;
use std::fs::{metadata};
use std::io::{Read, Seek, SeekFrom, Write};
use std::env::args;


// snippet-start:[rust.example_code.s3.large_files.scenario]

use std::io::prelude::*;




use futures::executor::block_on;
use async_std::task;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::error::DisplayErrorContext;
use aws_sdk_s3::operation::{
    create_multipart_upload::CreateMultipartUploadOutput, get_object::GetObjectOutput,
};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::{config::Region, Client as S3Client};
use aws_smithy_types::byte_stream::{ByteStream, Length};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use s3_service::error::Error;
use std::process;
use uuid::Uuid;

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

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

    let shared_config = aws_config::load_from_env().await;
    let mut client = &S3Client::new(&shared_config);

    let bucket_name = &format!("test-bucket-goyvabhi");
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-2"));
    let region = region_provider.region().await.unwrap();

    let key = &"testfile".to_string();

    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await
        .unwrap();
    // snippet-end:[rust.example_code.s3.create_multipart_upload]
    let upload_id = multipart_upload_res.upload_id().unwrap();



    const BLOCK_SIZE: usize = 16_777_216; //16M
    //const THREADSCONST: usize = 10;
    // How much each thread should read
    let mut division: usize = ((length / threads) as f64).ceil() as usize;
    let mut num_parts_per_div = division/BLOCK_SIZE;
    let rem_part_size = division%BLOCK_SIZE;
    if (rem_part_size>0) {
        num_parts_per_div += 1;
    }

    let mut upload_parts: &Vec<CompletedPart> = &Vec::new();

    // Use scoped threads to keep things simpler
    thread::scope(|scope| {
        for i in 0..threads {
            scope.spawn(async move || {
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
                let mut part_number = (num_parts_per_div*i)+1;
                while (read_total < division) && (read_length != 0) {
                    // Handle the case when the bytes remaining to be read are
                    // less than the block size
                    if read_total + BLOCK_SIZE > division {
                        contents.truncate(division - read_total);
                    }
                    read_length = thread_file.read(&mut contents).expect("Couldn't read file");


                        let byte_stream = ByteStream::from(contents.clone());


                    // snippet-start:[rust.example_code.s3.upload_part]

                        let upload_part_res = client
                            .upload_part()
                            .key(key)
                            .bucket(bucket_name)
                            .upload_id(upload_id)
                            .body(byte_stream)
                            .part_number(part_number as i32)
                            .send()
                            .await.unwrap();

                        upload_parts.push(
                            CompletedPart::builder()
                                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                                .part_number(part_number as i32)
                                .build(),
                        );

                    part_number = part_number+1;
                    read_total += read_length;
                }
                eprintln!("Thread Content Read = {}, Total Bytes Read = {}, Time={:?}", i, read_total, start_content_read.elapsed());
                eprintln!("Thread Total Time = {}, Time={:?}", i, start_thread.elapsed());
               // println!("{} {}", read_total, length);
            });
        }
    });
    eprintln!("{:?}", start.elapsed());

    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts.to_vec()))
        .build();
    // snippet-end:[rust.example_code.s3.upload_part.CompletedMultipartUpload]

    // snippet-start:[rust.example_code.s3.complete_multipart_upload]
    let _complete_multipart_upload_res = client
        .complete_multipart_upload()
        .bucket(bucket_name)
        .key(key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
    // snippet-end:[rust.example_code.s3.complete_multipart_upload]

    /*
    let data: GetObjectOutput = s3_service::download_object(&client, &bucket_name, &key).await?;
    let data_length: usize = data
        .content_length()
        .unwrap_or_default()
        .try_into()
        .unwrap();
    if length == data_length {
        println!("Data lengths match.");
    } else {
        println!("The data was not the same size!");
    }
    */
}


async fn run_example() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = S3Client::new(&shared_config);

    let bucket_name = format!("doc-example-bucket-{}", Uuid::new_v4());
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-2"));
    let region = region_provider.region().await.unwrap();
    s3_service::create_bucket(&client, &bucket_name, region.as_ref()).await?;

    let key = "sample.txt".to_string();
    // snippet-start:[rust.example_code.s3.create_multipart_upload]
    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(&bucket_name)
        .key(&key)
        .send()
        .await
        .unwrap();
    // snippet-end:[rust.example_code.s3.create_multipart_upload]
    let upload_id = multipart_upload_res.upload_id().unwrap();

    //Create a file of random characters for the upload.

    let mut file = File::create(&key).expect("Could not create sample file.");
    // Loop until the file is 5 chunks.
    while file.metadata().unwrap().len() <= CHUNK_SIZE * 4 {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(256)
            .map(char::from)
            .collect();
        let return_string: String = "\n".to_string();
        file.write_all(rand_string.as_ref())
            .expect("Error writing to file.");
        file.write_all(return_string.as_ref())
            .expect("Error writing to file.");
    }

    let path = Path::new(&key);
    let file_size = tokio::fs::metadata(path)
        .await
        .expect("it exists I swear")
        .len();

    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % CHUNK_SIZE;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = CHUNK_SIZE;
        chunk_count -= 1;
    }

    if file_size == 0 {
        panic!("Bad file size.");
    }
    if chunk_count > MAX_CHUNKS {
        panic!("Too many chunks! Try increasing your chunk size.")
    }

    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
        let this_chunk = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            CHUNK_SIZE
        };
        let stream = ByteStream::read_from()
            .path(path)
            .offset(chunk_index * CHUNK_SIZE)
            .length(Length::Exact(this_chunk))
            .build()
            .await
            .unwrap();
        //Chunk index needs to start at 0, but part numbers start at 1.
        let part_number = (chunk_index as i32) + 1;
        // snippet-start:[rust.example_code.s3.upload_part]
        let upload_part_res = client
            .upload_part()
            .key(&key)
            .bucket(&bucket_name)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await?;
        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );
        // snippet-end:[rust.example_code.s3.upload_part]
    }
    // snippet-start:[rust.example_code.s3.upload_part.CompletedMultipartUpload]
    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts))
        .build();
    // snippet-end:[rust.example_code.s3.upload_part.CompletedMultipartUpload]

    // snippet-start:[rust.example_code.s3.complete_multipart_upload]
    let _complete_multipart_upload_res = client
        .complete_multipart_upload()
        .bucket(&bucket_name)
        .key(&key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
    // snippet-end:[rust.example_code.s3.complete_multipart_upload]

    let data: GetObjectOutput = s3_service::download_object(&client, &bucket_name, &key).await?;
    let data_length: u64 = data
        .content_length()
        .unwrap_or_default()
        .try_into()
        .unwrap();
    if file.metadata().unwrap().len() == data_length {
        println!("Data lengths match.");
    } else {
        println!("The data was not the same size!");
    }

    /*
    s3_service::delete_objects(&client, &bucket_name)
        .await
        .expect("Error emptying bucket.");
    s3_service::delete_bucket(&client, &bucket_name)
        .await
        .expect("Error deleting bucket.");
*/
    Ok(())
}



use std::time::Duration;
use std::thread;
use std::fs::{metadata, File};
use std::io::{Read, Seek, SeekFrom};
use std::env::args;
use futures::future::join_all;
use futures::executor::block_on;
use async_std::task;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::error::DisplayErrorContext;
use aws_sdk_s3::operation::{
    create_multipart_upload::CreateMultipartUploadOutput, get_object::GetObjectOutput,
};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::{config::Region, Client as S3Client, Client};
use aws_smithy_types::byte_stream::{ByteStream, Length};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use s3_service::error::Error;
use std::process;
use std::sync::Arc;
use lazy_static::lazy_static;
use uuid::Uuid;
use std::sync::RwLock;
use futures_util::AsyncWriteExt;


lazy_static! {
    static ref GLOBAL_VEC: RwLock<Vec<CompletedPart>> = RwLock::new(Vec::new());
}

async fn read_file_segment (i: usize, path: String, block_size: usize, division: usize, client: Client, bucket_name: String, key: String, upload_parts: Arc<Vec<CompletedPart>>, upload_id: Arc<String>){

    let start_thread = std::time::Instant::now();
    let mut thread_file = File::open(&path).expect("Unable to open file");
    let mut contents = vec![0_u8; block_size];
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


    let mut num_parts_per_div = division/block_size;
    let rem_part_size = division%block_size;
    if (rem_part_size>0) {
        num_parts_per_div += 1;
    }

    let mut upload_parts_clone = &(*upload_parts);
    let mut part_number = (i*num_parts_per_div)+1;
    while (read_total < division) && (read_length != 0) {
        // Handle the case when the bytes remaining to be read are
        // less than the block size
        if read_total + block_size > division {
            contents.truncate(division - read_total);
        }
        read_length = thread_file.read(&mut contents).expect("Couldn't read file");
        let byte_stream = ByteStream::from(contents.clone());
/*
        let byte_stream_data = byte_stream.collect().await.unwrap();

       // Calculate size
        let size = byte_stream_data.into_bytes().len();

        println!("Size of ByteStream: {} bytes", size);
*/
        //eprintln!("upload_id {}, part_number {}, bucket_name {}, key {}",(*upload_id).clone(), part_number, bucket_name, key);


        let upload_part_res = client
            .upload_part()
            .key(&key)
            .bucket(&bucket_name)
            .upload_id((*upload_id).clone())
            .body(byte_stream)
            .part_number(part_number as i32)
            .send()
            .await
            .unwrap();

        GLOBAL_VEC.write().unwrap().push(
            CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number as i32)
                .build(),
        );



        let part_number = part_number + 1;
        read_total += read_length;
    }
    eprintln!("upload part size {}", GLOBAL_VEC.write().unwrap().len());
    eprintln!("Thread Content Read = {}, Total Bytes Read = {}, Time={:?}", i, read_total, start_content_read.elapsed());
    eprintln!("Thread Number = {}, Time={:?}", i, start_thread.elapsed());

}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
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

    let mut tasks = vec![];


    let shared_config = aws_config::load_from_env().await;
    //let client : Arc<Client> = Arc::new(S3Client::new(&shared_config));
    let client : Client = S3Client::new(&shared_config);

    let bucket_name = "test-bucket-goyvabhi".to_string();
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-2"));
    let region = region_provider.region().await.unwrap();
    let key = "test.dat".to_string();

    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(&bucket_name)
        .key(&key)
        .send()
        .await
        .unwrap();
    // snippet-end:[rust.example_code.s3.create_multipart_upload]
    let upload_id = Arc::new(multipart_upload_res.upload_id().unwrap());
    ///let upload_id = multipart_upload_res.upload_id().unwrap();
    eprintln!("initial upload_id {}", upload_id);
    let upload_id = Arc::new(upload_id.to_string().clone());
    let mut upload_parts: Arc<Vec<CompletedPart>> = Arc::new(Vec::new());
    //let mut upload_parts = Vec::new();
    for i in 0..threads {
        //let client = Arc::clone(&client);
        let client = client.clone();

        let upload_id = Arc::clone(&upload_id);
        let mut upload_parts = Arc::clone(&upload_parts);

        let task = task::spawn(read_file_segment(
            i,
            path.to_string(),
            BLOCK_SIZE,
            division,
            client,
            bucket_name.to_string(),
            key.to_string(),
            upload_parts,
            upload_id
        ));
        tasks.push(task);
    }

    join_all(tasks).await;

    //dbg!(&upload_parts);
    let mut vec : Vec<CompletedPart> = GLOBAL_VEC.write().unwrap().to_vec();
    eprintln!("Final Part Vector Size: {}", vec.len());

    vec.sort_by(|a, b| a.part_number.cmp(&b.part_number));

    eprintln!("Post Part Sort {:?}", vec);
    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(vec))
        .build();
    // snippet-end:[rust.example_code.s3.upload_part.CompletedMultipartUpload]

    // snippet-start:[rust.example_code.s3.complete_multipart_upload]
    let _complete_multipart_upload_res = client
        .complete_multipart_upload()
        .bucket(&bucket_name)
        .key(&key)
        .multipart_upload(completed_multipart_upload)
        .upload_id((*upload_id).clone())
        .send()
        .await
        .unwrap();

    eprintln!("{:?}", start.elapsed());
}
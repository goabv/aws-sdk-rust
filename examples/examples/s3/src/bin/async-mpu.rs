
use std::fs::{metadata, File};
use std::io::{Read, Seek, SeekFrom};
use std::env::args;
use std::ptr::addr_of;
use futures::future::join_all;
use hyper::Body;
use async_std::task;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::operation::{
    create_multipart_upload::CreateMultipartUploadOutput, get_object::GetObjectOutput,
};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::{config::Region, Client as S3Client, Client, config::Config};
use aws_smithy_types::byte_stream::{ByteStream, Length};
use std::sync::Arc;
use lazy_static::lazy_static;
use std::sync::RwLock;
use async_std::task::JoinHandle;
use futures_util::AsyncWriteExt;
use tracing_subscriber;
use hyper::client::HttpConnector;
use hyper::Client as HyperClient;
use tracing_subscriber::{EnvFilter, FmtSubscriber, Registry};
use tracing_subscriber::layer::SubscriberExt;

lazy_static! {
    static ref GLOBAL_VEC: RwLock<Vec<CompletedPart>> = RwLock::new(Vec::new());
    //static ref GLOBAL_MEM_BUFF: Vec<u8>=Vec::new();
}

lazy_static! {
    static ref GLOBAL_MEM_BUFF: Vec<u8> = {
        // Initialize the static variable
        let mut vec = Vec::new();
        for _ in 0..30 {
            let chunk: Vec<u8> = vec![0; 1*1024*1024];
            vec.extend_from_slice(&chunk);
        }
        vec
    };
}


async fn read_memory_segment (i: usize, starting_part_number: usize, num_parts_thread: usize, part_size: usize, last_part_size: usize, chunk_size: usize, offset: usize, client: Client, bucket_name: String, key: String, upload_id: Arc<String>){
    let mut part_size = part_size;
    let last_part_size = last_part_size;

    println!("In Memory Segment Read");
    let mut part_number = starting_part_number;
    let mut end_upload_part_res: u128 = 0;
    let mut part_counter:usize = 1;

    let mut read_offset = offset;
        while (part_counter <= num_parts_thread){

        if (part_counter == num_parts_thread){
            part_size=last_part_size;
        }


            let byte_stream;
            unsafe{byte_stream = ByteStream::from(*GLOBAL_MEM_BUFF[1..5]);}


        read_offset =read_offset+part_size;

        let start_upload_part_res = std::time::Instant::now();

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


        end_upload_part_res = end_upload_part_res + start_upload_part_res.elapsed().as_millis();

        part_counter = part_counter + 1;
        part_number = part_number + 1;

    }

}


async fn read_file_segment (i: usize, path: String, starting_part_number: usize, num_parts_thread: usize, part_size: usize, last_part_size: usize, chunk_size: usize, offset: usize, client: Client, bucket_name: String, key: String, upload_id: Arc<String>){

    let mut part_size = part_size;
    let last_part_size = last_part_size;
    let mut thread_file = File::open(&path).expect("Unable to open file");
    //let mut contents = vec![0_u8; chunk_size];

    thread_file
        .seek(SeekFrom::Start(offset as u64))
        .expect("Couldn't seek to position in file");

    let mut part_number = starting_part_number;
    let mut end_read: u128 = 0;
    let mut end_upload_part_res: u128 = 0;
    let mut part_counter:usize = 1;

    while (part_counter <= num_parts_thread){
        let mut read_total: usize = 0;
        let mut read_length: usize = 1;
        let mut contents = Vec::with_capacity(chunk_size);
        if (part_counter == num_parts_thread){
            part_size=last_part_size;
        }

        let mut buffer = Vec::with_capacity(part_size);
        let byte_stream:ByteStream;

        let start_read = std::time::Instant::now();

        read_length = chunk_size;
        while (read_total < part_size) && (read_length != 0) {
            // Handle the case when the bytes remaining to be read are
            // less than the block size


            if read_total + chunk_size > part_size {
                contents.truncate(part_size - read_total);
                read_length = part_size - read_total;
            }


            unsafe {
                contents.set_len(read_length); // Temporarily set the length for read_exact
                thread_file.read_exact(&mut contents).unwrap();
            }

            //read_length = thread_file.read(&mut contents).expect("Couldn't read file");

            if (chunk_size!=part_size){
                buffer.extend_from_slice(&contents[..read_length]);
            }

            read_total += read_length;
            //println!("part number {}, Total Read {}, Part Size {}", part_number, read_total, part_size);
        }
        //println!("thread number {}, part number {}, part count {}, Total Read {}, Part Size {}", i, part_number, part_counter, read_total, part_size);
        end_read = end_read +  start_read.elapsed().as_millis();
        //overall_read_total = overall_read_total + read_total;


        if (chunk_size!=part_size){
            byte_stream = ByteStream::from(buffer);
        }
        else {
            byte_stream = ByteStream::from(contents);
        }

        let start_upload_part_res = std::time::Instant::now();

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


        end_upload_part_res = end_upload_part_res + start_upload_part_res.elapsed().as_millis();

        part_counter = part_counter + 1;
        part_number = part_number + 1;

    }
    //info!("Inside my_function");
    //println!("Thread Number = {}, Bytes Read {}, Total File Read Time: {}, Total upload part {}, Total upload part stack push {}  ", i, overall_read_total, end_read, end_upload_part_res, end_upload_part_stack_push);

    //eprintln!("upload part size {}", GLOBAL_VEC.write().unwrap().len());
    //eprintln!("Thread Content Read = {}, Total Bytes Read = {}, Time={:?}", i, read_total, start_content_read.elapsed());
    //eprintln!("Thread Number = {}, Time={:?}", i, start_thread.elapsed());

}




#[tokio::main]
async fn main() {

    /*
    let logger = tracing_logstash::Layer::default()
        .event_format(tracing_logstash::logstash::LogstashFormat::default()
            .with_constants(vec![
                ("service.name", "tracing-logstash".to_owned()),
            ])
        );

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let collector = Registry::default().with(logger).with(env_filter);

    tracing::subscriber::set_global_default(collector).unwrap();
*/
    /*
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
*/


    const MIN_PART_SIZE: usize = 8*1024*1024; //8M

    // your code here

    let args: Vec<String> = args().collect();
    let path: &String = &args[1];
    let threads = (&args[2]).parse::<usize>().unwrap();
    let part_size = (&args[3]).parse::<usize>().unwrap()*1024*1024;
    let chunk_size= (&args[4]).parse::<usize>().unwrap()*1024*1024;


    let buffer_size_bytes = 30*1024*1024*1024;
    //let chunk_size_bytes = 1*1024*1024*1024;
    let mut length = 0;

    let mut buffer: Vec<u8> = vec![0,1];

    if (path.as_str()=="memory") {
        length=buffer_size_bytes;
    }
    else {
        length = metadata(path)
            .expect("Unable to query file details")
            .len()
            .try_into()
            .expect("Couldn't convert len from u64 to usize");
    }




    let mut total_num_parts = length/part_size;
    let mut last_part_size = length%part_size;

    if (last_part_size > 0 && last_part_size < MIN_PART_SIZE){
        last_part_size =  last_part_size + part_size;
    }
    else if (last_part_size > 0 && last_part_size >= MIN_PART_SIZE){
        total_num_parts = total_num_parts + 1;
    }
    else {
        last_part_size = part_size;
    }

    let mut parts_per_thread = total_num_parts/threads;
    let remainder_parts = total_num_parts%threads;

    let mut tasks = vec![];

    let shared_config = aws_config::load_from_env().await;
    let client : Client = S3Client::new(&shared_config);

    let bucket_name = "test-bucket-goyvabhi".to_string();
    let key = "test.dat".to_string();

    let start = std::time::Instant::now();

    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(&bucket_name)
        .key(&key)
        .send()
        .await
        .unwrap();

    let upload_id = Arc::new(multipart_upload_res.upload_id().unwrap());
    let upload_id = Arc::new(upload_id.to_string().clone());



    let mut offset: usize= 0;
    let mut starting_part_number = 1;
    for i in 0..threads {
        //let client = Arc::clone(&client);
        let client = client.clone();
        let upload_id = Arc::clone(&upload_id);
        let mut last_part_size_for_thread = part_size;
        let mut num_parts_thread = parts_per_thread;

        if (i<remainder_parts){
            num_parts_thread = num_parts_thread + 1;
        }

        if (i+1==threads){
            last_part_size_for_thread = last_part_size;
        }
        //info!("Inside main");
        println!("Thread Number: {}, num_parts_thread {}, part_size {}, last_part_size_for_thread {}, chunk_size {}, offset {}",i,num_parts_thread,part_size,last_part_size_for_thread,chunk_size,offset);
        let task: JoinHandle<()>;


        if (path.as_str()=="memory"){
            task = task::spawn(read_memory_segment(
                i,
                starting_part_number,
                num_parts_thread,
                part_size,
                last_part_size_for_thread,
                chunk_size,
                offset,
                client,
                bucket_name.to_string(),
                key.to_string(),
                upload_id
            ));
        }else {
            task = task::spawn(read_file_segment(
                i,
                path.to_string(),
                starting_part_number,
                num_parts_thread,
                part_size,
                last_part_size_for_thread,
                chunk_size,
                offset,
                client,
                bucket_name.to_string(),
                key.to_string(),
                upload_id
            ));
        }

        tasks.push(task);
        offset = offset + (num_parts_thread*part_size);
        starting_part_number = starting_part_number + num_parts_thread;
    }

    join_all(tasks).await;

    //dbg!(&upload_parts);
    let mut vec : Vec<CompletedPart> = GLOBAL_VEC.write().unwrap().to_vec();
    eprintln!("Final Part Vector Size: {}", vec.len());

    vec.sort_by(|a, b| a.part_number.cmp(&b.part_number));


    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(vec))
        .build();


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


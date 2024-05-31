use jemallocator::Jemalloc;
use std::fs::{metadata, File};
use std::io::{Read, Seek, SeekFrom};
use std::env::args;
use futures::future::join_all;
use async_std::task;
use aws_sdk_s3::operation::{create_multipart_upload::CreateMultipartUploadOutput};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::{ Client as S3Client, Client};
use aws_smithy_types::byte_stream::{ByteStream};
use std::sync::Arc;
use lazy_static::lazy_static;
use std::sync::RwLock;
use futures_util::AsyncWriteExt;
use tracing_subscriber;
use tracing_subscriber::prelude::*;
use tracing_flame::{FlameLayer, FlushGuard};
use tracing_subscriber::{EnvFilter, Registry};

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

lazy_static! {
    static ref GLOBAL_VEC: RwLock<Vec<CompletedPart>> = RwLock::new(Vec::new());
}


#[tracing::instrument]
async fn read_file_and_upload_single_part (i: usize, path: String, starting_part_number: usize, num_parts_thread: usize, part_size: usize, last_part_size: usize, chunk_size: usize, offset: usize, client: Client, bucket_name: String, key: String, upload_id: Arc<String>){


    //let _guard_1 = flame::start_guard(format!("read_file_and_upload_single_part: {}",i));
    tracing::info!(thread = i,"start read_file_and_upload_single_part");

    let mut part_size = part_size;
    let last_part_size = last_part_size;
    let mut thread_file = File::open(&path).expect("Unable to open file");

    thread_file
        .seek(SeekFrom::Start(offset as u64))
        .expect("Couldn't seek to position in file");

    let mut part_number = starting_part_number;
    let mut end_read: u128 = 0;
    let mut end_upload_part_res: u128 = 0;
    let mut part_counter:usize = 1;

    while (part_counter <= num_parts_thread){
        tracing::info!(thread = i,part counter=part_counter,"start reading file segment");
        //let _guard_2 = flame::start_guard(format!("reading part {} on thread id {}",part_counter,i));
        let mut read_total: usize = 0;
        let mut read_length: usize = 1;
        let byte_stream:ByteStream;
        let mut buffer = Vec::new();

        let mut contents = vec![0;chunk_size];
        if (part_counter == num_parts_thread){
            part_size=last_part_size;
        }

        let start_read = std::time::Instant::now();
        while (read_total < part_size) && (read_length != 0) {
            if read_total + chunk_size > part_size {
                contents.truncate(part_size - read_total);
            }
            read_length = thread_file.read(&mut contents).expect("Couldn't read file");

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
        tracing::info!(thread = i,part counter=part_counter,"end reading file segment");
        //flame::end(format!("reading part {} on thread id {}",part_counter,i));

        tracing::info!(thread = i,part counter=part_counter,"start uploading part");
        //let _guard_3 = flame::start_guard(format!("uploading part {} on thread id {}",part_counter,i));
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

      //  flame::end(format!("uploading part {} on thread id {}",part_counter,i));
        end_upload_part_res = end_upload_part_res + start_upload_part_res.elapsed().as_millis();
        part_counter = part_counter + 1;
        part_number = part_number + 1;
        tracing::info!(thread = i,part counter=part_counter,"end uploading part");
    }
    tracing::info!(thread = i,"end read_file_and_upload_single_part");
    //flame::end(format!("read_file_and_upload_single_part: {}",i));
    //println!("Thread Number = {}, Bytes Read {}, Total File Read Time: {}, Total upload part {}, Total upload part stack push {}  ", i, overall_read_total, end_read, end_upload_part_res, end_upload_part_stack_push);
}
#[tokio::main]
async fn main() {


    let flame_layer = FlameLayer::default();
    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
    let env_filter = EnvFilter::new("info");

    let subscriber = Registry::default()
        .with(flame_layer)
        .with(fmt_layer)
        .with(env_filter);

    tracing::subscriber::set_global_default(subscriber).expect("Could not set global default subscriber");

    tracing::info!("Starting application");
    let _flush_guard = FlushGuard::new("flamegraph.folded");
    const MIN_PART_SIZE: usize = 8*1024*1024; //8M

    let args: Vec<String> = args().collect();
    let path: &String = &args[1];
    let threads = (&args[2]).parse::<usize>().unwrap();
    let part_size = (&args[3]).parse::<usize>().unwrap()*1024*1024;
    let chunk_size= (&args[4]).parse::<usize>().unwrap()*1024*1024;
    let mut length = 0;

    length = metadata(path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Couldn't convert len from u64 to usize");

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

        println!("Thread Number: {}, num_parts_thread {}, part_size {}, last_part_size_for_thread {}, chunk_size {}, offset {}",i,num_parts_thread,part_size,last_part_size_for_thread,chunk_size,offset);
        let task = task::spawn(read_file_and_upload_single_part(
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
        tasks.push(task);
        offset = offset + (num_parts_thread*part_size);
        starting_part_number = starting_part_number + num_parts_thread;
    }
    join_all(tasks).await;

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
    //tracing_flame::Flush::flush().expect("Could not write flamegraph data");
//    flame::end("main");
  //  let mut file = File::create("flamegraph.html").unwrap();
    //flame::dump_html(&mut file).unwrap();
}
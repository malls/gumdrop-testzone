use tokio;
use serde_json;

use gumdrop::archive_getter::{Message, get_file_contents};


#[tokio::main]
async fn main() {

    let msg: &str = "{\"event_type\":\"mod\", \"file_path\":\"/Users/work/Desktop/file.txt\", \"context\":\"local\"}";

    let msg2: Message = serde_json::from_str(msg).unwrap();

    let res = get_file_contents(msg2).await;
    println!("local res: {:?}", res.ok().unwrap());


    let remote_msg: &str = "{\"event_type\":\"mod\", \"file_path\":\"https://c088-74-72-159-164.ngrok.io/file.txtxx\", \"context\":\"remote\"}";
    let remote_msg2: Message = serde_json::from_str(remote_msg).unwrap();

    let res = get_file_contents(remote_msg2).await;
    println!("remote res: {:?}", res.ok().unwrap());


}

use tokio;
use serde_json;

use gumdrop::archive_getter;


#[tokio::main]
async fn main() {

    let msg = "{\"event_type\":\"mod\", \"file_path\":\"/Users/work/Desktop/file.txt\", \"context\":\"local\"}";

    let msg2: archive_getter::Message = serde_json::from_str(msg).unwrap();


    let res = archive_getter::get_file_contents(msg2).await;

    println!("{:?}", res.ok());

}

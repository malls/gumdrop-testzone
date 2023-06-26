use gumdrop::archive_getter;



fn main() {

    let msg = "{\"event_type\":\"mod\", \"file_path\":\"~/Desktop/file.txt\", \"context\":\"local\"}";

    let res = archive_getter::get_file_contents(msg);

    println!("{:?}", res);

}

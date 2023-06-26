use gumdrop;


fn main() {

    let msg = "{\"event_type\":\"mod\", \"file_path\":\"~/Desktop/file.txt\", \"context\":\"local\"}";

    let res = gumdrop::archive_getter(msg);

    println!("{}", res)

}

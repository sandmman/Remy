use std::env;
use std::fs;
use std::io::Read;

fn main() {

    // Read from rom
    let file_name = env::args().nth(1).unwrap();        // get file name form console
    let mut file = fs::File::open(&file_name).unwrap(); // open file
    let mut file_buf = Vec::new();

    file.read_to_end(&mut file_buf).unwrap();           // read the file into a buffer
    let file_buf = file_buf;
                              // make it into an unmutable binding
    println!("{}",file_name);

}

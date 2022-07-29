use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::env;

fn read_bytes(filename: &String) -> io::Result<Vec<u8>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;

    Ok(buf)
}


// Shows the bytes as ASCII.
fn show_as_chars(chararr: &mut Vec<char>) {
    for i in 0..chararr.len() { 
        print!("{}", chararr[i]);
    }

    chararr.clear();
}


fn dump_bytes(vec: Vec<u8>, use_fileoffset: bool) {
    let mut tochar_array: Vec<char> = Vec::new();
    let mut file_offset = 0;

    for i in 0..vec.len() {
        if i % 8 == 0 {
            show_as_chars(&mut tochar_array);
            print!("\n");

            if use_fileoffset {
                print!("[0x0{:06x}] ", file_offset);
            }
        } 

        print!("0x0{:02x} ", vec[i]);

        if vec[i] >= 32 && vec[i] <= 122 {
            tochar_array.push(vec[i] as char);
        }
        else
        {
            tochar_array.push('.');
        }

        file_offset += 1;
    }
}


fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("OOPS!: Please pass in a filename of the file to dump!");
        return Ok(())
    }
    
    let mut use_file_offset = true;

    for i in 1..args.len() {
        if args[i] == "--no-foff" {
            use_file_offset = false;
            continue;
        }

        dump_bytes(read_bytes(&args[i])?, use_file_offset);
    }
    
    Ok(())
}

use std::io::prelude::*;
use std::io;
use std::fs::File;
fn main(){
    read_all_lines("test.txt");
}

//reading from file

fn read_all_lines(filename: &str) -> io::Result<()>{
    let file = File::open(&filename)?;
    let mut lines = Lines::new(file);
    while let Some(line) = lines.next() {
        let line = line?;
        println!("{} ", line);
    }
    Ok(())
}

struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String,
}

impl <R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines { reader: io::BufReader::new(r), buf: String::new() }
    }


    fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>> {
    self.buf.clear();
        match self.reader.read_line(&mut self.buf){
            Ok(nbyte) => if nbyte == 0 {
                None
            }else {
                let line = self.buf.trim_right();
                Some(Ok(line))
            },
            Err(e) => Some(Err(e))
        }
    }

}
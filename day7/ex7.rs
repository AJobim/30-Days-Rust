//7. Write an enum FileFormat with variants PDF, Word, and Excel, each variant containing a file size (integer). 
//Define a method on the enum to print the file format and size.
#[derive(Debug)]
enum FileFormat {
    PDF(isize),
    Word(isize),
    Excel(isize),
}

impl FileFormat {
    fn print_format(&self) {
        match self {
            FileFormat::PDF(size)   => println!("This file is a PDF with the size of {}kb", size),
            FileFormat::Word(size)  => println!("This file is a Word file with the size of {}kb", size),
            FileFormat::Excel(size) => println!("This file is an Excel file with the size of {}kb", size),
        }
    }
}

fn main(){
    let file = FileFormat::PDF(242);

    file.print_format();
}
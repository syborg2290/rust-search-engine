use std::fs::{self, File};
use std::io;
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};

fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);

    let mut content = String::new();
    for event in er.into_iter() {
        // Only get characters from the XML
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
            // println!("{text}");
        }
    }

    Ok(content)
}

fn main() -> io::Result<()> {
    let dir_path = "docs.gl/gl4";
    let dir = fs::read_dir(dir_path)?;
    for file in dir {
        let file_path = file?.path();
        let content = read_entire_xml_file(&file_path)?;
        // println!("{file:?}", file = file?.path());
        print!("{file_path:?} => {size}", size = content.len());
    }
    Ok(())

    // println!(
    //     "{content}",
    //     content = read_entire_xml_file(file_path).expect("TODO")
    // );

    // let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
    //     eprintln!("ERROR: could not read file {file_path}: {err}");
    //     exit(1);
    // });
    // println!("Length of {file_path} is {length}", length = content.len());
}

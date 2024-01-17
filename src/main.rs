use std::option::Option;
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let Some(file_path): Option<PathBuf> = env::args().nth(1).map(Into::into) else {
        println!("Provide a file path!");
        return;
    };

    // Check if file exists and can be read.
    match file_path.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            println!("File does not exist!");
            return;
        }
        Err(e) => {
            println!("Can't access file! Error: '{e}'");
            return;
        }
    }

    // TODO: Ability to optimize an entire folder of documents, recursively

    // Check if file is pdf.
    if file_path.extension() != Some(OsStr::new("pdf")) {
        println!("Provided file is not a PDF document!");
        return;
    }

    let mut _image_resolution = 72;
    if env::args().len() > 2 {
        let _number: i32 = match env::args().nth(2).unwrap().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Provide an integer to adjust image resolution!");
                return;
            }
        };

        _image_resolution = _number;
    }

    let _before_size = file_path.metadata().unwrap().len() as f32;
    let _outpath_str = modify_output_path(file_path.to_str().unwrap_or(""));
    let _output_path = Path::new(&_outpath_str);

    if let Err(e) = Command::new("gs")
        .arg("-dBATCH")
        .arg("-dNOPAUSE")
        .arg("-q")
        .arg("-dCompatibilityLevel=1.4")
        .arg("-dPDFSETTINGS=/screen")
        .arg(format!("-r{}", _image_resolution))
        .arg("-sDEVICE=pdfwrite")
        .arg(format!("-sOutputFile={}", &_outpath_str))
        .arg(&file_path)
        .output()
    {
        println!("Unable to run GhostScript! Error: '{e}'");
        return;
    }

    println!("Result saved in {}!", &_outpath_str);

    let _after_size = _output_path.metadata().unwrap().len() as f32;

    println!(
        "File size changes: {:.2}MB -> {:.2}MB",
        _before_size / 1000000.0,
        _after_size / 1000000.0
    );
}

fn modify_output_path(input_path: &str) -> String {
    let mut output_path = String::from(input_path);
    output_path.insert_str(output_path.len() - 4, "_compressed");
    output_path
}

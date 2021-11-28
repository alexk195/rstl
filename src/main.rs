use std::env;
use std::fs;
use std::io::Write;

struct Transcode
{
    inside_code : bool // inside multiline code section
}

// transcode one line, returns transcoded line
fn transcode_line(t:&mut Transcode, line:&String) -> String
{
   let tstr = line.trim_start();
   if tstr.starts_with("@@") // multi-line code section
   {
        t.inside_code = !t.inside_code;
        tstr[2..].to_string()
   }
   else
   {
        if t.inside_code
        {
            line.to_string()
        }
        else
        {
            if tstr.starts_with('@') && !tstr.starts_with("@[") // single line of code
            {
                tstr[1..].to_string()
            }
            else
            {
                let prestr = "out(&(String::new()+\"";
                let poststr = "\"));";
                let escstr = line.escape_default().to_string().replace("@[","\"+").replace("]@","+\"");
                let newstr = prestr.to_string()+&escstr.to_string()+&poststr.to_string();
                newstr.to_string()
            }            
        }
   }
}

// append content of fname file to the outfile
fn append_file(outfile: &mut fs::File, fname:&str)
{
    let content = fs::read_to_string(fname).expect(&(String::new()+"Error reading "+fname));
    outfile.write_all(content.as_bytes()).expect("Error writing to file");
}

// transcode rstl file into rs file
fn transcode_file(filename:&str, filename_out:&str)
{
    // create resulting file by appending precode.rs, transcoded content, postcode.rs and rstl.rs 

    let contents = fs::read_to_string(filename)
        .expect(&(String::new()+"Not able to read file"+filename));
    let lines = contents.lines();

    let mut transcode = Transcode
    {
        inside_code : false
    };

    let mut outfile = fs::File::create(filename_out)
        .expect(&(String::new()+"Error creating file:"+filename_out));

    append_file(&mut outfile,"precode.rs");
    for line in lines {
        let pline = transcode_line(&mut transcode, &line.to_string());
        outfile.write_all(pline.as_bytes()).expect("Error writing to file");
        outfile.write(b"\n").expect("Error writing to file");
    }
    append_file(&mut outfile,"postcode.rs");
    append_file(&mut outfile,"rstl.rs");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2
    {
        transcode_file( &args[1],&args[2]);
        println!("Transcoded {} -> {}",&args[1],&args[2]);
    }
    else
    {
        println!("Usage: {} infile outfile",args[0]);
    }
   
}




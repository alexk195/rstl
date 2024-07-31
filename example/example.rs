// start of precode.rs
use rstl::*;
pub fn exec_template(args:Vec<String>) 
{
//end of precode.rs
out(&(String::new()+"Hello template"));
let param="Myparam";
let param2="Myparam2";
folder("myfolder");
file(&(String::new() + "myfolder/" + &args[1]));
out(&(String::new()+"line 2 with "+param+" and \""+param2+"\""));
out(&(String::new()+"  line without params and quotes"));
out(&(String::new()+"\t   "+param+" line starts with param"));
manual_begin("section1");      
out(&(String::new()+"\t   line with \\n inside"));
out(&(String::new()+"\t   line with \\ and \\\\ and \\\\\\ "));
out(&(String::new()+"\t   next line"));
manual_end();      
 // code starts here
   // code continues
 // code stops
out(&(String::new()+"line after code   "));
close();
warning("Just be warned");
out(&(String::new()+"Template done"));
out(&(String::new()+"   "));
// begin of postcode.rs
}

fn main()
{
   exec_template(std::env::args().collect());
}

// end of postcode.rs
/**
 * Rstl module for Rust template language
 */

pub mod rstl
{

    use std::io::Write;

    // Writing strings into file which might already exist and contain manually edited code
    // in special sections. This sections must be preserved
    pub struct RstlWriter
    {
        out_fname  : String, // filename of current out file
        old_exist  : bool, // true if old out file exits
        old_cont   : String, // content of old file
        new_cont   : Vec<String>, // content of new file
        man_sec_begin : String, // key string for manual section begin
        man_sec_end : String // ... and for end
    }

    // Implementation for RstlWriter
    impl RstlWriter
    {
        // constructor expects out file name
        pub fn new(fname:&str) -> RstlWriter
        {
            let mut r = RstlWriter {
                out_fname : String::new()+fname,
                old_exist : false,
                old_cont : String::new(),
                new_cont : Vec::new(),
                man_sec_begin : String::from("//--rstl--@[id]@--begin"),
                man_sec_end : String::from("//--rstl--@[id]@--end")
            };

            r.load_old_file();
            r
        }

        // load old file into old_cont
        fn load_old_file(&mut self)
        {
            let exs = std::path::Path::new(&self.out_fname).exists(); 
            if exs{
                self.old_cont  = std::fs::read_to_string(&self.out_fname)
                    .expect(&(String::new()+"Not able to read file"+&self.out_fname));
            }
            self.old_exist = exs;
        }

        // write a line into new content
        fn writeln(&mut self, s:&str)
        {
            self.new_cont.push(s.to_string());
        }        

        // returns true if old content matches given content
        fn same_content(&self, newcont:&String)->bool
        {
            let mut oldcontstr = Vec::new();
            for l in self.old_cont.lines()
            {
                oldcontstr.push(l.to_string());
            }

            let mut oldcontstrn = oldcontstr.join("\n");
            oldcontstrn.push_str("\n"); 
            
            // keep for debugging purposes
            //let mut f = std::fs::File::create("oldcont.txt").expect("Error creating file");
            //f.write_all(oldcontstrn.as_bytes()).expect("Error writing to file");

            oldcontstrn == *newcont
        }

        // returns string to manual section beginning 
        fn ms_begin_str(&self,msid:&str) -> String
        {
            self.man_sec_begin.replace("@[id]@",msid)
        }

        // returns string to manual section ending 
        fn ms_end_str(&self,msid:&str) -> String
        {
            self.man_sec_end.replace("@[id]@",msid)
        }

        // starts a manual section with given id
        fn begin_ms(&mut self,msid:&str)
        {
            self.writeln(&self.ms_begin_str(msid));
        }

        // ends a manual section with given id
        fn end_ms(&mut self,msid:&str)
        {
            self.writeln(&self.ms_end_str(msid));
        }

        // copy manual section with given id from old to new content
        fn copy_ms(&mut self,msid:&str) -> bool
        {
            let ms_start = self.ms_begin_str(msid);
            let ms_end   = self.ms_end_str(msid);

            let mut inside_ms:bool = false;
            let mut ms_found:bool = false;
            let lines = self.old_cont.lines();
           

            for line in lines
            {

                if line.starts_with(&ms_start)
                {
                    inside_ms = true;
                    ms_found = true;
                }

                if inside_ms
                {
                    let s = line.clone().to_string();
                    self.new_cont.push(s.to_string());
                }

                if line.starts_with(&ms_end)
                {
                    inside_ms = false;
                }                
            }
            
            if inside_ms
            {
                panic!("End of manual section {} not found",msid);
            }

            ms_found
        }

        // writes and closes the out file
        fn write_and_close(&self)
        {
            let mut newcont = self.new_cont.join("\n");
            newcont.push_str("\n");
            
            // check if we have changes in content 
            // write only if content is changes. Changes in line endings are ignored
            if self.same_content(&newcont) == false
            {
                if self.old_exist
                {
                    let backname = String::new()+&self.out_fname+".bak";
                    std::fs::rename(&self.out_fname,&backname).expect("Error while renaming file");
                    println!("File {} was backuped to {} ",&self.out_fname,&backname);
                }

                let mut f = std::fs::File::create(&self.out_fname).expect("Error creating file");
                f.write_all(newcont.as_bytes()).expect("Error writing to file");
                f.sync_all().expect("Error syncing file");
                println!("File {} written",&self.out_fname);
            }
            else
            {
                println!("File {} not modified",&self.out_fname);
            }
        }
    }

    // Context for rstl 
    pub struct RstlCtx
    {
        writer : Option<RstlWriter>, // file writer 
        skip_ms : bool, // we are currently in manual section and output is skipped
        inside_ms : bool, // checking for correct manual sections start/end
        msid : String // current manual section id
    }

    static mut THE:RstlCtx = RstlCtx {
        writer : None,
        skip_ms : false,
        inside_ms : false,
        msid : String::new()
    };

    // Template functions

    // output to file/stdout
    pub fn out(s:&str) 
    {
        unsafe
        {

            if THE.skip_ms == false
            {
                match & mut THE.writer
                {
                    None => {println!("{}",s); },
                    Some(wr) => {
                        wr.writeln(s);
                    }
                }
            }

        }
    }

    // creates a file, output is piped to file than
    pub fn file(fname:&str)
    {
        unsafe
        {
            match & mut THE.writer
            {
                None => {},
                Some(_wr) => {
                    println!("New file opened before previous was closed. Old file will be closed.");
                    close();
                }
            }

            THE.writer = Some(RstlWriter::new(fname));
        }
    }

    // closes current file, output is piped to stdout than
    pub fn close()
    {
        unsafe
        {
            match & mut THE.writer
            {
                None => { println!("No file was open"); },
                Some(wr) => {
                    wr.write_and_close();
                }
            }

            THE.writer = None;
        }
    }

    // creates a folder with given path
    pub fn folder(path:&str)
    {
        std::fs::create_dir_all(path).expect(&(String::new()+"Folder could not be created:"+path));
    }

    // initiates manual section with given manual section id (msid)
    pub fn manual_begin(msid:&str)
    {
        unsafe
        {
            if THE.inside_ms
            {
                panic!("Nested manual sections, current is {}",THE.msid);
            }
            else
            {
                THE.inside_ms = true;
            }
            
            THE.msid = msid.to_string();

            match & mut THE.writer
            {
                None => {},
                Some(wr) => {
                    if wr.copy_ms(msid)
                    {
                        THE.skip_ms = true;
                    }
                    else
                    {
                        wr.begin_ms(msid);
                    }
                }
            }

        }
    }

    // closes manual section
    pub fn manual_end()
    {
        unsafe
        {
            if false == THE.inside_ms
            {
                panic!("manual_end failed: Not inside manual section");
            }
            else
            {
                THE.inside_ms = false;
            }

            if THE.skip_ms
            {
                THE.skip_ms = false;
            }
            else
            {
                match & mut THE.writer
                {
                    None => {},
                    Some(wr) => {
                        wr.end_ms(&THE.msid);
                    }
                }
            }
        }
    }

    // sets manual patters, include @[id]@ into the patterns. They will be replaced by given id later in method manual_begin/manual end
    pub fn manual_patterns(pbegin:&str, pend:&str)
    {
        unsafe
        {
            match & mut THE.writer
            {
                None => {},
                Some(wr) => {
                    wr.man_sec_begin = pbegin.to_string();
                    wr.man_sec_end = pend.to_string();
               }
            }
        }
    }

    // prints an error and exits
    pub fn error(error_str:&str)
    {
        panic!("Error: {}",error_str);
    }

    // prints a warning to stderr
    pub fn warning(warning_str:&str)
    {
        eprintln!("Warning: {}",warning_str);
    }

}


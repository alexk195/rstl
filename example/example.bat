copy ..\lib\*.rs .
cargo run example.rstl example.rs
rustc -o example.exe example.rs 
example.exe myfile.txt

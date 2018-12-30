Remove-Item -r -Force "./target/debug/content"
Copy-Item "./content/" -Destination "./target/debug/" -Recurse
cargo build
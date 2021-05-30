# About JmSscanner 
[![Rust](https://github.com/sidhaler/JmSscanner/actions/workflows/rust.yml/badge.svg)](https://github.com/sidhaler/JmSscanner/actions/workflows/rust.yml)
****
JmSscanner is simple port scanner. After ver. 1.5, sudo privileges are required to run scanner.
### Options:
```
-h || prints help
-p || port range from x to y, example: -p 44-443
-n || network scan, example: 192.168.1.0/24 -n  || note: It will be added in short future 
-w || scanning for websites on host, example: 192.168.1.1 -p 20-2000 -w 
```
note: _max range of ports is 65535_
# LINUX (Ubuntu) :
****
I don't know how scanner is doing on other distros. 

Usage:
```
sudo ./jmsscanner [TARGET] [OPTIONS]
```
Examples:
```
sudo ./jmsscanner 192.168.1.1 -p 33-1000
```


# Windows(it's retarded don't ever use this version) :
****
##### Current version of scanner doesn't compile on windows. 

Experimental usage:
```` 
jmsscanner.exe [TARGET] [OPTIONS]
````
Examples:
```
jmsscanner.exe 192.168.1.1 -p 33-1000
```

# Compiling : 

```` 
cargo run *target required* [options are otpional^^]
````
# Building :

#### For normal build go into src directory, then use ↓
```` 
cargo build
````
After that u will have release version ready in jmsscanner/target/debug
****

#### For building release version ↓
```` 
cargo build --release 
````
After that u will have release version ready in jmsscanner/target/release
****


#### For building .exe, on linux ↓
````
cargo build --release --target x86_64-pc-windows-gnu
````
After that u will have release version ready in jmsscanner/target/x86_64-pc-windows-gnu

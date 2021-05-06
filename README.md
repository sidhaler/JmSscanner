
# About JmSscanner 
****
JmSscanner is simple port scanner.
### Options:
```
-h || prints help
-p || port range from x to y, example: -p 44-443
-n || network scan, example: 192.168.1.0/24 -n  
-w || scanning for websites on host, example: 192.168.1.1 -p 20-2000 -w 
```
note: _max range of ports is 65535_
# LINUX (Ubuntu) :
****
I don't know how scanner is doing on other distros. 

Usage:
```
./jmsscanner [TARGET] [OPTIONS]
```
Examples:
```
./jmsscanner 192.168.1.1 -p 33-1000
```



# Windows(its retarded don't ever use this version) :
****
note:
_windows version is too bugged for normal use. I will try to rewrite project for better windows performance but in long future._


Experimental usage:
```` 
jmsscanner.exe [TARGET] [OPTIONS]
````
Examples:
```
jmsscanner.exe 192.168.1.1 -p 33-1000
```

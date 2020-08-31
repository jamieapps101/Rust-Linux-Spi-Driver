# SPI bus rust lib
Jamie Apps


## Structure

- All useful functionality written in C
- bindings created in rust, with tests written in rust


## Warning

- This is strongly in development and only a core set of features work right now. see the tests section for a simple working example. 
- This was developed targeting a raspberry pi zero w, which is a 32bit os board. When used on other devices running 64 bit linux, some of the types may need to be changed. I plan to add features to allow users to specify this in their cargo.toml . 
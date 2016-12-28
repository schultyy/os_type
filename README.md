[![Build Status](https://travis-ci.org/schultyy/os_type.svg?branch=master)](https://travis-ci.org/schultyy/os_type)

# os_type
Rust library to detect the operating system type

## Explain  
`os_type::current_platform()` return a enum.   

impl  `Debug, Display , To_String` for the enum.

## Usage

Include this into your `Cargo.toml`:

```toml
[dependencies]
os_type="0.7.0"
```

In your code:

```rust
extern crate os_type;

fn main() {
    println!("Current OS test:");
    println!("Debug: {:?}", os_type::current_platform());
    println!("Display: {}", os_type::current_platform());
    let os_type = os_type::current_platform().to_string();
    println!("to_string: {}", os_type);
    match os_type::current_platform().to_string().as_ref() {
        os @ "Windows" => {
            // Do something here
        }
        os @ "OSX" => {}
        os @ "openSUSE" => {}        
        os @ _ => {}
    };
}
```

Right now, the following operating systems are detected:

- Mac OS X
- Windows
- CentOS
- RedHat
- Ubuntu
- openSUSE
- Mint
- Manjaro
- elementary
- Fedora
- Zorin
- deepin

If you need support for more OS types, I am looking forward to your Pull Request.

## License

MIT

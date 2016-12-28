[![Build Status](https://travis-ci.org/schultyy/os_type.svg?branch=master)](https://travis-ci.org/schultyy/os_type)

# os_type
Rust library to detect the operating system type

## Usage

Include this into your `Cargo.toml`:

```toml
[dependencies]
os_type="0.5.0"
```

In your code:

```rust
extern crate os_type;

fn foo() {
    match format!("{}", os_type::current_platform()).as_ref() {
        os @ "Windows" => {
            // Do something here
        }
        os @ "OSX" => {}
        os @ "openSUSE" => {}        
        os @ _ => {}
    };
    println!("os_type: {}", os_type::current_platform());
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

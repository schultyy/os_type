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
      match os_type::current_platform() {
        os_type::OSType::Windows => {
            // Do something here ...
        }
        os_type::OSType::OSX => {
            // Do something here ...
        }
        os_type::OSType::Distro("openSUSE") =>{
             // Do something here ...
        },
        _ => {}
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

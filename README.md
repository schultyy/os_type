[![Build Status](https://travis-ci.org/schultyy/os_type.svg?branch=master)](https://travis-ci.org/schultyy/os_type)

# os_type
Rust library to detect the operating system type

## Usage

Include this into your `Cargo.toml`:

```toml
[dependencies]
os_type="1.0.0"
```

In your code:

```rust
extern crate os_type;
let os = os_type::current_platform();
println!("Type: {:?}", os.os_type);
println!("Version: {}", os.version);
```

Or to provide different handling on different operating systems:

```rust
match os_type::current_platform().os_type {
  os_type::OSType::OSX => {
    println!("This is probably an apple laptop!");
  }
  os_type::OSType::Ubuntu => {
    println!("This is running Ubuntu Linux!");
  }
  _ => {
    println!("I can't tell what system this is.");
  }
}
```


Right now, the following operating system types can be returned:
- Unknown
- Redhat
- CentOS
- OSX
- Ubuntu
- Debian
- Arch

If you need support for more OS types, I am looking forward to your Pull Request.

## License

MIT

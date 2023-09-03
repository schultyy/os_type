[![Rust](https://github.com/schultyy/os_type/actions/workflows/rust.yml/badge.svg)](https://github.com/schultyy/os_type/actions/workflows/rust.yml)

# os_type
Rust library to detect the operating system type, because sometimes you need to know.

## Usage

Include this into your `Cargo.toml`:

```toml
[dependencies]
os_type="2.6"
```

In your code:

```rust
extern crate os_type;
let os = os_type::current_platform();
println!("Type: {}", os.os_type);
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
    println!("Unknown Operating System");
  }
}
```


Using `os_type::current_platform().os_type`, expect one of these return values:

- Unknown,

### Windows
- Windows,
- Cygwin,

### MacOS
- MacOS,
- OSX,

### Linux
- GenericLinux,
- Alpine,
- Arch,
- CentOS,
- Debian,
- Deepin,
- Fedora,
- Kali,
- Manjaro,
- NixOS,
- OpenSUSE,
- PopOS,
- Redhat,
- Ubuntu,

### BSD
- FreeBSD,

If you need support for more OS types, please consider opening a Pull Request.

## Try project on your computer

```shell
cargo run --example os_type
```

## Contributing

Bug reports and pull requests are welcome on [GitHub](https://github.com/schultyy/os_type).
You can find more information about contributing in the [CONTRIBUTING.md](https://github.com/schultyy/os_type/blob/master/CONTRIBUTING.md).
This project is intended to be a safe, welcoming space for collaboration and discussion, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org/version/1/4/) code of conduct.

## License

MIT

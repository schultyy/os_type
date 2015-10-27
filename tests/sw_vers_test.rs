#[path="../src/sw_vers.rs"]
mod sw_vers;

fn file() -> String {
"
ProductName:	Mac OS X
ProductVersion:	10.10.5
BuildVersion:	14F27
".to_string()
}

#[test]
pub fn parses_product_name() {
    let info = sw_vers::parse(file());
    assert_eq!(info.product_name, "Mac OS X");
}

#[test]
pub fn parses_product_version() {
    let info = sw_vers::parse(file());
    assert_eq!(info.product_version, "10.10.5");
}

#[test]
pub fn parses_build_version() {
    let info = sw_vers::parse(file());
    assert_eq!(info.build_version, "14F27");
}

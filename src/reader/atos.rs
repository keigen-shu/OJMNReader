/// Creates a null-terminated UTF-8 string stored in a byte array into a String.
///
/// # Arguments
///
/// * `arr` - Array to read the UTF-8 string from.
///
/// # Examples
///
/// ```rust
/// let array: [u8; 18] = [b'H',b'e',b'l',b'l',b'o',b' ',b'W',b'o',b'r',b'l',b'd',b'!',b'\0',
///                        b'Z',b'a',b'l',b'g',b'o'];
/// assert_eq!(array_to_string(&array), String::from("Hello World!"));
/// ```
pub fn array_to_string(arr: &[u8]) -> String {
    let bin: String = String::from_utf8(Vec::from(arr)).unwrap_or(String::new());
    match bin.find('\0') {
        Some(at) => String::from(bin.split_at(at).0),
        None     => bin
    }
}

/// Creates a null-terminated UTF-8 string stored in a vector of byte arrays
/// into a String. For when you have fixed-length C-string field split into
/// multiple [u8] chunks because Rust's Debug trait doesn't support arrays with
/// more than 32 elements.
///
/// # Arguments
///
/// * `arrays` - Vector of arrays to read the UTF-8 string from.
///
/// # Examples
///
/// ```rust
/// let array1: &[u8] = &[b'H',b'e',b'l',b'l',b'o',b' '];
/// let array2: &[u8] = &[b'W',b'o',b'r',b'l',b'd',b'!',b'\0', b'Z',b'a',b'l',b'g',b'o'];
/// assert_eq!(arrays_to_string(Vec::from([&array1, &array2]), String::from("Hello World!"));
/// ```
pub fn arrays_to_string(arrays: Vec<&[u8]>) -> String {
    let mut stop: bool = false;
    let mut string: String = String::new();
    for arr in arrays {
        let bin: String = String::from_utf8(Vec::from(arr)).unwrap_or(String::new());
        string.push_str(
            match bin.find('\0') {
                Some(at) => { stop = true; bin.split_at(at).0 },
                None     => bin.as_str()
            });
        if stop { break };
    };

    string
}

#[test]
fn behaviour() {
    let array: [u8; 18] = [b'H',b'e',b'l',b'l',b'o',b' ',b'W',b'o',b'r',b'l',b'd',b'!',b'\0',
                           b'Z',b'a',b'l',b'g',b'o'];
    assert_eq!(array_to_string(&array), String::from("Hello World!"));

    let array1: &[u8] = &[b'H',b'e',b'l',b'l',b'o',b' '];
    let array2: &[u8] = &[b'W',b'o',b'r',b'l',b'd',b'!',b'\0',b'Z',b'a',b'l',b'g',b'o'];
    assert_eq!(arrays_to_string(Vec::from(&[array1, array2] as &[_])), String::from("Hello World!"));
}
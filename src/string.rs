// libera::string
//
//!
//

/// Returns a string where you always know each character's position.
///
/// A [*counter string*][0] is a graduated string of arbitrary `length`,
/// with asterisks positioned after the immediately preceding number.
///
/// ## Example
///
/// ```
/// # use libera::counter_string;
/// assert_eq!("2*4*6*8*11*14*", counter_string(14));
/// assert_eq!("*3*5*7*9*12*15*", counter_string(15));
/// ```
///
/// [0]: https://www.satisfice.com/blog/archives/22
pub fn counter_string(mut length: usize) -> String {
    let mut cstr = String::new();

    while length > 0 {
        let mut tmpstr = "*".to_string();
        tmpstr.push_str(&length.to_string().chars().rev().collect::<String>());

        if tmpstr.len() > length {
            tmpstr = tmpstr[..length].to_string();
        }

        cstr.push_str(&tmpstr);
        length -= tmpstr.len();
    }
    cstr.chars().rev().collect::<String>()
}

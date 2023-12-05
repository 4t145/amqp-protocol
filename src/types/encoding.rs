// constructor = format-code
// / %x00 descriptor constructor
// format-code = fixed / variable / compound / array
// fixed = empty / fixed-one / fixed-two / fixed-four
// / fixed-eight / fixed-sixteen
// variable = variable-one / variable-four
// compound = compound-one / compound-four
// array = array-one / array-four
// descriptor = value
// value = constructor untyped-bytes
// untyped-bytes = *OCTET ; this is not actually *OCTET, the
// ; valid byte sequences are restricted
// ; by the constructor
// ; fixed width format codes
// empty = %x40-4E / %x4F %x00-FF
// fixed-one = %x50-5E / %x5F %x00-FF
// fixed-two = %x60-6E / %x6F %x00-FF
// fixed-four = %x70-7E / %x7F %x00-FF
// fixed-eight = %x80-8E / %x8F %x00-FF
// fixed-sixteen = %x90-9E / %x9F %x00-FF
// ; variable width format codes
// variable-one = %xA0-AE / %xAF %x00-FF
// variable-four = %xB0-BE / %xBF %x00-FF
// ; compound format codes
// compound-one = %xC0-CE / %xCF %x00-FF
// compound-four = %xD0-DE / %xDF %x00-FF
// ; array format codes
// array-one = %xE0-EE / %xEF %x00-FF
// array-four = %xF0-FE / %xFF %x00-FF


pub mod de;
pub mod enc;


#[cfg(test)]
#[test]
fn test_decode() {
    use serde::Deserialize;

    use crate::types::{value::Value, encoding::de::{reader::Decode, slice::View}};

    let code = b"\x00\xA1\x03URL\xA1\x1Ehttp://example.org/hello-world";
    let mut reader = code.as_ref();
    let value = Value::decode(&mut reader).unwrap();
    println!("{:?}", value);
    let code = b"\x00\xA3\x11example:book:list\xC0\x40\x03\xA1\x15AMQP for & by Dummies\xE0\x25\x02\xA1\x0ERob J. Godfrey\x13Rafael H. Schloming\x40";
    let mut reader = code.as_ref();
    let value = Value::decode(&mut reader).unwrap();
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct BookList {
        title: String,
        authors: Vec<String>,
        isbn: Option<String>,
    }

    println!("{:?}", value);
    let booklist = BookList::deserialize(value).unwrap();
    dbg!(booklist);

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct BookListRef<'a> {
        title: &'a str,
        authors: Vec<&'a str>,
        isbn: Option<&'a str>,
    }

    let value = Value::view(&mut code.as_ref()).unwrap();
    println!("{:?}", value);
    let booklist = BookListRef::deserialize(value).unwrap();
    let title_ptr = booklist.title.as_ptr();
    let title_mem_raw = unsafe {code.as_ptr().add(25)};
    dbg!(title_ptr, title_mem_raw);
    assert_eq!(title_ptr, title_mem_raw);
    dbg!(booklist);
}

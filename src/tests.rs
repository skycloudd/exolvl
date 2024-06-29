use crate::{gzip::extract, types::exolvl::Exolvl, Read, Write};
use difference::assert_diff;
use std::io::Cursor;

macro_rules! level_tests {
    ($(($name:ident, $file:literal),)*) => {
        $(
            #[test]
            fn $name() {
                let in_bytes = extract(include_bytes!(concat!("test_files/", $file))).unwrap();

                inner(&in_bytes);
            }
        )*
    };
}

level_tests![
    (t1, "1.exolvl"),
    (t2, "2.exolvl"),
    (t3, "3.exolvl"),
    (t4, "4.exolvl"),
    (legacy1, "legacy1.exolvl"),
];

fn inner(in_bytes: &[u8]) {
    let file = Exolvl::read(&mut Cursor::new(in_bytes)).unwrap();

    let mut out_bytes = Vec::new();
    file.write(&mut out_bytes).unwrap();

    let in_str = format!("{in_bytes:?}");
    let out_str = format!("{out_bytes:?}");

    assert_diff!(&in_str, &out_str, ", ", 0);
}

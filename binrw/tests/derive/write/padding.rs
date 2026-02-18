extern crate binrw;
use super::t;

#[test]
fn padding_big() {
    #[derive(binrw::BinWrite)]
    struct Test(#[bw(pad_size_to = 0x100)] t::Vec<u8>);

    let mut data = binrw::io::Cursor::new(t::Vec::new());
    binrw::BinWrite::write_le(&Test(t::vec![b'a'; 0x80]), &mut data).unwrap();

    let mut expected = t::vec![0; 0x100];
    expected[0..0x80].fill(b'a');
    t::assert_eq!(data.into_inner(), expected);
}

#[test]
fn padding_round_trip() {
    #[derive(binrw::BinRead, binrw::BinWrite)]
    struct Test {
        #[brw(pad_before = 0x2_u32, align_after = 0x8)]
        x: u8,

        #[brw(align_before = 0x4_u32, pad_after = 0x3_u32)]
        y: u8,

        #[brw(pad_size_to = 0x6_u32)]
        z: u32,

        #[brw(align_size_to = 0x3_u32)]
        w: u32,
    }

    let data = &[
        /* pad_before: */ 0, 0, /* x */ 1, /* align: */ 0, 0, 0, 0, 0,
        /* align_before: (none)*/ /* y */ 2, /* pad_after: */ 0, 0, 0, /* z */ 0,
        0xab, 0xcd, 0xef, /* pad_size_to */ 0, 0, /* w */ 0x25,
        /* align_size_to */ 0, 0, 0, 0, 0,
    ];
    let test = <Test as binrw::BinRead>::read_be(&mut binrw::io::Cursor::new(data)).unwrap();

    let mut x = binrw::io::Cursor::new(t::Vec::new());

    binrw::BinWrite::write_options(&test, &mut x, binrw::Endian::Big, ()).unwrap();

    t::assert_eq!(x.into_inner(), data);
}

#[test]
fn padding_one_way() {
    #[derive(binrw::BinRead, binrw::BinWrite)]
    struct Test {
        #[brw(pad_before = 0x2_u32, align_after = 0x8)]
        x: u8,

        #[brw(align_before = 0x4_u32, pad_after = 0x3_u32)]
        y: u8,

        #[brw(pad_size_to = 0x6_u32)]
        z: u32,

        #[brw(align_size_to = 0x3_u32)]
        w: u32,
    }

    let data = &[
        /* pad_before: */ 0, 0, /* x */ 1, /* align: */ 0, 0, 0, 0, 0,
        /* align_before: (none)*/ /* y */ 2, /* pad_after: */ 0, 0, 0, /* z */ 0xef,
        0xcd, 0xab, 0, /* pad_size_to */ 0, 0, /* w */ 0x25, /* align_size_to */ 0,
        0, 0, 0, 0,
    ];

    let mut x = binrw::io::Cursor::new(t::Vec::new());

    binrw::BinWrite::write_options(
        &Test {
            x: 1,
            y: 2,
            z: 0xabcdef,
            w: 0x25,
        },
        &mut x,
        binrw::Endian::Little,
        (),
    )
    .unwrap();

    t::assert_eq!(x.into_inner(), data);
}

fn assert_align_error(err: binrw::Error, keyword: &str) {
    match err {
        binrw::Error::AssertFail { message, .. } => {
            t::assert_eq!(message, t::format!("`{keyword}` must be greater than 0"));
        }
        _ => t::panic!("bad error type"),
    }
}

#[test]
fn align_size_to_zero_is_error() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(align_size_to = 0)]
        x: u8,
    }

    let mut writer = binrw::io::Cursor::new(t::Vec::new());
    let err = binrw::BinWrite::write_le(&Test { x: 1 }, &mut writer).unwrap_err();
    assert_align_error(err, "align_size_to");
}

#[test]
fn align_size_to_negative_is_error() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(align_size_to = -1_i32)]
        x: u8,
    }

    let mut writer = binrw::io::Cursor::new(t::Vec::new());
    let err = binrw::BinWrite::write_le(&Test { x: 1 }, &mut writer).unwrap_err();
    assert_align_error(err, "align_size_to");
}

#[test]
fn align_before_zero_is_error() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(align_before = 0)]
        x: u8,
    }

    let mut writer = binrw::io::Cursor::new(t::Vec::new());
    let err = binrw::BinWrite::write_le(&Test { x: 1 }, &mut writer).unwrap_err();
    assert_align_error(err, "align_before");
}

#[test]
fn align_before_negative_is_error() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(align_before = -1_i32)]
        x: u8,
    }

    let mut writer = binrw::io::Cursor::new(t::Vec::new());
    let err = binrw::BinWrite::write_le(&Test { x: 1 }, &mut writer).unwrap_err();
    assert_align_error(err, "align_before");
}

#[test]
fn align_after_zero_is_error() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(align_after = 0)]
        x: u8,
    }

    let mut writer = binrw::io::Cursor::new(t::Vec::new());
    let err = binrw::BinWrite::write_le(&Test { x: 1 }, &mut writer).unwrap_err();
    assert_align_error(err, "align_after");
}

#[test]
fn align_after_negative_is_error() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(align_after = -1_i32)]
        x: u8,
    }

    let mut writer = binrw::io::Cursor::new(t::Vec::new());
    let err = binrw::BinWrite::write_le(&Test { x: 1 }, &mut writer).unwrap_err();
    assert_align_error(err, "align_after");
}

#[test]
fn padding_fill_value() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(
            fill_value = 0xff_u8,
            pad_before = 0x1_u32,
            align_before = 0x4_u32,
            pad_after = 0x1_u32,
            align_after = 0x8_u32
        )]
        x: u8,
    }

    let mut x = binrw::io::Cursor::new(t::Vec::new());
    binrw::BinWrite::write_options(&Test { x: 0x11 }, &mut x, binrw::Endian::Little, ()).unwrap();

    t::assert_eq!(
        x.into_inner(),
        t::vec![0xff, 0xff, 0xff, 0xff, 0x11, 0xff, 0xff, 0xff]
    );
}

#[test]
fn padding_fill_value_pad_size_to() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(fill_value = 0xaa_u8, pad_size_to = 0x4_u32)]
        x: u8,
    }

    let mut x = binrw::io::Cursor::new(t::Vec::new());
    binrw::BinWrite::write_options(&Test { x: 0x12 }, &mut x, binrw::Endian::Little, ()).unwrap();

    t::assert_eq!(x.into_inner(), t::vec![0x12, 0xaa, 0xaa, 0xaa]);
}

#[test]
fn padding_fill_value_align_size_to() {
    #[derive(binrw::BinWrite)]
    struct Test {
        #[bw(fill_value = 0xbb_u8, align_size_to = 0x4_u32)]
        x: u8,
    }

    let mut x = binrw::io::Cursor::new(t::Vec::new());
    binrw::BinWrite::write_options(&Test { x: 0x13 }, &mut x, binrw::Endian::Little, ()).unwrap();

    t::assert_eq!(x.into_inner(), t::vec![0x13, 0xbb, 0xbb, 0xbb]);
}

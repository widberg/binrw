extern crate binrw;

#[binrw::parser]
fn single_arg_parser(arg: u32) -> binrw::BinResult<u32> {
    binrw::BinResult::Ok(arg)
}

#[binrw::writer(writer)]
fn single_arg_writer(_object: &u32, arg: u32) -> binrw::BinResult<()> {
    binrw::BinWrite::write_le(&arg, writer)
}

#[binrw::binrw]
struct SingleArg {
    #[br(parse_with = single_arg_parser, args(0x42u32))]
    #[bw(write_with = single_arg_writer, args(0x42u32))]
    field: u32,
}

#[test]
fn single_arg() {
    use super::t::*;
    use binrw::{io::Cursor, BinRead, BinWrite};

    let result = SingleArg::read_le(&mut Cursor::new(b"")).unwrap();
    assert_eq!(result.field, 0x42);
    let mut written = Cursor::new(Vec::new());
    result.write_le(&mut written).unwrap();
    assert_eq!(written.into_inner(), b"\x42\x00\x00\x00");
}

mod with_read_only {
    use super::binrw;

    #[binrw::parser(reader, endian)]
    pub fn parse() -> binrw::BinResult<u16> {
        <u8 as binrw::BinRead>::read_options(reader, endian, ())
            .map(<u16 as ::core::convert::From<_>>::from)
    }
}

#[test]
fn field_with_read() {
    use super::t::*;
    use binrw::{io::Cursor, BinRead};

    #[derive(binrw::BinRead)]
    #[br(big)]
    struct Test {
        #[br(with = with_read_only)]
        value: u16,
    }

    let parsed = Test::read(&mut Cursor::new(b"\x24")).unwrap();
    assert_eq!(parsed.value, 0x24);
}

mod with_write_only {
    use super::binrw;

    #[binrw::writer(writer, endian)]
    pub fn write(value: &u16) -> binrw::BinResult<()> {
        <u8 as binrw::BinWrite>::write_options(
            &<u8 as ::core::convert::TryFrom<_>>::try_from(*value).unwrap(),
            writer,
            endian,
            (),
        )
    }
}

#[test]
fn field_with_write() {
    use super::t::*;
    use binrw::{io::Cursor, BinWrite};

    #[derive(binrw::BinWrite)]
    #[bw(big)]
    struct Test {
        #[bw(with = with_write_only)]
        value: u16,
    }

    let mut written = Cursor::new(Vec::new());
    Test { value: 0x7f }.write(&mut written).unwrap();
    assert_eq!(written.into_inner(), b"\x7f");
}

mod with_both {
    use super::binrw;

    #[binrw::parser(reader, endian)]
    pub fn parse() -> binrw::BinResult<u16> {
        <u8 as binrw::BinRead>::read_options(reader, endian, ())
            .map(<u16 as ::core::convert::From<_>>::from)
    }

    #[binrw::writer(writer, endian)]
    pub fn write(value: &u16) -> binrw::BinResult<()> {
        <u8 as binrw::BinWrite>::write_options(
            &<u8 as ::core::convert::TryFrom<_>>::try_from(*value).unwrap(),
            writer,
            endian,
            (),
        )
    }
}

#[test]
fn field_with_read_and_write() {
    use super::t::*;
    use binrw::{io::Cursor, BinRead, BinWrite};

    #[binrw::binrw]
    #[brw(big)]
    struct Test {
        #[brw(with = with_both)]
        value: u16,
    }

    let parsed = Test::read(&mut Cursor::new(b"\x42")).unwrap();
    assert_eq!(parsed.value, 0x42);

    let mut written = Cursor::new(Vec::new());
    parsed.write(&mut written).unwrap();
    assert_eq!(written.into_inner(), b"\x42");
}

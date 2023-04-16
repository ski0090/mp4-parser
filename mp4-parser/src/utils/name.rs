use strum_macros::AsRefStr;

macro_rules! boxtype {
    ($( $name:ident => $value:expr ),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, AsRefStr)]
        pub enum BoxType {
            $( $name, )*
            UnknownBox(u32),
        }

        impl From<u32> for BoxType {
            fn from(t: u32) -> BoxType {
                match t {
                    $( $value => BoxType::$name, )*
                    _ => BoxType::UnknownBox(t),
                }
            }
        }

        impl From<BoxType> for u32 {
            fn from(b: BoxType) -> u32 {
                match b {
                    $( BoxType::$name => $value, )*
                    BoxType::UnknownBox(t) => t,
                }
            }
        }
    }
}

boxtype! {
    Ftyp => 0x66747970,
    Mvhd => 0x6d766864,
    Mfhd => 0x6d666864,
    Free => 0x66726565,
    Mdat => 0x6d646174,
    Moov => 0x6d6f6f76,
    Mvex => 0x6d766578,
    Mehd => 0x6d656864,
    Trex => 0x74726578,
    Emsg => 0x656d7367,
    Moof => 0x6d6f6f66,
    Tkhd => 0x746b6864,
    Tfhd => 0x74666864,
    Tfdt => 0x74666474,
    Edts => 0x65647473,
    Mdia => 0x6d646961,
    Elst => 0x656c7374,
    Mdhd => 0x6d646864,
    Hdlr => 0x68646c72,
    Minf => 0x6d696e66,
    Vmhd => 0x766d6864,
    Stbl => 0x7374626c,
    Stsd => 0x73747364,
    Stts => 0x73747473,
    Ctts => 0x63747473,
    Stss => 0x73747373,
    Stsc => 0x73747363,
    Stsz => 0x7374737A,
    Stco => 0x7374636F,
    Co64 => 0x636F3634,
    Trak => 0x7472616b,
    Traf => 0x74726166,
    Trun => 0x7472756E,
    Udta => 0x75647461,
    Meta => 0x6d657461,
    Dinf => 0x64696e66,
    Dref => 0x64726566,
    UrlB  => 0x75726C20,
    Smhd => 0x736d6864,
    Avc1 => 0x61766331,
    AvcC => 0x61766343,
    Hev1 => 0x68657631,
    HvcC => 0x68766343,
    Mp4a => 0x6d703461,
    Esds => 0x65736473,
    Tx3g => 0x74783367,
    Vpcc => 0x76706343,
    Vp09 => 0x76703039,
    Data => 0x64617461,
    Ilst => 0x696c7374,
    Name => 0xa96e616d,
    Day => 0xa9646179,
    Covr => 0x636f7672,
    Desc => 0x64657363,
    Wide => 0x77696465,
    Elng => 0x656c6e67
}

// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `DoGachaInRollShopScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DoGachaInRollShopScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DoGachaInRollShopScRsp {
    // message fields
    // @@protoc_insertion_point(field:DoGachaInRollShopScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:DoGachaInRollShopScRsp.reward)
    pub reward: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:DoGachaInRollShopScRsp.LEJJOJGNIHK)
    pub LEJJOJGNIHK: u32,
    // @@protoc_insertion_point(field:DoGachaInRollShopScRsp.roll_shop_id)
    pub roll_shop_id: u32,
    // @@protoc_insertion_point(field:DoGachaInRollShopScRsp.AFMPDBBHCHM)
    pub AFMPDBBHCHM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DoGachaInRollShopScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DoGachaInRollShopScRsp {
    fn default() -> &'a DoGachaInRollShopScRsp {
        <DoGachaInRollShopScRsp as ::protobuf::Message>::default_instance()
    }
}

impl DoGachaInRollShopScRsp {
    pub fn new() -> DoGachaInRollShopScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &DoGachaInRollShopScRsp| { &m.retcode },
            |m: &mut DoGachaInRollShopScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "reward",
            |m: &DoGachaInRollShopScRsp| { &m.reward },
            |m: &mut DoGachaInRollShopScRsp| { &mut m.reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEJJOJGNIHK",
            |m: &DoGachaInRollShopScRsp| { &m.LEJJOJGNIHK },
            |m: &mut DoGachaInRollShopScRsp| { &mut m.LEJJOJGNIHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "roll_shop_id",
            |m: &DoGachaInRollShopScRsp| { &m.roll_shop_id },
            |m: &mut DoGachaInRollShopScRsp| { &mut m.roll_shop_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AFMPDBBHCHM",
            |m: &DoGachaInRollShopScRsp| { &m.AFMPDBBHCHM },
            |m: &mut DoGachaInRollShopScRsp| { &mut m.AFMPDBBHCHM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DoGachaInRollShopScRsp>(
            "DoGachaInRollShopScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DoGachaInRollShopScRsp {
    const NAME: &'static str = "DoGachaInRollShopScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.retcode = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reward)?;
                },
                32 => {
                    self.LEJJOJGNIHK = is.read_uint32()?;
                },
                64 => {
                    self.roll_shop_id = is.read_uint32()?;
                },
                16 => {
                    self.AFMPDBBHCHM = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.retcode);
        }
        if let Some(v) = self.reward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LEJJOJGNIHK != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.LEJJOJGNIHK);
        }
        if self.roll_shop_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.roll_shop_id);
        }
        if self.AFMPDBBHCHM != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.AFMPDBBHCHM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(7, self.retcode)?;
        }
        if let Some(v) = self.reward.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.LEJJOJGNIHK != 0 {
            os.write_uint32(4, self.LEJJOJGNIHK)?;
        }
        if self.roll_shop_id != 0 {
            os.write_uint32(8, self.roll_shop_id)?;
        }
        if self.AFMPDBBHCHM != 0 {
            os.write_uint32(2, self.AFMPDBBHCHM)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> DoGachaInRollShopScRsp {
        DoGachaInRollShopScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.reward.clear();
        self.LEJJOJGNIHK = 0;
        self.roll_shop_id = 0;
        self.AFMPDBBHCHM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DoGachaInRollShopScRsp {
        static instance: DoGachaInRollShopScRsp = DoGachaInRollShopScRsp {
            retcode: 0,
            reward: ::protobuf::MessageField::none(),
            LEJJOJGNIHK: 0,
            roll_shop_id: 0,
            AFMPDBBHCHM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DoGachaInRollShopScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DoGachaInRollShopScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DoGachaInRollShopScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoGachaInRollShopScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cDoGachaInRollShopScRsp.proto\x1a\x0eItemList.proto\"\xbb\x01\n\x16\
    DoGachaInRollShopScRsp\x12\x18\n\x07retcode\x18\x07\x20\x01(\rR\x07retco\
    de\x12!\n\x06reward\x18\t\x20\x01(\x0b2\t.ItemListR\x06reward\x12\x20\n\
    \x0bLEJJOJGNIHK\x18\x04\x20\x01(\rR\x0bLEJJOJGNIHK\x12\x20\n\x0croll_sho\
    p_id\x18\x08\x20\x01(\rR\nrollShopId\x12\x20\n\x0bAFMPDBBHCHM\x18\x02\
    \x20\x01(\rR\x0bAFMPDBBHCHMB\x15\n\x13emu.lunarcore.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DoGachaInRollShopScRsp::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
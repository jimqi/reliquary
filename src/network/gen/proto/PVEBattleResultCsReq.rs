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

//! Generated file from `PVEBattleResultCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PVEBattleResultCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PVEBattleResultCsReq {
    // message fields
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.turn_snapshot_hash)
    pub turn_snapshot_hash: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.client_res_version)
    pub client_res_version: u32,
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.stt)
    pub stt: ::protobuf::MessageField<super::BattleStatistics::BattleStatistics>,
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.stage_id)
    pub stage_id: u32,
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.battle_id)
    pub battle_id: u32,
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.end_status)
    pub end_status: ::protobuf::EnumOrUnknown<super::BattleEndStatus::BattleEndStatus>,
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.is_ai_consider_ultra_skill)
    pub is_ai_consider_ultra_skill: bool,
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.cost_time)
    pub cost_time: u32,
    // @@protoc_insertion_point(field:PVEBattleResultCsReq.op_list)
    pub op_list: ::std::vec::Vec<super::BattleOp::BattleOp>,
    // special fields
    // @@protoc_insertion_point(special_field:PVEBattleResultCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PVEBattleResultCsReq {
    fn default() -> &'a PVEBattleResultCsReq {
        <PVEBattleResultCsReq as ::protobuf::Message>::default_instance()
    }
}

impl PVEBattleResultCsReq {
    pub fn new() -> PVEBattleResultCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "turn_snapshot_hash",
            |m: &PVEBattleResultCsReq| { &m.turn_snapshot_hash },
            |m: &mut PVEBattleResultCsReq| { &mut m.turn_snapshot_hash },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_res_version",
            |m: &PVEBattleResultCsReq| { &m.client_res_version },
            |m: &mut PVEBattleResultCsReq| { &mut m.client_res_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BattleStatistics::BattleStatistics>(
            "stt",
            |m: &PVEBattleResultCsReq| { &m.stt },
            |m: &mut PVEBattleResultCsReq| { &mut m.stt },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stage_id",
            |m: &PVEBattleResultCsReq| { &m.stage_id },
            |m: &mut PVEBattleResultCsReq| { &mut m.stage_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battle_id",
            |m: &PVEBattleResultCsReq| { &m.battle_id },
            |m: &mut PVEBattleResultCsReq| { &mut m.battle_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_status",
            |m: &PVEBattleResultCsReq| { &m.end_status },
            |m: &mut PVEBattleResultCsReq| { &mut m.end_status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_ai_consider_ultra_skill",
            |m: &PVEBattleResultCsReq| { &m.is_ai_consider_ultra_skill },
            |m: &mut PVEBattleResultCsReq| { &mut m.is_ai_consider_ultra_skill },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cost_time",
            |m: &PVEBattleResultCsReq| { &m.cost_time },
            |m: &mut PVEBattleResultCsReq| { &mut m.cost_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "op_list",
            |m: &PVEBattleResultCsReq| { &m.op_list },
            |m: &mut PVEBattleResultCsReq| { &mut m.op_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PVEBattleResultCsReq>(
            "PVEBattleResultCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PVEBattleResultCsReq {
    const NAME: &'static str = "PVEBattleResultCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.turn_snapshot_hash = is.read_bytes()?;
                },
                16 => {
                    self.client_res_version = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.stt)?;
                },
                88 => {
                    self.stage_id = is.read_uint32()?;
                },
                24 => {
                    self.battle_id = is.read_uint32()?;
                },
                96 => {
                    self.end_status = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.is_ai_consider_ultra_skill = is.read_bool()?;
                },
                112 => {
                    self.cost_time = is.read_uint32()?;
                },
                10 => {
                    self.op_list.push(is.read_message()?);
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
        if !self.turn_snapshot_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(13, &self.turn_snapshot_hash);
        }
        if self.client_res_version != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.client_res_version);
        }
        if let Some(v) = self.stt.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.stage_id);
        }
        if self.battle_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.battle_id);
        }
        if self.end_status != ::protobuf::EnumOrUnknown::new(super::BattleEndStatus::BattleEndStatus::BATTLE_END_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.end_status.value());
        }
        if self.is_ai_consider_ultra_skill != false {
            my_size += 1 + 1;
        }
        if self.cost_time != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.cost_time);
        }
        for value in &self.op_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.turn_snapshot_hash.is_empty() {
            os.write_bytes(13, &self.turn_snapshot_hash)?;
        }
        if self.client_res_version != 0 {
            os.write_uint32(2, self.client_res_version)?;
        }
        if let Some(v) = self.stt.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.stage_id != 0 {
            os.write_uint32(11, self.stage_id)?;
        }
        if self.battle_id != 0 {
            os.write_uint32(3, self.battle_id)?;
        }
        if self.end_status != ::protobuf::EnumOrUnknown::new(super::BattleEndStatus::BattleEndStatus::BATTLE_END_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.end_status))?;
        }
        if self.is_ai_consider_ultra_skill != false {
            os.write_bool(15, self.is_ai_consider_ultra_skill)?;
        }
        if self.cost_time != 0 {
            os.write_uint32(14, self.cost_time)?;
        }
        for v in &self.op_list {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PVEBattleResultCsReq {
        PVEBattleResultCsReq::new()
    }

    fn clear(&mut self) {
        self.turn_snapshot_hash.clear();
        self.client_res_version = 0;
        self.stt.clear();
        self.stage_id = 0;
        self.battle_id = 0;
        self.end_status = ::protobuf::EnumOrUnknown::new(super::BattleEndStatus::BattleEndStatus::BATTLE_END_NONE);
        self.is_ai_consider_ultra_skill = false;
        self.cost_time = 0;
        self.op_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PVEBattleResultCsReq {
        static instance: PVEBattleResultCsReq = PVEBattleResultCsReq {
            turn_snapshot_hash: ::std::vec::Vec::new(),
            client_res_version: 0,
            stt: ::protobuf::MessageField::none(),
            stage_id: 0,
            battle_id: 0,
            end_status: ::protobuf::EnumOrUnknown::from_i32(0),
            is_ai_consider_ultra_skill: false,
            cost_time: 0,
            op_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PVEBattleResultCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PVEBattleResultCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PVEBattleResultCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PVEBattleResultCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aPVEBattleResultCsReq.proto\x1a\x15BattleEndStatus.proto\x1a\x0eBat\
    tleOp.proto\x1a\x16BattleStatistics.proto\"\xfd\x02\n\x14PVEBattleResult\
    CsReq\x12,\n\x12turn_snapshot_hash\x18\r\x20\x01(\x0cR\x10turnSnapshotHa\
    sh\x12,\n\x12client_res_version\x18\x02\x20\x01(\rR\x10clientResVersion\
    \x12#\n\x03stt\x18\x08\x20\x01(\x0b2\x11.BattleStatisticsR\x03stt\x12\
    \x19\n\x08stage_id\x18\x0b\x20\x01(\rR\x07stageId\x12\x1b\n\tbattle_id\
    \x18\x03\x20\x01(\rR\x08battleId\x12/\n\nend_status\x18\x0c\x20\x01(\x0e\
    2\x10.BattleEndStatusR\tendStatus\x12:\n\x1ais_ai_consider_ultra_skill\
    \x18\x0f\x20\x01(\x08R\x16isAiConsiderUltraSkill\x12\x1b\n\tcost_time\
    \x18\x0e\x20\x01(\rR\x08costTime\x12\"\n\x07op_list\x18\x01\x20\x03(\x0b\
    2\t.BattleOpR\x06opListB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::BattleEndStatus::file_descriptor().clone());
            deps.push(super::BattleOp::file_descriptor().clone());
            deps.push(super::BattleStatistics::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PVEBattleResultCsReq::generated_message_descriptor_data());
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
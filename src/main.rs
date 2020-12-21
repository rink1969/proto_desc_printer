use protobuf::{Message, compiler_plugin, UnknownValues};
use protobuf::descriptor::{FieldDescriptorProto, FieldDescriptorProto_Type};
use protobuf::types::{ProtobufTypeDouble, ProtobufType, ProtobufTypeFloat, ProtobufTypeInt64, ProtobufTypeUint64, ProtobufTypeInt32, ProtobufTypeFixed64, ProtobufTypeFixed32, ProtobufTypeBool, ProtobufTypeString, ProtobufTypeBytes, ProtobufTypeUint32, ProtobufTypeSfixed32, ProtobufTypeSfixed64, ProtobufTypeSint32, ProtobufTypeSint64};

fn parse_extension(extension_info: &[FieldDescriptorProto], field_number: u32, unknown_values: &UnknownValues) -> Option<(String, String, String)> {
    for ext in extension_info {
        if ext.get_number() == field_number as i32 {
            let name = ext.get_name();
            let (t, v) = match ext.get_field_type() {
                FieldDescriptorProto_Type::TYPE_DOUBLE=> {
                    if let Some(v) = ProtobufTypeDouble::get_from_unknown(unknown_values) {
                        ("doubel", v.to_string())
                    } else {
                        ("doubel", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_FLOAT => {
                    if let Some(v) = ProtobufTypeFloat::get_from_unknown(unknown_values) {
                        ("float", v.to_string())
                    } else {
                        ("float", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_INT64 => {
                    if let Some(v) = ProtobufTypeInt64::get_from_unknown(unknown_values) {
                        ("int64", v.to_string())
                    } else {
                        ("int64", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_UINT64 => {
                    if let Some(v) = ProtobufTypeUint64::get_from_unknown(unknown_values) {
                        ("uint64", v.to_string())
                    } else {
                        ("uint64", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_INT32 => {
                    if let Some(v) = ProtobufTypeInt32::get_from_unknown(unknown_values) {
                        ("int32", v.to_string())
                    } else {
                        ("int32", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_FIXED64 => {
                    if let Some(v) = ProtobufTypeFixed64::get_from_unknown(unknown_values) {
                        ("fixed64", v.to_string())
                    } else {
                        ("fixed64", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_FIXED32 => {
                    if let Some(v) = ProtobufTypeFixed32::get_from_unknown(unknown_values) {
                        ("fixed32", v.to_string())
                    } else {
                        ("fixed32", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_BOOL => {
                    if let Some(v) = ProtobufTypeBool::get_from_unknown(unknown_values) {
                        ("bool", v.to_string())
                    } else {
                        ("bool", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_STRING => {
                    if let Some(v) = ProtobufTypeString::get_from_unknown(unknown_values) {
                        ("string", v)
                    } else {
                        ("string", "".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_GROUP => {
                    ("group", "NA".to_string())
                },
                FieldDescriptorProto_Type::TYPE_MESSAGE => {
                    ("message", "NA".to_string())
                },
                FieldDescriptorProto_Type::TYPE_BYTES => {
                    if let Some(v) = ProtobufTypeBytes::get_from_unknown(unknown_values) {
                        ("bytes", format!("{:?}", v))
                    } else {
                        ("bytes", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_UINT32 => {
                    if let Some(v) = ProtobufTypeUint32::get_from_unknown(unknown_values) {
                        ("uint32", v.to_string())
                    } else {
                        ("uint32", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_ENUM => {
                    ("enum", "NA".to_string())
                },
                FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                    if let Some(v) = ProtobufTypeSfixed32::get_from_unknown(unknown_values) {
                        ("sfixed32", v.to_string())
                    } else {
                        ("sfixed32", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                    if let Some(v) = ProtobufTypeSfixed64::get_from_unknown(unknown_values) {
                        ("sfixed64", v.to_string())
                    } else {
                        ("sfixed64", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_SINT32 => {
                    if let Some(v) = ProtobufTypeSint32::get_from_unknown(unknown_values) {
                        ("sint32", v.to_string())
                    } else {
                        ("sint32", "NA".to_string())
                    }
                },
                FieldDescriptorProto_Type::TYPE_SINT64 => {
                    if let Some(v) = ProtobufTypeSint64::get_from_unknown(unknown_values) {
                        ("sint64", v.to_string())
                    } else {
                        ("sint64", "NA".to_string())
                    }
                },
            };
            return Some((name.to_string(), t.to_string(), v))
        }
    }
    None
}


fn main() {
    compiler_plugin::plugin_main(|r, s| {
        eprintln!("s: {:#?}", s);
        for d in r {
            if s.contains(&d.get_name().to_owned()) {
                eprintln!("*******proto info*******");
                eprintln!("name: {}", d.get_name());
                // print extension info
                let extension_info = d.get_extension();
                eprintln!("*****ext info*****");
                eprintln!("ext info:\n{:#?}", extension_info);
                // print message info
                eprintln!("*****message info*****");
                for m in d.get_message_type() {
                    eprintln!("message: {}", m.get_name());
                    if m.has_options() {
                        let opt = m.get_options();
                        let unknown_fields = opt.get_unknown_fields();
                        for (field_number, values) in unknown_fields {
                            if let Some((name, t, v)) = parse_extension(extension_info, field_number, values) {
                                eprintln!("*option*: {} {} = {}", t, name, v);
                            }
                        }
                    }
                    eprintln!("***field info***");
                    for f in m.get_field() {
                        eprintln!("field info:\n{:#?}", f);
                        if f.has_options() {
                            let opt = f.get_options();
                            let unknown_fields = opt.get_unknown_fields();
                            for (field_number, values) in unknown_fields {
                                if let Some((name, t, v)) = parse_extension(extension_info, field_number, values) {
                                    eprintln!("*option*: {} {} = {}", t, name, v);
                                }
                            }
                        }
                    }
                }
            }
        }

        let results: Vec<compiler_plugin::GenResult> = Vec::new();
        results
    });
}

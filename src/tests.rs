use crate::{ast, parse};
use std::borrow::Cow;

macro_rules! parse_ast {
    ($file:literal) => {{
        let data = include_str!(concat!("../proto/tests/", $file));

        match parse(&data) {
            Err(error) => panic!("{}", error),
            Ok(ast) => ast,
        }
    }};
}

#[test]
fn empty() {
    let ast = parse_ast!("empty.proto");
    assert!(ast.is_empty());
}

#[test]
fn syntax() {
    let ast = parse_ast!("syntax.proto");
    let target_ast = vec![ast::RootEntry::syntax("proto3")];

    assert_eq!(ast, target_ast);
}

#[test]
fn package_simple() {
    let ast = parse_ast!("package-simple.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::package("mypkg"),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn package_complex() {
    let ast = parse_ast!("package-complex.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::package("my.pkg"),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn import() {
    let ast = parse_ast!("import.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::import("google/protobuf/any.proto"),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn message_empty() {
    let ast = parse_ast!("message-empty.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::message(ast::Message::empty("Empty")),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn message() {
    let ast = parse_ast!("message.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::message(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::reserved_indices(vec![
                    ast::Range::from(2..3),
                    ast::Range::from(6..),
                ]),
                ast::MessageEntry::reserved_idents(["sample"]),
                ast::MessageEntry::field(ast::Field::new(
                    ast::FieldModifier::None,
                    "bool",
                    "first",
                    1,
                    vec![],
                )),
                ast::MessageEntry::field(ast::Field::new(
                    ast::FieldModifier::Optional,
                    "string",
                    "third",
                    3,
                    vec![],
                )),
                ast::MessageEntry::field(ast::Field::new(
                    ast::FieldModifier::Repeated,
                    "uint64",
                    "fourth",
                    4,
                    vec![],
                )),
                ast::MessageEntry::field(ast::Field::new(
                    ast::FieldModifier::None,
                    "map<string, string>",
                    "fifth",
                    5,
                    vec![],
                )),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn message_inner() {
    let ast = parse_ast!("message-inner.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::message(ast::Message::new(
            "Parent",
            vec![
                ast::MessageEntry::message(ast::Message::new(
                    "Child",
                    vec![ast::MessageEntry::field(ast::Field::new(
                        ast::FieldModifier::None,
                        "bool",
                        "var",
                        1,
                        vec![],
                    ))],
                )),
                ast::MessageEntry::field(ast::Field::new(
                    ast::FieldModifier::None,
                    "Child",
                    "child",
                    1,
                    vec![],
                )),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn r#enum() {
    let ast = parse_ast!("enum.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::r#enum(ast::Enum::new(
            "Enum",
            vec![
                ast::EnumEntry::variant("ZERO", 0, vec![]),
                ast::EnumEntry::variant("POSITIVE", 1, vec![]),
                ast::EnumEntry::variant("NEGATIVE", -1, vec![]),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn options() {
    let ast = parse_ast!("options.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::import("google/protobuf/descriptor.proto"),
        ast::RootEntry::option(ast::Option::new(
            "java_multiple_files",
            ast::MapValue::boolean(true),
        )),
        ast::RootEntry::option(ast::Option::new(
            "java_package",
            ast::MapValue::string("xd.xd"),
        )),
        ast::RootEntry::extend(ast::Extend::new(
            "google.protobuf.EnumValueOptions",
            vec![ast::ExtendEntry::field(ast::Field::new(
                ast::FieldModifier::Optional,
                "bool",
                "own_enum_value",
                2000,
                vec![],
            ))],
        )),
        ast::RootEntry::extend(ast::Extend::new(
            "google.protobuf.FieldOptions",
            vec![ast::ExtendEntry::field(ast::Field::new(
                ast::FieldModifier::Optional,
                "bool",
                "own_field_value",
                2000,
                vec![ast::Option::new("deprecated", ast::MapValue::boolean(true))],
            ))],
        )),
        ast::RootEntry::r#enum(ast::Enum::new(
            "Enum",
            vec![
                ast::EnumEntry::option(ast::Option::new(
                    "allow_alias",
                    ast::MapValue::boolean(true),
                )),
                ast::EnumEntry::variant(
                    "FIRST",
                    0,
                    vec![ast::Option::new("deprecated", ast::MapValue::boolean(true))],
                ),
                ast::EnumEntry::variant(
                    "SECOND",
                    0,
                    vec![ast::Option::new(
                        "(own_enum_value)",
                        ast::MapValue::boolean(true),
                    )],
                ),
            ],
        )),
        ast::RootEntry::message(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::option(ast::Option::new(
                    "deprecated",
                    ast::MapValue::boolean(true),
                )),
                ast::MessageEntry::field(ast::Field::new(
                    ast::FieldModifier::Optional,
                    "bool",
                    "var",
                    1,
                    vec![
                        ast::Option::new("deprecated", ast::MapValue::boolean(true)),
                        ast::Option::new("(own_field_value)", ast::MapValue::boolean(false)),
                        ast::Option::new(
                            "edition_defaults",
                            ast::MapValue::map(ast::Map::from([
                                (Cow::from("edition"), ast::MapValue::ident("EDITION_PROTO2")),
                                (Cow::from("value"), ast::MapValue::string("true")),
                            ])),
                        ),
                        ast::Option::new(
                            "edition_defaults",
                            ast::MapValue::map(ast::Map::from([
                                (Cow::from("edition"), ast::MapValue::ident("EDITION_PROTO3")),
                                (Cow::from("value"), ast::MapValue::string("false")),
                            ])),
                        ),
                    ],
                )),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn comments() {
    let ast = parse_ast!("comments.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::import("google/protobuf/descriptor.proto"),
        ast::RootEntry::comment(ast::Comment::single_line("// single line comment")),
        ast::RootEntry::comment(ast::Comment::single_line("// another single line comment")),
        ast::RootEntry::comment(ast::Comment::multi_line("/* multi\n   line\n   comment */")),
        ast::RootEntry::message(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::comment(ast::Comment::single_line("// in message")),
                ast::MessageEntry::field(ast::Field::new(
                    ast::FieldModifier::None,
                    "bool",
                    "var",
                    1,
                    vec![],
                )),
                ast::MessageEntry::comment(ast::Comment::single_line("// right after entry")),
                ast::MessageEntry::comment(ast::Comment::single_line("// at the bottom")),
            ],
        )),
        ast::RootEntry::r#enum(ast::Enum::new(
            "Enum",
            vec![
                ast::EnumEntry::comment(ast::Comment::single_line("// in enum")),
                ast::EnumEntry::variant("DEFAULT", 0, vec![]),
            ],
        )),
        ast::RootEntry::extend(ast::Extend::new(
            "google.protobuf.FieldOptions",
            vec![
                ast::ExtendEntry::comment(ast::Comment::single_line("// in extend")),
                ast::ExtendEntry::field(ast::Field::new(
                    ast::FieldModifier::Optional,
                    "bool",
                    "var",
                    1,
                    vec![],
                )),
            ],
        )),
        ast::RootEntry::comment(ast::Comment::single_line("// at the bottom of the file")),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn extensions() {
    let ast = parse_ast!("extensions.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto2"),
        ast::RootEntry::message(ast::Message::new(
            "Message",
            vec![ast::MessageEntry::extensions(vec![
                ast::Range::from(1..2),
                ast::Range::from(2..5),
                ast::Range::from(6..),
            ])],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn required() {
    let ast = parse_ast!("required.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto2"),
        ast::RootEntry::message(ast::Message::new(
            "Message",
            vec![ast::MessageEntry::field(ast::Field::new(
                ast::FieldModifier::Required,
                "bool",
                "var",
                1,
                vec![],
            ))],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn keywords() {
    let ast = parse_ast!("keywords.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::message(ast::Message::empty("Ident")),
        ast::RootEntry::message(ast::Message::new(
            "to",
            vec![ast::MessageEntry::message(ast::Message::empty("inner"))],
        )),
        ast::RootEntry::message(ast::Message::empty("max")),
        ast::RootEntry::message(ast::Message::empty("syntax")),
        ast::RootEntry::message(ast::Message::empty("option")),
        ast::RootEntry::message(ast::Message::empty("package")),
        ast::RootEntry::message(ast::Message::empty("import")),
        ast::RootEntry::message(ast::Message::empty("message")),
        ast::RootEntry::message(ast::Message::empty("oneof")),
        ast::RootEntry::message(ast::Message::empty("extend")),
        ast::RootEntry::message(ast::Message::empty("enum")),
        ast::RootEntry::message(ast::Message::empty("reserved")),
        ast::RootEntry::message(ast::Message::empty("extensions")),
        ast::RootEntry::message(ast::Message::empty("optional")),
        ast::RootEntry::message(ast::Message::empty("required")),
        ast::RootEntry::message(ast::Message::empty("repeated")),
        ast::RootEntry::message(ast::Message::empty("map")),
        ast::RootEntry::message(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::field(ast::Field::basic("bool", "var1", 1)),
                ast::MessageEntry::field(ast::Field::basic("Ident", "var2", 2)),
                ast::MessageEntry::field(ast::Field::basic("to", "var3", 3)),
                ast::MessageEntry::field(ast::Field::basic("to.inner", "var4", 4)),
                ast::MessageEntry::field(ast::Field::basic("max", "var5", 5)),
                ast::MessageEntry::field(ast::Field::basic("syntax", "var6", 6)),
                ast::MessageEntry::field(ast::Field::basic("package", "var7", 7)),
                ast::MessageEntry::field(ast::Field::basic("import", "var8", 8)),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn oneof() {
    let ast = parse_ast!("oneof.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::message(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::one_of(ast::OneOf::new(
                    "OneOf",
                    vec![
                        ast::OneOfEntry::option(ast::Option::new(
                            "uninterpreted_option",
                            ast::MapValue::map(ast::Map::from([(
                                Cow::from("string_value"),
                                ast::MapValue::string(""),
                            )])),
                        )),
                        ast::OneOfEntry::field(ast::Field::basic("bool", "oneof_var", 1)),
                    ],
                )),
                ast::MessageEntry::field(ast::Field::basic("bool", "message_var", 2)),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn service() {
    let ast = parse_ast!("service.proto");
    let target_ast = vec![
        ast::RootEntry::syntax("proto3"),
        ast::RootEntry::service(ast::Service::new(
            "Service",
            vec![
                ast::ServiceEntry::option(ast::Option::new(
                    "uninterpreted_option",
                    ast::MapValue::map(ast::Map::from([(
                        Cow::from("string_value"),
                        ast::MapValue::string(""),
                    )])),
                )),
                ast::ServiceEntry::rpc(ast::Rpc::new(
                    "RPC1",
                    "Request",
                    "Reply",
                    ast::RpcStream::new(false, false),
                )),
                ast::ServiceEntry::rpc(ast::Rpc::new(
                    "RPC2",
                    "Request",
                    "Reply",
                    ast::RpcStream::new(true, false),
                )),
                ast::ServiceEntry::rpc(ast::Rpc::new(
                    "RPC3",
                    "Request",
                    "Reply",
                    ast::RpcStream::new(false, true),
                )),
                ast::ServiceEntry::rpc(ast::Rpc::new(
                    "RPC4",
                    "Request",
                    "Reply",
                    ast::RpcStream::new(true, true),
                )),
            ],
        )),
        ast::RootEntry::message(ast::Message::empty("Request")),
        ast::RootEntry::message(ast::Message::empty("Reply")),
    ];

    assert_eq!(ast, target_ast);
}

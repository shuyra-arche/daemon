#[cfg(test)]
mod tests {
    use daemon_declaration::{BUILD, UNIT};
    use unit::{document as d, record};

    const TOML: &str = include_str!("../../unit.toml");

    fn document() -> d::Unit<String> {
        toml_edit::de::from_str(TOML).unwrap()
    }

    fn assert_legacy_policy<S: AsRef<str>>(value: &d::Unit<S>) {
        assert_eq!(value.schema, 1);
        assert_eq!(value.id.as_ref(), UNIT.name);
        assert_eq!(value.loader.image.as_ref(), UNIT.loader.image);
        assert!(matches!(UNIT.loader.space, env::ProgramKind::User));
        assert_eq!(value.loader.space, d::Space::User);
        assert_eq!(
            value.loader.args,
            UNIT.loader
                .args
                .iter()
                .map(|arg| *arg as u64)
                .collect::<Vec<_>>()
        );
        assert_eq!(value.loader.stack, UNIT.loader.stack as u64);

        assert!(UNIT.operator.namespace.is_empty());
        assert!(value.operator.namespace.is_empty());
        assert!(UNIT.identity.aliases.is_empty());
        assert!(value.identity.aliases.is_empty());
        assert!(UNIT.identity.grants.is_empty());
        assert!(value.identity.grants.is_empty());

        assert_eq!(
            UNIT.control.after.unwrap(),
            [
                legacy_unit::Dependency::Bootstrap(legacy_unit::Bootstrap::Operator),
                legacy_unit::Dependency::Target("supervised"),
            ]
        );
        assert_eq!(value.control.creation, d::Creation::Startup);
        // System bootstraps Operator. RTC was the sole PRODUCT target member.
        for providers in [&value.control.requires, &value.control.after] {
            assert_eq!(
                providers.iter().map(AsRef::as_ref).collect::<Vec<_>>(),
                ["rtc"]
            );
        }
        match (value.control.ready, UNIT.control.ready) {
            (d::Ready::Signal(current), legacy_unit::Ready::Signal(previous)) => {
                assert_eq!(current, previous.get());
                assert_eq!(current, unit::READY_MARK.get());
            }
            _ => panic!("ready policy changed"),
        }
        assert_eq!(unit::READY_MARK, legacy_unit::READY_MARK);
        assert!(matches!(
            UNIT.control.ending,
            Some(legacy_unit::Ending::Resident)
        ));
        assert_eq!(value.control.ending, d::Ending::Resident);
        assert_eq!(UNIT.control.supervision, Some("supervised"));
        assert_eq!(
            value.control.supervision.as_ref().map(AsRef::as_ref),
            UNIT.control.supervision
        );
        assert!(UNIT.control.construction.is_empty());
        assert!(value.control.construction.is_empty());
        assert!(UNIT.control.retirement.is_empty());
        assert!(value.control.retirement.is_empty());
        // Daemon supervises this group; only RTC joins it in its own document.
        assert!(value.control.groups.is_empty());
        assert!(!value.control.guarded);
        assert!(UNIT.supply.bindings.is_empty());
        assert!(value.supply.bindings.is_empty());
        assert!(UNIT.valid());
        value.validate().unwrap();
    }

    fn with_ready(literal: &str) -> String {
        let original = format!("\"{}\"", unit::READY_MARK.get());
        assert!(TOML.contains(&original));
        TOML.replace(&original, literal)
    }

    #[test]
    fn toml_preserves_every_legacy_field_and_the_supervision_contract() {
        assert_legacy_policy(&document());
    }

    #[test]
    fn binary_roundtrip_preserves_every_field_and_borrows_strings() {
        let declaration = document();
        let bytes = record::encode(&declaration).unwrap();
        let decoded = record::decode(&bytes).unwrap();
        assert_legacy_policy(&decoded);
        let text = toml_edit::ser::to_string(&decoded).unwrap();
        let owned: d::Unit<String> = toml_edit::de::from_str(&text).unwrap();
        assert_eq!(owned, declaration);

        let begin = bytes.as_ptr() as usize;
        let end = begin + bytes.len();
        for text in [
            decoded.id,
            decoded.loader.image,
            decoded.control.requires[0],
            decoded.control.after[0],
            decoded.control.supervision.unwrap(),
        ] {
            let pointer = text.as_ptr() as usize;
            assert!(pointer >= begin && pointer + text.len() <= end);
        }
    }

    #[test]
    fn runtime_conversion_preserves_the_legacy_native_contract() {
        // Native runtime strings borrow the immutable boot record.
        static BYTES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let bytes = BYTES.get_or_init(|| record::encode(&document()).unwrap());
        let decoded = record::decode(bytes).unwrap();
        assert_legacy_policy(&decoded);
        let native = unit::runtime::UnitFile::try_from(decoded).unwrap();
        assert_eq!(native.name, UNIT.name);
        assert_eq!(native.loader.image, UNIT.loader.image);
        assert!(matches!(native.loader.space, env::ProgramKind::User));
        assert_eq!(native.loader.args, UNIT.loader.args);
        assert_eq!(native.loader.stack, UNIT.loader.stack);
        assert!(native.operator.namespace.is_empty());
        assert_eq!(native.identity.aliases, UNIT.identity.aliases);
        assert_eq!(native.identity.grants, UNIT.identity.grants);
        assert_eq!(native.control.creation, d::Creation::Startup);
        assert!(native.listed());
        assert_eq!(
            native.control.after.as_deref(),
            Some(&[unit::Dependency::Unit("rtc")][..])
        );
        match (native.control.ready, UNIT.control.ready) {
            (unit::Ready::Signal(current), legacy_unit::Ready::Signal(previous)) => {
                assert_eq!(current, previous);
            }
            _ => panic!("native ready policy changed"),
        }
        assert!(matches!(
            (native.control.ending, UNIT.control.ending),
            (unit::Ending::Resident, Some(legacy_unit::Ending::Resident))
        ));
        assert_eq!(native.control.supervision, UNIT.control.supervision);
        assert!(native.control.construction.is_empty());
        assert!(native.control.retirement.is_empty());
        assert!(native.control.groups.is_empty());
        assert!(!native.control.guarded);
        assert!(native.supply.bindings.is_empty());
        assert!(native.valid());
    }

    #[test]
    fn quoted_decimal_and_hex_preserve_the_original_native_ready_mark() {
        assert!(unit::READY_MARK.get() > i64::MAX as u64);
        let hexadecimal = with_ready(&format!("\"0x{:016x}\"", unit::READY_MARK.get()));
        let value: d::Unit<String> = toml_edit::de::from_str(&hexadecimal).unwrap();
        assert_eq!(value, document());
        assert_legacy_policy(&record::decode(&record::encode(&value).unwrap()).unwrap());
    }

    #[test]
    fn integer_and_quoted_u64_forms_survive_binary_record_roundtrip() {
        for (literal, expected) in [
            ("17", 17),
            ("\"17\"", 17),
            ("\"0x11\"", 17),
            ("\"18446744073709551615\"", u64::MAX),
            ("\"0xffffffffffffffff\"", u64::MAX),
        ] {
            let value: d::Unit<String> = toml_edit::de::from_str(&with_ready(literal)).unwrap();
            value.validate().unwrap();
            assert_eq!(value.control.ready, d::Ready::Signal(expected));
            let bytes = record::encode(&value).unwrap();
            let decoded = record::decode(&bytes).unwrap();
            assert_eq!(decoded.control.ready, d::Ready::Signal(expected));
            let text = toml_edit::ser::to_string(&decoded).unwrap();
            let owned: d::Unit<String> = toml_edit::de::from_str(&text).unwrap();
            assert_eq!(owned, value);
        }
    }

    #[test]
    fn negative_and_overflowing_marks_are_rejected_before_record_encoding() {
        for literal in [
            "-1",
            "\"-1\"",
            "\"18446744073709551616\"",
            "\"0x10000000000000000\"",
        ] {
            assert!(toml_edit::de::from_str::<d::Unit<String>>(&with_ready(literal)).is_err());
        }
    }

    #[test]
    fn image_names_the_existing_cargo_binary_and_preserves_the_build_contract() {
        assert_eq!(UNIT.name, "daemon");
        assert_eq!(document().loader.image, BUILD.image);
        assert_eq!(BUILD.image, "daemon");
        assert_eq!(BUILD.manifest, "server/Cargo.toml");
        let manifest: toml_edit::DocumentMut =
            include_str!("../../server/Cargo.toml").parse().unwrap();
        assert_eq!(BUILD.package, "daemon-server");
        assert_eq!(manifest["package"]["name"].as_str(), Some(BUILD.package));
        let bins = manifest["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 1);
        let binary = bins.get(0).unwrap();
        assert_eq!(binary["name"].as_str(), Some(BUILD.bin));
        assert_eq!(BUILD.bin, BUILD.image);
        let entry = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../server")
            .join(binary["path"].as_str().unwrap());
        assert!(entry.is_file());
        assert!(BUILD.features.is_empty());
        assert!(BUILD.default_features);
        assert_eq!(BUILD.target, "riscv64gc-unknown-none-elf");
        assert_eq!(BUILD.profile, "release");
    }
}

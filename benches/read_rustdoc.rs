use benchmark_json::{serde_read_rustdoc, simd_read_rustdoc};
use criterion::{criterion_group, criterion_main};
use criterion::{BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;

const LINTS: &[&str] = &[
    "async_impl_future_equivalence",
    "auto_trait_impl_removed",
    "broken_rustdoc",
    "constructible_struct_adds_field",
    "constructible_struct_adds_private_field",
    "counterexample_for_ignoring_doc_hidden_items",
    "derive_trait_impl_removed",
    "enum_missing",
    "enum_must_use_added",
    "enum_now_doc_hidden",
    "enum_repr_int_changed",
    "enum_repr_int_removed",
    "enum_repr_transparent_removed",
    "enum_struct_field_hidden_from_public_api",
    "enum_struct_variant_field_added",
    "enum_struct_variant_field_missing",
    "enum_struct_variant_field_now_doc_hidden",
    "enum_tuple_variant_field_added",
    "enum_tuple_variant_field_missing",
    "enum_tuple_variant_field_now_doc_hidden",
    "enum_variant_added",
    "enum_variant_hidden_from_public_api",
    "enum_variant_missing",
    "exported_function_changed_abi",
    "features_no_default",
    "features_simple",
    "function_abi_no_longer_unwind",
    "function_changed_abi",
    "function_const_removed",
    "function_export_name_changed",
    "function_feature_changed",
    "function_missing",
    "function_must_use_added",
    "function_now_doc_hidden",
    "function_unsafe_added",
    "infinite_importable_paths",
    "inherent_associated_const_now_doc_hidden",
    "inherent_associated_pub_const_missing",
    "inherent_method_const_removed",
    "inherent_method_missing",
    "inherent_method_must_use_added",
    "inherent_method_now_doc_hidden",
    "inherent_method_unsafe_added",
    "item_relocation",
    "method_moved_to_trait_must_use_added",
    "module_contents_hidden_from_public_api",
    "module_missing",
    "move_item_and_reexport",
    "non_exhaustive",
    "nonbreaking_item_rename",
    "parameter_count_changed",
    "pub_module_level_const_missing",
    "pub_module_level_const_now_doc_hidden",
    "pub_static_missing",
    "pub_static_mut_now_immutable",
    "pub_static_now_doc_hidden",
    "pub_use_handling",
    "repr_c_removed",
    "repr_packed_added_removed",
    "sized_impl_removed",
    "struct_becomes_enum",
    "struct_field_hidden_from_public_api",
    "struct_missing",
    "struct_must_use_added",
    "struct_now_doc_hidden",
    "struct_pub_field_missing",
    "struct_pub_field_now_doc_hidden",
    "struct_repr_transparent_removed",
    "switch_away_from_reexport_as_underscore",
    "switch_to_reexport_as_underscore",
    "template",
    "trait_associated_const_added",
    "trait_associated_const_now_doc_hidden",
    "trait_associated_type_now_doc_hidden",
    "trait_associated_type_removed",
    "trait_default_impl_removed",
    "trait_items_hidden_from_public_api",
    "trait_method_missing",
    "trait_method_now_doc_hidden",
    "trait_method_unsafe_added",
    "trait_method_unsafe_removed",
    "trait_missing",
    "trait_missing_with_major_bump",
    "trait_must_use_added",
    "trait_newly_sealed",
    "trait_no_longer_object_safe",
    "trait_now_doc_hidden",
    "trait_removed_associated_constant",
    "trait_removed_supertrait",
    "trait_unsafe_added",
    "trait_unsafe_removed",
    "traits_hidden_from_public_api",
    "tuple_struct_to_plain_struct",
    "type_hidden_from_public_api",
    "type_marked_deprecated",
    "union_field_missing",
    "union_missing",
    "union_must_use_added",
    "union_now_doc_hidden",
    "union_pub_field_now_doc_hidden",
    "unit_struct_changed_kind",
];

const CSC_ROOT_PATH: &str = "../cargo-semver-checks/";

fn bench_read_rustdocs(c: &mut Criterion) {
    for p in LINTS {
        let mut group = c.benchmark_group(*p);

        let full_path = format!(
            "{}/localdata/test_data/{}/old/rustdoc.json",
            CSC_ROOT_PATH, p
        );
        let file_data = std::fs::read(full_path.clone()).expect("Failed to read file");
        group.bench_with_input(
            BenchmarkId::new("SIMD", "parse"),
            &file_data,
            |b, file_data| {
                b.iter_batched(
                    || (file_data.clone()),
                    |mut data| black_box(simd_read_rustdoc(black_box(&mut data))),
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(BenchmarkId::new("SERDE", "parse"), &file_data, |b, data| {
            b.iter_batched(
                || (data.clone()),
                |data| black_box(serde_read_rustdoc(black_box(&data))),
                BatchSize::SmallInput,
            )
        });
        group.finish();
    }
}

criterion_group!(benches, bench_read_rustdocs);
criterion_main!(benches);

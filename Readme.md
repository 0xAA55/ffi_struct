# FFI Struct

An attribute macro designed to create Rust structs used for FFI directly with adjusted paddings inserted, and reflection support for the struct to be able to iterate through its member info, such as member name, type ID, size, and offset.

## Language｜语言

Chinglish | [简体中文](Readme-CN.md)

## Example

The code below shows the typical usage of the crate:

```Rust
use ffi_struct::*;

fn test() {
	type Vec3 = (f32, f32, f32);
	type TVec3<FT> = (FT, FT, FT);

	#[ffi_struct]
	#[derive(Default, Debug)]
	#[size_of_type(Vec3 = 12, Vec4 = 16)]
	#[align_of_type(Vec3 = 12, Vec4 = 16)]
	struct TestStructRust<FT>
	where
		FT: Default + Clone + Copy + Sized + Any {
		field1: bool,
		#[align = 4]
		field2: u32,
		field3: Vec3,
		#[size = 24]#[align = 16]
		field4: TVec3<FT>,
		#[size = 8]
		field5: FT,
	}

	let ffi = TestStruct::<f64>::default();
	println!("======== members ========");
	for (name, member) in ffi.iter_members() {
		println!("{name}: {member:?}");
	}
	println!("======== all members ========");
	for (name, member) in ffi.iter_all_members() {
		println!("{name}: {member:?}");
	}
}
```

After the `#[ffi_struct]` has been applied to `TestStructRust`, a NEW struct is created and named `TestStruct`, which had the `Rust` suffix removed.

The struct `TestStruct` derives traits that are originally derived to `TestStructRust`, e. g. `Default` and `Debug`.

After specifying the member's alignments, the paddings were added to the new struct `TestStruct`.

## Improvements compared to another crate `struct_iterable`

* Generic type parameters were allowed in the structure.
* Struct member iteration doesn't have to access the struct members, so the tightly packed struct is allowed.
* When specifying alignments, the paddings were added to the new struct.


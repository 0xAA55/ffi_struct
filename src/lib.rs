
pub use std::any::Any;
pub use ffi_struct_derive::ffi_struct;
pub use ffi_struct_trait::{
	FFIStruct,
	MemberInfo,
};

#[test]
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

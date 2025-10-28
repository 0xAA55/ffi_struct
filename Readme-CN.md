# FFI 结构体

属性宏用于创建直接用于 FFI 的 Rust 结构体，并插入调整后的填充，以及结构体的反射支持，使其能够迭代其成员信息，例如成员名称、类型 ID、大小和偏移量。

## 语言｜语言

[Chinglish](Readme.md) | 简体中文

## 示例

以下代码展示了典型用法：

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

将 `#[ffi_struct]` 应用于 `TestStructRust` 后，将创建一个新的结构体，名为 `TestStruct`，其 `Rust` 后缀已被移除。

结构体 `TestStruct` 继承了最初继承自 `TestStructRust` 的特征，例如 `Default` 和 `Debug`。

指定成员的对齐方式后，将填充添加到新的结构体 `TestStruct` 中。

## 与另一个 crate `struct_iterable` 相比的改进

* 结构体中允许使用泛型类型参数。
* 结构体成员迭代无需访问结构体成员，因此允许使用紧密封装的结构体。
* 在指定对齐方式时，新结构体中添加了填充。

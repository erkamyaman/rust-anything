# Types

`bridge` reads each exported function's types from the source and writes the port's signature in
`port.rs` from them. This page lists, per language, what a source type becomes there, and which
types keep a function in its language, with the reason `bridge` gives.

How to read the tables:

- **Port type** is the Rust type of a parameter or result. A port function returns
  `Result<T, String>` and the table gives the `T`; in C it returns `T`, as the C function does.
- `serde_json::Value` means the value crosses as JSON as it is: objects, maps, unions of different
  types and anything the mapping does not know.
- **refused** means the function stays in its language. The note is the reason `bridge` prints,
  where it names the type.
- A parameter with a default becomes `Option<T>`, unless the default is a literal Rust can write
  (a number, `true`/`false` or a plain string), in Python, Ruby, PHP and C#. In TypeScript and
  JavaScript every parameter with a default is `Option<T>`. PowerShell has its own rule, under
  [PowerShell](#powershell).
- `cargo test --test types_doc` checks every row against the mapping, so a row that no longer
  matches the code fails the build.

## TypeScript

Types are read from the annotations. Local type aliases are resolved first.

| Source type | Port type | Notes |
|---|---|---|
| `string` | `String` | |
| `number` | `f64` | |
| `boolean` | `bool` | |
| `void`, `undefined` | `()` | |
| `string[]`, `Array<string>`, `readonly string[]` | `Vec<String>` | |
| `string \| undefined`, `string \| null` | `Option<String>` | |
| `'a' \| 'b'` | `String` | a union of string literals |
| `Promise<boolean>` | `bool` | the port function is `async` |
| `Record<string, number>`, `Map<string, number>`, `{ x: number }` | `serde_json::Value` | |
| `string \| number` | `serde_json::Value` | |
| `(x: number) => number`, `Function` | refused | takes or returns a function |
| `RegExp` | refused | takes or returns a RegExp |
| `Iterator<number>`, `Generator<number>` | refused | a lazy iterator |

A parameter with no annotation takes its JSDoc type, else the type of its default, else
`serde_json::Value`. A rest parameter (`...xs: string[]`) is `Option<Vec<String>>`. A type whose members include a method, and a function's own type parameter
that is not limited to data, are refused too.

## JavaScript

Types are read from JSDoc `@param {T}` and `@returns {T}` tags, or from a `.d.ts` file next to the
source.

| Source type | Port type | Notes |
|---|---|---|
| `{string}`, `{String}` | `String` | |
| `{number}`, `{Number}` | `f64` | |
| `{boolean}` | `bool` | |
| `{string[]}`, `{Array<string>}`, `{Array.<string>}` | `Vec<String>` | |
| `{?number}` | `Option<f64>` | |
| `{number=}` | `Option<f64>` | an optional parameter |
| `{...string}` | `Option<Vec<String>>` | a rest parameter, which may be left out |
| `{Object.<string, number>}`, `{Object}`, `{*}` | `serde_json::Value` | |
| `{function(string): number}` | `serde_json::Value` | not read: the parameter counts as untyped |
| `{RegExp}` | refused | takes or returns a RegExp |

## Python

Types are read from the hints. `typing.` and quotes around a forward reference are dropped.

| Source type | Port type | Notes |
|---|---|---|
| `str` | `String` | |
| `int` | `i64` | |
| `float` | `f64` | |
| `bool` | `bool` | |
| `bytes`, `bytearray`, `memoryview` | `Vec<u8>` | |
| `None` | `()` | |
| `list[str]`, `List[str]`, `Sequence[str]`, `Iterable[str]` | `Vec<String>` | |
| `list` | `Vec<serde_json::Value>` | |
| `Optional[int]`, `int \| None` | `Option<i64>` | |
| `list[int \| None]` | `Vec<Option<i64>>` | |
| `dict[str, float]`, `Mapping[str, float]` | `serde_json::Value` | |
| `dict[int, str]`, `dict` | `serde_json::Value` | |
| `tuple[int, int]`, `set[str]`, `frozenset[str]` | `serde_json::Value` | |
| `int \| str`, `Union[int, str]` | `serde_json::Value` | |
| `Literal['a', 'b']` | `String` | |
| `Any`, `object` | `serde_json::Value` | |
| `Callable[[int], int]` | refused | takes or returns a function |
| `Iterator[int]`, `Generator[int, None, None]` | refused | a lazy iterator |

A parameter with no hint takes the type of its default, else `f64` if the body does arithmetic
with it, else `serde_json::Value`. `*args`, `**kwargs`, generators and decorators other than `@cache` and
`@lru_cache` are refused.

## Go

| Source type | Port type | Notes |
|---|---|---|
| `string` | `String` | |
| `bool` | `bool` | |
| `int`, `int8`, `int16`, `int32`, `int64` | `i64` | |
| `uint8`, `uint16`, `uint32`, `byte`, `rune` | `i64` | |
| `uint64`, `uint` | `u64` | |
| `uintptr` | `usize` | |
| `float32`, `float64` | `f64` | |
| `[]string` | `Vec<String>` | |
| `[]uint64` | `Vec<u64>` | |
| `[]byte` | `Vec<i64>` | a list of numbers, not bytes |
| `map[string]int` | `serde_json::Value` | |
| `Point` | `serde_json::Value` | an exported struct: its exported fields, named by `json` tag |
| `map[int]string` | refused | JSON object keys are strings |
| `*Point` | refused | takes or returns a pointer |
| `func(int) int` | refused | takes or returns a function |
| `chan int` | refused | takes or returns a channel |
| `any`, `interface{}` | refused | values of any type |
| `[4]int` | refused | fixed-size array |
| `time.Time` | refused | from another package |

A trailing `error` result is dropped from the port type: the Rust function returns `Err` where the
Go one returns an error. Two or more other results cross as one `serde_json::Value` array. An
`error` parameter, a generic function and a variadic one are refused.

## C

`bridge` passes C scalars at the C ABI, so the port keeps their width and sign.

| Source type | Port type | Notes |
|---|---|---|
| `int` | `std::ffi::c_int` | |
| `unsigned int` | `std::ffi::c_uint` | |
| `short` | `std::ffi::c_short` | |
| `unsigned short` | `std::ffi::c_ushort` | |
| `long` | `std::ffi::c_long` | 32 bits on Windows |
| `unsigned long` | `std::ffi::c_ulong` | |
| `long long` | `std::ffi::c_longlong` | |
| `unsigned long long` | `std::ffi::c_ulonglong` | |
| `char` | `std::ffi::c_char` | |
| `signed char` | `std::ffi::c_schar` | |
| `unsigned char` | `std::ffi::c_uchar` | |
| `int8_t` | `i8` | |
| `int16_t` | `i16` | |
| `int32_t` | `i32` | |
| `int64_t` | `i64` | |
| `uint8_t` | `u8` | |
| `uint16_t` | `u16` | |
| `uint32_t` | `u32` | |
| `uint64_t` | `u64` | |
| `size_t`, `uintptr_t` | `usize` | |
| `ssize_t`, `ptrdiff_t`, `intptr_t` | `isize` | |
| `float` | `f32` | |
| `double` | `f64` | |
| `bool`, `_Bool` | `bool` | |
| `void` | `()` | |
| `long double`, `wchar_t` | refused | not a C scalar the bridge knows |

The specifiers may come in any order (`long unsigned int`), and `const` and `volatile` are
ignored. An `enum` is `std::ffi::c_int`, and a project `typedef` of a scalar is that scalar.

Pointers depend on the parameter next to them:

- `const char *` is a string: `Option<&[u8]>`.
- A `const` byte pointer (`const uint8_t *`, `const void *`) followed by an integer length is
  `&[u8]`, and the length is left out of the port.
- A writable byte pointer (`char *`, `uint8_t *`) with a size parameter is `&mut [u8]`.
- A pointer to a scalar the function writes (`int *`, `size_t *`) is `Option<&mut T>`, with `T`
  the scalar's port type.
- `char **` that points into one of the string parameters is `Option<&mut Option<usize>>`, an
  offset into that string. A returned pointer into one is `Option<usize>`.
- A struct passed by value whose fields are all scalars becomes a `#[repr(C)]` struct in the port.

Any other pointer, array or function pointer parameter is refused ("ownership must be designed"),
and so are any other returned pointer ("who frees it must be designed") and variadic functions.

## Java

| Source type | Port type | Notes |
|---|---|---|
| `String`, `CharSequence` | `String` | |
| `int`, `long`, `short`, `byte`, `Integer`, `Long` | `i64` | a boxed `Integer` is not optional |
| `double`, `float`, `Double`, `Float` | `f64` | |
| `boolean`, `Boolean` | `bool` | |
| `void` | `()` | |
| `int[]`, `List<Integer>`, `ArrayList<Integer>`, `Collection<Integer>`, `Iterable<Integer>` | `Vec<i64>` | |
| `Map<String, Integer>`, `HashMap<String, Integer>` | `serde_json::Value` | |
| `Optional<String>` | `Option<String>` | |
| `OptionalInt` | `Option<i64>` | |
| `List<? extends CharSequence>` | `Vec<String>` | in a parameter; refused in a result |
| `Map<Integer, String>` | refused | JSON object keys are strings |
| `List<? super String>` | refused | uses a wildcard |
| `char`, `Character` | refused | a UTF-16 code unit |
| `Object` | refused | values of any type |
| `TreeSet<String>`, `SortedSet<String>`, `NavigableSet<String>`, `LinkedHashSet<String>` | `Vec<String>` | in the set's order; repeated elements are a mismatch |
| `Set<String>`, `HashSet<String>` | refused | an unordered collection |
| `BigInteger`, `BigDecimal` | refused | uses the class |
| `Function<Integer, Integer>`, `Runnable` | refused | takes or returns a function |
| `Stream<String>` | refused | a lazy sequence |

Generic and variadic methods are refused.

## Kotlin

| Source type | Port type | Notes |
|---|---|---|
| `String` | `String` | |
| `Int`, `Long`, `Short`, `Byte` | `i64` | |
| `Double`, `Float` | `f64` | |
| `Boolean` | `bool` | |
| `Unit` | `()` | |
| `String?` | `Option<String>` | |
| `List<Int>`, `MutableList<Int>`, `Array<Int>`, `IntArray`, `LongArray` | `Vec<i64>` | |
| `List<Int?>` | `Vec<Option<i64>>` | |
| `DoubleArray` | `Vec<f64>` | |
| `BooleanArray` | `Vec<bool>` | |
| `Map<String, Int>` | `serde_json::Value` | |
| `List<out CharSequence>` | `Vec<String>` | in a parameter; refused in a result |
| `List<in String>` | refused | uses a projection |
| `UInt`, `ULong` | refused | uses the class |
| `ByteArray`, `ShortArray`, `FloatArray` | refused | which the Java wire cannot carry yet |
| `Char` | refused | a UTF-16 code unit |
| `Any` | refused | values of any type |
| `Set<String>`, `MutableSet<String>` | refused | an unordered collection |
| `Pair<Int, Int>` | refused | no tuple in the dialect |
| `(Int) -> Int` | refused | takes or returns a function |
| `Sequence<Int>` | refused | a lazy sequence |

`vararg` parameters are refused.

## C#

| Source type | Port type | Notes |
|---|---|---|
| `string` | `String` | |
| `int`, `long`, `short`, `byte`, `sbyte`, `uint`, `ushort` | `i64` | |
| `double`, `float` | `f64` | |
| `bool` | `bool` | |
| `void` | `()` | |
| `int?`, `Nullable<int>` | `Option<i64>` | |
| `int[]`, `List<int>`, `IEnumerable<int>`, `IReadOnlyList<int>`, `HashSet<int>`, `ISet<int>` | `Vec<i64>` | a set crosses as an array |
| `Dictionary<string, int>` | `serde_json::Value` | |
| `ulong`, `nint`, `nuint` | refused | wider than a JSON-safe integer |
| `decimal` | refused | a 128-bit decimal no JSON number keeps exactly |
| `Dictionary<int, string>` | refused | JSON object keys are strings |
| `(int, string)` | refused | uses a tuple |
| `char` | refused | a UTF-16 code unit |
| `object`, `dynamic` | refused | values of any type |
| `Func<int, int>`, `Action<int>` | refused | takes or returns a delegate |
| `Task<int>` | refused | is async |
| `DateTime`, `Guid` | refused | uses the type |

A record, class or struct declared in the project crosses as `serde_json::Value` when:

- it is `public`, and not `static`, `abstract` or generic, and has no base list (it neither
  derives from a class nor implements an interface);
- its name is declared once in the project (two classes named `Point` in different namespaces
  refuse both);
- it has no constructor that takes arguments and no public instance fields (static fields are
  fine);
- it has at least one property, and each property's type crosses (a `decimal` property refuses
  it, and so does a type that contains itself).

Its properties are the public instance auto-properties with `get` and `set` or `init`
(`{ get; set; }`, `{ get; init; }`), plus a positional record's parameters, as in
`public record Point(double X, double Y)`. Other members, such as a getter-only `{ get; }` or a
static property, are left out and their types are not checked.

`ref`, `out`, `in` and `params` parameters and generic methods are refused.

## Ruby

Types are read from the YARD tags in the comment right above the `def`: `@param name [Types]` (or
`@param [Types] name`) and `@return [Types]`. The table shows what goes between the brackets.

| Source type | Port type | Notes |
|---|---|---|
| `String`, `Symbol` | `String` | |
| `Integer` | `i64` | |
| `Float`, `Numeric` | `f64` | |
| `Boolean`, `true`, `false` | `bool` | |
| `nil` | `()` | |
| `String, nil` | `Option<String>` | |
| `Array<String>` | `Vec<String>` | |
| `Array` | `Vec<serde_json::Value>` | |
| `Hash{String => Integer}`, `Hash{Symbol => Integer}` | `serde_json::Value` | |
| `Hash<String, Integer>`, `Array(String)` | `serde_json::Value` | not read: write `Hash{K => V}` and `Array<T>` |
| `Integer, Float`, `Object` | `serde_json::Value` | |

A parameter with no tag takes the type of its default, else `f64` if the body does arithmetic with
it, else `serde_json::Value`. `*args`, `**options`, blocks and methods that `yield` are refused.

## PHP

Types are read from the declarations and from phpdoc (`@param int[] $xs`, `@return list<string>`).
A declared type wins, except `array` and `mixed`, where a phpdoc type is used when there is one.

| Source type | Port type | Notes |
|---|---|---|
| `string` | `String` | |
| `int` | `i64` | |
| `float` | `f64` | |
| `bool`, `true`, `false` | `bool` | |
| `void`, `null` | `()` | |
| `?int`, `int\|null` | `Option<i64>` | |
| `int[]`, `list<int>`, `array<int>`, `array<int, int>` | `Vec<i64>` | |
| `array<string, int>` | `serde_json::Value` | |
| `array`, `iterable`, `mixed` | `serde_json::Value` | |
| `int\|string` | `serde_json::Value` | |
| `array{a: int}` | `serde_json::Value` | a shape is not read |

A parameter with no type takes the type of its default, else `serde_json::Value`. A declared
`callable` or class type, a parameter by reference, `...$args`, generators and `public static`
methods are refused.

## PowerShell

Types are read from the `[type]` before each parameter in `param()`, and the result from
`[OutputType()]`. `System.` is optional, and case does not matter.

| Source type | Port type | Notes |
|---|---|---|
| `[string]`, `[char]` | `String` | |
| `[int]`, `[long]`, `[byte]`, `[uint64]` | `i64` | |
| `[double]`, `[float]`, `[decimal]` | `f64` | |
| `[bool]`, `[switch]` | `bool` | |
| `[string[]]` | `Vec<String>` | |
| `[array]`, `[object[]]` | `Vec<serde_json::Value>` | |
| `[hashtable]`, `[ordered]`, `[pscustomobject]`, `[object]` | `serde_json::Value` | |
| `[datetime]`, `[guid]` | refused | takes a .NET object |
| `[scriptblock]` | refused | takes a script block |

An untyped parameter is `serde_json::Value`. A parameter that is untyped or typed as an array,
`[hashtable]`, `[ordered]`, `[pscustomobject]` or `[object]` is `Option<T>`, unless it is
`Mandatory` or has a default other than `$null`. A result type the table refuses is
`serde_json::Value`. Filters, workflows and functions that read the pipeline are refused.

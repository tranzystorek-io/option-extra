/// Converts any enum to [`Option`].
///
/// Makes a [`Some`] from a selected variant of your enum.
///
/// # Examples
///
/// Short version for one-element variants:
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     Int(i32),
///     Other,
/// }
///
/// let int = MyEnum::Int(1);
/// let other = MyEnum::Other;
///
/// assert_eq!(some!(if let MyEnum::Int = int), Some(1));
/// assert_eq!(some!(if let MyEnum::Int = other), None);
/// ```
///
/// Pairs perfectly with [`Iterator`] methods like [`filter_map`](Iterator::filter_map):
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     A(i32, bool),
///     B(u8),
///     C
/// }
///
/// use MyEnum::*;
///
/// let v = vec![A(10, true), B(0), C, B(1), A(4, false), C];
/// let a_only: Vec<_> = v.into_iter().filter_map(|el| some!(if let A {n, b} = el)).collect();
///
/// assert_eq!(a_only, [(10, true), (4, false)]);
/// ```
///
/// Works with tuple variants:
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     Variant(i32, bool),
///     Other,
/// }
///
/// let v = MyEnum::Variant(10, true);
///
/// assert_eq!(some!(if let MyEnum::Variant {n, b} = v), Some((10, true)));
/// ```
///
/// Or with struct variants (when suffixing field names with colons):
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     Struct {
///         id: u16,
///         name: &'static str,
///     },
///     Other,
/// }
///
/// let s = MyEnum::Struct {
///     id: 20,
///     name: "abcd",
/// };
///
/// assert_eq!(some!(if let MyEnum::Struct {id:, name:} = s), Some((20, "abcd")));
/// ```
#[macro_export]
macro_rules! some {
    (if let $p:path = $x:expr) => {
        match $x {
            $p(inner) => ::std::option::Option::Some(inner),
            _ => ::std::option::Option::None,
        }
    };

    (if let $p:path {$($n:ident),+} = $x:expr) => {
        match $x {
            $p($($n),+) => ::std::option::Option::Some(($($n),+)),
            _ => ::std::option::Option::None,
        }
    };

    (if let $p:path {$($n:ident:),+} = $x:expr) => {
        match $x {
            $p{$($n),+} => ::std::option::Option::Some(($($n),+)),
            _ => ::std::option::Option::None,
        }
    };
}
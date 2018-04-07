# Language

## Contents

- [Comments](#comments)
- [Built-in types and their literals](#built-in-types-and-their-literals)
- [Type annotations](#type-annotations)
- [Types of types](#types-of-types)
- [Identifiers](#identifiers)
- [Functions](#functions)

## Comments

In Pikelet, line comments are proceeded by a double dash:

```pikelet
-- this is a comment!
```

Doc comments are proceeded by a triple pipe:

```pikelet
||| This is a doc comment!
```

## Built-in types and their literals

Pikelet has a number of fundamental types:

| Type     | Literal                                |
|----------|----------------------------------------|
| `String` | `"hello there!"`                       |
| `Char`   | `'a'`, `'b'`, ..., `'\n'`, `'\t'`, ... |
| `U8`     | `1`, `2`, `3`, ...                     |
| `U16`    | `1`, `2`, `3`, ...                     |
| `U32`    | `1`, `2`, `3`, ...                     |
| `U64`    | `1`, `2`, `3`, ...                     |
| `I8`     | `1`, `2`, `3`, ...                     |
| `I16`    | `1`, `2`, `3`, ...                     |
| `I32`    | `1`, `2`, `3`, ...                     |
| `I64`    | `1`, `2`, `3`, ...                     |
| `F32`    | `1`, `2`, `3`, ..., `0.0`, `1.0`, ...  |
| `F64`    | `1`, `2`, `3`, ..., `0.0`, `1.0`, ...  |

> **Note:** You can't do much with these built-in types yet. In the future we
> will add some primitive operations to allow you to maniplate them.

## Type annotations

If you note [above](#built-in-types-and-their-literals), a number of the
built-in types share a literal representation. Pikelet will try to predictably
infer the types, but if it fails to do so you will get an error. In that case
you can use the type annotation operator, `(:)`, to specify the intended type:

```pikelet-repl
Pikelet> 1
error: ambiguous integer literal
Pikelet> 1 : I32
1 : I32
Pikelet> 1 : F32
1 : F32
Pikelet> 1.0 : F32
1.0 : F32
Pikelet> 1.1 : U64
error: found a floating point literal, but expected a type `#U64`
```

## Types of types

Types also have types!

```pikelet-repl
Pikelet> :t I32
Type
```

You might then ask, “what is the type of `Type`?”

```pikelet-repl
Pikelet> :t Type
Type 1
```

Note that `Type` is actually just syntactic sugar for `Type 0`:

```pikelet-repl
Pikelet> :t Type 0
Type 1
```

In fact Pikelet has an infinte number of 'universes', each one 'bigger' than the
previous:

```pikelet-repl
Pikelet> :t Type 0
Type 1
Pikelet> :t Type 1
Type 2
Pikelet> :t Type 2
Type 3
...
```

You can think of these as larger and larger sets, with the smaller type
universes being contained within the larger type universes:

> TODO: Put a nice SVG diagram of the sets of types here
>
> ```
> (Type 2
>   (Type 1
>     (Type 0
>       (I32, F32, String,
>         (1, 4.0, "hello")))))
> ```

## Identifiers

> TODO

## Functions

### Simply typed functions

Here are some simple functions and their types:

```pikelet-repl
Pikelet> :t \x : I32 => x
I32 -> I32
Pikelet> :t \x : String => x
String -> String
Pikelet> :t \x : Char => x
Char -> Char
```

Note that all of these types follow the same pattern - they are the identity
function! This means that if you pass a value to them, they'll return the same
thing without alteration!

```pikelet-repl
Pikelet> (\x : I32 => x) 42
42 : I32
Pikelet> (\x : String => x) "hi"
"hi" : String
Pikelet> (\x : Char => x) 'b'
'b' : Char
```

### Polymorphic functions

Alas, we can't reuse one of these identity functions with other, incompatible
types:

```pikelet-repl
Pikelet> (\x : I32 => x) 4.0
error: found a floating point literal, but expected a type `#I32`
Pikelet> (\x : String => x) 'b'
error: found a character literal, but expected a type `#String`
Pikelet> (\x : Char => x) "yoho"
error: found a string literal, but expected a type `#Char`
```

Let's make this identity function polymorphic by adding a parameter for the type
of the argument:

```pikelet-repl
Pikelet> :t \(a : Type) (x : a) => x
(a : Type) -> a -> a
```

We now have a polymorphic identity function! We can specialise this function by
applying a type to it:

```pikelet-repl
Pikelet> (\(x : Type) (x : a) => x) String "hello"
"hello" : String
Pikelet> (\(x : Type) (x : a) => x) I32 1
1 : I32
```

### Syntactic Sugar for functions

In Pikelet all functions take a single argument - in order to pass multiple
arguments we use currying. The following functions are equivalent:

```pikelet
\(x : Type) (x : a) => x
\(x : Type) => \(x : a) => x
```

Non-dependent functions can be expressed without explicit parameter names. For
example the following function types are equivalent:

```pikelet
(a : Type) (x : a) -> a
(a : Type) -> (x : a) -> a
(a : Type) -> a -> a
```
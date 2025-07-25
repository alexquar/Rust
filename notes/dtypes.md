# Dtypes

## Other basics

### Other keywords
-To make a variables changable must preceed the variable name with mut 
    - Let mut name

## Primitives

### Ints 
-i8,i16,... (signed)(number of bits)
-u8,u16,... (unsigned)


### Floats 
-f32, f64


### Bools
- true/false


### Chars 
-char 
-single quotes


## Compound types

### Arrays
-let numbers [i32; {size} ] = [1,2,3,...]
-all same dtype
-fixed size not a vector or list 
-support normal [n] index accessing
### Tuples
-allows mixed types 
-let thing: (&str, i32, bool) = ("Thing", 20, true)
    -don't need type explicitely

### String vs String Slices

#### String Slices
-&str
-static
-immutable
#### String
-String
-let mut thing: String = String::from("----")
-growable
-mutable
-owned (not borrowed)
-dynamic (on heap memory)

##### Methods
-append to a string
    -thing.push_str("----")


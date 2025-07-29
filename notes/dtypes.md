# Dtypes

##  basics

### Other keywords
- To make a variables changable must preceed the variable name with mut 
    - Let mut name
- declare using let 
    - not mutable but variables can be shadowed (redeclared w a new value)
- constants 
    - const thing:i32 = 10000

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

### None
- same as none from python can check for is using == None 


## Compound types

### Arrays
- let numbers [i32; {size} ] = [1,2,3,...]
- all same dtype
- fixed size not a vector or list 
- support normal [n] index accessing
### Tuples
- allows mixed types 
- let thing: (&str, i32, bool) = ("Thing", 20, true)
    - don't need type explicitely
- access w .n

### Vectors

#### Creation 
- let nums = vec![1,2,3,...]

#### Methods
- 

### String vs String Slices

#### String Slices
-&str
-static
-immutable
-reference 
-not an owned string a reference to another


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
-get a subset of a str
    -thing[start..end];
-get length
    -s.len()


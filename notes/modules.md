# Modules and Structure 

## Structure

### Main 
- where the important files sit 
- to get code from another file use mod fileName
- then to get its public funcs
    - use fileName::funcname
    - use fileName::{
        func1,
        func2
    }

### Modules 
- can import and export across files 
- to make a function access make it public 
    - pub fn thing(){}

### Imports
- normal import one func 
    - use mod::func
- glob import 
    - use mod::type::*
- rename 
    - use std::collection::func as colfunc

### Dependecies
- cargo.toml has dependencies and version nums 
- once they're installed just use use to import needed stuff 
- no true install step just add the dep and ver to the file and run the cargo container it gets downloaded at compile time
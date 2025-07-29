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

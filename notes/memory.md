# Borrowing and Ownership

## Ownership

### Ownership rules
- each value in rust has an owner 
- There can only be one owner at a time 
- when the owner goes out of scope the value is dropped
- can only havew one mutable reference or many immutable references 


### References
- immutable by default
- enables you to borrow a value without taking ownership (its just a pointer no?)
- &variable_name
- taking in a reference &type

### Making changes to the value of a reference
- have to dereference *_r +=1

### Making a reference mutable 
- example the original needs to be a mut value then we take a mutable reference 
    let mut x:i32 = 5;
    let r: &mut i32 = &mut x;
    - we now have a reference to x in r that can be changes

### One owner
- If you go s1 = String::from("RUST") and then s2=s1 then s2 now owns the value and takes over the value using s1 will not trigger an error
- even if you call a function with the argument as your value and not take it as a reference then it is now the owner 
- if you pass it by reference then it just "borrows" the value and you remain the owner 

## Borrowing
- involved temp ownership through references &obj
- to have mutable borrowed value you must use &mut
# Custom Types

## Structs 

### Declaration
- struct name {
    field1:thing,
    field2:thing,
}

### Creation 
- let thing = thing { thing1: value }

### Implementations 
- alows you to define functions on structs 
    - create the stuct
    - impl same_name{
        fn method1(){}
        fn method2(){
            self.thing = method2value;
        }
    }
- you get self as a refererence to member vars
- construct and destruct but must call em
    - fn new(val1: val) -> Self {
        Self{val}
    }
    -fn deactivate(&mut self) {
        cleanup
    }
- calling methods
    - let mut thing = Thing::new(val);
    - thing.deactivate()

## Enums 

### Declaration 
- enum Thing { someting, something2}

### Creation 
- let thing = Role::Something

## Traits (Interfaces)
- define a trait 
    - trait Something{
        fn something(&self)
    }
- implement that train on a struct 
    - impl Something for Stuct {
        fn something(&self){
            ---
        }
    }
- use this trait with .something() notation like normal instead of the ::something notation of 

## Methods vs functions in rust
- methods are associated functions that operate on a specific instance of an object for example they take a reference to self as a n argument
- function do not and are called on the type itself not an instance 
- ex
    - function: String::from("---")
    - method: my_string.len()
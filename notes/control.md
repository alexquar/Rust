# Control Flow

## Basic Conditions 

### Conditionals 
- all same conditional operators 

### ifs
- uses normal if, else if, else hierarchy
- no brackets 

### Match statements 
- case statements 
    - match value{
        something => ...
        something else => ...
    }

## Loops 

### For loops 
- basic counting loop 
    - for i in start..stop{}
    - this is inclusive at start but not at stop 
    - to make inclusive at end use ..=
    - to change step use (start...stop).step_by(n)
- while loop 
    - while bool {}
- for in loops 
    for thing in terable {}

### Escaping
- supports normal break syntax 
- can use break with a label to break out of something other than the inner most loop
    - break labl;
- normal continue is supported

## Labels 
- can label any loop expression 
    - name: while---{}
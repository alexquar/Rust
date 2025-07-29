# Errors 

## Handling

### Options
- allows a thing to be none or the value 
- here is an example 
    - fn get(index: usize) -> Option<i32> {
    let nums = vec![1, 2, 3];
    nums.get(index).cloned()
}
    - you then need to check if you value is i32 or None

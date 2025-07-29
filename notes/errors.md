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
    - option looks like the following so check using a match statement 
    enum Option<T> {
    Some(T),
    None,
    }

### Panic 
- terminates thread (aborts)
    - panic!("Something went super wrong")

### Recoverable errors
- expected failures like file parsing or network stuff 
- you will get back a template type of Result<T,E>
    - the result type looks like enum Result<T,E>{ 
        Ok(T), success data
        Err(E), failure 
    }
- use a match statement to check for data or error
- example with file loading 
    let result = File::open("thing.txt")
    match result {
        Ok(file) => {
            println!("{:?}", file);
        },
        Err(e) => {
            println!("Failure: {:?}", e)
        }
    }
- can also use unwrap like we in our webserver basically just panics if we get an error else our result is the value 
    - let file = File::open("file.txt").unwrap();
    - can also add a .expect("Message") for if that request does fail to customize the message
- ? operator to propogate the error up 
- example 
        fn read_file() -> Result<String, io::Error> {
        let mut file = File::open("hello.txt")?;  // if error, return early
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;      // if error, return early
        Ok(contents)                              // all good, return content
    }
    - if the file load fails we will instantly return the error because of the ?

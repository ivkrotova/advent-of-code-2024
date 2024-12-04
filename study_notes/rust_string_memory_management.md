# Rust String Memory Management

## Basic String Creation
```rust
let mut guess = String::new();
```

The `String::new()` creates an empty string instance.
The `::new` syntax indicates an associated function (which is similar to `@staticmethod` in Python).

## String Types in Rust vs Python

### Python Strings
- Immutable
- Fixed memory block
- String modifications create new objects
- Old strings get garbage collected

```python
s = "hello"
s = s + " world"  # Creates new string object
```

### Rust Strings
- Mutable
- Growable buffer
- Modifications happen in-place
- Manual memory management (no GC)

```rust
let mut s = String::from("hello");
s.push_str(" world");  # Modifies existing string
```

## Memory Structure

### Rust String Memory Layout
1. **Pointer**: Points to heap memory containing the string data
2. **Length**: Current used bytes
3. **Capacity**: Total allocated space

```rust
struct String {
    ptr: *mut u8,    // Pointer to data
    len: usize,      // Current length
    capacity: usize  // Total allocated capacity
}
```

## Growth Strategy

When a Rust String needs more space:
1. Allocates new larger buffer (typically doubles)
2. Copies existing content
3. Updates pointer and capacity
4. Frees old buffer

Example:
```rust
let mut s = String::with_capacity(3);
println!("Capacity: {}", s.capacity()); // 3
s.push_str("hello");
println!("Capacity: {}", s.capacity()); // 8 (doubled + extra)
```

## Benefits of Rust's Approach
- Efficient appending operations
- Minimized allocations
- Contiguous memory
- Predictable performance
- No garbage collection pauses

## Trade-offs
- May use more memory than needed
- Manual memory management complexity
- Initial allocation overhead

## UTF-8 and String Types

### UTF-8 in Rust Strings
- `String` is always valid UTF-8 encoded
- Not just a byte buffer like `Vec<u8>`
- Invalid UTF-8 sequences are caught at compile time

```rust
// Converting bytes to String with error handling
let bytes = vec![0x66, 0x6f, 0x6f];  // "foo" in bytes
let s = String::from_utf8(bytes).unwrap();  // Can panic if invalid UTF-8

// Safer version with error handling
let bytes = vec![0x66, 0x6f, 0x6f];
match String::from_utf8(bytes) {
    Ok(s) => println!("Valid UTF-8: {}", s),
    Err(e) => println!("Invalid UTF-8: {}", e),
}
```

### String Slices (`&str`)
- Immutable reference to string data
- Can be stack or heap data
- Zero-cost abstraction
- Common in function parameters

```rust
// String literal - &str type
let greeting = "hello";     // &str, borrowed, immutable
let mut s = String::from("hello"); // String, owned, mutable

// String slice from String
let slice = &s[..];  // creates &str from String
```

```rust
fn take_slice(s: &str) {
    println!("{}", s);
}

let string = String::from("hello");
take_slice(&string);  // &String coerces to &str
```

### Key Differences: String vs &str
- `String`: Owned, mutable, heap-allocated
- `&str`: Borrowed, immutable, can be stack or heap
- `String` can be converted to `&str` via borrowing
- `&str` to `String` requires allocation

## Common Operations
```rust
// Create new string
let mut s = String::new();

// Create with initial content
let s = String::from("hello");

// Create with capacity
let mut s = String::with_capacity(10);

// Append
s.push_str(" world");

// Get length
println!("Length: {}", s.len());

// Get capacity
println!("Capacity: {}", s.capacity());
```
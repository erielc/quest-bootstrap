# Advanced Rust Syntax Explanation

This document explains the intermediate-to-advanced Rust concepts and syntax used in the bootstrapping scripts (including `downloads.rs` and the GLPK installation helper):

```rust
use std::error::Error;
use reqwest::blocking::Client;
use flate2::read::GzDecoder;
use tar::Archive;

fn install_glpk_on_macos() -> Result<(), Box<dyn Error>> {
    let url = "https://ftpmirror.gnu.org/gnu/glpk/glpk-latest.tar.gz";
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36";

    let client = Client::builder()
        .user_agent(user_agent)
        .build()?;

    let response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("HTTP request failed with status: {}", response.status()).into());
    }

    let tar_gz = GzDecoder::new(response);
    let mut archive = Archive::new(tar_gz);
    archive.unpack(".")?;

    Ok(())
}
```

---

## 1. `Result<(), Box<dyn Error>>`

This is the return type of the function, which dictates how Rust handles success and failures.

### The `Result` Enum
Rust doesn't have exceptions like Python or Java. Instead, it uses a generic enum:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
* **`()` (The Unit Type)**: The success variant `Ok(())` contains `()`, which is an empty tuple. It represents "nothing" or `void`.
* **`Box<dyn Error>`**: The error variant `Err(...)` contains a boxed trait object.

### Decoding `Box<dyn Error>`
* **`dyn Error` (Trait Object)**: `Error` is a trait (like an interface in other languages). `dyn Error` refers to *any dynamic type* that implements the `std::error::Error` trait. 
* **`Box<...>` (Smart Pointer)**: In Rust, the compiler must know the exact memory size of every type at compile-time. Because different errors have different sizes, we cannot return `dyn Error` directly on the stack. Wrapping it in a `Box` allocates the error on the heap and leaves a fixed-size pointer on the stack.

> [!NOTE]
> Returning `Result<(), Box<dyn Error>>` allows this single function to return many different types of errors (e.g., connection errors from `reqwest`, filesystem errors from `tar`/`std::io`), and lets the caller handle them uniformly.

---

## 2. The `?` (Try) Operator

The `?` suffix is Rust's syntactic sugar for error propagation.

When you append `?` to a `Result` value:
1. **If the result is `Ok(value)`**: The operator unwraps the result and evaluates to `value` so execution continues.
2. **If the result is `Err(error)`**: The operator immediately returns the error from the surrounding function (an early return).

### Implicit Error Conversion
Behind the scenes, `?` automatically calls `From::from(error)`. 
This converts the specific error type (such as `reqwest::Error` or `std::io::Error`) into the type specified in the function signature (`Box<dyn Error>`).

```rust
// Without the ? operator (verbose):
let client = match Client::builder().user_agent(user_agent).build() {
    Ok(c) => c,
    Err(e) => return Err(Box::new(e)),
};

// With the ? operator (clean):
let client = Client::builder().user_agent(user_agent).build()?;
```

---

## 3. Ownership and Streams (`GzDecoder::new(response)`)

Rust's signature feature is its compile-time ownership model.

```rust
let response = client.get(url).send()?;
let tar_gz = GzDecoder::new(response);
```

### Transfer of Ownership (Moving)
1. `client.get(url).send()?` returns a `Response` struct and binds it to `response`.
2. When we call `GzDecoder::new(response)`, we pass `response` **by value**, not by reference.
3. This **moves** ownership of the `response` instance into the `GzDecoder` constructor.
4. Because the `response` implements `std::io::Read`, `GzDecoder` consumes it as the underlying network data stream.
5. If you try to use `response` later in the function, the compiler will refuse to compile, preventing bugs like reading from a closed/moved socket.

---

## 4. Mutability: `mut` and Mutable References (`&mut`)

By default, all variables in Rust are **immutable** (read-only). This helps guarantee thread safety and prevents accidental modification of state.

### Declaring Mutable Variables (`mut`)
If you need to change the value stored in a variable or modify its state, you must explicitly mark it with the `mut` keyword:

```rust
let mut resp = client.get(item.url).send()?;
let mut f = File::create(&path)?;
```

Here, `resp` and `f` are marked mutable because their internal states will be modified when we read from `resp` and write to `f`.

### Borrowing Mutably (`&mut`)
In Rust, you can borrow a variable either immutably (`&T`) or mutably (`&mut T`).
* **Immutable Borrow (`&T`)**: Allows reading the data, but not modifying it. You can have multiple immutable borrows at the same time.
* **Mutable Borrow (`&mut T`)**: Allows modifying the data. You can only have **one** active mutable borrow at a time for a particular piece of data to prevent data races.

In `downloads.rs`, we pass mutable references to `std::io::copy`:

```rust
copy(&mut resp, &mut f)?
```

> [!NOTE]
> `copy` takes its arguments as `&mut R` (reader) and `&mut W` (writer). Reading from a socket stream (`resp`) modifies its internal buffer and cursor position, and writing to a file (`f`) modifies the file pointer/system state, which is why both must be borrowed mutably.

---

## 5. Constants & Static Lifetimes (`&'static str`)

In `downloads.rs`, the `Download` struct is defined as:

```rust
pub struct Download {
    pub name: &'static str,
    pub url: &'static str,
    pub file: &'static str,
}
```

### String Slices vs. Owned Strings
* `String` is a heap-allocated, growable string (e.g., `String::from("hello")`).
* `&str` is a string slice—a reference to a sequence of UTF-8 bytes stored elsewhere.

### The `'static` Lifetime
The `'static` lifetime is a special lifetime in Rust that indicates the referenced data **lives for the entire duration of the program**.

In Rust, all hardcoded string literals (e.g., `"Python"`, `"Git"`) are stored directly in the read-only data segment of the compiled binary. Because they are never deallocated, their lifetime is automatically `'static`.

### Why Use `&'static str` for Struct Fields?
By defining `Download` fields as `&'static str`, we indicate that the struct only stores references to hardcoded compile-time string constants. This is highly efficient because:
1. It avoids heap allocation at runtime.
2. It avoids copying the string contents (no `.clone()` or `.to_string()` needed).

---

## 6. Pattern Matching with Tuples and Alternations

Rust's `match` control flow operator is incredibly expressive. In `downloads.rs`, we match on a tuple of variables:

```rust
match (os, arch) {
    ("windows", "x86_64") => Ok(vec![...]),
    ("macos", "x86_64") | ("macos", "aarch64") => Ok(vec![...]),
    _ => bail!("unsupported platform: os={}, arch={}", os, arch),
}
```

### Tuple Matching
By wrapping the two variables into a tuple `(os, arch)`, we can match against both values simultaneously.

### The Pattern OR Operator (`|`)
You can use `|` (OR) to match multiple patterns in a single match arm:
```rust
("macos", "x86_64") | ("macos", "aarch64") => ...
```
If the tuple matches either of these patterns, this match arm executes.

### The Wildcard Pattern (`_`)
The `_` is a wildcard that matches *any* value. Because Rust `match` statements must be **exhaustive** (you must cover every possible input), `_` serves as the `default` or `else` block to handle all unspecified combinations.

---

## 7. The `vec!` Macro

In the `get_downloads` function, we return a vector of `Download` structs:

```rust
Ok(vec![
    Download { name: "Python", ... },
    Download { name: "Git", ... }
])
```

* `Vec<T>` (vector) is a growable, heap-allocated collection of elements of type `T`.
* `vec![...]` is a built-in macro that makes it easy to initialize a vector with a comma-separated list of elements, similar to array syntax in other languages.

---

## 8. Closures (Anonymous Functions)

Closures in Rust are anonymous functions that can capture variables from their enclosing scope. They are defined using the pipe syntax `|params| expression`.

We see two examples in `downloads.rs`:

1. **Inline check with `is_ok_and`**:
   ```rust
   Command::new("git")
       .arg("--version")
       .output()
       .is_ok_and(|o| o.status.success())
   ```
   Here, `|o| o.status.success()` is a closure that takes the program's output `o` and returns `true` if the program exited successfully.

2. **Lazy evaluation with `with_context`**:
   ```rust
   let mut resp = client
       .get(item.url)
       .send()
       .with_context(|| format!("request failed: {}", item.url))?;
   ```
   `with_context` accepts a closure `|| format!(...)` that takes no arguments.
   
   > [!TIP]
   > Unlike `context(...)` which evaluates its argument immediately, `with_context(|| ...)` evaluates its closure **lazily** (only if an error actually occurs). This avoids the overhead of constructing and allocating the format string during successful runs.

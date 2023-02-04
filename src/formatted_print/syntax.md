# Format Syntax

```
format_string := text [ maybe_format text ] *
maybe_format := '{' '{' | '}' '}' | format
format := '{' [ argument ] [ ':' format_spec ] [ ws ] * '}'
argument := integer | identifier

format_spec := [[fill]align][sign]['#']['0'][width]['.' precision]type
fill := character
align := '<' | '^' | '>'
sign := '+' | '-'
width := count
precision := count | '*'
type := '' | '?' | 'x?' | 'X?' | identifier
count := parameter | integer
parameter := argument '$'
```

####  <pre>{[argument]':'[[fill] align][sign]['#'][width [$]]['.' precision [$]][type]}</pre>
- argument => number(0, 1, 2, ..) or name
    ```rust
    println!("{0} {1}", "arg_1", "arg_2"); // with argument number
    let arg_1 = "arg_1";
    let arg_2 = "arg_2";
    println!("{arg_1} {arg_2}"); // with argument name
    ```
- fill => the character to fill empty spaces, if `width` is specified
- align => left(<), center (^), right (>), if `width` is specified
    ```rust 
    println!("{:!>6}", "@@");     // !!!!@@
    println!("{0:!<6}", "@@");    // @@!!!!
    let double_at = "@@";
    println!("{double_at:!^6}");  // !!@@!!
    ```
- width [$] => width to be padded with `fill` (default to space), can be number or paramater, if paramater '$' must be used
    ```rust 
    println!("{:0width$}", 1, width = 4);     // 0001
    println!("{:0<width$}", 1, width = 4);    // 1000
    println!("{:0^width$}", 1, width = 4);    // 0100

    println!("{:01$}", 1, width = 4);     // 0001
    println!("{:0<1$}", 1, width = 4);    // 1000
    println!("{:0^1$}", 1, width = 4);    // 0100
    
    println!("{:!>width$}", "1", width = 4);  // !!!1
    println!("{:!<width$}", "1", width = 4);  // 1!!!
    println!("{:!^width$}", "1", width = 4);  // !1!!
    
    println!("{:>4}", 1); // padded with space (space is default)
    ```
    
- sign => `+` can be used for numeric types to display sign (negative sign is only displayed for signed values)
    ```rust 
    println!("{}", 1000);   // 1000
    println!("{:+}", 1000); // +1000
    ```
    
- precision [$] => decimal digits for number or max width for non-number, can be number or paramater, if paramater '$' must be used. `.*` means that this {...} is associated with two format inputs rather than one:

    ```rust 
    // the output of all lines below is
    // => Hello x is 0.01000
    println!("Hello {0} is {1:.5}", "x", 0.01);
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);
    println!("Hello {} is {:.*}",    "x", 5, 0.01);
    println!("Hello {1} is {2:.*}",  5, "x", 0.01);
    println!("Hello {} is {2:.*}",   "x", 5, 0.01);
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);
    ```
- `#` indicates that the "alternate” form of printing should be used. The alternate forms are:
    - `#?` pretty-print the Debug formatting (adds linebreaks and indentation)
    - `#x` precedes the argument with a 0x
    - `#X` precedes the argument with a 0x
    - `#b` precedes the argument with a 0b
    - `#o` precedes the argument with a 0o
    
    ```rust 
    println!("{:#?}", "test"); // => "test"
    println!("{:#x}", 16); // => 0x10
    println!("{:#X}", 16); // => 0x10
    println!("{:#b}", 2); // => 0b10
    println!("{:#o}", 8); // => 0o10
    println!("{:#?}", (1, "one", 3.3)); // => (
                                        //      1,
                                        //      "one",
                                        //      3.3,
                                        //    )
    ```

#### Formatting Traits
-   nothing ⇒ Display
-   ? ⇒ Debug
-   x? ⇒ Debug with lower-case hexadecimal integers
-   X? ⇒ Debug with upper-case hexadecimal integers
-   o ⇒ Octal
-   x ⇒ LowerHex
-   X ⇒ UpperHex
-   p ⇒ Pointer
-   b ⇒ Binary
-   e ⇒ LowerExp
-   E ⇒ UpperExp

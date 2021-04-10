# Chapter 1

## Install

- `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh` installs rust
- `rustup update` updates rust
- `rustup self uninstall` uninstalls rust
- `rustc --version` tells you about the install


#### Actively useful 

- `rustup doc` will open the rust book locally

## Hello, World!

- rust files are always `.rs`
- rust likes `snake_case` in `file_names`

#### Running programs

- rust compiles to executables
- in the right directory ( `cd path/to/there`), run `rustc filename.rs`

### Anatomy of a rust program

- `fn main() {}` defines a function 
	- `main() {}` always goes at the start of a program
	
#####  functions
	
- basic structure:
	- `fn function_name(parameters) { \n body \n }`
	- `rustfmt` makes your code match the standard style and comes in the box



`    println!("Hello, world!");`

##### Directly from the docs:::

This line does all the work in this little program: it prints text to the screen. 
There are four important details to notice here.

First, Rust style is to indent with four spaces, not a tab.

Second, println! calls a Rust macro.
 If it called a function instead, it would be entered as println (without the !).
  We’ll discuss Rust macros in more detail in Chapter 19. 
  For now, you just need to know that using a ! means that you’re calling a macro 
  instead of a normal function.
  
  >> Anthony does not know what macro means in this context

Third, you see the "Hello, world!" string. 
We pass this string as an argument to println!,
 and the string is printed to the screen.

Fourth, we end the line with a semicolon (;),
 which indicates that this expression is over and the next one is ready to begin.
  Most lines of Rust code end with a semicolon.

#### Compiling and Running Are Separate Steps

- `rustc` means `COMPILE RUST PROGRAM`
	- so, `rustc main.rs` compiles the program
	- this compiles an application
	- `./main` then runs it
	
##### Directly from the docs:::

 Rust is an ahead-of-time compiled language, 
 meaning you can compile a program and give the executable to someone else,
  and they can run it even without having Rust installed. 
  If you give someone a .rb, .py, or .js file, 
  they need to have a Ruby, Python, or JavaScript
   implementation installed (respectively). 
   But in those languages, you only need one command to compile and run your program.
    Everything is a trade-off in language design.
    
## Hello, Cargo!

    
##### Directly from the docs:::

- Cargo is Rust’s build system and package manager. 
- Because the vast majority of Rust projects use Cargo, the rest of this book assumes that
 you’re using Cargo too.
- The simplest Rust programs, like the one we’ve written so far, don’t have any 
dependencies.
- cargo --version:  `cargo 1.50.0 (f04e7fab7 2021-02-04`

	
### Creating a Project with Cargo

-` $ cargo new hello_cargo`
- `$ cd hello_cargo`

##### Directly from the docs:::

The first command creates a new directory called `hello_cargo`. 
We’ve named our project `hello_cargo`, and Cargo creates its files 
in a directory of the same name.

Go into the hello_cargo directory and list the files. 
You’ll see that Cargo has generated two files and one directory for us:
 a Cargo.toml file and a src directory with a main.rs file inside.

__It has also initialized a new Git repository along with a .gitignore file. 
Git files won’t be generated if you run cargo new within an existing Git repository; 
you can override this behavior by using cargo new --vcs=git.__

When you are ready to build, `cargo build` when in the `src` folder puts the
 executable in `target/debug/` as `project_name`, make a `lock` file of your dependencies.
 
 `cargo run` will build and run the executable
 
 `cargo check` makes sure that it will compile, but doesn't build the executable. 
 Obviously this is quicker.
 
 ##### Directly from the docs ::: releasing
 
 When your project is finally ready for release, 
 you can use cargo `build --release` to compile it with optimizations. 
 This command will create an executable in `target/release` instead of target/debug.
 The optimizations make your Rust code run faster, but turning them on lengthens
 the time it takes for your program to compile. This is why there are two
 different profiles: one for development, when you want to rebuild quickly 
 and often, and another for building the final program you’ll give to a user 
 that won’t be rebuilt repeatedly and that will run as fast as possible. 
 If you’re benchmarking your code’s running time, 
 be sure to run cargo `build --release` and benchmark with the
 executable in `target/release`.
 
 For more information about Cargo, 
 check out its documentation: https://doc.rust-lang.org/cargo/
 
 # Chapter 2
 
 ## Guessing game
 
 You have to import a bunch of stuff for stuff to work
 
 - namespacing/scope looks like R (i.e. ::)
 - don't forget the semi-colons at the end of every line
 
 ```
 use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
``` 

- `use` declares your dependencies
- `fn function_name(arguments){body}` defines functions
- `fn main(){your entire program goes here}`
- `println!("Prints a thing to the screen. Is a macro.")` 
- `let x = 10` is variable assignment. Without including `let mut x = 10`, 
variables are immutable. Trying to reassign will throw an error.
- `//` comments out stuff on a single line


##### Directly from the docs ::: types

`String` is a string type provided by the standard library that is a growable, 
UTF-8 encoded bit of text.
- The :: syntax in the ::new line indicates that new is an associated function
 of the `String` type. An associated function is implemented on a type,
 in this case String, rather than on a particular instance of a String.
 Some languages call this a static method.
- This new function creates a new, empty string.
 You’ll find a new function on many types, 
 because it’s a common name for a function that makes a new value of some kind.
- The `&` indicates that this argument is a reference, 
which gives you a way to let multiple parts of your code access 
one piece of data without needing to copy that data into memory multiple times. 
References are a complex feature, and one of Rust’s major advantages is how 
safe and easy it is to use references. __You don’t need to know a lot of those details 
to finish this program.__ For now, all you need to know is that like variables, 
references are immutable by default. Hence, you need to write `&mut guess` 
rather than `&guess` to make it mutable. 
(Chapter 4 will explain references more thoroughly.)


__If we hadn’t put the use `std::io` line at the beginning of the program, 
we could have written this function call as `std::io::stdin`.__

- Seems like R namspacing, only deeper? The next bit really looks a bit python-y?


Another neat feature of Cargo is that you can run the `cargo doc --open` command, 
which will build documentation provided by all of your dependencies locally 
and open it in your browser. If you’re interested in other functionality
in the rand crate, for example, run cargo doc --open and click rand 
in the sidebar on the left.


__**traits will be covered in chapter 10, 
 enums will be covered in chapter 6 (error handling/results etc), 
 references will be covered in chapter 4 **__

#### Comparing the Guess to the Secret Number

ADD
`use std::cmp::Ordering;`

```
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```


- Multi-line things are done with curly braces => {
	line one;
	line two;
}

- `match` statements are just like `switch` statements 

- `continue` takes you to the next iteration of the loop
- other things can be returned before `continue`
- _ is the catch-all (like *)
- not sure what the & before `variable` is doing
- `break` breaks the loop - makes it act like a while?
- `let` shadows variables  - reassigning


## Chapter 3

NB: there are reserved keywords - find them in rust book appendix A

CONTENT

variables, basic types, functions, comments, and control flow


### Variables & Mutability

- variables default to immutable

```
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
`cargo run variables`

```
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: make this binding mutable: `mut x`
3 |        println!("The value of x is: {}", x);
4 |        x = 6;
  |        ^^^^^ cannot assign twice to immutable variable
```

- confusing; they say use `let` for assignment, but `let` causes shadowing so these errors do not happen


```
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
}
```
`cargo run variables`

```
   Compiling variables v0.1.0 (/Users/jprep/Documents/github_personal_projects/rust_book/chapter_3/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 1.00s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

- double confusing - it doesn't allow assignment without let 

```
fn main() {
    x = 5;
       println!("The value of x is: {}", x);
    x = 6;
       println!("The value of x is: {}", x);
}
```

gives:

```
(base) 1-01-184:variables jprep$ cargo run
   Compiling variables v0.1.0 (/Users/jprep/Documents/github_personal_projects/rust_book/chapter_3/variables)
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:2:5
  |
2 |     x = 5;
  |     ^ not found in this scope

error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:3:42
  |
3 |        println!("The value of x is: {}", x);
  |                                          ^ not found in this scope

```
but if you explicitly define it with `let mut`, it is happy with = as a reassignment:


```
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```


```
(base) 1-01-184:variables jprep$ cargo run
   Compiling variables v0.1.0 (/Users/jprep/Documents/github_personal_projects/rust_book/chapter_3/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s 	
	Running `target/debug/variables`
	The value of x is: 5
	The value of x is: 6
```

#### Differences Between Variables and Constants

- cannot `mut` a constant (makes sense)
- `const` is used to define a constant
- constants must have an explicitly defined type
- constants can be declared in any scope
	- not sure about this yet, but assume global/package/namespace?
- constants must be a constant expression, not the result of a function or a value at runtime
	- i think that just means it must be clearly defined from the start
- conventions:
	- always CAPS constants
	- underscores can be used to enhance readability (of numbers) 
		- 100_000_000 == 100,000,000 == 100000000

##### Direct from the docs :::

__Constants are valid for the entire time a program runs, within the scope they were declared in, 
making them a useful choice for values in your application domain that multiple parts of 
the program might need to know about, such as the maximum number of points any player of a 
game is allowed to earn or the speed of light.

Naming hardcoded values used throughout your program as constants is useful in conveying the 
meaning of that value to future maintainers of the code. It also helps to have only one place 
in your code you would need to change if the hardcoded value needed to be updated in the future.__

#### Shadowing

- __you can declare a new variable with the same name as a previous variable,
 and the new variable shadows the previous variable__
- I don't see how this is different from default mutable variables 
	- does it have any kind of warning that a variable name has been used before?
	- doesn't seem to?
	- __Shadowing is different from marking a variable as mut, because we’ll get a 
	compile-time error if we accidentally try to reassign to this variable without 
	using the let keyword. By using let, we can perform a few transformations on a
	value but have the variable be immutable after those transformations have been 
	completed.__
		- Sure, but I guess I have just trained myself into that with R
	- `let` allows you to alter the type (again, yep, makes sense)
- oh ok:
```
let space = "   ";
let space = space.len();
```
is fine

``` 
let mut space = "   ";
spac = space.len();
```
is a type error

- `let` is actual reassignment
- `mut` allows you to change JUST THE VALUE - so I guess useful for IO/interactive things


### Data types

- Keep in mind that Rust is a statically typed language,
 which means that it must know the types of all variables at compile time.
- scalar/compound
	- scalar == single value
		- int
			- u = unsigned (positive only)
			- i = signed (pos + neg)
			- 6/16/23/62/128/arch bit numbers
			- `isize`/`usize` are also available as default-bit-level-for-your-system
			
#### Docs say :::

__So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are 
generally good choices, and integer types default to i32: this type is generally the 
fastest, even on 64-bit systems. The primary situation in which you’d use isize or usize 
is when indexing some sort of collection.__

- float
	- `f32` and `f64`
	- defaults to f64 because it is more precise
- `bool`
	- true/false (lower case)
- char
	- Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. 
	- see chaper 8 for more
	
	
### Compound Types

- Compound types can group multiple values into one type.
- Rust has two primitive compound types: tuples and arrays.



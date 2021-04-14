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

### THE & THING
- it is about OWNERSHIP of memory
- https://youtu.be/s19G6n0UjsM?t=1353
- T = any object 
	- `T` means you own it and are responsible for freeing it
	- `&mut T` means you have a exclusive read/write access, but you are not responsible for freeing it
	- `&T` means you have read-only access
	- rust compiler checks that you do not have
		- `&mut T` AND `T` at the same time
		- multiple `&mut T` at the same time
	
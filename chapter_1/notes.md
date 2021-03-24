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
    
    
	
	
	
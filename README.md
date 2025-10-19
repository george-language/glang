<div align="center">
  <picture>
    <img
         src="assets/george_language_icon.svg"
         width="25%">
  </picture>

[Website](https://george-language.github.io/) | [Download](https://george-language.github.io/docs/install/) | [Learn](https://george-language.github.io/book/) | [Documentation](https://george-language.github.io/docs/)

_A dog-themed, interpreted programming language for beginners._
</div>

## Why George Language?

GLang was made because beginner-friendly languages don’t have to be boring. GLang is:

- **Approachable:** Simple syntax and fun naming conventions to help users learn quick. Functions like `bark`, `chew`, and `uhoh` make coding more intuitive and playful.

- **Educational:** Teaches real programming concepts under the hood, like data structures (lists, strings) and collections such as hashmap.

- **All-In-One:** The `glang` binary includes the GLang runtime, package management, and component tools like `glang self update`

## Syntax

GLang’s syntax is designed to be fun, easy, and good on the eyes, while still teaching real programming concepts.

### 🦴 Basic Types

```
# glang has 3 primary types
obj list = [1, 2, 3];
obj string = "This is a string";
obj number = 3.14;
```

### 🔒 Constants

```
# "stay" makes a value constant
stay CONSTANT = true;
```

### 🐕 Functions

```
# Define a function with "func"
func example(arg1) {
    give arg1;  # 'give' = return
}
```

### 🤔 Conditionals

```
if 1 == 2 {
    bark("Math broke!");
} alsoif 1 == 3 {
    bark("Math is very broken!");
} otherwise {
    bark("Math is working just fine.");
}
```

### 🔁 Loops

```
# While loop
while true {
    leave;  # stop a loop
}

# For loop
walk i = 0 through 10 {
    bark(i);
}
```

### 📦 Imports

```
# Bring in external modules
fetch std_math;

bark(math_pi);
```

### 🧩 Error Handling

```
try {
    1 / 0;
} catch error {
    bark("Some error occurred: " + error);
}
```

## Features

- 🐶 Whimsical, ultra-friendly syntax
- 📚 Built-in modules for math, strings, and more
- 💬 Easy-to-understand functions like `dig()`, `bury()`, and `uhoh()`
- 📦 Package management with `kennels` and extensibility with `fetch`
- 🌐 Open source and growing community

## Installation

You can download GLang [here](https://george-language.github.io/docs/install/), or check out the quick setup instructions in the [guide book](https://george-language.github.io/book/).

## License

George Language is licensed under **GPL v3**. That means it's **free, open source, and always will be** just like George's spirit!

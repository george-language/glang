> [!NOTE]
> This is GLang's rust-based branch, and is under active development. We plan to rewrite everything to Rust and
merge into the `main` branch by mid 2025.

<div align="center">
  <picture>
    <img
         src="resources/icons/george_language_icon.svg"
         width="25%">
  </picture>

[Website][GLang] | [Download] | [Learn] | [Documentation]
</div>

This is the main source code repository for GeorgeLanguage ([GLang]). It contains the interpreter,
standard library, and built-in modules.

[GLang]: https://sites.google.com/view/george-lang/home/
[Download]: https://sites.google.com/view/george-lang/install/
[Learn]: https://sites.google.com/view/george-lang/documentation/guide-book/
[Documentation]: https://sites.google.com/view/george-lang/documentation/standard-docs/

## Showcase
```
# import the math module
fetch("modules/math.glang");

obj x = 0;

# let's go for a walk!
walk i = 0 through 10 {
    obj x = x + 1;

    if x == 5 {
        leave;
    }
}

# print the value of 'x'
bark(x);

# greet someone
func greet(name) {
    bark("Hello, " + name + "!");
}

greet("John Doe");

bark("Pi is equal to: " + tostring(math_pi));
bark("We have reached the end of our program. I hope you enjoyed!");
```

## Why Make George Language?
It all starts from a joke😆

You see, one of our team members has a cute pug named George. One day we said
_"Why not make a language named after George that's funny, cute, and easy to learn?"_ And that's when it was born.

We made GLang because we believe there needs to be a more modern language for beginners. Not only is it easy to learn, it teaches users common programming concepts (and it's fun to use!)

- **If you want to print something**, just use the `bark` function.

- **If you want to get user input**, just use the `chew` function.

- **If you want to panic a program**, just use the `uhoh` function.

It's names like these that bring humor to programming. Beginners remember "Hey, I want to see this variable, let's make the computer bark!"

## License
GLang is licensed entirely under the GPL-v3. This means GLang is open source forever, and free until the end of time.

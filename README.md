> [!NOTE]
> This is GLang's rust-based branch, and is under active development. We plan to rewrite everything to Rust and
merge into the `main` branch by mid 2025.

<img src="resources/icons/george_language_icon.svg" width="125">

# George Language
George Language (GLang for short) is a dynamically-typed, interpreted programming language where program syntax is
represented as a dog doing day-to-day activities.

```
gettoy "modules/math.glang";

obj x = 0;

# Lets go for a walk!
walk i = 0 through 10 {
    obj x = x + 1;

    if x == 5 {
        leave;
    }
}

bark(x);

# Greet someone!
func greet(name) {
    bark("Hello, " + name + "!");
}

greet("George");

bark("Pi is equal to: " + tostring(math_pi));
bark("We have reached the end of our program. I hope you enjoyed!");
```

## Why Make George Language?
Well, it all started as a joke. You see, one of our team members has a cute pug named George. One day we said
_"Why not make a language named after George that's funny and cute?"_ And that's when it was born.

GLang makes programming more expressive and easier to read, using a mix of humor and fun to define the syntax.

To a beginner programmer, they have to spend a lot of time learning and remembering keywords, built-ins, and
values in the language. GLang instead uses _easy-to-remember_ language, for example the `print` statement
is actually `bark`!

```
bark("Hello, World!")
```

This is just one of the many examples that makes GLang easier to learn and work with.

## Speed
As of recent updates, GLang gets _fast_ performance due to it's simplicity and new Rust backend. It doesn't require a garbage collector or type checker, just lexing -> parsing -> interpreting. _That's it._

## REPL
GLang includes it's own Read-Eval-Print-Loop (REPL). As long as you have GLang installed, there's no need for a text editor if you want to use GLang strictly in the terminal.


_More info on this project is available on our [website](https://sites.google.com/view/george-lang/home)._

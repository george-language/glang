> [!NOTE]
> This is GLang's rust-based branch, and is under active development. We plan to rewrite everything to Rust and
merge into the `main` branch by mid 2025.

<img src="resources/icons/george_language_icon.svg" width="125">

# George Language
George Language (GLang for short) is a dynamically-typed, interpreted programming language where program syntax is
similar to a dog's day-to-day activities.

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
It all starts from a joke.

You see, one of our team members has a cute pug named George. One day we said
_"Why not make a language named after George that's funny and cute?"_ And that's when it was born.

We made GLang because we believe there needs to be a more modern beginner language. Not only is it easier to learn, it **teaches users** common programming concepts and it's fun to use. If we want to print something, just use the `bark` built-in. If we want to indicate an error, just use the `uhoh` built-in.

## Its Fast
As of recent updates, GLang gets _fast_ performance due to it's simplicity and new Rust backend. It doesn't require a garbage collector or type checker, just lexing -> parsing -> interpreting. _That's it._

## Its REPL Based
GLang includes it's own Read-Eval-Print-Loop (REPL). As long as you have GLang installed, there's no need for a text editor if you want to use GLang strictly in the terminal.


_More info on this project is available on our [website](https://sites.google.com/view/george-lang/home)._

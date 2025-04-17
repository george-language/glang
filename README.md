<img src="resources/icons/george_language_icon.svg" width="125">

# George Language
George Language (GLang for short) is a dynamically-typed, interpreted programming language where program syntax is 
represented as a dog doing day-to-day activities.

```
gettoy(["math_pi"], "modules/math.glang")

object x = 0

# Lets go for a walk!
walk i = 0 through 10 then
    object x = x + 1

    if x == 5 then
        leave
    endbody
endbody

bark(x)

# Greet someone!
func greet(name)
    bark("Hello, " + name + "!")
endbody

greet("George")

bark("Pi is equal to: " + tostring(math_pi))
bark("We have reached the end of our program. I hope you enjoyed!")
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

## Is It fast?
GLang gets performance matching languages like Python (as it is implemented in Python.) 
It uses a very light type checker, but nothing large that creates a major overhead. It also 
doesn't require a garbage collector, as that is already managed by Python. This makes GLang
**FAST** for many applications.

## In The Terminal?
GLang works in the terminal by itself as well. Use the `;` character to define multi-line programs
on one line.

More info on this project is available on our [website](https://sites.google.com/view/george-lang/home).
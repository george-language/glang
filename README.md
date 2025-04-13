<img src="resources/icons/george_language_icon.svg" width="125">

# George Language
George Language is a dynamically-typed, interpreted programming language where program syntax is 
represented as a dog doing day-to-day activities.

```
gettoy math_pi from "modules/math.glang"

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

bark("Pi is equal to: " + math_pi)
bark("We have reached the end of our program. I hope you enjoyed!")
```

## Why make George Language?
Well, it all started as a joke. You see, one of our team members has a cute pug named George. One day we said 
"Why not make a language named after George with a cute syntax?" And that's when it was born.

George Language makes programming more expressive; think of it like turning "not" into "opposite of", it 
turns the word "not" into an easier to understand value, meaning "the opposite of something". George Language 
already does this by replacing the `not` keyword with `oppositeof`:

```
object x = oppositeof true # false!
```

Another great example is instead of using `var` for variables, we use `object` to represent an "object":

```
object x = 10
```

## Is it fast?
George Language gets performance matching languages like Python (as it is implemented in Python.) 
It uses a very light type checker, but nothing large that creates a major overhead. It also 
doesn't require a garbage collector, as that is already managed by Python. This makes George
Language **FAST** for many applications.

## In The Terminal?
George Language works in the terminal by itself as well. Use the `;` character to define multi-line programs
on one line.

# Keywords, values, built-ins, and more
A full list of all keywords, values, and built-ins is listed below:

| Syntax                                                   | Purpose                                                                               |
|----------------------------------------------------------|---------------------------------------------------------------------------------------|
| `object [var_name] = [value]`                            | Assign a variable, can either be a number, string (`"string"`), or anonymous function |
| `bark([value])`                                          | Print an output to the terminal                                                       |
| `func [function_name]([args])`                           | Define a callable function                                                            |
| `true`                                                   | Value representing `True`                                                             |
| `false`                                                  | Value representing `False`                                                            |
| `nothing`                                                | Value representing `Null` or `None`                                                   |
| `oppositeof [value]`                                     | Return the opposite of a value                                                        |
| `[value] and [value]`                                    | Return `true` if `[value]` and `[value]` otherwise return `false`                     |
| `[value] or [value]`                                     | Return `true` if one `[value]` is correct otherwise return `false`                    |
| `walk [var_name] = [a] through [b] step [i] then <expr>` | For loop (step argument is optional)                                                  |
| `while [condition] then <expr>`                          | While loop                                                                            |
| `if [expr] then <expr>`                                  | If expression                                                                         |
| `alsoif [expr] then <expr>`                              | Else-if expression (used with if expression)                                          |
| `otherwise <expr>`                                       | Else expression (used with if expression)                                             |
| `gettoy [var_names, functions] from [glang_file]`        | Import variables, functions, or lists into your program from another program file     |
| `endbody`                                                | Specify the ending of a `funky`, `walk`, or `while`                                   |

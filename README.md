<img src="resources/icons/george_language_icon.svg" width="125">

# George Language
Programming, _Simplified_.

```
smt my_var = oppositeof yes

# For loop!
walk i = 0 through 10 do
    bark(i) # print i to the terminal
    if i == 5
        smt my_var = yes # redeclare the variable
        end # end the loop
end # end the loop by default

# Greet someone!
funky greet(name)
    bark("Hello, " + name + "!")
end

greet()

# Greet someone on a single line!
funky greet(name) -> bark("Hello, " + name + "!")

greet()

bark("We have reached the end of our program. I hope you enjoyed!")
```

# Why make George Language?
Well, it all started as a joke. You see, one of our team members has a cute pug named George. One day we said 
"Why not make a language named after George with a simple syntax?" And that's when it was born.

George Language makes programming syntax more expressive; think of it like turning "not" into "opposite of", it 
turns the word "not" into an easier to understand value, meaning "the opposite of something". George Language 
already does this by replacing the `not` keyword with `oppositeof`:

```
smt x = oppositeof true # false!
```

Another great example is instead of using `var` for variables, we use `smt` to represent "something":

```
smt x = 10
```

A full list of all functions and built-ins is listed below:

| Syntax                                    | Purpose                                                            |
|-------------------------------------------|--------------------------------------------------------------------|
| `smt [var_name] = [value]`                | Assign a variable                                                  |
| `bark([value])`                           | Print an output to the terminal                                    |
| `funky [function_name]([args])`           | Define a callable function                                         |
| `true`                                    | Value representing `True`                                          |
| `false`                                   | Value representing `False`                                         |
| `nothing`                                 | Value representing `Null` or `None`                                |
| `oppositeof [value]`                      | Return the opposite of a value                                     |
| `[value] and [value]`                     | Return `true` if `[value]` and `[value]` otherwise return `false`  |
| `[value] or [value]`                      | Return `true` if one `[value]` is correct otherwise return `false` |
| `walk [var_name] = [a] through [b] do []` | For loop                                                           |
| `while [condition] do []`                 | While loop                                                         |
| `end`                                     | Specify the ending of a `call`, `walk`, or `while`                 |

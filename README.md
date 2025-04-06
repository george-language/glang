# GeorgeLanguage
The simple programming language for everyone!

```
smt my_var = oppositeof yes

# For loop!
walk i = 0 through 10 do
    shoutout(i) # print i to the terminal
    if i == 5
        smt my_var = yes # redeclare the variable
        end # end the loop
end # end the loop by default

# Greet someone!
call greet(name)
    print("Hello, " + name + "!")
end

greet()

# Greet someone on a single line!
call greet(name) -> print("Hello, " + name + "!")

greet()

print("We have reached the end of our program. I hope you enjoyed!")
```

GeorgeLanguage was created to simplify programming syntax with common day-to-day language and more _expressive_ 
functions. Instead of using `var` for variables, we use `smt` to represent a "something":

```
smt x = 10
```

A full list of all functions and built-ins is listed below:

| Syntax                                    | Purpose                                            |
|-------------------------------------------|----------------------------------------------------|
| `smt [var_name] = [value]`                | Assign a variable                                  |
| `shoutout([value])`                       | Print an output to the terminal                    |
| `call [function_name]([args])`            | Define a callable function                         |
| `true`                                    | Value representing `True`                          |
| `false`                                   | Value representing `False`                         |
| `nothing`                                 | Value representing `Null` or `None`                |
| `oppositeof [value]`                      | Return the opposite of a value                     |
| `walk [var_name] = [a] through [b] do []` | For loop                                           |
| `while [condition] do []`                 | While loop                                         |
| `end`                                     | Specify the ending of a `call`, `walk`, or `while` |

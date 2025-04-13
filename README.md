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
_"Why not make a language named after George with a cute syntax?"_ And that's when it was born.

George Language makes programming more expressive and easier to read, for example, let's declare
a variable "the opposite of true", or false.

```
object x = oppositeof true
```

This syntax replaces the "not" keyword used in other languages, only building off of simplicity.

## Is It fast?
GLang gets performance matching languages like Python (as it is implemented in Python.) 
It uses a very light type checker, but nothing large that creates a major overhead. It also 
doesn't require a garbage collector, as that is already managed by Python. This makes GLang
**FAST** for many applications.

## In The Terminal?
GLang works in the terminal by itself as well. Use the `;` character to define multi-line programs
on one line.

## Install
Pre-built installers are currently available for Windows only. To install, go to the releases page and find 
the latest release binary (.exe) and download it. Run the installer and follow the instructions inside it.

After GLang is installed, add the `GeorgeLanguage` folder location to the 
[system path variable](https://superuser.com/questions/1861276/how-to-set-a-folder-to-the-path-environment-variable-in-windows-11)
then you can use it anywhere on your PC!

# Keywords, Values, Built-ins, and More
A full list of all keywords, values, and built-ins is listed below:

| Syntax                                                   | Purpose                                                                               |
|----------------------------------------------------------|---------------------------------------------------------------------------------------|
| `object [var_name] = [value]`                            | Assign a variable, can either be a number, string (`"string"`), or anonymous function |
| `bark([value])`                                          | Print an output to the terminal                                                       |
| `chew([query])`                                          | Get user input from the terminal                                                      |
| `chewnum([query])`                                       | Get user input as a Number from the terminal                                          |
| `gettoy([var_names, functions], [glang_file])`           | Import variables, functions, or lists into your program from another program file     |
| `isnumber([object])`                                     | Check if a value is type Number                                                       |
| `isstring([object])`                                     | Check if a value is type String                                                       |
| `islist([object])`                                       | Check if a value is type List                                                         |
| `isfunc([object])`                                       | Check if a value is type Function                                                     |
| `tonumber([object])`                                     | Return a String (e.g. "1.0") as a Number                                              |
| `tostring([object])`                                     | Return a Number (e.g. 1.0) as a String                                                |
| `append([list], [object])`                               | Append an object to a List                                                            |
| `pop([list], [index])`                                   | Pop the List at that index                                                            |
| `extend([list_a], [list_b])`                             | Extend a List to another List (combine the two lists)                                 |
| `reverse([list])`                                        | Reverse a List                                                                        |
| `reversed([list])`                                       | Return a reversed version of a List                                                   |
| `clear([list])`                                          | Clear all the values in a List                                                        |
| `lengthof([list or string])`                             | Return the length of a List or String                                                 |
| `throw([details])`                                       | Toss an error to the interpreter with the specified details                           |
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
| `endbody`                                                | Specify the ending of a `func`, `walk`, `if` or `while` statement                     |

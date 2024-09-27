# CobaiLang
> [!NOTE]
> This project is on a experimental stage. It may not be completed anytime soon (if ever).

Cobai is a high level statically typed object oriented programming language that aims for simplicity and strong typing.

Inspired by GDScript, Lua, Java and my own stupidity.

## Examples
[Longer examples](https://github.com/Wolfyxon/cobai-lang/tree/main/examples)

```js
// Instead of just true and false there's also maybe
if maybe:
  print("Maybe this will show up")
end
```

```js

// input() returns String, so 'name' is automatically assigned to that type
var name = input("What's your name? ")
print("Hello", name)

name = 2 + 2 // Error! 'name' can only hold String values
```
```rs
public class LifeForm:
  public var age: int = 0
end

public class Human of LifeForm:
  public var name: String

  public func new(name: String) -> self:
    self.name = name
  end

  public func getGreeting(someone: Human) -> String:
    return "Hi " + someone.name
  end
end

var h1 = Human("Bob")
var h2 = Human("Alice")

print(h1.getGreeting(h2))
```
## Why 'Cobai'
*Cobai* means *guinea pig* in Romanian and I think gunea pigs are cute so why not use that.

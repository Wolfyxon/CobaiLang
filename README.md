# MorskaLang
> [!NOTE]
> This project is in a very early experimental stage. It won't be completed anytime soon (if ever).

Morska is a high level statically typed object oriented programming language that aims for simplicity and strong typing.

Inspired by GDScript, Lua, Java and my own stupidity.

## Examples
[Longer examples](https://github.com/Wolfyxon/MorskaLang/tree/main/examples)

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
pub class LifeForm:
  pub var age: int = 0
end

pub class Human of LifeForm:
  pub var name: String

  pub func new(name: String) -> self:
    self.name = name
  end

  pub func getGreeting(someone: Human) -> String:
    return "Hi " + someone.name
  end
end

var h1 = Human("Bob")
var h2 = Human("Alice")

print(h1.getGreeting(h2))
```
## Why 'Morska'
*Morska* refers to *Świnka morska* which means *guinea pig* in Polish.

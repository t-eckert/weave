# Weave

This is an experimental scripting language. My goal is to have something that is "batteries included" like Python, allows for simple integration of command line tools like Bash, and works with YAML and JSON as first-class citizens.

Don't take any of this too seriously.

Weave aims to add some of the niceties of scripting with a language like Python, with minimal overhead.

Argument reading works like it does in many shell-scripting languages like Bash and Zsh.

```wv
# hello.wv
print("Hello, " + $1 + ".")
```

```shell
$ weave run hello.wv Sam
Hello, Sam.
```

## Installation with Cargo

```shell
cargo install github.com/t-eckert/weave
```

## Features

### Discriminated Unions

Define type aliases with string literal unions for type-safe values:

```wv
type Status = "active" | "inactive" | "pending"
type Size = "sm" | "md" | "lg"
```

### Structs with Typed Fields

Create structured data with type-checked fields:

```wv
struct Pizza {
    crust: Crust,
    size: Size,
    price: number,
    discount: number,
}

let pizza = Pizza {
    crust: "thin",
    size: "md",
    price: 10.99,
    discount: 0.2,
}
```

### Associated Functions (Method Syntax)

Functions with a struct as the first parameter can be called with dot notation:

```wv
fn tax(pizza: Pizza, percent: number) -> number {
    return pizza.price * pizza.discount * percent
}

# Both calling styles work:
tax(pizza, 0.06)       # Traditional function call
pizza.tax(0.06)        # Method syntax
```

### Type Annotations

Add optional type checking to function parameters:

```wv
fn process(status: Status, count: number) {
    print(status)
}

process("active", 5)   # ✅ Type-checked at runtime
process("invalid", 5)  # ❌ Type error: not in union
```

### Comments

Line comments start with `#`:

```wv
# This is a line comment
type Color = "red" | "green" | "blue"  # inline comments work too
```

## Examples

Try running the examples:

```bash
cargo run -- run examples/hello.wv
cargo run -- run examples/pizza.wv
cargo run -- run examples/type-test.wv
```

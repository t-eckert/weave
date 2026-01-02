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


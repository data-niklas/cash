# cash
Simple interpreter built in Rust using Pest

## Why?
This interpreter was built for educational purposes. It is not optimized and not meant for heavy usage.

## Improvements
Improvements, which a newer interpreter would benefit from
- Higher reference usage (instead of slow data cloning)
- Custom tokenizer / AST builder
- Custom terminal library (which handles hotkeys / auto-completion / history / ...)
- AST optimizations
- threads
- better error handling

## Features
- High level functions
- Loops / Conditionals / ...
- Several datatypes: `int`, `float`, `bool`, `string`, `array`, `dict`, `range`, `none`, `function`, `error`
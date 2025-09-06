# Ractor Macro DSL - Best Practices Guide

## Current State Analysis

Your macro provides a clean interface for defining ractor actors:

```rust
#[actor(msg=MyMessage, state=MyState, args=MyArgs, pre_start=init_fn)]
struct MyActor;
```

However, there are several areas for improvement to follow proc macro best practices.

## Issues to Address

### 1. Code Duplication
- `lib.rs` and `args.rs` contain duplicate `ActorArgs` parsing logic
- Consolidate into a single, well-structured implementation

### 2. Error Handling
- `lib.rs:87-88` uses `.unwrap()` instead of proper error propagation
- Should use `?` operator for cleaner error handling

### 3. File Organization
- Multiple implementations suggest refactoring in progress
- Need to remove old code and use the cleaner `args.rs` version

## Best Practices for Proc Macro Design

### 1. Modular Architecture
```
src/
├── lib.rs              # Main entry point, minimal logic
├── parse/              # All parsing logic
│   ├── mod.rs
│   ├── actor.rs        # Actor-specific parsing
│   └── validation.rs   # Input validation
├── expand/             # Code generation
│   ├── mod.rs
│   └── actor.rs        # Actor trait implementation
└── error.rs            # Centralized error handling
```

### 2. Robust Error Handling
```rust
// Instead of unwrap()
let msg = require_field(msg, input.span(), "missing `msg=...`")?;
let state = require_field(state, input.span(), "missing `state=...`")?;
```

### 3. Type Validation
Add compile-time checks that types implement required traits:
```rust
// Generate validation code
quote! {
    const _: fn() = || {
        fn assert_msg<T: Send + 'static>() {}
        fn assert_state<T: Send + 'static>() {}
        assert_msg::<#msg>();
        assert_state::<#state>();
    };
}
```

### 4. Ergonomic DSL Design

#### Option 1: Named Parameters (Current)
```rust
#[actor(msg=GameMessage, state=GameState)]
struct GameActor;
```

#### Option 2: Derive-Style (More Ergonomic)
```rust
#[derive(Actor)]
#[actor(msg=GameMessage, state=GameState)]
struct GameActor;
```

#### Option 3: Builder Pattern
```rust
#[actor]
impl GameActor {
    type Msg = GameMessage;
    type State = GameState;
    
    async fn pre_start(&self, args: Self::Arguments) -> Result<Self::State> {
        // custom logic
    }
}
```

### 5. Default Implementations
Provide sensible defaults to reduce boilerplate:
```rust
// If no args specified, default to ()
// If no pre_start specified, default to args.into()
#[actor(msg=GameMessage, state=GameState)]
struct GameActor; // args=(), pre_start uses Into trait
```

### 6. Documentation Generation
Generate helpful documentation:
```rust
quote! {
    #[doc = concat!("Actor implementation for ", stringify!(#name))]
    #[doc = concat!("Message type: ", stringify!(#msg))]
    #[doc = concat!("State type: ", stringify!(#state))]
    impl ractor::Actor for #name {
        // ...
    }
}
```

## Recommended Action Plan

1. **Consolidate Code**: Use `args.rs` parsing, remove duplicate from `lib.rs`
2. **Fix Error Handling**: Replace unwrap with proper Result propagation
3. **Add Validation**: Implement compile-time trait bound checking
4. **Improve Structure**: Separate parsing, validation, and expansion concerns
5. **Enhance DSL**: Consider adding derive-style syntax or builder patterns
6. **Add Tests**: Create comprehensive test suite with `trybuild` for error cases

## Example Clean Implementation Structure

```rust
// lib.rs - minimal entry point
#[proc_macro_attribute]
pub fn actor(attr: TokenStream, item: TokenStream) -> TokenStream {
    match actor_impl(attr, item) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}

fn actor_impl(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let args = syn::parse2::<ActorArgs>(attr.into())?;
    let input = syn::parse2::<DeriveInput>(item.into())?;
    
    let validated = args.validate(input.span())?;
    let expanded = expand_actor_impl(&input, &validated)?;
    
    Ok(expanded.into())
}
```

This approach separates concerns, provides better error messages, and makes the macro more maintainable and extensible.
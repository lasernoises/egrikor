# Egrikor

An attempt at a Rust GUI library.

**Note:** As of writing this I haven't worked on this project in a few years. My obsession about
this topic hasn't gone anywhere however. One thing I have done in this time is I've created
[fluorine](https://github.com/lasernoises/fluorine), a reactivity library. I've also been working on
various other experiments, which aren't public as of writing this.

## Design

The exact design changed a lot while I was working on this.

One thing that was relatively constant however is this:

```rust
trait Widget {
    type State;
}
```

The widgets would always exist only temporarily during a given pass and could reference other state.
The state would instead hold persistent state of a widget. That includes both layout related state
and anything else that is local to the widget.

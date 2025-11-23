# Egrikor

An attempt at a Rust GUI library.

**Note:** As of writing this I haven't worked on this project in a few years. My obsession about
this topic hasn't gone anywhere however. One thing I have done in this time is I've created
[fluorine](https://github.com/lasernoises/fluorine), a reactivity library. I've also been working on
various other experiments, which aren't public as of writing this.

## Design

The exact design changed a lot while I was working on this.

Some things were relatively constant however:

### Associated State Type

```rust
trait Widget {
    type State;
}
```

The widgets would always exist only temporarily during a given pass and could reference other state.
The state would instead hold persistent state of a widget. That includes both layout related state
and anything else that is local to the widget.

### No Widget IDs

I tried to build it such that no widget IDs would be necessary. At least not global ones. The basic
idea being that the state is all that's needed. I feel that having IDs for the widgets could lead
to the state being a bit entangled. But this is definitely a matter of taste to a degree and it also
interacts with some other design decisions. Most importantly it interacts with the decision to have
a singly owned widget and state tree. This means that getting to a widget (or its state) just by its
ID isn't an easy problem because it would mean that each container would have to have some sort of
list of all the widget IDs it contains, both directly and indirectly. I've seen bloom filters used
for this, but I don't like that very much. I felt that instead simply not having global identifiers
and designing around that would be more elegant. It does definitely come with a set of challenges
however.

### Layout

The layout process loosley follows the flutter layout protocol. The main idea being that a widget
gets passed in a simple constraint (the details vary) and returns a size. The difference to flutter
in my approach is that I treat the returned size as a min-size instead of a fixed one. That way we
can do things like expand all the widgets in a row to the maximal height all the children of the row
without needing to measure multiple times.

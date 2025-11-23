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

## Problems

Some interesting problems I ran into along the way:

### Lifetimes in Closure Returns

Unfortunately Rust doesn't really have a way to write anything like this:

```rust
fn abc(f: impl for<'a> Fn(&'a ()) -> impl Widget + 'a);
```

If there isn't a lifetime from the closure involved you can do something like this:

```rust
fn abc<W: Widget>(f: impl Fn() -> W);
```

The workaround for this was to instead pass some sort of thunk to the closure and then have the
closure pass the widget to that. But that in turn usually requires that we dynamically allocate the
state for the widget.

### Impl Trait Returns

```rust
fn my_widget<W: Widget>(inner: W) -> impl Widget;
```

In this case the issue is that if `W` contains a lifetime, the returned widget also has that
lifetime in its type now, which is correct. But the problem is that then the associated `State` type
does as well. That's a problem because `State` is supposed to be `'static`, or at least it has to be
able to outlive `W`.

To work around this I enabled the unstable feature `type_alias_impl_trait` so we could say that the
associated state type is an existential type. We could of course also try to always fully name the
`State` type we return, but that would become very bothersome very quickly and would possibly run
into trouble around closures.

My hope is that in future versions of Rust we'll be able to do something like this:


```rust
fn my_widget<S, W: Widget<State = S>>(inner: W) -> impl Widget<State = impl Sized + use<S>>;
```

Currently that is not allowed because all type parameters have to appear in the `use`.

### State Management and Mutability

The design fundamentally does make it possible to pass mutable references down the tree, as long as
they don't get stored in any `State`. The trouble starts very quickly though if there's multiple
children that need to access the same state.

```rust
// this works fine:
fn my_widget<'a>(state: &'a mut State) -> impl Widget {
    row(flex_content![
        button("abc", || *state.a += 1),
    ])
}

// but this causes trouble
fn my_widget<'a>(state: &'a mut State) -> impl Widget {
    row(flex_content![
        button("abc", || *state.a += 1),
        label(&state.a),
    ])
}
```

This is a bit unfortunate because fundamentally our design doesn't require all the widgets to exist
at the same time. But designing an API such that only ever one child widget exists at the same time
and is somewhat nice to use is a bit of a challenge. I also experimented with things that are more
like the one-way-data-flow idea and work around returning events.

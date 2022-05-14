use super::*;

pub struct IterFlexContent<I> {
    pub iter: I,
}

pub struct IterFlexContentState<T> {
    state: Vec<T>,
}

impl<T> FlexContentState for IterFlexContentState<T> {
    fn new() -> Self {
        Self { state: Vec::new() }
    }
}

impl<E, C: FlexContent<E>, I: Iterator<Item = C> + Clone> FlexContent<E> for IterFlexContent<I> {
    type State = IterFlexContentState<C::State>;

    fn all<H: FlexContentHandler<E>>(
        &mut self,
        state: &mut Self::State,
        handler: &mut H,
    ) {
        for (i, mut item) in self.iter.clone().enumerate() {
            let item_state = if let Some(s) = state.state.get_mut(i) {
                s
            } else {
                state.state.push(C::State::new());
                state.state.last_mut().unwrap()
            };
            item.all(item_state, handler);
        }
    }
}

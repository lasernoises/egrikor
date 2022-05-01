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

impl<P, E: FlexContent<P>, I: Iterator<Item = E> + Clone> FlexContent<P> for IterFlexContent<I> {
    type State = IterFlexContentState<E::State>;

    fn all<H: FlexContentHandler>(
        &mut self,
        params: &mut P,
        state: &mut Self::State,
        handler: &mut H,
    ) {
        for (i, mut item) in self.iter.clone().enumerate() {
            let item_state = if let Some(s) = state.state.get_mut(i) {
                s
            } else {
                state.state.push(E::State::new());
                state.state.last_mut().unwrap()
            };
            item.all(params, item_state, handler);
        }
    }
}

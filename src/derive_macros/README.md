Macros used to ease creation of recursive reducers. 

Reducers that are primarily made up of other reducers can have much of their behavior written by the compiler. Reducing both the amount of work required and the number of bugs that may creep into a complex application.



## Example

Extracted from an example in the repository.

- Both `menu::State` and `wgpu::State` are nested features/reducers.
- The `Box<Window>` just holds the window the two other feature are attached to.

```rust ignore
#[derive(RecursiveReducer)] // ⒈
pub struct State {
    menu: menu::State,
    wgpu: wgpu::State,

    #[not_a_reducer] // ⒉
    window: Box<Window>,
}

#[derive(Clone, From, TryInto)] // ⒊ and ⒋
pub enum Action {
    Resize { width: u32, height: u32 },
    Render,
    Redraw,

    Menu(menu::Action), // ⒌
    WGpu(wgpu::Action),
}

impl RecursiveReducer for State { // ⒍
    type Action = Action;

    fn reduce(&mut self, action: Action, _effects: impl Effects<Action = Action>) {
        match action {
            Action::Render => {
                effects.send(wgpu::Action::Render);
            }
            Action::Resize { width, height } => {
                effects.send(wgpu::Action::Resize { width, height });
                effects.send(Action::Redraw);
            }
            Action::Redraw => {
                self.window.request_redraw();
            }
            Action::Menu(_) => {}
            Action::Wgpu(_) => {}
        }
    }
}
```



1. Adding the `derive` macro to 
2. …
3. …
4. …
5. …
6. The `RecursiveReducer` trait is implemented for `State`. If `Reducer` is accidentally implemented here, the macro will produce a compiler error



## Compiler Errors

The are a few common mistakes that will produce well-known compiler errors



##### the trait bound `winit::State: composable::RecursiveReducer` is not satisfied

```sh
| #[derive(RecursiveReducer)]
|          ^^^^^^^^^^^^^^^^ the trait `composable::RecursiveReducer` is not implemented for `State`
|
= note: this error originates in the derive macro `RecursiveReducer`
```

**Cause**: You haven't yet written an `impl RecursiveReducer` for the type you added `#[derive(RecursiveReducer)]` to.

<br />


##### conflicting implementation for `State`

```sh
| #[derive(RecursiveReducer)]
|          ^^^^^^^^^^^^^^^^ conflicting implementation for `State`
...
| impl Reducer for State {
| ---------------------- first implementation here
|
= note: this error originates in the derive macro `RecursiveReducer`
```

**Cause**: You declared an `impl Reducer`, perhaps out of habit, rather than an `impl RecursiveReducer`.

<br />

##### the trait bound `…: composable::Reducer` is not satisfied

```sh
| #[derive(RecursiveReducer)]
|          ^^^^^^^^^^^^^^^^ the trait `composable::Reducer` is not implemented for `…`
|
= help: the following other types implement trait `composable::Reducer`:
          ⋮
= note: this error originates in the derive macro `RecursiveReducer`
```

where `…`  is replaced with the type of one of the struct's fields in the error message.

**Cause**: … `#[not_a_reducer]` (see ⒉).

<br />

#####  type mismatch resolving `<impl Effects<Action = Action> as Effects>::Action == Action`

```sh
| #[derive(RecursiveReducer)]
|          ^^^^^^^^^^^^^^^^ expected `child::Action`, found `parent::Action`
|
= note: `parent::Action` and `child::Action` have similar names, but are actually distinct types
```

**Cause**: … `From` (see ⒊).

<br />

##### the trait bound `menu::Action: composable::From<winit::Action>` is not satisfied

```sh
| #[derive(RecursiveReducer)]
|          ^^^^^^^^^^^^^^^^ the trait `composable::From<parent::Action>` is not implemented for `child::Action`
|
```

**Cause**: … `TryInto` (see ⒋)

- Or there is no wrapper around a child action for the `From` macro to wrap a child action with (see ⒌).

<br />

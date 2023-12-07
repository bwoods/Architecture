# WGPU

Creating a **wgpu** context in a **winit** window.



## Features

- [x] Driving a `Store` from an external event loop
  - [x] Using winit’s `UserEvent` to handle graceful shutdown between the two event loops
- [x] Mixing `reduce` and `reduce_async`
- [x] Using direct method calls when `Action`s are not necessary.
  - See `Render`
  - See `Resize`
- [x] Using the `Reducers` macro to create a combing `Reducer`.
  - [ ] Dispatching a parent event to two (sibling) child `Reducer`s
    - See `Setup`

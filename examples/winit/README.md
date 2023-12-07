# Winit

Creating a **wgpu** context in a **winit** window with **muda** (native) menus.



## Features

- [x] Managing all of the construction-order requirements between wgpu, winit and muda.
  - [x] Including use of the [`ouroboros`](https://docs.rs/ouroboros/latest/ouroboros/index.html) crate the manage the lifetime dependency between a `winit::Window` and its `wgpu::surface`.
  - This, harder, lifetime model is not yet release, but is assumed to be the new way forward.  
    Might as well solve it now.
- [x] Using the `RecursiveReducer` macros to create a combining `Reducer`.
- [x] Driving a `Store` from an external event loop.
  - [x] Using winit’s `UserEvent` to handle graceful shutdown between the two event loops

- [ ] An example of a multi-`Store` design.

  - [x] The outer `Store` is a `blocking` `Store` to satisfy the various pieces of the three libraries that require being on the main-thread or simply or not `Send`.
  
  
  

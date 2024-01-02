# Winit

Creating a **wgpu** context in a **winit** window with **muda** (native) menus.



## Features

- [x] Managing all of the construction-order requirements between wgpu, winit and muda.
  
  - [x] Satisfying the various pieces of the three libraries that require being on the main-thread or simply not being `Send`.
- [x] Driving a `Store` from an external event loop.
  - [x] Using winit’s `UserEvent` to handle graceful shutdown between the two event loops








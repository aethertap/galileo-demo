
- [ ] DONE busy render loop

The [Galileo Map::animate (line 87)](../galileo/galileo/src/map/mod.rs)
function requests redraw until k (animation time parameter) is 1. k seems to
never reach 1, so I need to see why so I can shut off the render loop when
nothing is happening.

The call is coming from egui_map.rs:247 in `resize_map`. 

Resolution: call the `floor()` method on the `ui.available_size` result in
`EguiMap::render`. `available_size` returns a `Vec2`, which has `f32` members
that may have non-zero fractional parts, so testing equality with the app's
current size was failing forever.



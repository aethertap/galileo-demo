# Map Command

This is a GIS app that uses a command line and WASM scripting to edit, search,
and use the map data. It's initially being built as a demo of the awesome
[galileo](https://github.com/maximkaaa/galileo) rust library, but who knows,
maybe it'll become useful in its own right.

I'm modeling the command line behavior on one of my favorite ancient programs:
AutoCAD r12. You can access all of the behavior of the mapping and digitizing
systems from the command line, and it has a nice way of dealing with selections
as selection sets. Commands operate on whatever the current selection set is,
and the selection set can be made by a combination of code, filters, and user
screen interaction.

The backend is going to be either PostGIS or Spatialite, depending on which one
is easier to get started.

## Roadmap

- [x] Get a simple command line UI set up that can react to entered text
- [ ] Create a WASM-ready API with one or two functions to call from scripted commands
- [ ] Add a WASM module loader that can load and run scripts against the API
- [ ] Decide on Spatialite or PostGIS as the data backend and get it integrated
- [ ] Design a table structure that supports layers and data properties for geo
      data types and attachments



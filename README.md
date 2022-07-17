# node-move-rs
This is a small project to get experience in reference handling in Rust, as well as the Nannou graphics framework.

The application is a two dimensional grid, where each grid contains a tile that may or may not contain a node. 
A cursor object navigates the grid and is capable of picking up and putting down nodes.
There are some basic validations in place regarding move legality, such as validating request bounds and collision detection.
Most of the business logic occurs in the Grid module, due to the difficulty in directly sharing references to objects in Rust.

I originally wanted to use this as the foundation of building a game, but I'm "capping" this as an educational project to explore some other ideas I learned about during the course of this project. 

The Nannou framework is pretty cool, I'll likely try out the Bevy framework and try the Entity Component System architectural pattern next if I do something similar (i.e. having a system with lots of objects). 
ECS seems like a good pattern to solve the reference sharing issues I ran into, though it indirectly will reduce the compiler-enforced memory safety rust provides by requiring more manual data handling/validation.

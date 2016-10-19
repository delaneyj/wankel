# wankel
<img src="art/logo.svg"/>

## A rustification of the three.js codebase

### Why?
* To learn Rust with a real world port
* Three.js has a billion examples and is a complete solution for rendering OpenGL ES.
* By porting to rust long term hope to have a fast multi-threaded engine that had the same features natively and via Web Assembly
* Because of the limitations of javascript there is nothing but speed improvements to be gained.
* Since its almost solely about rendering it should fit well into the current Piston environment of crates

### Name?
* Semi pun for being **pistionless** (mostly) 
* Wankel rotors' have **three** sides... `wink`
* I owned an RX8 for years 

### Stuff to do
* Obviously early but focusing right now on the math library, new to Rust so if you see anything that's no idiomatic please let me know.
* As internal dependencies get fleshed out I'm moving the unit/integration tests into the codebase.  
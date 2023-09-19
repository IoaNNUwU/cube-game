## Cube Game in Rust Language

Minecraft-like game which aims to overtake the original one 
by adding new mechanics, bosses and RPG elements inspired by terraria.

```bash
cargo depgraph --all-deps --dedup-transitive-deps --exclude bytemuck,time,bytes,octets,chacha20poly1305,log,renetcode,bevy_ecs,heck,hex,base64,ryu,itoa,bevy_internal,chrono,serde_with_macros,indexmap,strum_macros,serde_derive | dot -Tpng -odepgraph.png
```
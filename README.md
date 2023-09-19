## Cube Game

Minecraft-like game which aims to overtake the original one by adding new mechanics, bosses and RPG elements inspired by terraria.

- Uses __Rust🦀__ & __Bevy Engine🐦__

## Cube Game is early in development!🛠️

- [X] Client/Server protocol:atom:
- [ ] Infinite server world (even height is unlimited) with __cubic chunks__ based world♾️
- [ ] Client world with simple graphics🧊
- [ ] Player Inventory & Block inventories👩‍🦲
- [ ] Instruments⛏️
- [ ] Creatures🐷
- [ ] Monsters🧟
- [ ] Procedural biomes which can change depending on blocks placed⏳
- [ ] Procedural generation🌳
- [ ] Multiple dungeons🏰

### Cool graph 🥇

![degraph](https://github.com/IoaNNUwU/cube-game/blob/main/depgraph.png)

```bash
cargo depgraph --all-deps --dedup-transitive-deps --exclude bytemuck,time,bytes,octets,chacha20poly1305,log,renetcode,bevy_ecs,heck,hex,base64,ryu,itoa,bevy_internal,chrono,serde_with_macros,indexmap,strum_macros,serde_derive | dot -Tpng -odepgraph.png
```

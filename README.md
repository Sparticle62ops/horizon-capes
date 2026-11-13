# horizon-capes (1:1 port)

Exact 1:1 Rust translation of [Capes](https://modrinth.com/mod/capes)
(CaelTheColher, LGPL-2.1) for Horizon.

Upstream client mixins (`MixinPlayerInfo.java`, `MixinCapeFeatureRenderer.java`
and friends) are carried 1:1 in `mixins/capes/runtime.rs` as a Horizon client
runtime splice (`crates/horizon-client/src/game.rs`): all eight modules
(`capes_mod`, `config`, `handler`, `menus`, `mixins`, `render`, `util`) plus
the test module, with `crate::` mapped to `super::` for the carrier. Host seams
where Minecraft/Fabric APIs do not exist in Horizon live behind the port's seam
traits in the private master (`ports/capes/src`).

Ported by sparticle.

## License

Upstream Capes is Copyright (C) CaelTheColher, LGPL-2.1-or-later. This port
carries the upstream license and credit per the port policy.

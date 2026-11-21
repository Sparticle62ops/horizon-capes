# horizon-capes (1:1 port)

Exact 1:1 Rust translation of [Capes](https://github.com/CaelTheColher/Capes) for
Horizon. Upstream is Java under the **GNU LGPL v2.1**; this port carries the
upstream license and credit.

Cape providers (OptiFine, LabyMod, MinecraftCapes, Cosmetica, Cloaks+), the cape
selector screen with the live player preview, animated-cape frame stepping, and
the elytra texture toggle.

The mod API wires the original's config as settings and runs the original's
create-or-load against `config/capes.json5` on client setup. The engine-stage
seams (skin/cape render hooks, the client command surface) are tracked in
`horizon-mod.toml` and link to their tracker issues.

Build: `cargo test` (pinned to Horizon `f00125f`, horizon-mixins `e59da909`).

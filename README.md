## Seed's official website

- `https://seed-rs.org/`

> [!WARNING]
> The link to seed-rs.org is temporally disabled because a third party may have acquired it (see https://github.com/seed-rs/seed-rs.org/issues/126)

- Based on [Seed-quickstart-webpack](https://github.com/seed-rs/seed-quickstart-webpack).

- To build this site in a local environment, follow the instructions in the above link.

---

## How to add docs for a new Seed version

1. Create a new folder `/crate/guides/x.x.x`
1. Add new guides into the folder. Guide `about.md` is required.
1. Open `/crate/src/lib.rs` and:
   1. Add `SeedVersion` variant `Vx_x_x`
   1. Update `SeedVersion::version` and `SeedVersion::date`
   1. Update `SEED_VERSIONS` and `DEFAULT_SEED_VERSION`
1. Open `/crate/src/guide.rs` and:
   1. Update `guides()`

# Kerbal Rocket Kalculator 2: Electric Boogaloo
This is a rocket-building calculator for KSP 2. It seems to work okay, but it is currently only implemented for non-radial Methalox engines and tanks.
I've added the hydrogen tanks and engines, but have not implemented them yet since those rockets low-key suck compared to methalox.

# To use:
```
cargo build
cargo run
```
Then answer the prompts. It will loop until you either type `exit` at any point, or if you `Ctrl+C`.

# To do:
-implement other fuel types?
-output the specific wet mass required for the delta-v, so that you know exactly how heavy the rocket needs to be for the desired delta-v?
-add support for radial tanks and engines?
-fix the FuelStack initialization to use a function instead of manually adding the configs so that it could:
-support more than 2 full-sized fuel tanks in the FuelStack configs

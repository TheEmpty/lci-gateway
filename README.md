## Rust LCI Gateway API

Some Rust APIs for integrating with an LCI gateway, commonly found in RVs.

**Disclaimer:** Not officaily supported. Developer(s) not associated with the LCI brand or company.

This may not support all types of devices. Check out the current supported items in `examples/` which can be ran for ex: `cargo run --example dimmers`

Currently supported:
* All: label, "online" state
* Dimmers (Lights, get state/brightness)
* Switches (on/off, get state, relay current, faults) [ex waterpump]
* Generator (on/off, get state)
* Tanks (get %)
* HVAC (get state, get/set temperatures, get/set mode, get/set temperature limits)

Things I don't need to / intend to add:
If you need one of these, feel free to add it and send a pull request.
* Dimmers (get/set sleep timers)
* RGB lights
* Dimmers (get/set mode)

## Add to your Rust project

Available via [crates.io](https://crates.io/crates/lci-gateway).
In your project you simply need to run `cargo add lci-gateway`.

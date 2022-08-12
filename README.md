## Rust LCI Gateway API

**NOTE:** Not ready for integrating with anything "prod" due to panics.

Some Rust APIs for integrating with an LCI gateway, commonly found in RVs.

**Disclaimer:** Not officaily supported. Developer(s) not associated with the LCI brand or company.

This has panics and may not support all types of devices. Check out the current supported items in `examples/` which can be ran for ex: `cargo run --example dimmers`

Currently supported:
* Dimmers (Lights, get state/brightness)
* Switches (on/off, get state) [ex waterpump]
* Generator (on/off, get state)
* Tanks (get %)
* HVAC (get/set temperatures, get/set mode, get/set temperature limits)

To cleanup:
* panics via `expect`s
* Better support around DeviceType and converting types (ex `thing.is_a(ThingType::Tank)`)
* Better types for state, enums for on/off or u8 for brightness, etc

Things I don't need to / intend to add:
If you need one of these, feel free to add it and send a pull request.
* Dimmers (get/set sleep timers)
* RGB lights
* Dimmers (get/set mode)

## Add to your Rust project

Available via [crates.io](https://crates.io/crates/lci-gateway).
In your project you simply need to run `cargo add lci-gateway`.

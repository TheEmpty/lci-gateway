## Rust LCI Gateway API

**NOTE:** Not ready for integrating with anything "prod".

This has panics and does not support all types of devices. Check out the current supported items in `examples/` which can be ran for ex: `cargo run --example dimmers`

Currently supported:
* Dimmers (Lights)
* Switches (on/off like water pump)
* Generator (on/off)

To support:
* Dimmers (get state, get/set mode)
* Switches (get state)
* Generator (get state)
* HVAC (get/set state, get temprature, get/set limits)
* Tanks (get states)
 
To cleanup:
* panics via `expects`
* Better support around DeviceType and converting types

Things I don't need to support:
* Dimmers (get/set sleep timers)
* RGB lights

## PurpleAir Telegarf Plugin

This is a simple wrapper that reads sensor data from a PurpleAir sensor and writes
the results to a Telegraf client via TCP. Sensors must be on the local network

It is setup for cross-compilation on a Rasperry Pi (ARM64).

You'll need to set TELGRAF_HOST and SENSOR_IDS as a comma-separated list of PurpleAir IDs,
e.g.

SENSOR_IDS=3a1a,41ba TELEGRAF_HOST=tcp://localhost:8064 purple-telegraf-rust


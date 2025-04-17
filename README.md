# Small Simulator with configuration

### sample config
```
[simulation]
# amount of cars in the simulation
cars = 2_000_000
# the percent of rule breaking cars (0 - 100)
rule_breakers = 20

# all cars should start at the start or at random positions
base_point = false

# the cars top speed in mph (10 mph = one road segment)
# will be rounded up to nearest whole after conversion
top_speed = 20

# debug
debug = false
```

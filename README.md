# Zero: The orchestrator app.

## Deployment scenario
Zero sits on each host. It must not be killed (or must always be available).
Zero orchestrate using ZK. Nothing starts before ZK comes up.


### Startup steps
1. Zero is up. Zero has a config file (json) that tells the config of site.
2. Waits for all apps to register.
3. Zero does nothing till ZK comes up.
4. Zero starts RW to ZK, manage the events, manages the schedule, etc
5. SBI/NBI are then distributed to make the system work
6. Another app may be required to see and restart zero if that is killed

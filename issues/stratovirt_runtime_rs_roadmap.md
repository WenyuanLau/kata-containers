# Implement StratoVirt runtime-rs hypervisor plugin

According to our last roadmap#8062 and the dicussion with Kata AC, we will make implementing StratoVirt runtime-rs plugin our top priority in the future, and the all new features will go into runtime-rs first.

The remaining tasks of StratoVirt runtime-rs plugin (`MicroVM` machine type):

- [ ] Implement the ConfigPlugin trait and default configuration file
- [ ] Implement the Hypervisor trait
    - [ ] VM lifecycle APIs (prepare_vm(), start_vm(), stop_vm())
    - [ ] Device management APIs (add_device(), remove_device())
    - [ ] Other helper functions
- [ ] Implement StratoVirt support for runtime-rs GHA CI

After `MicroVM` being implemented, we will continue to implement the `StandardVM` machine type for StratoVirt runtime-rs plugin. Remaining tasks includes: 

- [ ] Implement `StandardVM` machine type for common scenario (virtio-pci, with ACPI)
- [ ] Implement CPU & memory hotplug
- [ ] Implement vhost-user-blk
- [ ] Implement virtio-user-net
- [ ] Implement device pass-through (VFIO)
- [ ] Implement device hotplug with ACPI
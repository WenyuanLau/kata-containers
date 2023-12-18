# Issue

## mounts: Lots of mount points in k8s deployment with Kata go runtime

Recently, we notice one problem when deploying Kata Containers go runtime through k8s. When we deploy more than 400 Pods on a node, the number of mount points on the system will exceed 10,000, no matter what VMM we use. 

A large number of mount points will cause the `systemd` process to be very busy (100% CPU load), executing system calls such as `open`/`read` the mounts, due to some kind of container health check mechanism. The high load of the `systemd` process will cause the deployment of more Pods to become extremely slow, making it difficult to meet the requirements for fast and high-density deployment.

Inspired by rust runtime, we add the mount namespace isolation to go runtime, and it can prevent the `systemd` process from seeing more mounts, thereby reducing the burden on the `systemd`, making it faster to deploy more Pods.

Considering the runtime-rs VMMs are under heavy developments, adding such isolation is still useful for many user-cases.



# PR

## mounts: Reduce go runtime mounts in k8s deployment with mount namespace isolation

We notice a 100% CPU load `systemd` process when deploying more than 400 Kata Containers Pods (using go runtime) through k8s in one node, as described in issue#.

Inspired by rust runtime, we add the mount namespace isolation to go runtime, and it can prevent the `systemd` process from seeing more mounts, thereby reducing the burden on the `systemd`, making it faster to deploy more Pods.

Considering the runtime-rs VMMs are under heavy developments, adding such isolation is still useful for many user-cases.
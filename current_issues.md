
- tcp/listener.rs
    - if we bind to a localhost (maps to 127.0.0.1, by src/dns.rs), then another host won't be able to bind to localhost on the same port. 
        - we'll be able too. binds are hosts-local.
    - we can bind to IP we do not own from a different host, example error: src/hosts.rs, io::Error::new
- topology doesnt understand loopbacks?


Advantages of using Topology:
- we can reuse the partitions, holds etc. 
- simpler code?

Advantages of not using Topology:
-

Problems? 
- probably host's tcp ain't recognizing binds on different IPs.
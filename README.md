#mrt-rs

##Overview
A library to parse MRT messages in rust based on RFC 6396
(https://tools.ietf.org/html/rfc6396). Contains parsing of bgp messages
according to RFC 4271 (https://tools.ietf.org/html/rfc4271).

##Messages Parsing
####MRT Type
* [ ] OspfV2
* [ ] TableDump
* [ ] TableDumpV2
* [x] Bgp4mp
* [ ] Bgp4mpet
* [ ] Isis
* [ ] IsisEt
* [ ] OspfV3
* [ ] OspfV3Et

####MRT SubType
* [ ] Bgp4mpStateChange
* [x] Bgp4mpMessage
* [x] Bgp4mpMessageAs4
* [ ] Bgp4mpStateChangeAs4
* [ ] Bgp4mpMessageLocal
* [ ] Bgp4mpMessageAs4Local

####BGP Type
* [ ] Open
* [ ] Update
* [ ] Modification
* [ ] KeepAlive

##TODO
- parse BGP update message
- complete initial implementation

@0x9c3ce2c65111bd30;

using import "aliases.capnp".Empty;
using import "aliases.capnp".Id;
using import "aliases.capnp".Version;
using import "trigger.capnp".Trigger;

struct Request {
  version @0 :Version;
  payload @1 :Payload;
}

struct Payload {
  union {
    ping @0 :Empty;
    shutdown @1 :Empty;
    runtime @2 :Runtime;
    rotate @3 :Rotate;
  }
}

struct Runtime {
  union {
    status @0 :Empty;
    disconnect @1 :Empty;
    connect @2 :Connect;
    replace @3 :Replace;
  }
}

struct Connect {
  id @0 :Id;
}

struct Replace {
  trigger @0 :Trigger;
  candidate @1 :Candidate;
}

struct Candidate {
  union {
    any @0 :Empty;
    id @1 :Id;
  }
}

struct Rotate {
  union {
    status @0 :Empty;
    disable @1 :Empty;
    enable @2 :Empty;
  }
}

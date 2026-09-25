@0xe516f3c7c37e5020;

using import "aliases.capnp".Empty;
using import "aliases.capnp".String;
using import "aliases.capnp".Version;
using import "code.capnp".Code;

struct Option(T) {
  union {
    none @0 :Empty;
    some @1 :T;
  }
}

struct Response(T) {
  version @0 :Version;
  code @1 :Code;
  message @2 :String;
  payload @3 :Option(T);
}

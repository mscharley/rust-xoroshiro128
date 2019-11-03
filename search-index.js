var N=null,E="",T="t",U="u",searchIndex={};
var R=["option","result","to_owned","clone_into","try_from","try_into","borrow_mut","borrow","type_id","typeid","formatter","next_u32","next_u64","xoroshiro128rng","xorshift1024rng","splitmix64rng","from_seed","reseed","SeedableRng","Xoroshiro128Rng","XorShift1024Rng","SplitMix64Rng","from_seed_u64","fill_bytes","try_fill_bytes"];
searchIndex["xoroshiro128"]={"doc":"Utilities for generating psuedo-random numbers quickly.","i":[[8,R[18],"xoroshiro128","A random number generator that can be explicitly seeded.",N,N],[16,"Seed",E,"Seed type, which is restricted to types…",0,N],[10,R[16],E,"Create a new PRNG using the given seed.",0,[[],["self"]]],[11,"seed_from_u64",E,"Create a new PRNG using a `u64` seed.",0,[[["u64"]],["self"]]],[11,"from_rng",E,"Create a new PRNG seeded from another `Rng`.",0,[[["r"]],[["error"],[R[1],["error"]]]]],[8,"RngCore",E,"The core of a random number generator.",N,N],[10,R[11],E,"Return the next random `u32`.",1,[[["self"]],["u32"]]],[10,R[12],E,"Return the next random `u64`.",1,[[["self"]],["u64"]]],[10,R[23],E,"Fill `dest` with random data.",1,[[["self"]]]],[10,R[24],E,"Fill `dest` entirely with random data.",1,[[["self"]],[["error"],[R[1],["error"]]]]],[3,R[19],E,E,N,N],[3,R[20],E,E,N,N],[3,R[21],E,E,N,N],[11,"new_unseeded",E,"Creates a new `Xoroshiro128Rng` instance which is not…",2,[[],["self"]]],[11,R[22],E,E,2,[[],["self"]]],[11,R[22],E,E,3,[[],["self"]]],[11,R[22],E,E,4,[[["u64"]],["self"]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[2],E,E,2,[[["self"]],[T]]],[11,R[3],E,E,2,[[["self"],[T]]]],[11,"into",E,E,2,[[],[U]]],[11,R[4],E,E,2,[[[U]],[R[1]]]],[11,R[5],E,E,2,[[],[R[1]]]],[11,R[6],E,E,2,[[["self"]],[T]]],[11,R[7],E,E,2,[[["self"]],[T]]],[11,R[8],E,E,2,[[["self"]],[R[9]]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[2],E,E,3,[[["self"]],[T]]],[11,R[3],E,E,3,[[["self"],[T]]]],[11,"into",E,E,3,[[],[U]]],[11,R[4],E,E,3,[[[U]],[R[1]]]],[11,R[5],E,E,3,[[],[R[1]]]],[11,R[6],E,E,3,[[["self"]],[T]]],[11,R[7],E,E,3,[[["self"]],[T]]],[11,R[8],E,E,3,[[["self"]],[R[9]]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[2],E,E,4,[[["self"]],[T]]],[11,R[3],E,E,4,[[["self"],[T]]]],[11,"into",E,E,4,[[],[U]]],[11,R[4],E,E,4,[[[U]],[R[1]]]],[11,R[5],E,E,4,[[],[R[1]]]],[11,R[6],E,E,4,[[["self"]],[T]]],[11,R[7],E,E,4,[[["self"]],[T]]],[11,R[8],E,E,4,[[["self"]],[R[9]]]],[11,"clone",E,E,2,[[["self"]],[R[13]]]],[11,"clone",E,E,3,[[["self"]],[R[14]]]],[11,"clone",E,E,4,[[["self"]],[R[15]]]],[11,"fmt",E,E,2,[[["self"],[R[10]]],[R[1]]]],[11,"fmt",E,E,3,[[["self"],[R[10]]],[R[1]]]],[11,"fmt",E,E,4,[[["self"],[R[10]]],[R[1]]]],[11,R[16],E,E,2,[[],["self"]]],[11,R[16],E,E,3,[[],["self"]]],[11,R[16],E,E,4,[[],["self"]]],[11,R[11],E,E,2,[[["self"]],["u32"]]],[11,R[12],E,E,2,[[["self"]],["u64"]]],[11,R[23],E,E,2,[[["self"]]]],[11,R[24],E,E,2,[[["self"]],[["error"],[R[1],["error"]]]]],[11,R[11],E,E,3,[[["self"]],["u32"]]],[11,R[12],E,E,3,[[["self"]],["u64"]]],[11,R[23],E,E,3,[[["self"]]]],[11,R[24],E,E,3,[[["self"]],[["error"],[R[1],["error"]]]]],[11,R[11],E,E,4,[[["self"]],["u32"]]],[11,R[12],E,E,4,[[["self"]],["u64"]]],[11,R[23],E,E,4,[[["self"]]]],[11,R[24],E,E,4,[[["self"]],[["error"],[R[1],["error"]]]]]],"p":[[8,R[18]],[8,"RngCore"],[3,R[19]],[3,R[20]],[3,R[21]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);
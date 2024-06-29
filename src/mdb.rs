use sel4_common::plus_define_bitfield;


/// Generate from two words, implement a biddirectional link list used to record cap's derivative relationship.
/// 
/// Two anthor bits:
/// 
/// revocable: If set, the cap can be removed without informing the cap's maintainer.
/// 
/// firstbadged: The first notification or endpoint cap with badge not equal zero.
plus_define_bitfield! {
    mdb_node_t, 2, 0, 0, 0 => {
        new, 0 => {
            mdbNext, get_next, set_next, 1, 2, 37, 2, true,
            mdbRevocable, get_revocable, set_revocable, 1, 1, 1, 0, false,
            mdbFirstBadged, get_first_badged, set_first_badged, 1, 0, 0, 0, false,
            mdbPrev, get_prev, set_prev, 0, 0, 64, 0, false
        }
    }
}

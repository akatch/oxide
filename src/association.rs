
#[derive(Default)]
#[derive(Debug)]
struct Association {
    // message ID must be association-unique
    // 2 bytes (US VR)
    current_message_id: u32,
}

impl Association {
    // FIXME i feel like we shouldn't need the entire Association to be mutable
    fn generate_message_id(&mut self) -> u32 {
        // cannot get message_id without incrementing to ensure Association-uniqueness
        let old_message_id = self.current_message_id;
        self.current_message_id += 1;
        old_message_id
    }
}

#[test]
fn message_ids_are_unique() {
    let mut a = Association::default();
    let id = a.generate_message_id();
    let id2 = a.generate_message_id();
    assert_ne!(id, id2);
}

#[test]
fn verify_connectivity() {
    // send c-echo-rq
    // to self
    assert!(false)
}

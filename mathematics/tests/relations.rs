use mathematics::relations::{
    first_collision, invert_assignment, remaining_jobs, share_input, two_steps,
};

#[test]
fn membership_operations_respect_the_universe() {
    assert_eq!(remaining_jobs(0b0010_1100, 0b0110_0100), 0b0000_1000);
    assert_eq!(remaining_jobs(0, 8), 0);
    assert_eq!(remaining_jobs(0b1000_0000, 255), 0);
    assert!(share_input(&[1], &[1, 2]));
    assert!(share_input(&[1, 1, 2], &[2]));
    assert!(!share_input(&[1], &[2]));
}

#[test]
fn graph_query_checks_direction_length_and_domain() {
    let edges = [[false, true, false], [false, false, true], [false; 3]];
    assert_eq!(two_steps(&edges, 0, 2), Some(true));
    assert_eq!(two_steps(&edges, 0, 1), Some(false));
    assert_eq!(two_steps(&edges, 2, 0), Some(false));
    assert_eq!(two_steps(&edges, 3, 2), None);
    assert_eq!(two_steps(&edges, 0, 3), None);
    assert_eq!(two_steps(&[[true]], 0, 0), Some(true));
    assert_eq!(two_steps::<0>(&[], 0, 0), None);
}

#[test]
fn slot_mapping_detects_the_first_collision_and_checks_inversion() {
    assert_eq!(first_collision(&[]), None);
    assert_eq!(first_collision(&[7, 2, 3]), None);
    assert_eq!(first_collision(&[7, 2, 2, 7]), Some((0, 3)));
    assert_eq!(invert_assignment([2, 0, 1]), Some([1, 2, 0]));
    assert_eq!(invert_assignment([2, 0, 0]), None);
    assert_eq!(invert_assignment([2, 0, 3]), None);
}

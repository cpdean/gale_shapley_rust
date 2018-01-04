#![feature(vec_remove_item)]

#[allow(dead_code)]
mod optimal_pairing {
    use std::cmp::Ordering;
    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct Student {
        pub id: i32,
        pub location: i32,
    }
    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct Hospital {
        pub id: i32,
        pub location: i32,
    }

    // TODO: stubbing out rewrite of best_pairings
    // in a new fn because i was running into tricky syntax errors
    // by putting it in a new fn i can remove large sections to more easily
    // experiment with what i've written wrong.
    pub fn syntax(
        students: Vec<Student>,
        mut hospitals: Vec<Hospital>,
    ) -> Vec<(Student, Option<Hospital>)> {
        let mut o = Vec::new();
        for s in students {
            // find best
            if let Some(best) = hospitals.iter().min_by(
                |a: &Hospital, b: &Hospital| -> Option<Ordering> {
                    let a_distance = (s.location - a.location).abs();
                    let b_distance = (s.location - b.location).abs();
                    a_distance.partial_cmp(&b_distance)
                },
            ) {
                // when assigning this hospital, remove it from the vec of available assignable
                // hospitals
                if let Some(best_hospital) = hospitals.remove_item(best) {
                    o.push((s, Some(best_hospital)));
                } else {
                    o.push((s, None));
                }
            } else {
                // the ranked hospitals list was empty, so no more
                // pairings
                o.push((s, None));
            }
        }
        o
    }

    pub fn best_pairings(
        students: Vec<Student>,
        mut hospitals: Vec<Hospital>,
    ) -> Vec<(Student, Option<Hospital>)> {
        let mut o = Vec::new();
        for s in students {
            // find best
            if let Some(any_hospital) = hospitals.pop() {
                o.push((s, Some(any_hospital)));
            } else {
                o.push((s, None));
            }
        }
        o
    }

    pub fn score(pairings: Vec<(Student, Option<Hospital>)>) -> i32 {
        let mut total = 0;
        for (s, h) in pairings {
            if let Some(hospital) = h {
                let diff = s.location - hospital.location;
                total += diff.abs();
            }
        }
        total
    }

}

#[cfg(test)]
mod tests {
    use optimal_pairing::{best_pairings, score, Hospital, Student};

    #[test]
    fn twenty_eight_days_later() {
        // your degree is worthless to zombies
        let students = vec![
            Student { id: 1, location: 1 },
            Student { id: 2, location: 2 },
            Student { id: 3, location: 3 },
        ];
        let hospitals = vec![];
        let pairings = best_pairings(students, hospitals);
        let expected_outcome = vec![
            (Student { id: 1, location: 1 }, None),
            (Student { id: 2, location: 2 }, None),
            (Student { id: 3, location: 3 }, None),
        ];
        assert_eq!(expected_outcome, pairings);
    }

    #[test]
    fn parents_basement() {
        // "I'd invite you over, but my roommates are home"
        let students = vec![
            Student { id: 1, location: 1 },
            Student { id: 2, location: 2 },
            Student { id: 3, location: 3 },
        ];
        let hospitals = vec![
            Hospital { id: 3, location: 3 },
            Hospital { id: 2, location: 2 },
            Hospital { id: 1, location: 1 },
        ];
        let pairings = best_pairings(students, hospitals);
        let expected_outcome = vec![
            (
                Student { id: 1, location: 1 },
                Some(Hospital { id: 1, location: 1 }),
            ),
            (
                Student { id: 2, location: 2 },
                Some(Hospital { id: 2, location: 2 }),
            ),
            (
                Student { id: 3, location: 3 },
                Some(Hospital { id: 3, location: 3 }),
            ),
        ];
        assert_eq!(expected_outcome, pairings);
    }

    #[test]
    fn perfect_pairs_scored() {
        let students = vec![
            Student { id: 1, location: 1 },
            Student { id: 2, location: 2 },
            Student { id: 3, location: 3 },
        ];
        let hospitals = vec![
            Hospital { id: 3, location: 3 },
            Hospital { id: 2, location: 2 },
            Hospital { id: 1, location: 1 },
        ];
        let pairing_score = score(best_pairings(students, hospitals));
        assert_eq!(0, pairing_score);
    }

    #[test]
    fn slight_imperfect_pairs_scored() {
        let students = vec![
            Student { id: 1, location: 1 },
            Student {
                id: 2,
                location: 20,
            },
            Student {
                id: 3,
                location: 30,
            },
        ];
        let hospitals = vec![
            Hospital {
                id: 3,
                location: 31,
            },
            Hospital {
                id: 2,
                location: 21,
            },
            Hospital { id: 1, location: 2 },
        ];
        let pairing_score = score(best_pairings(students, hospitals));
        assert_eq!(3, pairing_score);
    }

    #[test]
    fn parents_basement_reversed() {
        // the naiive impl of pairing things was order-dependent. order shouldn't really matter if
        // there's an objective, optimal pairing
        let students = vec![
            Student { id: 1, location: 1 },
            Student { id: 2, location: 2 },
            Student { id: 3, location: 3 },
        ];
        let hospitals = vec![
            Hospital { id: 1, location: 1 },
            Hospital { id: 2, location: 2 },
            Hospital { id: 3, location: 3 },
        ];
        let pairings = best_pairings(students, hospitals);
        let expected_outcome = vec![
            (
                Student { id: 1, location: 1 },
                Some(Hospital { id: 1, location: 1 }),
            ),
            (
                Student { id: 2, location: 2 },
                Some(Hospital { id: 2, location: 2 }),
            ),
            (
                Student { id: 3, location: 3 },
                Some(Hospital { id: 3, location: 3 }),
            ),
        ];
        assert_eq!(expected_outcome, pairings);
    }

    #[test]
    fn too_many_students() {
        // unemployment :(
        let students = vec![
            Student { id: 1, location: 1 },
            Student {
                id: 2,
                location: 30,
            },
        ];
        let hospitals = vec![Hospital { id: 1, location: 1 }];
        let pairings = best_pairings(students, hospitals);
        let expected_outcome = vec![
            (
                Student { id: 1, location: 1 },
                Some(Hospital { id: 1, location: 1 }),
            ),
            (
                Student {
                    id: 2,
                    location: 30,
                },
                None,
            ),
        ];
        assert_eq!(expected_outcome, pairings);
    }

    #[test]
    fn test_eq() {
        // TIL #[derive(Debug, PartialEq, PartialOrd)]
        let a = vec![Student { id: 1, location: 1 }];
        let b = vec![Student { id: 1, location: 1 }];
        assert_eq!(a, b);
    }
}

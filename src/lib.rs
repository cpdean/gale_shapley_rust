#[allow(dead_code)]
mod optimal_pairing {
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

    pub fn best_pairings(
        students: Vec<Student>,
        mut hospitals: Vec<Hospital>,
    ) -> Vec<(Student, Option<Hospital>)> {
        let mut o = Vec::new();
        for s in students {
            if let Some(any_hospital) = hospitals.pop() {
                o.push((s, Some(any_hospital)));
            }
            else {
                o.push((s, None));
            }
        }
        o
    }

}

#[cfg(test)]
mod tests {
    use optimal_pairing::{best_pairings, Hospital, Student};

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
            (Student { id: 1, location: 1 }, Some(Hospital { id: 1, location: 1 })),
            (Student { id: 2, location: 2 }, Some(Hospital { id: 2, location: 2 })),
            (Student { id: 3, location: 3 }, Some(Hospital { id: 3, location: 3 })),
        ];
        assert_eq!(expected_outcome, pairings);
    }


    #[test]
    fn one_perfect_pair() {
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
        let a = vec![Student { id: 1, location: 1 }];
        let b = vec![Student { id: 1, location: 1 }];
        assert_eq!(a, b);
    }
}

use std::fmt::Display;

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T> ReportCard<T> {
    // T 泛型默认不支持`format!()` 进行格式化，可通过 `Display` 以支持.
    pub fn print(&self) -> String where T: Display {
        format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };

        assert_eq!(report_card.print(),
                   "Tom Wriggle (12) - achieved a grade of 2.1");
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };

        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
            );
    }
}
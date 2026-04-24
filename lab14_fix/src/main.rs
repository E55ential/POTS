pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: u32,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        // 1. обновляем счетчик
        self.visit_count += 1;

        // 2. считаем изменение роста
        let height_change = measurements.height - self.height;

        // 3. считаем изменение давления
        let blood_pressure_change = match self.last_blood_pressure {
            None => None,
            Some((old_sys, old_dia)) => {
                Some((
                    measurements.blood_pressure.0 as i32 - old_sys as i32,
                    measurements.blood_pressure.1 as i32 - old_dia as i32,
                ))
            }
        };

        // 4. обновляем данные пользователя
        self.height = measurements.height;
        self.last_blood_pressure = Some(measurements.blood_pressure);

        // 5. возвращаем отчет
        HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count,
            height_change,
            blood_pressure_change,
        }
    }
}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("Меня зовут {} и мой возраст {}", bob.name, bob.age);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Гиппократ"), 32, 155.2);

    assert_eq!(bob.visit_count, 0);

    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (120, 80) });

    assert_eq!(report.patient_name, "Гиппократ");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);
    assert!((report.height_change - 0.9).abs() < 0.00001);

    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (115, 76) });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    assert_eq!(report.height_change, 0.0);
}
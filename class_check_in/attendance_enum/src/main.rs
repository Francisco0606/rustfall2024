#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {
        //TODO! (implement printing info about Student
        // use match statement)
        //Enum degree
        let grade = match self.grade {
            GradeLevel::Bachelor => "Bachelor",
            GradeLevel::Master => "Master",
            GradeLevel::PhD => "PhD",
        };
        //Enum major
        let major = match self.major {
            Major::ComputerScience => "Computer Science",
            Major::ElectricalEngineering => "Electrical Engineering",
        };
        //Display Info
        println!("Hello, my name is {}. I'm pursuing a {} degree in {}.", self.name, grade, major);
    }
}

fn main() {
    let s1 = Student::new("John".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
    let s2 = Student::new("Frank".to_string(),
    GradeLevel::Bachelor,
    Major::ElectricalEngineering);
    s1.introduce_yourself();
    s2.introduce_yourself();
}

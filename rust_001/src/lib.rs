#[derive(Debug)]

pub struct Student {
    id: u8,
    pub name: String,
    pub age: u8,
}

impl Student {
    // pub fn new(std_name: String, std_age: u8) -> Self {
    //     Self {
    //         id: 0,
    //         name: std_name,
    //         age: std_age
    //     }
    // }

    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|x| matches!(x, 'a'..='z')) {
            Ok(Self {
                id: 0,
                name: std_name,
                age: 17,
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}

impl Default for Student {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Unknown".to_string(),
            age: 17
        }
    }
}
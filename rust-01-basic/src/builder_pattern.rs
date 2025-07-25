// -------------------------------------------
// 			Builder Pattern
// -------------------------------------------

#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: Membershiptype,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Clone)]
enum Membershiptype {
    new,
    causual,
    loyal,
}

impl Default for Membershiptype {
    fn default() -> Self {
        Membershiptype::new
    }
}

impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name: name,
            ..Default::default() // username: None,
            // membership: None,
            // gender: None,
            // country: None,
            // age: None,
        }
    }
    // fn new(name: String) -> Self {
    //     Customer {
    //         name: name,
    //         ..Default::default()
    //     }
    // }

    // fn new_2(name: String, username: String) -> Self {
    //     Customer {
    //         name: name,
    //         username: username,
    //         ..Default::default()
    //     }
    // }

    // fn new_3(name: String, username: String, membership: Membershiptype) -> Self {
    //     Customer {
    //         name: name,
    //         username: username,
    //         membership: membership,
    //         ..Default::default()
    //     }
    // }
}

#[derive(Default)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<Membershiptype>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: Membershiptype) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }
    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }
    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }
    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
        }
    }
}


#[test]
fn test_1() {
    // let new_user = Customer::new("Nouman".to_string());
    // let user_with_login = Customer::new_2("Joseph".to_string(), "joe123".to_string());
    // let user_with_membership = Customer::new_3(
    //     "Micheal".to_string(),
    //     "micheal2000".to_string(),
    //     Membershiptype::loyal,
    // );

    let new_user = Customer::new("Nouman".to_string()).build();
    let user_with_login = Customer::new("Joseph".to_string())
        .username("joe123".to_string())
        .build();

    let user_with_membership = Customer::new("Micheal".to_string())
        .username("micheal2000".to_string())
        .membership(Membershiptype::loyal)
        .build();
}

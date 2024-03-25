use std::collections::HashMap;

use crate::colors::{
    bi::{self, PURPLE},
    trans, RGB,
};

#[derive(Debug)]
pub struct QueerFlag {
    pub layers: Vec<RGB>,
}

pub struct FlagRepository {
    all_flags: HashMap<&'static str, QueerFlag>,
}

impl FlagRepository {
    pub fn new() -> Self {
        let mut all_flags: HashMap<&'static str, QueerFlag> = HashMap::new();

        let trans_flag = QueerFlag {
            layers: vec![
                trans::LIGHT_BLUE,
                trans::PINK,
                trans::WHITE,
                trans::PINK,
                trans::LIGHT_BLUE,
            ],
        };

        let lgbt_flag = QueerFlag {
            layers: vec![
                RGB(228, 3, 3),
                RGB(255, 140, 0),
                RGB(255, 237, 0),
                RGB(0, 128, 38),
                RGB(36, 64, 142),
                RGB(115, 41, 130),
            ],
        };

        let ace_flag = QueerFlag {
            layers: vec![
                RGB(0, 0, 0),
                RGB(163, 163, 163),
                RGB(255, 255, 255),
                RGB(128, 0, 128),
            ],
        };

        let nb_flag = QueerFlag {
            layers: vec![
                RGB(252, 244, 52),
                RGB(255, 255, 255),
                RGB(156, 89, 209),
                RGB(44, 44, 44),
            ],
        };

        let lesbian_flag = QueerFlag {
            layers: vec![
                RGB(213, 45, 0),
                RGB(239, 118, 39),
                RGB(255, 154, 86),
                RGB(255, 255, 255),
                RGB(209, 98, 164),
                RGB(181, 86, 144),
                RGB(163, 2, 98),
            ],
        };

        let bi_flag = QueerFlag {
            layers: vec![bi::PINK, bi::PINK, bi::PURPLE, bi::BLUE, bi::BLUE],
        };

        let pan_flag = QueerFlag {
            layers: vec![RGB(255, 33, 140), RGB(255, 216, 0), RGB(33, 177, 255)],
        };

        let gender_fluid_flag = QueerFlag {
            layers: vec![
                RGB(255, 118, 164),
                RGB(255, 255, 255),
                RGB(192, 17, 215),
                RGB(0, 0, 0),
                RGB(47, 60, 190),
            ],
        };

        let gender_queer_flag = QueerFlag {
            layers: vec![RGB(181, 126, 220), RGB(255, 255, 255), RGB(74, 129, 35)],
        };

        all_flags.insert("transgender", trans_flag);
        all_flags.insert("lgbt", lgbt_flag);
        all_flags.insert("assexual", ace_flag);
        all_flags.insert("nonbinary", nb_flag);
        all_flags.insert("lesbian", lesbian_flag);
        all_flags.insert("bisexual", bi_flag);
        all_flags.insert("pansexual", pan_flag);
        all_flags.insert("genderfluid", gender_fluid_flag);
        all_flags.insert("genderqueer", gender_queer_flag);

        FlagRepository { all_flags }
    }

    pub fn get_flag(&self, name: &str) -> Option<&QueerFlag> {
        self.all_flags.get(name)
    }
}

#[cfg(test)]
mod test {
    use super::FlagRepository;

    #[test]
    fn test_get_available_flag() {
        let repository = FlagRepository::new();
        let flag = repository.get_flag("lgbt");

        assert!(flag.is_some())
    }

    #[test]
    fn test_get_non_available_flag() {
        let repository = FlagRepository::new();
        let flag = repository.get_flag("uwu");

        assert!(flag.is_none())
    }
}

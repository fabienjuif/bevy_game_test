use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct Teams {
    pub map: HashMap<String, Team>,
}

impl Teams {
    pub fn new() -> Self {
        let mut teams = Teams {
            map: HashMap::new(),
        };

        teams.add(Team {
            id: "a".to_string(),
            color: Color::rgb(0.3, 0.3, 0.8),
        });
        teams.add(Team {
            id: "b".to_string(),
            color: Color::rgb(0.8, 0.3, 0.3),
        });
        teams.add(Team {
            id: "c".to_string(),
            color: Color::rgb(0.8, 0.8, 0.3),
        });
        teams
    }

    pub fn get(&self, id: String) -> Option<&Team> {
        self.map.get(&id)
    }

    pub fn get_expect(&self, id: String) -> Team {
        self.get(id).expect("no team found").clone()
    }

    pub fn add(&mut self, team: Team) -> &mut Self {
        self.map.insert(team.id.clone(), team);
        self
    }
}

#[derive(Component, Clone)]
pub struct Team {
    pub id: String,
    pub color: Color,
}

pub struct TeamsPlugin;

impl Plugin for TeamsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Teams::new());
    }
}

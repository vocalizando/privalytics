use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Scope {
    /// This doesn't grant the user any access to any resource
    None = 0,
    /// This grants the user read access to all _non-sensitive_ resources
    /// (a.k.a. analytics information)
    Read = 1,
    /// This grants the user write access to all _non-sensitive_ resources.
    /// This allows the user to delete analytics' information.
    ///
    /// Read scope is required
    Write = 2,
    /// This grants the user read access to all _sensitive_ resources.
    /// This includes things like:
    /// - API Endpoints
    /// - All settings
    /// - General information of the server environment
    /// - etc.
    ///
    /// Read scope is required
    Auditor = 3,
    /// This grants the user write access to all _sensitive_ resources.
    /// This includes things like:
    /// - API Endpoints
    /// - All settings
    /// - General information of the server environment
    /// - etc.
    ///
    /// Read, write and auditor scope is required
    Manager = 4,
    /// This grants the user full access of the server where the instance is hosted.
    /// This includes things like:
    /// - Running commands
    /// - Stopping the instance and the server
    /// - Accessing server data
    /// - etc.
    ///
    /// Read, write, auditor and manager scope is required
    Admin = 5,
}

impl Scope {
    pub fn is_valid(&self, scopes: &[Scope]) -> bool {
        let mut should_include: Vec<Scope> = vec![*self];

        // Ik, ik, this is spaghetti code 🍝 but I'm lazy, I'll refactor this tomorrow
        match self {
            Scope::Write => should_include.push(Scope::Read),
            Scope::Auditor => {
                should_include.push(Scope::Read);
                should_include.push(Scope::Write);
            }
            Scope::Manager => {
                should_include.push(Scope::Read);
                should_include.push(Scope::Write);
                should_include.push(Scope::Auditor);
            }
            Scope::Admin => {
                should_include.push(Scope::Read);
                should_include.push(Scope::Write);
                should_include.push(Scope::Auditor);
                should_include.push(Scope::Manager);
            }
            _ => {}
        };

        let filtered_scopes: Vec<&Scope> = should_include
            .iter()
            .filter(|item| !scopes.contains(item))
            .collect();

        if filtered_scopes.iter().len() > 0 {
            return false;
        }

        true
    }
}

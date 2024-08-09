use crate::{state::State, text_fields::TextFields, EframeApp};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Model {
    pub state: State,
    previous_state: State,

    text_field: TextFields,
}

impl Model {
    pub fn update_previous_state(&mut self) {
        self.previous_state = self.state.clone();
    }

    pub fn save(&self, app: &EframeApp) -> Result<(), String> {
        let State::PendingSave { ref path_buffer } = self.state else {
            panic!("Can only save when State is PendingSave");
        };

        std::fs::write(path_buffer, serde_json::to_string(app).unwrap())
            .map_err(|e| format!("File save failed:\n{}", e))
    }

    pub fn load(&self, app: &mut EframeApp) -> Result<(), String> {
        let State::PendingLoad { ref path_buffer } = self.state else {
            panic!("Can only save when State is PendingLoad");
        };

        match std::fs::read(path_buffer)
            .map(|file| serde_json::from_slice::<EframeApp>(file.as_slice()))
        {
            Ok(Ok(saved_app)) => *app = saved_app,
            Ok(Err(e)) => return Err(format!("File load failed:\n{}", e)),
            Err(e) => return Err(format!("File load failed:\n{}", e)),
        }

        Ok(())
    }
}

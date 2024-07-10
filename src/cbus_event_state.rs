use serde::Serialize;

use crate::fcu_module_data::MergModuleDataSet;

/// State of CBus event
#[derive(Clone, Serialize, Debug, PartialEq)]
#[allow(dead_code)]
pub enum State {
    UNKN,
    // acoff
    ZERO,
    // acon
    ONE,
}

/// Definition of the state of a CBus event
#[derive(Clone, Serialize, Debug)]
pub struct CbusState {
    /// Item name
    pub name: String,
    /// Event number - either long or short format
    pub event: String,
    /// Current state of the event
    pub state: State,
}

#[derive(Clone, Serialize, Debug)]
pub struct CBusInterface {
    /// List of 'userEvents' from FCU configuration file
    pub cbusstates: Vec<CbusState>,
}

impl CBusInterface {
    pub fn new(fcu_config: MergModuleDataSet) -> CBusInterface {
        // Create cbusstate
        let mut cbusstates: Vec<CbusState> = Vec::new();
        for event in fcu_config.userEvents {
            if event.ownerNode == event.eventNode {
                let name = event.eventName;
                let event = "N".to_owned()
                    + &event.eventNode.to_string()
                    + "E"
                    + &event.eventValue.to_string();
                let state = State::UNKN;
                let cbusstate = CbusState { name, event, state };
                cbusstates.push(cbusstate);
            }
        }
        CBusInterface { cbusstates }
    }

    pub fn pretty_print(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

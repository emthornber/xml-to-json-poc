use serde::{Deserialize, Serialize};
use serde_xml_rs::from_reader;
use std::{fs::File, io::BufReader, path::Path, string::String};

/// FCU Configuration File 'schema'

/// Module Definition
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[allow(non_snake_case)]
struct MergModule {
    moduleId: u16,
    moduleName: String,
    moduleType: u16,
    moduleEvents: u16,
    moduleValues: u16,
    numNvs: u16,
}

/// CBus Event Definition
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct UserEvent {
    eventId: u16,
    pub ownerNode: u16,
    nodeName: String,
    pub eventName: String,
    Values: String,
    pub eventNode: u16,
    pub eventValue: u16,
}

/// CBus Node Definition
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[allow(non_snake_case)]
struct UserNode {
    moduleId: u16,
    moduleName: String,
    nodeNum: u16,
    nodeName: String,
    numEvents: u16,
    inUse: bool,
    Flim: bool,
    NodeVars: String,
    maxEvents: u16,
    Version: String,
    CanId: u16,
    maxNVs: u16,
    ProcId: String,
}

/// Overall structure
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct MergModuleDataSet {
    mergModules: Vec<MergModule>,
    pub userEvents: Vec<UserEvent>,
    userNodes: Vec<UserNode>,
}

#[allow(non_snake_case)]
impl MergModuleDataSet {
    pub fn populate<P: AsRef<Path> + std::fmt::Display>(
        module_dataset_file: P,
    ) -> Result<MergModuleDataSet, &'static str> {
        let module_data = Self::read_fcu_file(module_dataset_file);
        match module_data {
            Ok(md) => {
                return Ok(md);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    /// Read the contents of a file as XML and return an instance of 'MergModuleDataSet'
    fn read_fcu_file<P: AsRef<Path> + std::fmt::Display>(
        xml_file: P,
    ) -> Result<MergModuleDataSet, &'static str> {
        // Open the file in read-only mode with buffer
        if let Ok(file) = File::open(xml_file.as_ref()) {
            // Setup reader
            let reader = BufReader::new(file);
            // Deserialize XML data and place in module data structures
            let md: Result<MergModuleDataSet, serde_xml_rs::Error> = from_reader(reader);
            match md {
                Ok(module_data) => return Ok(module_data),
                Err(_) => return Err("error during serialization"),
            }
        } else {
            Err("error opening file")
        }
    }
}

#[cfg(test)]
mod test_fcu_data {
    use super::*;
    use env_logger::Target;
    use log::{error, LevelFilter};
    use std::{fs::File, io::Write, path::Path};

    // Extract from TheDemoPlankII.xml
    const BAD_DATA: &str = r#"<?xml version="1.0" standalone="yes"?>
        <MergModuleDataSet>
            <mergModules moduleId="1">
                <moduleName>CANACC4</moduleName>
                <moduleEvents>32</moduleEvents>
                <moduleValues>2</moduleValues>
                <numNvs>16</numNvs>
            </mergModules>
            <userEvents>
                <eventId>1</eventId>
                <ownerNode>2011</ownerNode>
                <nodeName>CANVOUT</nodeName>
                <eventName>TO UP Reverse</eventName>
                <Values />
                <eventNode>2011</eventNode>
                <eventValue>13</eventValue>
                </userEvents>
            <userNodes>
                <moduleId>46</moduleId>
                <moduleName>CANPiWi</moduleName>
                <nodeNum>257</nodeNum>
                <nodeName>ThePlankMkII</nodeName>
                <numEvents>1</numEvents>
                <inUse>true</inUse>
                <Flim>true</Flim>
                <NodeVars>7BB315AE150663616E70697769000000000000000000546865506C616E6B7475726E6F75742E747874200000000100</NodeVars>
                <maxEvents>0</maxEvents>
                <Version>6</Version>
                <CanId>100</CanId>
                <maxNVs>47</maxNVs>
                <ProcId>Raspberry Pi</ProcId>
            </userNodes>
        </MergModuleDataSet>"#;

    const GOOD_DATA: &str = r#"<?xml version="1.0" standalone="yes"?>
        <MergModuleDataSet>
            <mergModules moduleId="1">
                <moduleName>CANACC4</moduleName>
                <moduleType>1</moduleType>
                <moduleEvents>32</moduleEvents>
                <moduleValues>2</moduleValues>
                <numNvs>16</numNvs>
            </mergModules>
            <userEvents>
                <eventId>1</eventId>
                <ownerNode>2011</ownerNode>
                <nodeName>CANVOUT</nodeName>
                <eventName>TO UP Reverse</eventName>
                <Values />
                <eventNode>2011</eventNode>
                <eventValue>13</eventValue>
            </userEvents>
            <userNodes>
                <moduleId>46</moduleId>
                <moduleName>CANPiWi</moduleName>
                <nodeNum>257</nodeNum>
                <nodeName>ThePlankMkII</nodeName>
                <numEvents>1</numEvents>
                <inUse>true</inUse>
                <Flim>true</Flim>
                <NodeVars>7BB315AE150663616E70697769000000000000000000546865506C616E6B7475726E6F75742E747874200000000100</NodeVars>
                <maxEvents>0</maxEvents>
                <Version>6</Version>
                <CanId>100</CanId>
                <maxNVs>47</maxNVs>
                <ProcId>Raspberry Pi</ProcId>
            </userNodes>
        </MergModuleDataSet>"#;

    fn init_logging() {
        let _ = env_logger::builder()
            .target(Target::Stdout)
            .filter_level(LevelFilter::max())
            .is_test(true)
            .try_init();
    }

    fn setup_file<P: AsRef<Path> + std::fmt::Display>(test_file: P, data: &str) {
        if let Ok(mut f) = File::create(&test_file) {
            if let Err(e) = f.write_all(data.as_bytes()) {
                error!("{}: file {} write failed", e, test_file);
            }
        } else {
            error!("file {} creation failed", test_file);
        }
    }

    fn teardown_file<P: AsRef<Path> + std::fmt::Display>(test_file: P) {
        if let Err(e) = std::fs::remove_file(&test_file) {
            error!("{}: file {} deletion failed", e, test_file);
        }
    }

    #[test]
    fn read_fcu_file_missing() {
        // Initialise Logger

        init_logging();
        let xml_file = "static/nonexistent_file.json";
        let md = MergModuleDataSet::read_fcu_file(xml_file);
        match md {
            Ok(_) => assert!(false, "'Ok' returned)"),
            Err(_) => assert!(true, "'Err' returned"),
        }
    }

    #[test]
    fn read_fcu_file_not_valid() {
        // Initialise Logger
        init_logging();

        let xml_file = "static/bad_fcu_data.xml";
        setup_file(&xml_file, BAD_DATA);
        let bad_result = MergModuleDataSet::read_fcu_file(&xml_file);
        teardown_file(&xml_file);
        match bad_result {
            Ok(_) => assert!(false, "'Ok' returned"),
            Err(_) => {
                assert!(true, "'Err' returned");
            }
        }
    }

    #[test]
    fn read_fcu_file_successfully() {
        // Initialise Logger
        init_logging();

        let xml_file = "static/good_fcu_data.xml";
        setup_file(&xml_file, GOOD_DATA);
        let good_result = MergModuleDataSet::read_fcu_file(&xml_file);
        teardown_file(&xml_file);
        match good_result {
            Ok(_) => assert!(true, "'Ok' returned"),
            Err(e) => {
                assert!(false, "'Err' returned - {}", e);
            }
        }
    }
}

/// CBus Event 'schema'

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

use log::{error, warn};
use serde::Deserialize;
use serde_xml_rs::from_reader;
use std::{fs::File, io::BufReader, path::Path, string::String};

/// FCU Configuration File 'schema'
///

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
    pub fn new<P: AsRef<Path> + std::fmt::Display>(module_dataset_file: P) -> MergModuleDataSet {
        let module_data = Self::read_fcu_file(module_dataset_file);
        match module_data {
            Some(md) => {
                return md;
            }
            None => {
                // Return empty structure
                let mergModules: Vec<MergModule> = Vec::new();
                let userEvents: Vec<UserEvent> = Vec::new();
                let userNodes: Vec<UserNode> = Vec::new();
                let md = MergModuleDataSet {
                    mergModules,
                    userEvents,
                    userNodes,
                };
                warn!("problem reading module data - vectors are empty");
                return md;
            }
        }
    }

    /// Read the contents of a file as XML and return an instance of 'MergModuleDataSet'
    fn read_fcu_file<P: AsRef<Path> + std::fmt::Display>(xml_file: P) -> Option<MergModuleDataSet> {
        // Open the file in read-only mode with buffer
        let f = File::open(xml_file.as_ref());
        match f {
            Ok(file) => {
                // Setup reader
                let reader = BufReader::new(file);
                // Deserialize XML data and place in module data structures
                let module_data: Result<MergModuleDataSet, serde_xml_rs::Error> =
                    from_reader(reader);
                match module_data {
                    Ok(md) => {
                        return Some(md);
                    }
                    Err(_) => {
                        error!(
                            "error during serialization {}",
                            xml_file.as_ref().to_str().unwrap()
                        );
                    }
                }
            }
            Err(_) => {
                error!("error opening file {}", xml_file.as_ref().to_str().unwrap())
            }
        }
        None
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
            Some(_) => assert!(false, "'Some' returned)"),
            None => assert!(true, "'None' returned"),
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
            Some(_) => assert!(false, "'Some' returned"),
            None => {
                assert!(true, "'None' returned");
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
            Some(_) => assert!(true, "'Some' returned"),
            None => {
                assert!(false, "'None' returned");
            }
        }
    }
}

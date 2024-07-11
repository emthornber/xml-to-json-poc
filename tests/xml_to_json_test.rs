use xml_to_json::MergModuleDataSet;

use env_logger::Target;
use log::{error, LevelFilter};
use std::{fs::File, io::Write, path::Path};

// Extract from TheDemoPlankII.xml
const CONFIGURATION: &str = r#"<?xml version="1.0" standalone="yes"?>
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
fn local_test() {
    init_logging();
    let cfg_file = "tests/test_fcu_cfg.xml";
    setup_file(&cfg_file, CONFIGURATION);
    // Deserialize the string into the MergModuleDataSet structures
    let test_config = MergModuleDataSet::populate(&cfg_file);
    teardown_file(&cfg_file);
    match test_config {
        Ok(tc) => {
            assert_eq!(tc.userEvents.len(), 1, "userEvents count");
            assert_eq!(tc.userEvents[0].eventNode, 2011);
        }
        Err(e) => assert!(false, "{}", e),
    }
}

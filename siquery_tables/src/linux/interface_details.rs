use tables::InterfaceDetails;
use utils;
use windows::SystemReaderInterface;

impl InterfaceDetails {
    pub(crate) fn new() -> InterfaceDetails {
        InterfaceDetails {
            interface: String::new(),
            mac: String::new(),
            mtu: 1500,
            enabled: 1,
        }
    }

    pub fn get_interfaces(system_reader: &SystemReaderInterface) -> Vec<InterfaceDetails> {
        let mut interfaces: Vec<InterfaceDetails> = Vec::new();
        // TODO interface_details table implementation
        interfaces
    }

}
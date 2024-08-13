use crate::errors::InterfaceError;
use pnet::datalink;

pub fn try_find_interface(interface_name: String) -> Result<datalink::NetworkInterface, InterfaceError> {
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(|iface| iface.name == interface_name);
    
    match interface {
        Some(iface) => Ok(iface),
        None => Err(InterfaceError::NotFound(interface_name)),
    }
}

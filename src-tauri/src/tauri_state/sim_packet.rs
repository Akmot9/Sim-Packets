use crate::errors::{Error, InterfaceError};
use pnet::datalink;

pub fn try_find_interface(interface_name: String) -> Result<datalink::NetworkInterface, Error> {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name);

    match interface {
        Some(iface) => Ok(iface),
        None => Err(Error::InterfaceError(InterfaceError::NotFound(
            interface_name,
        ))),
    }
}
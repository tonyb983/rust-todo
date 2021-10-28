use std::ffi::OsString;
use windows_service::{define_windows_service, service, service_control_handler, service_dispatcher, service_manager};

define_windows_service!(ffi_service_main, todo_service_main);

fn todo_service_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service(arguments) {
        // handle errors
    }
}

fn run_service(arguments: Vec<OsString>) -> Result<(), windows_service::Error> {
    let event_handler = move |control_event: service::ServiceControl| -> service_control_handler::ServiceControlHandlerResult {
        match control_event {
            service::ServiceControl::Continue => todo!(),
            service::ServiceControl::Interrogate => service_control_handler::ServiceControlHandlerResult::NoError,
            service::ServiceControl::NetBindAdd => todo!(),
            service::ServiceControl::NetBindDisable => todo!(),
            service::ServiceControl::NetBindEnable => todo!(),
            service::ServiceControl::NetBindRemove => todo!(),
            service::ServiceControl::ParamChange => todo!(),
            service::ServiceControl::Pause => todo!(),
            service::ServiceControl::Preshutdown => todo!(),
            service::ServiceControl::Shutdown => todo!(),
            service::ServiceControl::Stop => service_control_handler::ServiceControlHandlerResult::NoError,
            service::ServiceControl::HardwareProfileChange(hw_prof_chg_params) => todo!(),
            service::ServiceControl::PowerEvent(pwr_event_params) => todo!(),
            service::ServiceControl::SessionChange(sess_chg_params) => todo!(),
            service::ServiceControl::TimeChange => todo!(),
            service::ServiceControl::TriggerEvent => todo!(),
        }
    };

    let status_handler = service_control_handler::register("thingstodo_service", event_handler)?;
    
    Ok(())
}
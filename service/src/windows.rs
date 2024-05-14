//! The windows specific code for service handling

use std::path::PathBuf;

use std::os::windows::ffi::OsStrExt;

use std::sync::{Arc, Mutex};

use winapi::shared::minwindef::DWORD;
use winapi::um::winsvc::CloseServiceHandle;
use winapi::um::winsvc::OpenSCManagerW;
use winapi::um::winsvc::OpenServiceW;
use winapi::um::winsvc::QueryServiceStatus;
use winapi::um::winsvc::StartServiceW;
use winapi::um::winsvc::SC_HANDLE;
use winapi::um::winsvc::SERVICE_RUNNING;
use winapi::um::winsvc::SERVICE_START_PENDING;

pub struct ServiceStatusHandle(winapi::um::winsvc::SERVICE_STATUS_HANDLE);

impl ServiceStatusHandle {
    pub fn new(a: winapi::um::winsvc::SERVICE_STATUS_HANDLE) -> Self {
        Self(a)
    }
}

unsafe impl Send for ServiceStatusHandle {}

lazy_static::lazy_static! {
    pub static ref SERVICE_HANDLE : Arc<Mutex<ServiceStatusHandle>> = Arc::new(Mutex::new(ServiceStatusHandle(std::ptr::null_mut())));
}

/// The function used to dispatch a windows service.
pub type DispatchFn =
    extern "system" fn(winapi::shared::minwindef::DWORD, *mut winapi::um::winnt::LPWSTR);

pub type ServiceFn<T> = fn(
    rx: std::sync::mpsc::Receiver<crate::ServiceEvent<T>>,
    tx: std::sync::mpsc::Sender<crate::ServiceEvent<T>>,
    args: Vec<String>,
    standalone_mode: bool,
) -> u32;

pub type ServiceFnAsync<T> = fn(
    rx: std::sync::mpsc::Receiver<crate::ServiceEvent<T>>,
    tx: std::sync::mpsc::Sender<crate::ServiceEvent<T>>,
    args: Vec<String>,
    standalone_mode: bool,
) -> u32;

pub struct Session(u32);

/// Converts a utf8 string into a utf-16 string for windows
pub fn get_utf16(value: &str) -> Vec<u16> {
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Convert the windows style arguments to a vec of string
pub fn convert_args(
    argc: winapi::shared::minwindef::DWORD,
    argv: *mut winapi::um::winnt::LPWSTR,
) -> Vec<String> {
    let mut args = Vec::new();

    args
}

/// Converts an optional utf8 string into an optional utf-16 string for windows
pub fn get_optional_utf16(value: Option<&str>) -> winapi::um::winnt::LPCWSTR {
    if let Some(s) = value {
        get_utf16(s).as_ptr()
    } else {
        std::ptr::null_mut()
    }
}

/// Represents a service control handle
pub struct ServiceHandle {
    /// The actual handle
    handle: SC_HANDLE,
}

impl ServiceHandle {
    /// Retrieve the handle, use ServiceController::open_service to get a ServiceHandle
    pub fn get_handle(&self) -> SC_HANDLE {
        self.handle
    }
}

impl Drop for ServiceHandle {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { CloseServiceHandle(self.handle) };
        }
    }
}

/// Represents a service controller manager
pub struct ServiceController {
    /// The actual handle
    handle: SC_HANDLE,
}

impl ServiceController {
    /// Retrieve the handle
    pub fn get_handle(&self) -> SC_HANDLE {
        self.handle
    }

    /// Request access to the service controller manager using the specified access level. [winapi::um::winsvc::SC_MANAGER_ALL_ACCESS](winapi::um::winsvc::SC_MANAGER_ALL_ACCESS) enumerates all possibilites for access levels.
    pub fn open(access: DWORD) -> Option<Self> {
        let handle = unsafe { OpenSCManagerW(std::ptr::null_mut(), std::ptr::null_mut(), access) };
        if handle.is_null() {
            None
        } else {
            Some(Self { handle })
        }
    }

    /// Request access to the specified service, with the specified access permissions. [winapi::um::winsvc::SC_MANAGER_ALL_ACCESS](winapi::um::winsvc::SC_MANAGER_ALL_ACCESS) enumerates all possibilites for access levels.
    pub fn open_service(&self, name: &str, access: DWORD) -> Result<ServiceHandle, DWORD> {
        let handle = unsafe { OpenServiceW(self.handle, get_utf16(name).as_ptr(), access) };
        if handle.is_null() {
            Err(unsafe { winapi::um::errhandlingapi::GetLastError() })
        } else {
            Ok(ServiceHandle { handle })
        }
    }
}

/// The configuration for constructing a Service
pub struct ServiceConfig {
    /// The arguments for the service
    arguments: Vec<String>,
    /// The description of the service as presented to the user
    description: String,
    /// The path to the service binary
    binary: PathBuf,
    /// The username that the service should run as
    username: Option<String>,
    /// The display name of the service for the user.
    pub display: String,
    /// The password for the user that the service should run as
    pub user_password: Option<String>,
    pub desired_access: DWORD,
    pub service_type: DWORD,
    pub start_type: DWORD,
    pub error_control: DWORD,
    pub tag_id: DWORD,
    pub load_order_group: Option<String>,
    pub dependencies: Option<String>,
    pub status_handle: winapi::um::winsvc::SERVICE_STATUS_HANDLE,
    pub controls_accepted: DWORD,
}

impl ServiceConfig {
    /// Build a new service config with reasonable defaults.
    /// # Arguments
    /// * arguments - The list of arguments for the service
    /// * description - The description of the service
    /// * binary - The path to the binary that runs the service
    /// * config_path - The configuration path for the service
    /// * username - The username the service runs as
    /// * display - The display name of the service
    /// * user_password - The password for the user to run the service as
    pub fn new(
        arguments: Vec<String>,
        description: String,
        binary: PathBuf,
        username: Option<String>,
    ) -> Self {
        Self {
            arguments,
            description,
            binary,
            username: username,
            display: String::new(),
            user_password: None,
            desired_access: winapi::um::winsvc::SERVICE_ALL_ACCESS,
            service_type: winapi::um::winnt::SERVICE_WIN32_OWN_PROCESS,
            start_type: winapi::um::winnt::SERVICE_AUTO_START,
            error_control: winapi::um::winnt::SERVICE_ERROR_NORMAL,
            tag_id: 0,
            load_order_group: None,
            dependencies: None,
            status_handle: std::ptr::null_mut(),
            controls_accepted: winapi::um::winsvc::SERVICE_ACCEPT_STOP,
        }
    }
}

/// Represents a service on the system
pub struct Service {
    /// The name of the service, as known by the operating system
    name: String,
}

impl Service {
    /// Construct a new self
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Initialize a new log instance
    pub fn new_log(&self, level: super::LogLevel) {
        eventlog::init(&format!("{} Log", self.name), level.level()).unwrap();
    }

    /// Does the service already exist?
    pub fn exists(&self) -> bool {
        let service_manager = ServiceController::open(winapi::um::winsvc::SC_MANAGER_ALL_ACCESS)
            .unwrap_or_else(|| panic!("Unable to get service controller")); //TODO REMOVE RIGHTS NOT REQUIRED
        let service =
            service_manager.open_service(&self.name, winapi::um::winsvc::SERVICE_ALL_ACCESS);
        service.is_ok()
    }

    /// Stop the service
    pub fn stop(&mut self) -> Result<(), ()> {
        let service_manager = ServiceController::open(winapi::um::winsvc::SC_MANAGER_ALL_ACCESS); //TODO REMOVE RIGHTS NOT REQUIRED
        if let Some(service_manager) = service_manager {
            let service = service_manager
                .open_service(&self.name, winapi::um::winsvc::SERVICE_ALL_ACCESS)
                .unwrap();
            let mut service_status: winapi::um::winsvc::SERVICE_STATUS =
                winapi::um::winsvc::SERVICE_STATUS {
                    dwServiceType: winapi::um::winnt::SERVICE_WIN32_OWN_PROCESS,
                    dwCurrentState: winapi::um::winsvc::SERVICE_STOPPED,
                    dwControlsAccepted: 0,
                    dwWin32ExitCode: 0,
                    dwServiceSpecificExitCode: 0,
                    dwCheckPoint: 0,
                    dwWaitHint: 0,
                };
            if unsafe {
                winapi::um::winsvc::ControlService(
                    service.get_handle(),
                    winapi::um::winsvc::SERVICE_CONTROL_STOP,
                    &mut service_status,
                )
            } != 0
            {
                while unsafe {
                    winapi::um::winsvc::QueryServiceStatus(
                        service.get_handle(),
                        &mut service_status,
                    )
                } != 0
                {
                    if service_status.dwCurrentState != winapi::um::winsvc::SERVICE_STOP_PENDING {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
            }

            if unsafe { winapi::um::winsvc::DeleteService(service.get_handle()) } == 0 {
                return Err(());
            }
            Ok(())
        } else {
            Err(())
        }
    }

    /// Start the service
    pub fn start(&mut self) -> Result<(), ()> {
        let service_manager = ServiceController::open(winapi::um::winsvc::SC_MANAGER_ALL_ACCESS); //TODO REMOVE RIGHTS NOT REQUIRED
        if let Some(service_manager) = service_manager {
            let service = service_manager
                .open_service(&self.name, winapi::um::winsvc::SERVICE_ALL_ACCESS)
                .unwrap();
            let mut service_status: winapi::um::winsvc::SERVICE_STATUS =
                winapi::um::winsvc::SERVICE_STATUS {
                    dwServiceType: winapi::um::winnt::SERVICE_WIN32_OWN_PROCESS,
                    dwCurrentState: winapi::um::winsvc::SERVICE_STOPPED,
                    dwControlsAccepted: 0,
                    dwWin32ExitCode: 0,
                    dwServiceSpecificExitCode: 0,
                    dwCheckPoint: 0,
                    dwWaitHint: 0,
                };
            if unsafe { StartServiceW(service.handle, 0, std::ptr::null_mut()) } != 0 {
                while unsafe { QueryServiceStatus(service.handle, &mut service_status) } != 0 {
                    if service_status.dwCurrentState != SERVICE_START_PENDING {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
            }

            if service_status.dwCurrentState != SERVICE_RUNNING {
                println!("Failed to start service {}", service_status.dwCurrentState);

                Err(())
            } else {
                Ok(())
            }
        } else {
            Err(())
        }
    }

    /// Delete the service
    pub fn delete(&mut self) -> Result<(), ()> {
        let service_manager = ServiceController::open(winapi::um::winsvc::SC_MANAGER_ALL_ACCESS); //TODO REMOVE RIGHTS NOT REQUIRED
        let _e = eventlog::deregister(&format!("{} Log", self.name));
        if let Some(service_manager) = service_manager {
            let service = service_manager
                .open_service(&self.name, winapi::um::winsvc::SERVICE_ALL_ACCESS)
                .map_err(|_| ())?;
            if unsafe { winapi::um::winsvc::DeleteService(service.get_handle()) } == 0 {
                return Err(());
            }
            Ok(())
        } else {
            Err(())
        }
    }

    #[cfg(feature = "async")]
    /// Delete the service
    pub async fn delete_async(&mut self) -> Result<(), ()> {
        self.delete()
    }

    /// Create the service
    pub fn create(&mut self, config: ServiceConfig) -> Result<(), ()> {
        eventlog::register(&format!("{} Log", self.name)).unwrap();
        let service_manager = ServiceController::open(winapi::um::winsvc::SC_MANAGER_ALL_ACCESS); //TODO REMOVE RIGHTS NOT REQUIRED
        let exe = config.binary.as_os_str().to_str().unwrap();
        let args = config.arguments.join(" ");
        let exe_with_args = if config.arguments.is_empty() {
            exe.to_string()
        } else {
            format!("{} {}", exe, args)
        };
        if let Some(service_manager) = service_manager {
            let service = unsafe {
                winapi::um::winsvc::CreateServiceW(
                    service_manager.get_handle(),
                    get_utf16(&self.name).as_ptr(),
                    get_utf16(&config.display).as_ptr(),
                    config.desired_access,
                    config.service_type,
                    config.start_type,
                    config.error_control,
                    get_utf16(&exe_with_args).as_ptr(),
                    get_optional_utf16(config.load_order_group.as_deref()),
                    std::ptr::null_mut(),
                    get_optional_utf16(config.dependencies.as_deref()),
                    get_optional_utf16(config.username.as_deref()),
                    get_optional_utf16(config.user_password.as_deref()),
                )
            };
            if service.is_null() {
                return Err(());
            }
            let mut description = get_utf16(&config.description);

            let mut sd = winapi::um::winsvc::SERVICE_DESCRIPTIONW {
                lpDescription: description.as_mut_ptr(),
            };

            let p_sd = &mut sd as *mut _ as *mut winapi::ctypes::c_void;
            unsafe {
                winapi::um::winsvc::ChangeServiceConfig2W(
                    service,
                    winapi::um::winsvc::SERVICE_CONFIG_DESCRIPTION,
                    p_sd,
                )
            };
            unsafe { winapi::um::winsvc::CloseServiceHandle(service) };
            Ok(())
        } else {
            Err(())
        }
    }

    #[cfg(feature = "async")]
    /// Create the service
    pub async fn create_async(&mut self, config: ServiceConfig) -> Result<(), ()> {
        self.create(config)
    }

    /// Run the required dispatch code for windows
    pub fn dispatch(&self, service_main: DispatchFn) -> Result<(), ()> {
        std::env::set_var("SERVICE_NAME", &self.name);
        // The service is setup with SERVICE_WIN32_OWN_PROCESS, so this argument is ignored, but cannot be null
        let service_name = get_utf16("");
        let service_table: &[winapi::um::winsvc::SERVICE_TABLE_ENTRYW] = &[
            winapi::um::winsvc::SERVICE_TABLE_ENTRYW {
                lpServiceName: service_name.as_ptr() as _,
                lpServiceProc: Some(service_main),
            },
            // the last item has to be { null, null }
            winapi::um::winsvc::SERVICE_TABLE_ENTRYW {
                lpServiceName: std::ptr::null_mut(),
                lpServiceProc: None,
            },
        ];

        let result =
            unsafe { winapi::um::winsvc::StartServiceCtrlDispatcherW(service_table.as_ptr()) };
        if result == 0 {
            println!("The error was {}", unsafe {
                winapi::um::errhandlingapi::GetLastError()
            });
            Err(())
        } else {
            Ok(())
        }
    }
}

/// The macro generates the service function required for windows
#[macro_export]
macro_rules! ServiceMacro {
    ($entry:ident, $function:ident) => {
        extern "system" fn $entry(
            argc: service::winapi::shared::minwindef::DWORD,
            argv: *mut service::winapi::um::winnt::LPWSTR,
        ) {
            let name = std::env::var("SERVICE_NAME").unwrap();
            service::run_service($function, &name, argc, argv);
        }
    };
}

/// The macro generates the service function required for windows
#[macro_export]
macro_rules! ServiceAsyncMacro {
    ($entry:ident, $function:ident, $t:ident) => {
        extern "system" fn $entry(
            argc: service::winapi::shared::minwindef::DWORD,
            argv: *mut service::winapi::um::winnt::LPWSTR,
        ) {
            let name = std::env::var("SERVICE_NAME").unwrap();
            let args = service::convert_args(argc, argv);
            service::log::debug!("The arguments are {:?}", args);
            service::log::debug!("Env args are {:?}", std::env::args());
            let (mut tx, rx) = std::sync::mpsc::channel();
            let tx2: std::sync::mpsc::Sender<service::ServiceEvent<$t>> = tx.clone();
            let handle = unsafe {
                service::winapi::um::winsvc::RegisterServiceCtrlHandlerExW(
                    service::get_utf16(&name).as_ptr(),
                    Some(service::service_handler::<$t>),
                    &mut tx as *mut _ as service::winapi::shared::minwindef::LPVOID,
                )
            };
            let mut sh = service::SERVICE_HANDLE.lock().unwrap();
            *sh = service::ServiceStatusHandle::new(handle);
            service::set_service_status(
                handle,
                service::winapi::um::winsvc::SERVICE_START_PENDING,
                0,
            );
            service::set_service_status(handle, service::winapi::um::winsvc::SERVICE_RUNNING, 0);

            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap();

            let service_thread = runtime.spawn(async move {
                service::log::debug!("Running async function now");
                $function(rx, tx2, args, false).await;
            });
            runtime.block_on(service_thread).unwrap();
            service::set_service_status(handle, service::winapi::um::winsvc::SERVICE_STOPPED, 0);
        }
    };
}

pub unsafe extern "system" fn service_handler<T>(
    control: winapi::shared::minwindef::DWORD,
    event_type: winapi::shared::minwindef::DWORD,
    event_data: winapi::shared::minwindef::LPVOID,
    context: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::DWORD {
    let tx = context as *mut std::sync::mpsc::Sender<crate::ServiceEvent<T>>;
    log::debug!("The command for service handler is {}", control);
    match control {
        winapi::um::winsvc::SERVICE_CONTROL_STOP | winapi::um::winsvc::SERVICE_CONTROL_SHUTDOWN => {
            use std::ops::DerefMut;
            let mut sh = SERVICE_HANDLE.lock().unwrap();
            let ServiceStatusHandle(h) = sh.deref_mut();
            set_service_status(*h, winapi::um::winsvc::SERVICE_STOP_PENDING, 10);
            let _ = (*tx).send(crate::ServiceEvent::Stop);
            0
        }
        winapi::um::winsvc::SERVICE_CONTROL_PAUSE => {
            let _ = (*tx).send(crate::ServiceEvent::Pause);
            0
        }
        winapi::um::winsvc::SERVICE_CONTROL_CONTINUE => {
            let _ = (*tx).send(crate::ServiceEvent::Continue);
            0
        }
        winapi::um::winsvc::SERVICE_CONTROL_SESSIONCHANGE => {
            let event = event_type;
            let session_notification =
                event_data as *const winapi::um::winuser::WTSSESSION_NOTIFICATION;
            let session_id = (*session_notification).dwSessionId;
            let session = Session(session_id);

            match event as usize {
                winapi::um::winuser::WTS_CONSOLE_CONNECT => {
                    let _ = (*tx).send(crate::ServiceEvent::SessionConnect(session));
                    0
                }
                winapi::um::winuser::WTS_CONSOLE_DISCONNECT => {
                    let _ = (*tx).send(crate::ServiceEvent::SessionDisconnect(session));
                    0
                }
                winapi::um::winuser::WTS_REMOTE_CONNECT => {
                    let _ = (*tx).send(crate::ServiceEvent::SessionRemoteConnect(session));
                    0
                }
                winapi::um::winuser::WTS_REMOTE_DISCONNECT => {
                    let _ = (*tx).send(crate::ServiceEvent::SessionRemoteDisconnect(session));
                    0
                }
                winapi::um::winuser::WTS_SESSION_LOGON => {
                    let _ = (*tx).send(crate::ServiceEvent::SessionLogon(session));
                    0
                }
                winapi::um::winuser::WTS_SESSION_LOGOFF => {
                    let _ = (*tx).send(crate::ServiceEvent::SessionLogoff(session));
                    0
                }
                winapi::um::winuser::WTS_SESSION_LOCK => {
                    let _ = (*tx).send(crate::ServiceEvent::SessionLock(session));
                    0
                }
                winapi::um::winuser::WTS_SESSION_UNLOCK => {
                    let _ = (*tx).send(crate::ServiceEvent::SessionUnlock(session));
                    0
                }
                _ => 0,
            }
        }
        _ => winapi::shared::winerror::ERROR_CALL_NOT_IMPLEMENTED,
    }
}

/// Report the status of the service to windows
pub fn set_service_status(
    status_handle: winapi::um::winsvc::SERVICE_STATUS_HANDLE,
    current_state: winapi::shared::minwindef::DWORD,
    wait_hint: winapi::shared::minwindef::DWORD,
) {
    let mut service_status = winapi::um::winsvc::SERVICE_STATUS {
        dwServiceType: winapi::um::winnt::SERVICE_WIN32_OWN_PROCESS,
        dwCurrentState: current_state,
        dwControlsAccepted: winapi::um::winsvc::SERVICE_ACCEPT_STOP
            | winapi::um::winsvc::SERVICE_ACCEPT_SHUTDOWN,
        dwWin32ExitCode: 0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: wait_hint,
    };
    unsafe {
        winapi::um::winsvc::SetServiceStatus(status_handle, &mut service_status);
    }
}

/// Runs the main service function
pub fn run_service<T: std::marker::Send + 'static>(
    service_main: ServiceFn<T>,
    name: &str,
    argc: winapi::shared::minwindef::DWORD,
    argv: *mut winapi::um::winnt::LPWSTR,
) {
    let args = convert_args(argc, argv);
    log::debug!("The arguments are {:?}", args);
    log::debug!("Env args are {:?}", std::env::args());
    let (mut tx, rx) = std::sync::mpsc::channel();
    let tx2: std::sync::mpsc::Sender<crate::ServiceEvent<T>> = tx.clone();
    let handle = unsafe {
        winapi::um::winsvc::RegisterServiceCtrlHandlerExW(
            get_utf16(name).as_ptr(),
            Some(service_handler::<T>),
            &mut tx as *mut _ as winapi::shared::minwindef::LPVOID,
        )
    };
    let mut sh = SERVICE_HANDLE.lock().unwrap();
    *sh = ServiceStatusHandle(handle);
    set_service_status(handle, winapi::um::winsvc::SERVICE_START_PENDING, 0);
    set_service_status(handle, winapi::um::winsvc::SERVICE_RUNNING, 0);
    let service_thread = std::thread::spawn(move || {
        service_main(rx, tx2, args, false);
    });
    service_thread.join();
    set_service_status(handle, winapi::um::winsvc::SERVICE_STOPPED, 0);
}
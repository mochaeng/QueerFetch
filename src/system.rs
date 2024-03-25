use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct SysRepository {
    functions: Vec<Box<dyn Fn() -> String>>,
}

impl SysRepository {
    pub fn new() -> Self {
        let mut functions: Vec<Box<dyn Fn() -> String>> = Vec::new();

        functions.push(Box::new(|| get_host_info()));
        functions.push(Box::new(|| get_dashes()));
        functions.push(Box::new(|| get_os_name()));
        functions.push(Box::new(|| get_kernel_version()));
        functions.push(Box::new(|| get_cpu_info()));
        functions.push(Box::new(|| get_memory_info()));

        SysRepository { functions }
    }

    pub fn get_func(&mut self) -> Option<Box<dyn Fn() -> String>> {
        if !self.functions.is_empty() {
            Some(self.functions.remove(0))
        } else {
            None
        }
    }
}

fn get_host_info() -> String {
    format!("{}@{}", whoami::username(), whoami::distro())
}

fn get_dashes() -> String {
    "-".repeat(get_host_info().len())
}

fn get_os_name() -> String {
    format!("OS: {} {}", whoami::platform(), whoami::arch())
}

fn get_kernel_version() -> String {
    let version = System::kernel_version();
    if let Some(version) = version {
        format!("Kernel: {}", version)
    } else {
        format!("Kernel: UNKNOW")
    }
}

fn get_memory_info() -> String {
    let mut s = System::new();
    s.refresh_all();

    format!(
        "RAM: {}MiB >> {}MiB",
        bytes_to_mib(s.used_memory()).ceil(),
        bytes_to_mib(s.total_memory()).ceil()
    )
}

fn bytes_to_mib(bytes: u64) -> f64 {
    bytes as f64 / 1.049e+6
}

fn get_cpu_info() -> String {
    let s = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    let mut cpu_info = String::new();

    cpu_info.push_str(s.cpus().get(0).unwrap().brand());

    format!("CPU: {}", cpu_info)
}

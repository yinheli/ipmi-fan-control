use std::process::Command;

use anyhow::{Error, Ok};
use regex::Regex;

pub(crate) trait Ipmi {
    fn get_info_fan_temp(&self) -> Result<String, Error>;
    fn get_cpu_temperature(&self) -> Result<u16, Error>;
    fn set_fan_speed(&self, speed: u16) -> Result<(), Error>;
}

pub(crate) trait Executer {
    fn get_info_fan_temp(&self) -> Result<String, Error>;
    fn get_cpu_temperature(&self) -> Result<String, Error>;
    fn set_fan_speed(&self, speed: u16) -> Result<(), Error>;

    fn execute(&self, program: &str, args: Vec<&str>) -> Result<String, Error> {
        let output = Command::new(program).args(args).output()?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.to_string())
        } else {
            Err(anyhow!(
                "status:{}, stderr: {}",
                output.status.code().unwrap(),
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }
}

pub(crate) struct Cmd {}

impl Cmd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executer for Cmd {
    fn get_info_fan_temp(&self) -> Result<String, Error> {
        self.execute("ipmitool", vec!["sdr", "list", "full"])
    }

    fn get_cpu_temperature(&self) -> Result<String, Error> {
        self.execute("ipmitool", vec!["sdr", "type", "Temperature"])
    }

    fn set_fan_speed(&self, speed: u16) -> Result<(), Error> {
        let v = format!("{:#04x}", speed);
        self.execute("ipmitool", vec!["raw", "0x30", "0x30", "0x01", "0x00"])?;
        self.execute("ipmitool", vec!["raw", "0x30", "0x30", "0x02", "0xff", &v])?;
        Ok(())
    }
}

pub(crate) struct IpmiTool {
    cmd: Box<dyn Executer>,
}

impl IpmiTool {
    pub fn new(cmd: Box<dyn Executer>) -> Self {
        Self { cmd }
    }
}

impl Ipmi for IpmiTool {
    fn get_info_fan_temp(&self) -> Result<String, Error> {
        let res = self.cmd.get_info_fan_temp()?;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?i)^fan|temp").unwrap();
        }

        let filtered: String = res
            .lines()
            .filter(|x| RE.is_match(x))
            .map(|x| x.to_owned() + "\n")
            .collect();
        Ok(filtered)
    }

    fn get_cpu_temperature(&self) -> Result<u16, Error> {
        let res = self.cmd.get_cpu_temperature()?;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?i)(\d*)\sdegrees").unwrap();
        }

        for x in res.lines() {
            if x.to_lowercase().starts_with("temp ") {
                if let Some(v) = RE.captures(x) {
                    let tmp = v.get(1).unwrap().as_str().parse::<u16>()?;
                    return Ok(tmp);
                }
            }
        }

        Err(anyhow!("not found"))
    }

    fn set_fan_speed(&self, speed: u16) -> Result<(), Error> {
        self.cmd.set_fan_speed(speed)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockCommand {}

    impl Executer for MockCommand {
        fn get_info_fan_temp(&self) -> Result<String, Error> {
            let output = "
Fan1             | 4800 RPM          | ok
Fan2             | 4800 RPM          | ok
Fan3             | 4800 RPM          | ok
Fan4             | 4680 RPM          | ok
Fan5             | 4800 RPM          | ok
Fan6             | 4800 RPM          | ok
Inlet Temp       | 25 degrees C      | ok
CPU Usage        | 0 percent         | ok
IO Usage         | 0 percent         | ok
MEM Usage        | 0 percent         | ok
SYS Usage        | 0 percent         | ok
Exhaust Temp     | 32 degrees C      | ok
Temp             | 45 degrees C      | ok
Temp             | 45 degrees C      | ok
Current 1        | 1 Amps            | ok
Current 2        | 0.20 Amps         | ok
Voltage 1        | 236 Volts         | ok
Voltage 2        | 232 Volts         | ok
Pwr Consumption  | 238 Watts         | ok
            ";
            Ok(output.to_string())
        }

        fn get_cpu_temperature(&self) -> Result<String, Error> {
            let output = "
Inlet Temp       | 04h | ok  |  7.1 | 25 degrees C
Exhaust Temp     | 01h | ok  |  7.1 | 32 degrees C
Temp             | 0Eh | ok  |  3.1 | 45 degrees C
Temp             | 0Fh | ok  |  3.2 | 45 degrees C
            ";
            Ok(output.to_string())
        }

        fn set_fan_speed(&self, speed: u16) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn test_mock_works() {
        let cmd = MockCommand {};
        assert!(cmd.get_info_fan_temp().is_ok());
    }

    #[test]
    fn test_ipmi_tool() {
        let ipmi = IpmiTool::new(Box::new(MockCommand {}));

        let res = ipmi.get_info_fan_temp();
        assert!(res.is_ok());
        println!("{}", res.unwrap());

        let res = ipmi.get_cpu_temperature();
        assert!(res.is_ok());
        assert_eq!(45, res.unwrap());

        let res = ipmi.set_fan_speed(10);
        assert!(res.is_ok());
    }
}

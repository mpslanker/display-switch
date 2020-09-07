use crate::display_control::DDCControl;
use anyhow::{anyhow, Result};
use ddc_hi::{Ddc, Display};

pub struct DDCControlDdcHi();

/// VCP feature code for input select
const INPUT_SELECT: u8 = 0x60;

fn ddc_for(screen_idx: isize) -> Result<Display> {
    let mut displays = Display::enumerate();
    if (screen_idx >= 0) && ((screen_idx as usize) < displays.len()) {
        Ok(displays.remove(screen_idx as usize))
    } else {
        Err(anyhow!("Monitor not found"))
    }
}

fn display_name(display: &Display, screen_idx: isize) -> String {
    // TODO: Verify that formatting here makes sense on Linux as well
    format!("'{}' #{}", display.info.id, screen_idx)
}

impl DDCControl for DDCControlDdcHi {
    fn get_display_range() -> std::ops::Range<isize> {
        0..Display::enumerate().len() as isize
    }

    fn ddc_read_input_select(screen_idx: isize) -> Result<u16> {
        let mut display = ddc_for(screen_idx)?;
        let display_name = display_name(&display, screen_idx);
        match display.handle.get_vcp_feature(INPUT_SELECT) {
            Ok(source) => {
                info!(
                    "Monitor {} is currently set to 0x{:x}",
                    display_name,
                    source.value()
                );
                Ok(source.value())
            }
            Err(err) => {
                error!(
                    "Failed to get current input for monitor {}: {:?}",
                    display_name, err
                );
                Err(anyhow!(err))
            }
        }
    }

    fn ddc_write_input_select(screen_idx: isize, source: u16) -> Result<()> {
        let mut display = ddc_for(screen_idx)?;
        let display_name = display_name(&display, screen_idx);
        debug!("Setting monitor '{}' to 0x{:x}", display_name, source);
        match display.handle.set_vcp_feature(INPUT_SELECT, source) {
            Ok(_) => {
                info!("Monitor {} set to 0x{:x}", display_name, source);
                Ok(())
            }
            Err(err) => {
                error!(
                    "Failed to set monitor {} to 0x{:x} ({:?})",
                    display_name, source, err
                );
                Err(anyhow!(err))
            }
        }
    }
}
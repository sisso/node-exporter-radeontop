use std::error::Error;
use std::io::BufRead;
use std::process::Child;

#[derive(Debug, Clone)]
pub struct RadeonData {
    pub timestamp: f64,
    pub bus: f64,
    pub gpu: f64,
    pub ee: f64,
    pub vgt: f64,
    pub ta: f64,
    pub tc: f64,
    pub sx: f64,
    pub sh: f64,
    pub spi: f64,
    pub smx: f64,
    pub cr: f64,
    pub sc: f64,
    pub pa: f64,
    pub db: f64,
    pub cb: f64,
    pub vram: f64,
    pub gtt: f64,
    pub mclk: f64,
    pub sclk: f64,
}

pub struct RadeonListener {
    child: Child,
}

impl RadeonListener {
    pub fn new() -> Result<RadeonListener, Box<dyn Error>> {
        let mut cmd = std::process::Command::new("radeontop");
        cmd.arg("-d").arg("-");
        let mut child = cmd.stdout(std::process::Stdio::piped()).spawn()?;
        Ok(Self { child })
    }

    pub fn next(&mut self) -> Result<RadeonData, Box<dyn Error>> {
        let stdout = self
            .child
            .stdout
            .as_mut()
            .expect("Failed to get child stdout");
        let mut reader = std::io::BufReader::new(stdout);
        let mut buffer = String::new();
        loop {
            reader.read_line(&mut buffer)?;
            let data = parse_radeontop_line(&buffer);
            if let Ok(data) = data {
                return Ok(data);
            }
        }
    }
}

fn parse_radeontop_line(line: &str) -> Result<RadeonData, Box<dyn Error>> {
    // use regular expression to parse the sample line "1708806303.77193: bus 03, gpu 3.33%, ee 0.00%, vgt 0.83%, ta 0.00%, tc 0.00%, sx 0.83%, sh 0.83%, spi 0.83%, smx 0.83%, cr 4.17%, sc 0.83%, pa 0.00%, db 0.83%, cb 0.83%, vram 6.08% 1489.07mb, gtt 0.39% 126.32mb, mclk 7.69% 0.096ghz, sclk 4.12% 0.098ghz" into RadeonData struct
    let re = regex::Regex::new(
        r"(\d+).(\d+): bus (\d+), gpu ([\d.]+)%, ee ([\d.]+)%, vgt ([\d.]+)%, ta ([\d.]+)%, tc ([\d.]+)%, sx ([\d.]+)%, sh ([\d.]+)%, spi ([\d.]+)%, smx ([\d.]+)%, cr ([\d.]+)%, sc ([\d.]+)%, pa ([\d.]+)%, db ([\d.]+)%, cb ([\d.]+)%, vram ([\d.]+)% ([\d.]+)mb, gtt ([\d.]+)% ([\d.]+)mb, mclk ([\d.]+)% ([\d.]+)ghz, sclk ([\d.]+)% ([\d.]+)ghz",
    )?;
    let Some(caps) = re.captures(line) else {
        return Err(format!("fail to parse line {:?}", line).into());
    };

    Ok(RadeonData {
        timestamp: caps[1].parse::<f64>()?,
        bus: caps[3].parse()?,
        gpu: caps[4].parse()?,
        ee: caps[5].parse()?,
        vgt: caps[6].parse()?,
        ta: caps[7].parse()?,
        tc: caps[8].parse()?,
        sx: caps[9].parse()?,
        sh: caps[10].parse()?,
        spi: caps[11].parse()?,
        smx: caps[12].parse()?,
        cr: caps[13].parse()?,
        sc: caps[14].parse()?,
        pa: caps[15].parse()?,
        db: caps[16].parse()?,
        cb: caps[17].parse()?,
        vram: caps[18].parse()?, // memory 19
        gtt: caps[20].parse()?,  // memory 21
        mclk: caps[22].parse()?,
        sclk: caps[24].parse()?,
    })
}

// create test module
#[cfg(test)]
mod test {
    use super::*;

    // create test for parse_radeontop_line
    #[test]
    fn test_parse_radeontop_line() {
        let line = "1708806303.77193: bus 03, gpu 3.33%, ee 0.00%, vgt 0.83%, ta 0.00%, \
            tc 0.00%, sx 0.83%, sh 0.83%, spi 0.83%, smx 0.83%, cr 4.17%, sc 0.83%, pa 0.00%, db \
            0.83%, cb 0.83%, vram 6.08% 1489.07mb, gtt 0.39% 126.32mb, mclk 7.69% 0.096ghz, \
            sclk 4.12% 0.098ghz";

        let data = parse_radeontop_line(line).unwrap();
        assert_eq!(data.timestamp, 1708806303.0);
        assert_eq!(data.bus, 3.0);
        assert_eq!(data.gpu, 3.33);
        assert_eq!(data.ee, 0.0);
        assert_eq!(data.vgt, 0.83);
        assert_eq!(data.ta, 0.0);
        assert_eq!(data.tc, 0.0);
        assert_eq!(data.sx, 0.83);
        assert_eq!(data.sh, 0.83);
        assert_eq!(data.spi, 0.83);
        assert_eq!(data.smx, 0.83);
        assert_eq!(data.cr, 4.17);
        assert_eq!(data.sc, 0.83);
        assert_eq!(data.pa, 0.0);
        assert_eq!(data.db, 0.83);
        assert_eq!(data.cb, 0.83);
        assert_eq!(data.vram, 6.08);
        assert_eq!(data.gtt, 0.39);
        assert_eq!(data.mclk, 7.69);
        assert_eq!(data.sclk, 4.12);
    }
}

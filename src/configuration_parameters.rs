use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    pub stg_non_sec_exposure_fn:String,
    pub finnone_extract:String,
    pub finnone_npa:String,
    pub finnone_master:String,
    pub stg_company_details:String,
    pub restructure_merged:String,
    pub fn_collateral:String,
    pub output_file:String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub finnone_master_sheet_name:String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());


        info!(logger,"The stg.non_sec_exposure_fn is:{}",self.stg_non_sec_exposure_fn());
        info!(logger,"the path of I2 file is:{}",self.finnone_extract());
        info!(logger,"The path of finnone npa file is:{}",self.finnone_npa());
        info!(logger,"The path of finnone master file is:{}",self.finnone_master());
        info!(logger,"The path of stg company details file is:{}",self.stg_company_details());
        info!(logger,"The path of restructure merged file is:{}",self.restructure_merged());
        info!(logger,"The path of fn collateral file is:{}",self.fn_collateral());
        info!(logger,"output_file_path:{}",self.output_file());
        info!(logger,"sheet_name is:{}",self.finnone_master_sheet_name());

    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
         let mut stg_non_sec_exposure_fn=matches.value_of("stg_non_sec_exposure_fn").unwrap().to_string();
        //let mut output_file_path=matches.value_of("output_file_path").unwrap().to_string();
        let mut finnone_extract=matches.value_of("finnone_extract").unwrap().to_string();
        let mut finnone_npa=matches.value_of("finnone_npa").unwrap().to_string();
        let mut finnone_master=matches.value_of("finnone_master").unwrap().to_string();
        let mut stg_company_details=matches.value_of("stg_company_details").unwrap().to_string();
        let mut restructure_merged=matches.value_of("restructure_merged").unwrap().to_string();
        let mut fn_collateral=matches.value_of("fn_collateral").unwrap().to_string();
        let mut output_file=matches.value_of("output_file").unwrap().to_string();
        let mut finnone_master_sheet_name=matches.value_of("finnone_master_sheet_name").unwrap().to_string();
        
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            stg_non_sec_exposure_fn,
            finnone_extract,
            finnone_npa,
            finnone_master,
            stg_company_details,
            restructure_merged,
            fn_collateral,
            log_file_path,
            log_level,
            diagnostics_file_path,
            is_perf_diagnostics_enabled,
            output_file,
            finnone_master_sheet_name,

        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn stg_non_sec_exposure_fn(&self)-> &str{
        &self.stg_non_sec_exposure_fn
    }
    pub fn finnone_extract(&self) -> &str{
        &self.finnone_extract
    }
    pub fn finnone_npa(&self) -> &str{
        &self.finnone_npa
    }
    pub fn finnone_master(&self) -> &str{
        &self.finnone_master
    }
    pub fn stg_company_details(&self) -> &str{
        &self.stg_company_details
    }
    pub fn restructure_merged(&self) -> &str{
        &self.restructure_merged
    }
    pub fn fn_collateral(&self) -> &str{
        &self.fn_collateral
    }
    pub fn output_file(&self) -> &str{
        &self.output_file
    }
    pub fn finnone_master_sheet_name(&self)->&str{
        &self.finnone_master_sheet_name
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)    
    .about("generating command line arguements for arguements!!")
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("log-file")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("diagnostics-log-file")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("stg_non_sec_exposure_fn")
                .long("stg-non-sec-exposure-fn")
                .value_name("stg-non-sec-exposure_fn")
                .help("Helps to get contents of non-sec-exposure file.")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_extract")
                .long("finnone-extract")
                .value_name("finnone-extract")
                .help("Help get the contents of the finnone-extract file.")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_npa")
                .long("finnone-npa")
                .value_name("finnone-npa")
                .help("Help get the contents of the finnone-npa-file.")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_master")
                .long("finnone-master")
                .value_name("finnone-master")
                .help("Help get the contents of finnone-master-file.")
                .required(true)
        )
        .arg(
            Arg::with_name("stg_company_details")
                .long("stg-company-details")
                .value_name("stg-company-details")
                .help("Help get the contents of the stg_company_details file.")
                .required(true)
        )
        .arg(
            Arg::with_name("restructure_merged")
                .long("restructure-merged")
                .value_name("restructure-merged")
                .help("Help get the contents from I6 file.")
                .required(true)
        )
      
        .arg(
            Arg::with_name("fn_collateral")
                .long("fn-collateral")
                .value_name("fn-collateral")
                .help("Help get the contents of the master1 file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output-file")
                .help("Helps get the output file path to write contents to text file")
           
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_master_sheet_name")
                .long("finnone-master-sheet-name")
                .value_name("finnone-master-sheet-name")
                .help("Helps get the output file path to write contents to text file")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .required(false)
        )
        .get_matches()
}

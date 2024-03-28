use configuration_parameters::ConfigurationParameters;
use calamine::{Reader,open_workbook_auto, Xlsx,Sheets};
use calamine::open_workbook;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use macros;
use sdb_io::buf_file_wrtr;

//use sdb_io::

pub fn care_poc_lacks(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) 
{
#[derive(Debug, Clone)]

    struct FinnoneNPAStruct
    {
        
        FINAL_NPA_STATUS_BANK:String,
        PROVISION_AMOUNT_V1:f64

    };

    let mut finnone_extract_hashmap: HashMap<String, String> = HashMap::new();
    let mut fn_collateral_hashmap: HashMap<String, String> = HashMap::new();
    let mut finnone_npa_hashmap: HashMap<String, FinnoneNPAStruct> = HashMap::new();
    let mut finnone_master_hashmap: HashMap<String, String> = HashMap::new();
    let mut stg_company_details_hashmap: HashMap<String, String> = HashMap::new();
    let mut restructure_merger_hashmap: HashMap<String, String> = HashMap::new();

    let mut non_security_exposure_vec: Vec<&str> = Vec::new();
    let mut finnone_extract_vec:Vec<&str>=Vec::new();
    let mut finnone_npa_vec:Vec<&str>=Vec::new();
    let mut stg_company_details_vec:Vec<&str>=Vec::new();
    let mut i:String;
    
    let finnone_extract_file = File::open(config_params.finnone_extract()).expect("could not read finnone_extract file");
    let mut finnoneextractreader=BufReader::new(finnone_extract_file);
    let mut finnone_npa_file = File::open(config_params.finnone_npa()).expect("could not finnone_npa i3 file"); 
    let mut finnonenpareader=BufReader::new(finnone_npa_file);
    let mut stg_company_details_file= File::open(config_params.stg_company_details()).expect("could not read stg_company_details file");
    let mut stgdetailscompanyreader=BufReader::new(stg_company_details_file);
    let mut restructure_merged_file = File::open(config_params.restructure_merged()).expect("could not read restructure_merged file");
    let mut restructuremergedreader=BufReader::new(restructure_merged_file);
    let fn_collateral_file = File::open(config_params.fn_collateral()).expect("could not read fn_collateral file");
    let mut fncollateralreader=BufReader::new(fn_collateral_file);
         
    
    for (line_num, lines) in finnoneextractreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.finnone_extract(),
                line_num + 1,
                error
            ),
        };
        let finnone_extract_vec = line.split('|').collect::<Vec<&str>>();
        finnone_extract_hashmap.insert(finnone_extract_vec[2].to_string(),finnone_extract_vec[20].to_string());

    }
    for (line_num, lines) in finnonenpareader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.finnone_npa(),
                line_num + 1,
                error
            ),
        };
        finnone_npa_vec = line.split('|').collect::<Vec<&str>>();

        let mut String_Amount:String=finnone_npa_vec[5].to_string();
        let mut PROVISION_AMOUNT_V1:f64=String_Amount.parse().unwrap();
        let mut finnonenpacontents=FinnoneNPAStruct{
            FINAL_NPA_STATUS_BANK:finnone_npa_vec[4].to_string(),
            PROVISION_AMOUNT_V1,
        };
     
       
        finnone_npa_hashmap.insert(finnone_npa_vec[3].to_string(),finnonenpacontents);

    }
   
    let mut finnonemasterreader: Xlsx<_> = open_workbook(config_params.finnone_master()).expect("unable to read excel");
    if let Some(Ok(reader)) = finnonemasterreader.worksheet_range(config_params.finnone_master_sheet_name()) {
        for row in reader.rows().skip(1) {
            let Prod_code: String = row[0].to_string();
            let BALM_L2: String = row[6].to_string();
            finnone_master_hashmap.insert(Prod_code,BALM_L2);
        }
    }
    for (line_num, lines) in stgdetailscompanyreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.stg_company_details(),
                line_num + 1,
                error
            ),
        };
   
        let stg_company_details_vec = line.split('~').collect::<Vec<&str>>();
        stg_company_details_hashmap.insert(stg_company_details_vec[20].to_string(), stg_company_details_vec[14].to_string());
   
    }
    for (line_num, lines) in restructuremergedreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.restructure_merged(),
                line_num + 1,
                error
            ),
        };
        let restructure_merger_vec = line.split("~|").collect::<Vec<&str>>();
        restructure_merger_hashmap.insert(restructure_merger_vec[1].to_string(), "yes".to_string());

    }
    for (line_num, lines) in fncollateralreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.fn_collateral(),
                line_num + 1,
                error
            ),
        };
   
        let fn_collateral_vec = line.split('|').collect::<Vec<&str>>();
     
        fn_collateral_hashmap.insert(fn_collateral_vec[1].to_string(), fn_collateral_vec[6].to_string());

    }
   
    let mut non_security_exposure_file = File::open(config_params.stg_non_sec_exposure_fn()).expect("could not read stg_non_sec_exposure_fn  file");
    let mut nonsecurityexposurereader=BufReader::new(non_security_exposure_file);
    let mut output=File::create(config_params.output_file()).expect("Failed to create a file");
    for line in nonsecurityexposurereader.lines() {
        let non_security_exposure_file_line = line.expect("Failed to read contents of i1 file");
        let mut non_security_exposure_vec:Vec<String> = non_security_exposure_file_line.split("~|").map(|s| s.to_string()).collect();
        let mut account_id:String=non_security_exposure_vec[39].to_string();
        let mut customer_id:String=non_security_exposure_vec[20].to_string();
        let mut group_id:String=non_security_exposure_vec[38].to_string();
        let mut outstanding_amount:f64=(non_security_exposure_vec[10]).parse().unwrap();
        let mut outstanding_amount_lcy:f64=(non_security_exposure_vec[10]).parse().unwrap();
        let mut ccy:String=non_security_exposure_vec[19].to_string();
        let mut maturity_date:i32=(non_security_exposure_vec[30]).parse().unwrap();
        let mut gl_code="";
        //PAN number
        let mut pan_number=String::new();
        if finnone_extract_hashmap.contains_key(&non_security_exposure_vec[39].to_string()) 
        {
              pan_number=(finnone_extract_hashmap.get(&non_security_exposure_vec[39]).unwrap()).to_string();
              
        }
        else{
            pan_number="NA".to_string();
        }
        let mut customer_classification_code=String::new();
        if stg_company_details_hashmap.contains_key(&pan_number){
            customer_classification_code=(stg_company_details_hashmap.get(&pan_number).unwrap()).to_string();
        }
        else{
            customer_classification_code="NA".to_string();
        }
        //P11 - i
        let mut i=String::new();
        let mut original_string=non_security_exposure_vec[39].to_string();
        let mut len:usize=original_string.len();
        println!("the length is:{}",len);
        if len>2{
            let mut trimmed_system_id=&original_string[2..];
            println!("{}",trimmed_system_id);
            if finnone_npa_hashmap.contains_key(&trimmed_system_id.to_string())
            {
                let NPAStruct=finnone_npa_hashmap.get(&trimmed_system_id.to_string()).unwrap();
                i=NPAStruct.FINAL_NPA_STATUS_BANK.to_string();
            }
            else{
                i="NA".to_string(); 
            }
        }    
        else{
        i="NA".to_string(); 
        }

        //provisional amount
        let mut provisional_amount:f64=0.0;
        if len>2{
            let mut trimmed_system_id=&original_string[2..];
            if finnone_npa_hashmap.contains_key(&trimmed_system_id.to_string())
            {
            let NPAStruct1=finnone_npa_hashmap.get(&trimmed_system_id.to_string()).unwrap();
            provisional_amount=NPAStruct1.PROVISION_AMOUNT_V1;
            }
            else {
                provisional_amount=0.0;
            }
        }
        else {
           provisional_amount=0.0;
        }
        //proviosnal percentage
        let mut provisionalpercenatge:f64;
        if (outstanding_amount_lcy==0.0) || (provisional_amount==0.0)
        {
            provisionalpercenatge=0.0;

        }
        else{

            provisionalpercenatge = outstanding_amount_lcy/provisional_amount;
        }
        //Restructured flag
         let mut restructed_flag=String::new();
         if len>2{
         let mut trimmed_system_id=&original_string[2..];
         if restructure_merger_hashmap.contains_key(&trimmed_system_id.to_string())
          {
            restructed_flag="Y".to_string();
          }
         }
         else
         {
            restructed_flag="N".to_string();
         }

        //sanction date
        let mut sanction_date=non_security_exposure_vec[29].to_string();
        //product code
        let mut product_code=String::new();
        if finnone_master_hashmap.contains_key(&non_security_exposure_vec[24].to_string()){
            let mut Product_code=finnone_master_hashmap.get(&non_security_exposure_vec[24]).unwrap().to_string();
            product_code=Product_code.to_string();
        }
        else{
            product_code="NA".to_string();
        }
        //product description
        let mut product_description=String::new();
        if finnone_master_hashmap.contains_key(&non_security_exposure_vec[24].to_string()){
            product_description=non_security_exposure_vec[24].to_string();
        }
        else{
            product_description="NA".to_string();
            
        }
        let mut ltv="";
        let mut residential_mortgage_flag="";
        let mut sub_sector="";
        let mut group_level_total_exposure="";
        let mut rating_Agency="";
        let mut rating="";
        let mut bank_category="";
        let mut cet_ratio="";
        let mut guaranteed_by="";
        //Collateral
        let mut collateral=String::new();
        if finnone_master_hashmap.contains_key(&non_security_exposure_vec[24].to_string()){
            collateral=finnone_master_hashmap.get(&non_security_exposure_vec[24]).unwrap().to_string();
        }
        else {
            collateral="NA".to_string();
        }

        let mut final_output_text = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
        account_id,"|",customer_id,"|",group_id,"|",outstanding_amount,"|",outstanding_amount_lcy,"|",ccy,"|",maturity_date,"|",
        gl_code,"|",pan_number,"|",customer_classification_code,"|",i,"|",provisional_amount,"|",
        provisionalpercenatge.to_string(),"|",restructed_flag,"|",sanction_date,"|",
        product_code,"|",product_description,"|",ltv,"|",residential_mortgage_flag,"|",sub_sector,"|",group_level_total_exposure,"|",
        rating_Agency,"|",rating,"|",bank_category,"|",cet_ratio,"|",guaranteed_by,"|",collateral);
         output.write_all(&final_output_text.as_bytes()).expect("Failed to write to a file");
    }

}



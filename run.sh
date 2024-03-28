#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diagnostics.txt"
STG_NON_SEC_EXPOSURE_FN_FILE=$"test-bed/STG_NON_SEC_EXPOSURE_FN.dat"
FINNONE_EXTRACT_FILE=$"test-bed/FINNONE_EXTRACT.csv"
FINNONE_NPA_FILE=$"test-bed/FINNONE_NPA.dat"
FINNONE_MASTER_FILE=$"test-bed/Finnone_Master.xlsx"
STG_COMPANY_DETAILS_FILE=$"test-bed/STG_COMPANY_DETAILS.txt"
RESTRUCTURE_MERGED_FILE=$"test-bed/Restructure_Merged.txt"
FN_COLLATERAL_FILE=$"test-bed/FN_Collateral.txt"
OUTPUT_FILE=$"test-bed/Output.txt"

cargo run  -- \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file ${LOG_FILE} \
--log-level trace \
--stg-non-sec-exposure-fn ${STG_NON_SEC_EXPOSURE_FN_FILE} \
--finnone-extract ${FINNONE_EXTRACT_FILE} \
--finnone-npa ${FINNONE_NPA_FILE} \
--finnone-master ${FINNONE_MASTER_FILE} \
--stg-company-details ${STG_COMPANY_DETAILS_FILE} \
--restructure-merged ${RESTRUCTURE_MERGED_FILE} \
--fn-collateral ${FN_COLLATERAL_FILE} \
--output-file ${OUTPUT_FILE} \
--finnone-master-sheet-name "Sheet1" \
--diagnostics-flag false


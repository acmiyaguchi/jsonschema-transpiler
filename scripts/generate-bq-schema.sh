#!/bin/bash

# This script will generate corresponding BigQuery schemas from avro files that
# have been created with transpiled schemas. This is used to verify that the
# integration tests are written correctly.
#
# Assumptions:
# - The current project has a bucket that is named after the project itself
# - Avro schemas exist in avro/
# - JSON data exists under data/

DATASET="test_bq_schema"
PROJECT=$(gcloud config get-value project)

./scripts/mps-generate-avro-data.sh
gsutil cp avro-data/* gs://${PROJECT}/${DATASET}/

bq mk -f ${DATASET}

names=$(find avro/ -type file)
for name in ${names}; do 
    name=$(basename ${name} | cut -d. -f1)
    table=${name//-/_}

    bq load --source_format=AVRO \
        ${DATASET}.${table} \
        gs://${PROJECT}/${DATASET}/${name}.avro
    
    echo "original avro schema:"
    jq '.' avro/${name}.avro.json
    
    echo "bigquery schema:"
    bq show --format=json ${DATASET}.${table} | jq ".schema.fields"
done

tables=$(bq ls --format json ${DATASET} | jq -r '.[] | .tableReference.tableId')
for table in $tables; do
    bq rm -f ${DATASET}.${table}
done

bq rm -f ${DATASET}
gsutil rm -r gs://${PROJECT}/${DATASET}
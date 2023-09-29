#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE USER fhir WITH PASSWORD 'password';
	CREATE DATABASE medplum_r4 OWNER "fhir" lc_collate "C" lc_ctype "en_US.UTF-8" template "template0";
	CREATE DATABASE hapi_r4b OWNER "fhir" lc_collate "C" lc_ctype "en_US.UTF-8" template "template0";
	CREATE DATABASE hapi_r5 OWNER "fhir" lc_collate "C" lc_ctype "en_US.UTF-8" template "template0";
EOSQL
